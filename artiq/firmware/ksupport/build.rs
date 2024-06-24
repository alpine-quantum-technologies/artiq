use std::path::Path;
use std::{env, fs};

fn main() {
    let buildinc_dir = env::var("BUILDINC_DIRECTORY").unwrap();
    let ddb_path = Path::new(&buildinc_dir)
        .join("generated")
        .join("device_db.py");
    println!("cargo:rerun-if-changed={}", ddb_path.to_str().unwrap());

    let ddb = fs::read_to_string(&ddb_path).unwrap();
    let _ = ddb_parser::parse(&ddb).unwrap();
    // TODO: emit peripherals instantiation code

    build_misoc::cfg();
    println!("cargo:rustc-cfg={}", "ksupport");
}
