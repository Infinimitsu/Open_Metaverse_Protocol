use std::sync::Arc;
use std::net::SocketAddr;
use quinn::{Endpoint, Connection, ClientConfig};
use std::error::Error;
use bytes::Bytes; // Make sure this is imported

pub struct NetworkDriver {
    pub endpoint: Endpoint,
    pub connection: Option<Connection>,
}

impl NetworkDriver {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let bind_addr = "0.0.0.0:0".parse::<SocketAddr>()?;
        let mut endpoint = Endpoint::client(bind_addr)?;
        
        // DEV ONLY: Skip Certificate Verification
        let crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth();
            
        endpoint.set_default_client_config(ClientConfig::new(Arc::new(crypto)));

        Ok(Self { endpoint, connection: None })
    }

    pub async fn connect(&mut self, addr_str: &str) -> Result<(), Box<dyn Error>> {
        let remote_addr: SocketAddr = addr_str.parse()?;
        let connection = self.endpoint.connect(remote_addr, "localhost")?.await?;
        self.connection = Some(connection);
        Ok(())
    }

    // NEW: Send Data
    // This sends an unreliable UDP packet via QUIC
    pub fn send_datagram(&self, data: Vec<u8>) {
        if let Some(conn) = &self.connection {
            // We convert Vec<u8> to Bytes and send.
            // We ignore errors here because datagrams are fire-and-forget.
            let _ = conn.send_datagram(Bytes::from(data));
        }
    }
}

// --- TLS VERIFICATION SKIPPER (DEV ONLY) ---
use std::sync::Arc as StdArc;
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> StdArc<Self> {
        StdArc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}