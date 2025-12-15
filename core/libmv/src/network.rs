use std::sync::Arc;
use std::net::SocketAddr;
use quinn::{Endpoint, Connection, ClientConfig};
use std::error::Error;
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt; // Required for writing streams

pub struct NetworkDriver {
    pub endpoint: Endpoint,
    pub connection: Option<Connection>,
    pub packet_queue: mpsc::UnboundedReceiver<Vec<u8>>,
    internal_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
}

impl NetworkDriver {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let bind_addr = "0.0.0.0:0".parse::<SocketAddr>()?;
        let mut endpoint = Endpoint::client(bind_addr)?;
        
        let crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth();
            
        endpoint.set_default_client_config(ClientConfig::new(Arc::new(crypto)));

        let (tx, rx) = mpsc::unbounded_channel();

        Ok(Self { 
            endpoint, 
            connection: None,
            packet_queue: rx,
            internal_tx: Some(tx),
        })
    }

    pub async fn connect(&mut self, addr_str: &str) -> Result<(), Box<dyn Error>> {
        let remote_addr: SocketAddr = addr_str.parse()?;
        let connection = self.endpoint.connect(remote_addr, "localhost")?.await?;
        self.connection = Some(connection.clone());

        // Spawn Background Reader Task
        if let Some(tx) = self.internal_tx.take() {
            let conn_clone = connection.clone();
            tokio::spawn(async move {
                // 1. Read Datagrams (Movement - Unreliable)
                let tx_udp = tx.clone();
                let conn_udp = conn_clone.clone();
                tokio::spawn(async move {
                    loop {
                        match conn_udp.read_datagram().await {
                            Ok(data) => { let _ = tx_udp.send(data.to_vec()); }
                            Err(_) => break,
                        }
                    }
                });

                // 2. Read Streams (Identity - Reliable)
                // We accept incoming uni-directional streams from the server
                loop {
                    match conn_clone.accept_uni().await {
                        Ok(mut stream) => {
                            let tx_stream = tx.clone();
                            tokio::spawn(async move {
                                // Quinn 0.10 requires a size limit for read_to_end
                                const MAX_SIZE: usize = 10 * 1024 * 1024; // 10 MB limit
                                if let Ok(data) = stream.read_to_end(MAX_SIZE).await {
                                    let _ = tx_stream.send(data);
                                }
                            });
                        }
                        Err(_) => break, // Connection closed
                    }
                }
            });
        }
        
        Ok(())
    }

    // Unreliable (Movement)
    pub fn send_datagram(&self, data: Vec<u8>) {
        if let Some(conn) = &self.connection {
            let _ = conn.send_datagram(Bytes::from(data));
        }
    }

    // NEW: Reliable (Identity/Chat)
    pub async fn send_reliable(&self, data: Vec<u8>) {
        if let Some(conn) = &self.connection {
            // Open a uni-directional stream
            if let Ok(mut stream) = conn.open_uni().await {
                let _ = stream.write_all(&data).await;
                let _ = stream.finish().await;
            }
        }
    }
}

// --- TLS VERIFICATION SKIPPER (DEV ONLY) ---
use std::sync::Arc as StdArc;
struct SkipServerVerification;
impl SkipServerVerification { fn new() -> StdArc<Self> { StdArc::new(Self) } }
impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(&self, _: &rustls::Certificate, _: &[rustls::Certificate], _: &rustls::ServerName, _: &mut dyn Iterator<Item = &[u8]>, _: &[u8], _: std::time::SystemTime) -> Result<rustls::client::ServerCertVerified, rustls::Error> { Ok(rustls::client::ServerCertVerified::assertion()) }
}