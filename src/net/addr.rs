use std::{io, path::Path};

use windows::Win32::Networking::WinSock::{ADDRESS_FAMILY, AF_UNIX, SOCKADDR_UN};

pub fn socketaddr_un(path: &Path) -> io::Result<(SOCKADDR_UN, i32)> {
    // let bytes = path.as_os_str().as_encoded_bytes();
    let mut sockaddr = SOCKADDR_UN::default();
    // Winsock2 expects 'sun_path' to be a Win32 UTF-8 file system path
    let bytes = path.to_str().map(|s| s.as_bytes()).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "path contains invalid characters",
        )
    })?;

    if bytes.contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "paths may not contain interior null bytes",
        ));
    }

    if bytes.len() >= sockaddr.sun_path.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path must be shorter than SUN_LEN",
        ));
    }
    let src_i8 = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i8, bytes.len()) };
    sockaddr.sun_family = ADDRESS_FAMILY(AF_UNIX);
    sockaddr.sun_path[..src_i8.len()].copy_from_slice(src_i8);
    let socklen = size_of::<SOCKADDR_UN>() as _;
    Ok((sockaddr, socklen))
}
#[derive(Debug)]
pub struct SocketAddr {
    pub addr: SOCKADDR_UN,
    pub addrlen: i32,
}
