mod listener;
mod stream;
pub use listener::*;
pub use socket2::SockAddr;
use socket2::Socket;
pub use stream::*;
mod async_uds;
pub use async_uds::*;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::{io, path::Path};

/// Validates that a path doesn't contain null bytes.
/// Returns an error if null bytes are found, matching Unix behavior.
fn validate_path<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let os_str: &OsStr = path.as_ref().as_os_str();
    for wchar in os_str.encode_wide() {
        if wchar == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "paths must not contain null bytes",
            ));
        }
    }
    Ok(())
}
