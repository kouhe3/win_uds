use crate::net::{SockAddr, Socket, UnixStream};
use socket2::{Domain, Type};
use std::{
    io,
    ops::{Deref, DerefMut},
    os::windows::io::{AsRawSocket, AsSocket, IntoRawSocket},
    path::Path,
};
pub struct UnixListener(pub Socket);

impl UnixListener {
    /// Creates a new `UnixListener` bound to the specified socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///
    /// let listener = match UnixListener::bind("/path/to/the/socket") {
    ///     Ok(sock) => sock,
    ///     Err(e) => {
    ///         println!("Couldn't connect: {:?}", e);
    ///         return
    ///     }
    /// };
    /// ```
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let addr = SockAddr::unix(path)?;
        Self::bind_addr(&addr)
    }
    pub fn bind_addr(socket_addr: &SockAddr) -> io::Result<Self> {
        let s = Socket::new(Domain::UNIX, Type::STREAM, None)?;
        s.bind(socket_addr)?;
        s.listen(5)?;
        Ok(Self(s))
    }
    /// Accepts a new incoming connection to this listener.
    ///
    /// This function will block the calling thread until a new Unix connection
    /// is established. When established, the corresponding [`UnixStream`] and
    /// the remote peer's address will be returned.
    ///
    /// [`UnixStream`]: struct.UnixStream.html
    ///
    /// # Examples
    ///
    /// ```no_run
    ///
    /// let listener = UnixListener::bind("/path/to/the/socket").unwrap();
    ///
    /// match listener.accept() {
    ///     Ok((socket, addr)) => println!("Got a client: {:?}", addr),
    ///     Err(e) => println!("accept function failed: {:?}", e),
    /// }
    /// ```
    pub fn accept(&self) -> io::Result<(UnixStream, SockAddr)> {
        let (s, addr) = self.0.accept()?;
        Ok((UnixStream(s), addr))
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// The returned `UnixListener` is a reference to the same socket that this
    /// object references. Both handles can be used to accept incoming
    /// connections and options set on one listener will affect the other.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///
    /// let listener = UnixListener::bind("/path/to/the/socket").unwrap();
    ///
    /// let listener_copy = listener.try_clone().expect("Couldn't clone socket");
    /// ```
    pub fn try_clone(&self) -> io::Result<UnixListener> {
        self.0.try_clone().map(UnixListener)
    }
}
impl AsSocket for UnixListener {
    fn as_socket(&self) -> std::os::windows::prelude::BorrowedSocket<'_> {
        self.0.as_socket()
    }
}
impl AsRawSocket for UnixListener {
    fn as_raw_socket(&self) -> std::os::windows::prelude::RawSocket {
        self.0.as_raw_socket()
    }
}
impl IntoRawSocket for UnixListener {
    fn into_raw_socket(self) -> std::os::windows::prelude::RawSocket {
        self.0.into_raw_socket()
    }
}
impl Deref for UnixListener {
    type Target = Socket;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for UnixListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
