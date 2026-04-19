#[cfg(target_os = "android")]
pub mod android_util;
pub(crate) mod crypto;
pub(crate) mod fs;
pub(crate) mod logging;
pub(crate) mod protocol;

pub(crate) use crypto::blake2b_u64;
pub use crypto::get_random_range;
pub(crate) use crypto::handler as crypto_handler;
pub(crate) use fs::change_file_ownership;
pub(crate) use fs::resolve_path;
pub(crate) use logging::info;
pub(crate) use protocol::client_data;
pub(crate) use protocol::parser as data_parser;

pub(crate) fn now_nanos() -> anyhow::Result<u128> {
    use anyhow::Context;
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .with_context(|| "system clock before epoch")?
        .as_nanos())
}
