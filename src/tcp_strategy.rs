use crate::errors::SocketInitErrors;
use crate::SERVER_ADDR;
use log::error;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::Deref;
use std::ptr::copy;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpSocket};
use tokio::sync::{Mutex, MutexGuard};

static SERVER_TCP_SOCKET: Lazy<Mutex<Option<Arc<TcpListener>>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct TcpStrategy {
    pub stateless: AtomicBool,
    pub(crate) state_holder: Arc<Mutex<Box<HashMap<Arc<&'static str>, AtomicBool>>>>,
}
pub(crate) trait TcpConnection {
    async fn initialize_socket(&self) -> Result<(), SocketInitErrors>;
    async fn bind_socket(&self);
}

impl Clone for TcpStrategy {
    fn clone(&self) -> Self {
        TcpStrategy {
            stateless: AtomicBool::new(self.stateless.fetch_or(true, Ordering::Acquire)),
            state_holder: self.state_holder.clone(),
        }
    }
}

impl Default for TcpStrategy {
    fn default() -> Self {
        TcpStrategy {
            stateless: AtomicBool::new(true),
            state_holder: Arc::new(Mutex::new(Box::new(HashMap::new()))),
        }
    }
}

impl TcpConnection for TcpStrategy {
    async fn initialize_socket(&self) -> Result<(), SocketInitErrors> {
        let socket = TcpSocket::new_v4()?;

        socket.set_linger(Some(Duration::new(5, 0)))?;
        socket.set_reuseaddr(true)?;

        socket.bind(SocketAddr::from_str(&SERVER_ADDR)?)?;

        let listener = socket.listen(1)?;

        let mut socket_guard = SERVER_TCP_SOCKET.lock().await;
        *socket_guard = Some(Arc::new(listener));

        if !self.stateless.fetch_or(true, Ordering::Acquire) {
            let mut bind_ = self.state_holder.lock().await;
            bind_.insert(Arc::new(SERVER_ADDR), AtomicBool::new(true));
        }
        Ok(())
    }

    async fn bind_socket(&self) {
        if let Some(ref _socket) = *SERVER_TCP_SOCKET.lock().await {
            let _socket_cp = Arc::clone(_socket);
            tokio::spawn(async move {
                listen_tcp(_socket_cp).await;
            })
            .await
            .unwrap();
            println!("{:#?}", self.state_holder);
        } else {
            error!("Socket is not initialized");
        }
    }
}
async fn send() {}

async fn listen_tcp(serv: Arc<TcpListener>) {
    println!("Connected service");

    match serv.accept().await {
        Ok((mut _socket, addr)) => {
            println!("New client: {:?}", addr);

            loop {
                let mut buf = vec![0; 16];
                match _socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Client {:?} closed the connection", addr);
                        break;
                    }
                    Ok(n) => {
                        println!("Received {} bytes from {:?}", n, addr);

                        if let Err(e) = _socket.write_all(&buf[0..n]).await {
                            println!("Failed to send response: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Error reading from socket: {:?}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => println!("couldn't get client: {:?}", e),
    }
}
