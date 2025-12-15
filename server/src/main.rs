use quinn::{Endpoint, ServerConfig, Connection};
use std::{error::Error, net::SocketAddr};
use tokio::sync::broadcast;
// Remove AsyncReadExt since Quinn handles reading internally differently
// use tokio::io::AsyncReadExt; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Generate Certs (Dev Mode)
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_chain = vec![rustls::Certificate(cert.serialize_der()?)];
    let key = rustls::PrivateKey(cert.serialize_private_key_der());
    let server_config = ServerConfig::with_single_cert(cert_chain, key)?;
    
    // 2. Bind
    let addr = "0.0.0.0:4433".parse::<SocketAddr>()?;
    let endpoint = Endpoint::server(server_config, addr)?;
    println!("[Server] Listening on {} (Stream + Datagram Mode)", addr);

    // 3. Broadcast Channel
    let (tx, _rx) = broadcast::channel::<(u32, Vec<u8>)>(100);
    let mut next_id = 1; 

    // 4. Accept Loop
    while let Some(conn) = endpoint.accept().await {
        let connection = conn.await?;
        let id = next_id;
        next_id += 1;

        let tx_clone = tx.clone();
        let rx_clone = tx.subscribe();

        println!("[Server] Client #{} Connected", id);

        tokio::spawn(async move {
            handle_client(id, connection, tx_clone, rx_clone).await;
        });
    }

    Ok(())
}

async fn handle_client(
    id: u32, 
    connection: Connection, 
    tx: broadcast::Sender<(u32, Vec<u8>)>, 
    mut rx: broadcast::Receiver<(u32, Vec<u8>)>
) {
    let conn_send = connection.clone();

    // TASK A: READ Datagrams (Movement - Unreliable)
    let conn_dgram = connection.clone();
    let tx_dgram = tx.clone();
    let read_dgram = tokio::spawn(async move {
        loop {
            match conn_dgram.read_datagram().await {
                Ok(data) => {
                    // Forward raw data with ID
                    let _ = tx_dgram.send((id, data.to_vec())); 
                }
                Err(_) => break,
            }
        }
    });

    // TASK B: READ Streams (Identity - Reliable)
    let conn_stream = connection.clone();
    let tx_stream = tx.clone();
    let read_stream = tokio::spawn(async move {
        loop {
            // Accept incoming uni-directional streams
            match conn_stream.accept_uni().await {
                Ok(mut stream) => {
                    let tx_inner = tx_stream.clone();
                    tokio::spawn(async move {
                        // FIX: quinn 0.10 read_to_end takes a size limit (e.g. 10MB)
                        // and returns the vector directly.
                        const MAX_SIZE: usize = 10 * 1024 * 1024;
                        if let Ok(data) = stream.read_to_end(MAX_SIZE).await {
                            // Forward reliable data same as datagrams for now
                            let _ = tx_inner.send((id, data));
                        }
                    });
                }
                Err(_) => break,
            }
        }
    });

    // TASK C: WRITE (Forward to Client)
    let write_task = tokio::spawn(async move {
        loop {
            if let Ok((sender_id, msg)) = rx.recv().await {
                if sender_id != id {
                    // Format: [SenderID (4 bytes)] + [Original Message]
                    let mut packet = Vec::with_capacity(4 + msg.len());
                    packet.extend_from_slice(&sender_id.to_le_bytes());
                    packet.extend_from_slice(&msg);
                    
                    // For simplicity in this demo, we forward everything as datagrams.
                    // In a production environment, you would check the message type
                    // and decide whether to forward as a stream or datagram.
                    let _ = conn_send.send_datagram(packet.into());
                }
            }
        }
    });

    // Wait for disconnection
    let _ = tokio::select! {
        _ = read_dgram => {},
        _ = read_stream => {},
        _ = write_task => {},
    };
    println!("[Server] Client #{} Disconnected", id);
}