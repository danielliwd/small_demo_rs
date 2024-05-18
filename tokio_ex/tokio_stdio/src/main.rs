use anyhow::{Context, Result};
use tokio::io::{stdin, stdout};
use tokio::io::{BufReader, BufWriter, AsyncBufReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() -> Result<()>{
    //io_copy().await?;
    io_copy_buf().await?;

    Ok(())
}

#[allow(dead_code)]
async fn io_copy() -> Result<()>{
    let mut stdin = stdin();
    let mut stdout = stdout();
    tokio::io::copy(&mut stdin, &mut stdout).await?;

    Ok(())
}

#[allow(dead_code)]
async fn io_copy_buf() -> Result<()>{
    let mut reader = BufReader::new(stdin()).lines();
    let mut writer = BufWriter::new(stdout());
    loop{
        tokio::select!{
            result = reader.next_line() => {
                match result{
                    Ok(Some(data)) => {
                        writer.write(data.as_bytes()).await;
                        writer.write(b"\n").await;
                        writer.flush().await;
                    },
                    Err(err) => {
                        println!("{err}");
                        break;
                    },
                    _ => {},
                }
            }
        }
    }

    Ok(())
}
