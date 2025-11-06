use std::{io, mem, path::Path};

use windows::Win32::Networking::WinSock::{self, SOCKADDR_UN, SOCKET_ERROR};

use crate::{
    common::{startup, wsa_error},
    net::{Socket, SocketAddr, UnixStream, socketaddr_un},
};
pub struct UnixListener(Socket);

impl UnixListener {
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        unsafe {
            startup()?;
            let s = Socket::new()?;
            let (addr, len) = socketaddr_un(path.as_ref())?;
            match WinSock::bind(s.0, &addr as *const _ as *const _, len) {
                SOCKET_ERROR => Err(wsa_error()),
                _ => Ok(Self(s)),
            }
        }
    }
    pub fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        unsafe {
            let mut addr: SOCKADDR_UN = mem::zeroed();
            let addrlen = size_of::<SOCKADDR_UN>() as _;
            let s = self
                .0
                .accept(Some(&mut addr as *mut _ as *mut _), Some(addrlen as *mut _))?;
            Ok((UnixStream(s), SocketAddr { addr, addrlen }))
        }
    }
}
