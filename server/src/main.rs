use quinn::{Endpoint, ServerConfig, Connection};
use std::{error::Error, net::SocketAddr, sync::{Arc, Mutex}, collections::HashMap};
use tokio::sync::broadcast;

// Standard "Entity State" packet size (Position + Rotation)
const MAX_DATAGRAM_SIZE: usize = 128; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Generate Certs (Dev Mode)
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();
    let cert_chain = vec![rustls::Certificate(cert_der)];
    let key = rustls::PrivateKey(priv_key);

    let server_config = ServerConfig::with_single_cert(cert_chain, key)?;
    
    // 2. Bind to 0.0.0.0 (Allows LAN/Internet connections)
    let addr = "0.0.0.0:4433".parse::<SocketAddr>()?;
    let endpoint = Endpoint::server(server_config, addr)?;
    
    println!("[Server] Listening on {} (Multiplayer Ready)", addr);

    // 3. Shared State: Map of ConnectionID -> ConnectionHandle
    // We use a Broadcast channel to send messages from one thread to all others
    let (tx, _rx) = broadcast::channel::<(usize, Vec<u8>)>(100); // (SenderID, Data)

    let mut next_id = 0;

    // 4. Accept Loop
    while let Some(conn) = endpoint.accept().await {
        println!("[Server] Incoming connection...");
        let connection = conn.await?;
        let id = next_id;
        next_id += 1;

        let tx_clone = tx.clone();
        let rx_clone = tx.subscribe();

        println!("[Server] Client {} Connected: {}", id, connection.remote_address());

        // Spawn a handler for this client
        tokio::spawn(async move {
            handle_client(id, connection, tx_clone, rx_clone).await;
        });
    }

    Ok(())
}

async fn handle_client(
    id: usize, 
    connection: Connection, 
    tx: broadcast::Sender<(usize, Vec<u8>)>, 
    mut rx: broadcast::Receiver<(usize, Vec<u8>)>
) {
    // Split into Read/Write tasks
    let conn_send = connection.clone();

    // TASK A: READ from Client, PUBLISH to everyone
    let read_task = tokio::spawn(async move {
        loop {
            // Read Datagram (Unreliable)
            match connection.read_datagram().await {
                Ok(data) => {
                    // Broadcast to all other threads
                    let _ = tx.send((id, data.to_vec()));
                }
                Err(_) => break, // Disconnected
            }
        }
    });

    // TASK B: LISTEN to everyone, WRITE to Client
    let write_task = tokio::spawn(async move {
        loop {
            if let Ok((sender_id, data)) = rx.recv().await {
                // Don't echo back to self
                if sender_id != id {
                    let _ = conn_send.send_datagram(data.into());
                }
            }
        }
    });

    // Wait for either to fail (disconnect)
    let _ = tokio::select! {
        _ = read_task => {},
        _ = write_task => {},
    };

    println!("[Server] Client {} Disconnected", id);
}