use std::{io, net::Shutdown};

use windows::Win32::Networking::WinSock::{
    self, AF_UNIX, INVALID_SOCKET, SEND_RECV_FLAGS, SOCK_STREAM, SOCKADDR, SOCKET, SOCKET_ERROR,
    WSA_FLAG_OVERLAPPED,
};

use crate::common::{startup, wsa_error};
// wrap Winsock method like std
pub struct Socket(pub SOCKET);

impl Socket {
    pub fn new() -> io::Result<Self> {
        unsafe {
            startup()?;
            match WinSock::socket(AF_UNIX as _, SOCK_STREAM, 0)? {
                INVALID_SOCKET => Err(wsa_error()),
                s => Ok(Self(s)),
            }
        }
    }
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            match WinSock::send(self.0, buf, SEND_RECV_FLAGS(0)) {
                SOCKET_ERROR => Err(wsa_error()),
                len => Ok(len as _),
            }
        }
    }
    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            match WinSock::recv(self.0, buf, SEND_RECV_FLAGS(0)) {
                0 => Err(io::Error::other("Connection closed")),
                len if len > 0 => Ok(len as _),
                _ => Err(wsa_error()),
            }
        }
    }
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        use WinSock::{SD_BOTH, SD_RECEIVE, SD_SEND};
        let shutdown_how = match how {
            Shutdown::Read => SD_RECEIVE,
            Shutdown::Write => SD_SEND,
            Shutdown::Both => SD_BOTH,
        };
        unsafe {
            match WinSock::shutdown(self.0, shutdown_how) {
                0 => Ok(()),
                _ => Err(wsa_error()),
            }
        }
    }
    pub fn accept(
        &self,
        addr: Option<*mut SOCKADDR>,
        addrlen: Option<*mut i32>,
    ) -> io::Result<Socket> {
        unsafe {
            match WinSock::accept(self.0, addr, addrlen)? {
                INVALID_SOCKET => Err(wsa_error()),
                s => Ok(Socket(s)),
            }
        }
    }
}
