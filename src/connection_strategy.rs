use crate::tcp_strategy::TcpStrategy;
use crate::udp_strategy::UdpConnection;

pub enum ConnectionStrategy {
    TCP(TcpStrategy),
    UDP(UdpConnection),
}