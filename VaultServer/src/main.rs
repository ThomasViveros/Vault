use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};


async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            // client closed the connection
            break;
        }

        println!("Recieved {} bytes from {}", n, stream.peer_addr()?);
        // Echo back what we received
        stream.write_all(&buf[..n]).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9418").await?;
    eprintln!("listening on {}", listener.local_addr()?);

    loop {
        let (stream, addr) = listener.accept().await?;
        eprintln!("accepted {addr}");
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("client error: {e}");
            }
        });
    }
}

/* Steps:

1. We need a way to receive data from a source.
2. Then we need a way to take that data and apply that to the current data.
3. Then we need to make backups


*/