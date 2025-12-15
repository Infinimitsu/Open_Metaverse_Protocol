mod network;
mod physics;
mod identity;
mod spatial; // NEW: Include the spatial module

use std::ffi::CStr;
use std::os::raw::{c_char, c_float, c_void, c_int, c_uint};
use std::sync::Arc;
use tokio::sync::Mutex; 
use tokio::runtime::Runtime;
use crate::network::NetworkDriver;
use crate::physics::PhysicsEngine;
use crate::identity::IdentityManager;
use crate::spatial::Geography; // NEW: Use Geography
use std::collections::HashMap;
use std::time::{Instant, Duration};

// --- DATA STRUCTURES ---
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MvVector3 { x: c_float, y: c_float, z: c_float }

#[repr(C)]
pub struct MvTransform {
    pub position: MvVector3,
    pub rotation: MvVector3,
    pub scale: MvVector3,
    pub velocity: MvVector3,
    pub color_hash: c_uint,
}

#[repr(C)]
pub struct MvClientConfig {
    pub user_id: *const c_char,
    pub auth_token: *const c_char,
    pub vram_budget_mb: u32,
}

#[repr(C)]
pub struct MvInputCmd {
    pub move_x: c_float,
    pub move_z: c_float,
    pub look_yaw: c_float,
    pub jump: bool,
}

// --- STATE MANAGEMENT ---
struct RemotePeer {
    position: MvVector3,
    color_hash: u32,
    last_update: Instant,
}

struct CoreClient {
    runtime: Runtime,
    net_driver: Arc<Mutex<NetworkDriver>>,
    physics: PhysicsEngine,
    identity: IdentityManager,
    local_position: MvVector3,
    peers: HashMap<u32, RemotePeer>,
    last_identity_broadcast: Instant,
}

// --- C-API EXPORTS ---

#[no_mangle]
pub extern "C" fn mv_core_create(_config: MvClientConfig) -> *mut c_void {
    let runtime = Runtime::new().unwrap();
    let net_driver = runtime.block_on(async {
        NetworkDriver::new().expect("Failed to init network")
    });

    let mut physics = PhysicsEngine::new();
    physics.setup_player();
    let identity = IdentityManager::new();

    let client = CoreClient {
        runtime,
        net_driver: Arc::new(Mutex::new(net_driver)),
        physics,
        identity,
        local_position: MvVector3{x:0.0,y:0.0,z:0.0},
        peers: HashMap::new(),
        last_identity_broadcast: Instant::now(),
    };

    Box::into_raw(Box::new(client)) as *mut c_void
}

#[no_mangle]
pub extern "C" fn mv_core_connect(ptr: *mut c_void, url: *const c_char) {
    let client = unsafe { &mut *(ptr as *mut CoreClient) };
    let url_str = unsafe { CStr::from_ptr(url).to_string_lossy().into_owned() };
    
    let driver = client.net_driver.clone();
    
    client.runtime.spawn(async move {
        let mut d = driver.lock().await;
        if d.connect(&url_str).await.is_ok() {
            println!("[Rust] Connection established.");
        }
    });
}

#[no_mangle]
pub extern "C" fn mv_core_destroy(ptr: *mut c_void) {
    if ptr.is_null() { return; }
    unsafe { let _ = Box::from_raw(ptr as *mut CoreClient); }
}

#[no_mangle]
pub extern "C" fn mv_core_send_input(ptr: *mut c_void, input: MvInputCmd, _dt: c_float) {
    let client = unsafe { &mut *(ptr as *mut CoreClient) };
    
    // Physics Step
    let speed = 5.0;
    client.physics.set_player_velocity(input.move_x * speed, input.move_z * speed);
    
    // Process Jump
    if input.jump {
        client.physics.jump(); 
    }

    let (px, py, pz) = client.physics.get_player_position();
    client.local_position = MvVector3 { x: px, y: py, z: pz };

    // Send Position Packet (Type 1)
    let mut packet = Vec::with_capacity(13);
    packet.push(1);
    packet.extend_from_slice(&px.to_le_bytes());
    packet.extend_from_slice(&py.to_le_bytes());
    packet.extend_from_slice(&pz.to_le_bytes());

    let driver = client.net_driver.clone();
    client.runtime.spawn(async move {
        let d = driver.lock().await;
        d.send_datagram(packet);
    });
}

#[no_mangle]
pub extern "C" fn mv_core_tick(ptr: *mut c_void, dt: c_float) {
    let client = unsafe { &mut *(ptr as *mut CoreClient) };
    client.physics.step(dt);

    let now = Instant::now();
    client.peers.retain(|_, peer| {
        now.duration_since(peer.last_update) < Duration::from_secs(5)
    });

    if client.last_identity_broadcast.elapsed() > Duration::from_secs(3) {
        let pub_key_bytes = client.identity.as_bytes().to_vec();
        let driver = client.net_driver.clone();
        client.runtime.spawn(async move {
            let d = driver.lock().await;
            let mut packet = Vec::with_capacity(33);
            packet.push(2);
            packet.extend_from_slice(&pub_key_bytes);
            d.send_datagram(packet);
        });
        client.last_identity_broadcast = Instant::now();
    }

    if let Ok(mut driver) = client.net_driver.try_lock() {
        while let Ok(packet) = driver.packet_queue.try_recv() {
            if packet.len() < 5 { continue; }
            
            let id_bytes: [u8; 4] = packet[0..4].try_into().unwrap();
            let sender_id = u32::from_le_bytes(id_bytes);
            let packet_type = packet[4];

            match packet_type {
                1 => { // POSITION
                    if packet.len() >= 17 {
                        let x = f32::from_le_bytes(packet[5..9].try_into().unwrap());
                        let y = f32::from_le_bytes(packet[9..13].try_into().unwrap());
                        let z = f32::from_le_bytes(packet[13..17].try_into().unwrap());
                        
                        client.peers.entry(sender_id)
                            .and_modify(|p| {
                                p.position = MvVector3{x,y,z};
                                p.last_update = Instant::now();
                            })
                            .or_insert(RemotePeer { 
                                position: MvVector3{x,y,z}, 
                                color_hash: 0xFFFFFFFF,
                                last_update: Instant::now(),
                            });
                    }
                },
                2 => { // IDENTITY
                    if packet.len() >= 37 {
                        let key_bytes = &packet[5..37];
                        let mut hash: u32 = 5381;
                        for b in key_bytes {
                            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(*b as u32);
                        }
                        
                        client.peers.entry(sender_id)
                            .and_modify(|p| {
                                p.color_hash = hash;
                                p.last_update = Instant::now();
                            })
                            .or_insert(RemotePeer {
                                position: MvVector3{x:0.0, y:0.0, z:0.0},
                                color_hash: hash,
                                last_update: Instant::now(),
                            });
                    }
                },
                _ => {}
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn mv_core_get_entity_transform(ptr: *mut c_void, id: u64, out: *mut MvTransform) -> bool {
    let client = unsafe { &mut *(ptr as *mut CoreClient) };
    
    if id == 0 {
        unsafe {
            (*out).position = client.local_position;
            (*out).rotation = MvVector3 { x:0.0, y:0.0, z:0.0 };
            (*out).scale = MvVector3 { x:1.0, y:1.0, z:1.0 };
            (*out).velocity = MvVector3 { x:0.0, y:0.0, z:0.0 };
            (*out).color_hash = 0x0000FFFF;
        }
        return true;
    }

    if let Some(peer) = client.peers.get(&(id as u32)) {
        unsafe {
            (*out).position = peer.position;
            (*out).rotation = MvVector3 { x:0.0, y:0.0, z:0.0 };
            (*out).scale = MvVector3 { x:1.0, y:1.0, z:1.0 };
            (*out).color_hash = peer.color_hash;
        }
        return true;
    }

    return false;
}

#[no_mangle]
pub extern "C" fn mv_core_get_peer_ids(ptr: *mut c_void, out_ids: *mut u64, max_count: c_int) -> c_int {
    let client = unsafe { &mut *(ptr as *mut CoreClient) };
    let mut count = 0;
    for (id, _) in &client.peers {
        if count >= max_count { break; }
        unsafe { *out_ids.offset(count as isize) = *id as u64; }
        count += 1;
    }
    count
}

// NEW: Spatial Query API
// Returns true if the coordinate is on public land (The Street)
#[no_mangle]
pub extern "C" fn mv_core_is_public_street(x: c_float, z: c_float) -> bool {
    Geography::is_public_street(x, z)
}