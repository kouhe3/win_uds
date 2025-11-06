use std::{io, mem, path::Path};

use windows::Win32::Networking::WinSock::{
    self, SOCKADDR_UN, SOCKET_ERROR,
};

use crate::{
    common::{startup, wsa_error},
    net::{Socket, addr::SocketAddr, socketaddr_un, stream::UnixStream},
};
pub struct UnixListener(Socket);

impl UnixListener {
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        unsafe {
            startup()?;
            let s = Socket::new()?;
            let addr = socketaddr_un(path)?;
            match WinSock::bind(
                s.0,
                &addr as *const _ as *const _,
                size_of::<SOCKADDR_UN>() as _,
            ) {
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
                .accept(Some(&mut addr as *mut _ as *mut _), Some(addrlen as _))?;
            Ok((UnixStream(s), SocketAddr { addr, addrlen }))
        }
    }
}
