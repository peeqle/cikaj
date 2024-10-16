use crate::errors::SocketInitErrors;
use crate::SERVER_ADDR;
use log::error;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

static SERVER_UDP_SOCKET: Lazy<Arc<Mutex<Option<UdpSocket>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

#[derive(Debug)]
pub struct UdpConnection {
    state_holder: &'static Arc<Mutex<Box<HashMap<Arc<&'static str>, AtomicBool>>>>,
}

impl UdpConnection {
    async fn connect() {
        let serv = Arc::clone(&SERVER_UDP_SOCKET);
        tokio::spawn(async move {
            listen_udp(serv).await;
        }).await.unwrap();
    }

    async fn bind_socket(&self)

    {
        let socket_guard = SERVER_UDP_SOCKET.lock().await;
        if let Some(ref socket) = *socket_guard {
            Self::connect().await;
            println!("{:#?}", self.state_holder);
        } else {
            error!("Socket is not initialized");
        }
    }

    async fn initialize_socket(&self) -> Result<(), SocketInitErrors> {
        let socket = UdpSocket::bind(&SERVER_ADDR).await?;

        let mut socket_guard = SERVER_UDP_SOCKET.lock().await;
        *socket_guard = Some(socket);

        let mut bind_ = self.state_holder.lock().await;
        bind_.insert(Arc::new(SERVER_ADDR), AtomicBool::new(true));
        Ok(())
    }
}

async fn listen_udp(serv: Arc<Mutex<Option<UdpSocket>>>) {
    let l_serv = serv.lock().await;
    let mut buf = vec![0; 1024];

    let (len, client_addr) = &l_serv.as_ref()
        .expect("Receiving error occurred")
        .recv_from(&mut buf).await.unwrap();
    println!("Received {} bytes from {}", len, client_addr);

    let data = buf[..*len].to_vec();

    let received_data = String::from_utf8_lossy(&data);
    println!("Received data: {}", received_data);

    let response = format!("Echo: {}", received_data);
    if let Err(e) = &l_serv.as_ref()
        .expect("Sending error occurred")
        .send_to(response.as_bytes(), &client_addr).await {
        eprintln!("Failed to send response: {}", e);
    }
}