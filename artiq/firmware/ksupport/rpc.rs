use crate::rpc_send_async;
use cslice::AsCSlice;
use rpc_data::service_id;

/// Store a i32 result on the host.
///
/// This function emits an async RPC call to the host.
///
/// # Arguments
///
/// * `identifier` - identifier for this result.
/// * `result` - result value to store.
pub fn store_result_i32(identifier: i32, result: i32) {
    let tag = &(b"ii:n")[..];
    let args = [
        &identifier as *const i32 as *const (),
        &result as *const i32 as *const (),
    ];

    rpc_send_async(
        service_id::STORE_RESULT_I32,
        &tag.as_c_slice(),
        args.as_ptr(),
    );
}
