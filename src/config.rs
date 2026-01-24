use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Config {
    pub host_ip: String,
    pub port: u32,
    pub rust_log: String,
 }

impl Config {
    /// Config Constructor, reads env variables and sets config
    pub fn new() -> Config {
        let host_ip = std::env::var("HOST_IP").expect("HOST_IP not set");
        let port = std::env::var("PORT").map_or(8080, |port_string| {
            port_string.parse::<u32>().expect("PORT not parsable")
        });
        let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "todo_axum=debug,tower_http=debug".into());
       Config {
            host_ip,
           port,
           rust_log,
        }
    }
    pub fn get_host_socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host_ip, self.port)[..]).unwrap()
    }
    pub fn get_rust_log(&self) -> &str {
        &self.rust_log
    }
}
impl Default for Config{
    fn default() -> Self {
        Config::new()
    }
}
