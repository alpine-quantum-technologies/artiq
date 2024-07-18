#![no_std]

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[cfg(feature = "alloc")]
extern crate alloc;
extern crate cslice;
#[cfg(feature = "log")]
#[macro_use]
extern crate log;

extern crate byteorder;
extern crate dyld;
extern crate eh;
extern crate io;

// Internal protocols.
pub mod drtioaux_proto;
pub mod kernel_proto;

// External protocols.
#[cfg(feature = "alloc")]
pub mod analyzer_proto;
#[cfg(feature = "alloc")]
pub mod mgmt_proto;
#[cfg(feature = "alloc")]
pub mod moninj_proto;
pub mod rpc_proto;
#[cfg(feature = "alloc")]
pub mod session_proto;
