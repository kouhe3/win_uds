use std::io;

use futures::{AsyncReadExt, AsyncWriteExt};
use win_uds::net::{AsyncListener, AsyncStream, UnixListener, UnixStream};

#[tokio::test]
async fn async_echo() -> io::Result<()> {
    let tmp = std::env::temp_dir();
    let path = tmp.join("test_async_echo.sock");
    let _ = std::fs::remove_file(&path);

    let listener = AsyncListener::bind(&path)?;

    let srv = tokio::spawn(async move {
        let (mut s, _addr) = listener.accept().await.unwrap();
        let mut buf = [0u8; 1024];
        let n = s.read(&mut buf).await.unwrap();
        s.write_all(&buf[..n]).await.unwrap();
        s.close().await.unwrap();
    });

    let mut cli = AsyncStream::connect(&path).await?;
    cli.write_all(b"Hello").await?;
    cli.flush().await?;

    // Read echo response and verify
    let mut buf = [0u8; 5];
    cli.read_exact(&mut buf).await?;
    assert_eq!(&buf, b"Hello");

    srv.await.unwrap();

    let _ = std::fs::remove_file(&path);
    Ok(())
}

#[tokio::test]
async fn async_bidirectional() -> io::Result<()> {
    let tmp = std::env::temp_dir();
    let path = tmp.join("test_async_bidir.sock");
    let _ = std::fs::remove_file(&path);

    let listener = AsyncListener::bind(&path)?;

    let path_clone = path.clone();
    let client_handle = tokio::spawn(async move {
        let mut client = AsyncStream::connect(&path_clone).await.unwrap();
        client.write_all(b"ping").await.unwrap();
        client.flush().await.unwrap();

        let mut buf = [0u8; 4];
        client.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"pong");
    });

    let (mut server, _addr) = listener.accept().await?;
    let mut buf = [0u8; 4];
    server.read_exact(&mut buf).await?;
    assert_eq!(&buf, b"ping");

    server.write_all(b"pong").await?;
    server.flush().await?;

    client_handle.await.unwrap();
    let _ = std::fs::remove_file(&path);
    Ok(())
}

#[tokio::test]
async fn async_multiple_clients() -> io::Result<()> {
    let tmp = std::env::temp_dir();
    let path = tmp.join("test_async_multi.sock");
    let _ = std::fs::remove_file(&path);

    let listener = AsyncListener::bind(&path)?;

    // Spawn 3 clients
    let mut client_handles = Vec::new();
    for i in 0..3u8 {
        let path_clone = path.clone();
        client_handles.push(tokio::spawn(async move {
            let mut client = AsyncStream::connect(&path_clone).await.unwrap();
            client.write_all(&[i]).await.unwrap();
            client.flush().await.unwrap();

            let mut buf = [0u8; 1];
            client.read_exact(&mut buf).await.unwrap();
            assert_eq!(buf[0], i + 10);
        }));
    }

    // Accept and handle 3 connections
    for _ in 0..3 {
        let (mut server, _addr) = listener.accept().await?;
        let mut buf = [0u8; 1];
        server.read_exact(&mut buf).await?;
        server.write_all(&[buf[0] + 10]).await?;
        server.flush().await?;
    }

    for handle in client_handles {
        handle.await.unwrap();
    }

    let _ = std::fs::remove_file(&path);
    Ok(())
}

#[tokio::test]
async fn async_try_clone_stream() -> io::Result<()> {
    let tmp = std::env::temp_dir();
    let path = tmp.join("test_async_clone.sock");
    let _ = std::fs::remove_file(&path);

    let listener = AsyncListener::bind(&path)?;

    let path_clone = path.clone();
    let client_handle = tokio::spawn(async move {
        let client = AsyncStream::connect(&path_clone).await.unwrap();
        let mut client2 = client.try_clone().unwrap();
        // Write using the clone
        client2.write_all(b"from clone").await.unwrap();
        client2.flush().await.unwrap();
    });

    let (mut server, _addr) = listener.accept().await?;
    let mut buf = [0u8; 10];
    server.read_exact(&mut buf).await?;
    assert_eq!(&buf, b"from clone");

    client_handle.await.unwrap();
    let _ = std::fs::remove_file(&path);
    Ok(())
}

#[tokio::test]
async fn async_try_clone_listener() -> io::Result<()> {
    let tmp = std::env::temp_dir();
    let path = tmp.join("test_async_clone_listener.sock");
    let _ = std::fs::remove_file(&path);

    let listener = AsyncListener::bind(&path)?;
    let listener2 = listener.try_clone()?;

    let path_clone = path.clone();
    let client_handle = tokio::spawn(async move {
        let mut client = AsyncStream::connect(&path_clone).await.unwrap();
        client.write_all(b"hello").await.unwrap();
        client.flush().await.unwrap();
    });

    // Accept from the cloned listener
    let (mut server, _addr) = listener2.accept().await?;
    let mut buf = [0u8; 5];
    server.read_exact(&mut buf).await?;
    assert_eq!(&buf, b"hello");

    client_handle.await.unwrap();
    let _ = std::fs::remove_file(&path);
    Ok(())
}

#[tokio::test]
async fn async_connect_nonexistent() {
    let tmp = std::env::temp_dir();
    let path = tmp.join("nonexistent_socket.sock");
    let _ = std::fs::remove_file(&path);

    let result = AsyncStream::connect(&path).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn async_concurrent_accept() -> io::Result<()> {
    let tmp = std::env::temp_dir();
    let path = tmp.join("test_async_concurrent.sock");
    let _ = std::fs::remove_file(&path);

    let listener = AsyncListener::bind(&path)?;

    // Spawn 3 clients
    let mut client_handles = Vec::new();
    for i in 0..3u8 {
        let path_clone = path.clone();
        client_handles.push(tokio::spawn(async move {
            let mut client = AsyncStream::connect(&path_clone).await.unwrap();
            client.write_all(&[i]).await.unwrap();
            client.flush().await.unwrap();

            let mut buf = [0u8; 1];
            client.read_exact(&mut buf).await.unwrap();
            assert_eq!(buf[0], i + 10);
        }));
    }

    // Accept concurrently using separate tasks
    let mut accept_handles = Vec::new();
    for _ in 0..3 {
        let listener_clone = listener.try_clone()?;
        accept_handles.push(tokio::spawn(async move {
            let (mut server, _addr) = listener_clone.accept().await.unwrap();
            let mut buf = [0u8; 1];
            server.read_exact(&mut buf).await.unwrap();
            server.write_all(&[buf[0] + 10]).await.unwrap();
            server.flush().await.unwrap();
        }));
    }

    for handle in client_handles {
        handle.await.unwrap();
    }
    for handle in accept_handles {
        handle.await.unwrap();
    }

    let _ = std::fs::remove_file(&path);
    Ok(())
}

// Sync tests remain unchanged
use io::Read;

#[test]
fn no_blocking_test() {
    let tmp = std::env::temp_dir();
    let path = tmp.join("test_no_blocking.sock");
    let _ = std::fs::remove_file(&path);

    let l = UnixListener::bind(&path).unwrap();
    l.set_nonblocking(true).unwrap();
    let mut s = UnixStream::connect(&path).unwrap();
    s.set_nonblocking(true).unwrap();
    let mut buf = [0u8; 1024];
    assert_eq!(
        s.read(&mut buf).unwrap_err().kind(),
        io::ErrorKind::WouldBlock
    );
    let _ = std::fs::remove_file(&path);
}
