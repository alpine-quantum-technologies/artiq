use thiserror::Error;

use indoc::indoc;
use pyo3::types::{PyDict, PyModule};
use std::collections::HashMap;

pub mod core;
pub mod devices;
mod i2c;

pub use devices::Device;
pub type DeviceDb = HashMap<String, Device>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("type error: {0}")]
    Type(String),
    #[error(transparent)]
    Python(#[from] pyo3::PyErr),
    #[error(transparent)]
    Parse(#[from] serde_json::Error),
}

pub fn parse(python_code: &str) -> Result<DeviceDb, Error> {
    pyo3::Python::with_gil(|py| {
        let module = PyModule::from_code(py, python_code, "device_db.py", "device_db")?;
        let ddb = module
            .getattr("device_db")?
            .downcast::<PyDict>()
            .map_err(|_| Error::Type("device_db is not a PyDict".into()))?;

        let pp_module =
            PyModule::from_code(py, PREPROCESSING_CODE, "preprocessing.py", "preprocessing")?;

        let json: String = pp_module.getattr("preprocess")?.call1((ddb,))?.extract()?;
        Ok(serde_json::from_str(&json)?)
    })
}

const PREPROCESSING_CODE: &str = indoc! {r#"
      import json

      def add_qual_class(entry: dict) -> dict:
          if (module := entry.get("module")) and (class_ := entry.get("class")):
              entry["_qualclass"] = f"{module}.{class_}"
          return entry

      def preprocess(ddb: dict) -> str:
          return json.dumps({key: add_qual_class(value) for key, value in ddb.items()})
      "#
};
