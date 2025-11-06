use std::{io, path::Path, ptr};

use windows::Win32::Networking::WinSock::{ADDRESS_FAMILY, AF_UNIX, SOCKADDR_UN};

pub fn socketaddr_un(path: impl AsRef<Path>) -> io::Result<SOCKADDR_UN> {
    let bytes = path.as_ref().as_os_str().as_encoded_bytes();
    let mut addr = SOCKADDR_UN::default();
    if bytes.len() > addr.sun_path.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "path too long"));
    }
    addr.sun_family = ADDRESS_FAMILY(AF_UNIX);
    unsafe { ptr::copy_nonoverlapping(bytes.as_ptr() as _, &mut addr.sun_path, bytes.len()) };
    Ok(addr)
}
pub struct SocketAddr {
    pub addr: SOCKADDR_UN,
    pub addrlen: i32,
}
