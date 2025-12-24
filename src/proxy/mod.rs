use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn start_pure_std_proxy(listen_addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(listen_addr)?;
    println!("纯标准库 SOCKS5 服务监听中: {}", listen_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(client) => {
                // 每个连接开启一个独立线程
                println!("接入新连接: {}", client.peer_addr()?);
                thread::spawn(move || {
                    if let Err(e) = handle_client(client) {
                        // eprintln!("连接错误: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("接入失败: {}", e),
        }
    }
    Ok(())
}

fn handle_client(mut client: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 512];

    // 1. 握手
    client.read_exact(&mut buf[..2])?;
    let nmethods = buf[1] as usize;
    client.read_exact(&mut buf[..nmethods])?;
    client.write_all(&[0x05, 0x00])?;

    // 2. 请求
    client.read_exact(&mut buf[..4])?;
    let atyp = buf[3];
    let target_addr = match atyp {
        0x01 => {
            // IPv4
            client.read_exact(&mut buf[..4])?;
            format!("{}.{}.{}.{}", buf[0], buf[1], buf[2], buf[3])
        }
        0x03 => {
            // Domain
            let len = client.read_u8()? as usize; // 注意：标准库没read_u8，下文有辅助函数
            client.read_exact(&mut buf[..len])?;
            String::from_utf8_lossy(&buf[..len]).to_string()
        }
        _ => return Ok(()),
    };

    let mut port_buf = [0u8; 2];
    client.read_exact(&mut port_buf)?;
    let port = u16::from_be_bytes(port_buf);

    // 3. 连接目标并响应成功
    let mut target = TcpStream::connect(format!("{}:{}", target_addr, port))?;
    client.write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])?;

    // 4. 数据透传 (双向拷贝)
    let mut client_clone = client.try_clone()?;
    let mut target_clone = target.try_clone()?;

    // 开启一个子线程负责写，主线程负责读，实现双工
    let t1 = thread::spawn(move || io::copy(&mut client_clone, &mut target));
    let _ = io::copy(&mut target_clone, &mut client);

    Ok(())
}

// 简单的辅助扩展
trait ReadExt: Read {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut b = [0u8; 1];
        self.read_exact(&mut b)?;
        Ok(b[0])
    }
}
impl<R: Read + ?Sized> ReadExt for R {}
