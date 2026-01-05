//! Async Unix domain socket types using async-io for Windows.
//!
//! These types use `futures_io::{AsyncRead, AsyncWrite}` traits for runtime-agnostic async I/O.
//! Use `tokio_util::compat` to adapt these types for tokio.

use crate::net::{UnixListener, UnixStream};
use async_io::Async;
use futures_io::{AsyncRead, AsyncWrite};
use socket2::SockAddr;
use std::{
    io,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

/// Async Unix domain socket stream.
///
/// Implements `futures_io::{AsyncRead, AsyncWrite}` for runtime-agnostic async I/O.
/// For tokio, use `tokio_util::compat::FuturesAsyncReadCompatExt`.
pub struct AsyncStream(Async<UnixStream>);

impl AsyncStream {
    /// Connects to a Unix domain socket at the given path.
    pub async fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let stream = UnixStream::connect(path)?;
        let async_stream = Async::new(stream)?;
        Ok(Self(async_stream))
    }

    /// Connects to a Unix domain socket at the given address.
    pub async fn connect_addr(socket_addr: &SockAddr) -> io::Result<Self> {
        let stream = UnixStream::connect_addr(socket_addr)?;
        let async_stream = Async::new(stream)?;
        Ok(Self(async_stream))
    }

    /// Creates a new independently owned handle to the underlying socket.
    pub fn try_clone(&self) -> io::Result<Self> {
        let cloned = self.0.get_ref().try_clone()?;
        let async_stream = Async::new(cloned)?;
        Ok(Self(async_stream))
    }

    /// Returns a reference to the inner stream.
    pub fn get_ref(&self) -> &UnixStream {
        self.0.get_ref()
    }
}

impl AsyncRead for AsyncStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl AsyncWrite for AsyncStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}

/// Async Unix domain socket listener.
pub struct AsyncListener(Async<UnixListener>);

impl AsyncListener {
    /// Creates a new listener bound to the given path.
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let listener = UnixListener::bind(path)?;
        let async_listener = Async::new(listener)?;
        Ok(Self(async_listener))
    }

    /// Creates a new listener bound to the given address.
    pub fn bind_addr(socket_addr: &SockAddr) -> io::Result<Self> {
        let listener = UnixListener::bind_addr(socket_addr)?;
        let async_listener = Async::new(listener)?;
        Ok(Self(async_listener))
    }

    /// Accepts a new incoming connection.
    pub async fn accept(&self) -> io::Result<(AsyncStream, SockAddr)> {
        loop {
            match self.0.get_ref().accept() {
                Ok((stream, addr)) => {
                    let async_stream = Async::new(stream)?;
                    return Ok((AsyncStream(async_stream), addr));
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    self.0.readable().await?;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Creates a new independently owned handle to the underlying listener.
    pub fn try_clone(&self) -> io::Result<Self> {
        let cloned = self.0.get_ref().try_clone()?;
        let async_listener = Async::new(cloned)?;
        Ok(Self(async_listener))
    }

    /// Returns a reference to the inner listener.
    pub fn get_ref(&self) -> &UnixListener {
        self.0.get_ref()
    }
}
