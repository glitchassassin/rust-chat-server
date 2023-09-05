use std::{fs, time::Duration};

use async_std::{
    io::{Read, Write, WriteExt},
    net::TcpListener,
    prelude::*,
    task::{self, spawn},
};
use futures::StreamExt;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).await;

    let (status_line, file) = if buffer.starts_with(b"GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", "static/index.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1") {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "static/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "static/404.html")
    };

    let contents = fs::read_to_string(file).unwrap();
    let response = format!("{status_line}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

#[cfg(test)]
mod test {
    use futures::io::Error;
    use std::cmp::min;
    use std::pin::Pin;

    use futures::task::{Context, Poll};

    use super::*;
    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<Result<usize, Error>> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }

    impl Write for MockTcpStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);

            Poll::Ready(Ok(buf.len()))
        }
        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    }

    impl Unpin for MockTcpStream {}

    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;

        let expected_contents = fs::read_to_string("static/index.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
}
