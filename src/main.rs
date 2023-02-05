use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8001").await?;
    println!("Connection established: {}", stream.local_addr().unwrap());

    let (reader, writer) = stream.split();
    let mut input_buffer = String::new();
    let mut output_buffer = String::new();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);
    let mut stdin = BufReader::new(stdin());
    loop {
        tokio::select! {
            _inp = stdin.read_line(&mut input_buffer) => {
                if input_buffer.trim() == "exit" {
                    break;
                }
                writer.write_all(input_buffer.as_bytes()).await?;
                writer.flush().await?;
                input_buffer.clear();
            }
            _out = reader.read_line(&mut output_buffer) => {
                print!("{output_buffer}");
                output_buffer.clear();
            }
        }
    }
    Ok(())
}
