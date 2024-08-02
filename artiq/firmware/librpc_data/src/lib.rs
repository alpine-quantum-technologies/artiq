#![no_std]

/// Known RPC service IDs.
pub mod service_id {
    /// Store a i32 result on the host.
    /// async fn(identifier: i32, result: i32)
    pub const STORE_RESULT_I32: u32 = 0x100;
}
