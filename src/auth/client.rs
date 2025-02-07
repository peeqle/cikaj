use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;

struct Client {
    name: String,
    ipv4addr: Ipv4Addr,
    ipv6addr: Ipv6Addr
}
