use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:9418").await?;
    let (mut reader, mut writer) = stream.into_split();

    let stdin = io::BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    let mut buf = [0u8; 4096];

    loop {
        if let Some(line) = lines.next_line().await? {
            writer.write_all(line.as_bytes()).await?;

            let n = reader.read(&mut buf).await?;
            if n == 0 {
                println!("server closed connection");
                break;
            }

            println!("server: {}", String::from_utf8_lossy(&buf[..n]));
        } else {
            break;
        }
    }

    Ok(())
}

fn add(){

}

fn commit(){

}

fn push(){

}

fn pull(){

}