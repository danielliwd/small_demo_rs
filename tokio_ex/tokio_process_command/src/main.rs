#![allow(unused_imports)]
use tokio::io::{BufReader, AsyncBufReadExt, BufWriter, AsyncWriteExt, AsyncReadExt};
use tokio::process::{Command, ChildStdin, ChildStdout};

use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let _ = tokio::join!(
        //command_ls(),
        // command_perl_io_filter(),
        command_cat_io(),
    );

    Ok(())
}

#[allow(dead_code)]
async fn command_ls() -> Result<(), Box<dyn std::error::Error>>{
    log::info!("command ls");
    let output = tokio::process::Command::new("ls")
        .args("-l -a".split(' '))
        .kill_on_drop(true)
        .output()
        .await
        .expect("wrong");

    println!("{}", String::from_utf8(output.stdout)?);

    log::info!("command ls done");
    Ok(())

}

#[allow(dead_code)]
async fn command_perl_io_filter() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("io filter");
    // spawn 启动子进程
    let mut child = tokio::process::Command::new("perl")
        .args(&["-ne", r#"BEGIN{$|=1}print if /^#/"#]) // 添加命令参数, perl输出带缓存不会自动刷出，必须强制flush,才能实现异步读写
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;

    // 行缓存的stdout bufreader
    let mut stdout_reader = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut stderr_reader = BufReader::new(child.stderr.take().unwrap()).lines();
    let mut writer = BufWriter::new(child.stdin.take().expect("Fail to get stdin"));

    // 每秒输入一行
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        for i in 1..10{
            interval.tick().await;
            let _ = tokio::time::sleep(Duration::from_secs(2));
            let msg = format!("{}msg: {}\n", if i%2==0 {"#"}else{""}, i);
            println!("--- {}", msg);
            writer.write_all(msg.as_bytes()).await.unwrap();
            writer.flush().await.unwrap();
        }
    });

    'brk:
    loop {

        tokio::select! {
            result = stdout_reader.next_line() => {
                match result {
                    Ok(Some(line)) => println!("Stdout: {}", line),
                    Err(e) => {
                        log::error!("{e}");
                        break 'brk;

                    },
                    _ => (),
                }
            }
            result = stderr_reader.next_line() => {
                match result {
                    Ok(Some(line)) => println!("Stderr: {}", line),
                    Err(e) => {
                        log::error!("{e}");
                        break 'brk;

                    },
                    _ => (),
                }
            }
            result = child.wait() => {
                match result {
                    Ok(exit_code) => println!("Child process exited with {}", exit_code),
                    Err(e) => {
                        log::error!("{e}");
                        break 'brk;
                    },
                }
                break // child process exited
            }
        };
    };

    log::info!("command cat io filter done");
    Ok(())

}


#[allow(dead_code)]
async fn command_cat_io() -> Result<(), Box<dyn std::error::Error>> {
    // 创建命令对象
    let mut child = Command::new("cat")
        .stdin(std::process::Stdio::piped()) // 允许标准输入
        .stdout(std::process::Stdio::piped()) // 允许标准输出
        .kill_on_drop(true)
        .spawn()?;

    // 获取子进程的标准输入和标准输出
    let mut stdout = child.stdout.take().expect("Failed to open stdout");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    let mut counter = 0;
    let mut buf = [0u8; 1024];
    let mut interval = tokio::time::interval(Duration::from_secs(1));

    loop {

        tokio::select!{
            _ = interval.tick() => {
                counter += 1;
                if counter < 10{
                    let msg = format!("{}\n",counter);
                    let _ = stdin.write_all(msg.as_bytes()).await;
                    let _ = stdin.flush().await;
                } else{
                    let _ = stdin.shutdown().await;
                }
            }
        len = stdout.read(&mut buf[..]) => {
            match len{
                Ok(len) => {
                    if len != 0{
                        print!("output:{}", std::str::from_utf8(&buf[..len]).unwrap());
                    }
                },
                Err(e) => {
                    println!("{e}");
                    break;
                }
            }}

        result = child.wait() => {
                // 等待子进程退出
           match result {
                    Ok(exit_code) => println!("Child process exited with {}", exit_code),
                    Err(e) => {
                        log::error!("{e}");
                        break;
                    },
           }
           break; // child process exited
           }
        }
    }


    Ok(())
}

