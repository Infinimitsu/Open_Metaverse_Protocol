mod network;

use std::ffi::CStr;
use std::os::raw::{c_char, c_float, c_void};
use std::sync::Arc;
use tokio::sync::Mutex; 
use tokio::runtime::Runtime;
use crate::network::NetworkDriver;

// --- DATA STRUCTURES (Prefixed to avoid C collisions) ---

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MvVector3 { x: c_float, y: c_float, z: c_float }

#[repr(C)]
pub struct MvTransform {
    pub position: MvVector3,
    pub rotation: MvVector3,
    pub scale: MvVector3,
    pub velocity: MvVector3,
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
}

// --- THE CORE CLIENT ---
struct CoreClient {
    runtime: Runtime,
    net_driver: Arc<Mutex<NetworkDriver>>,
    local_position: MvVector3,
}

// --- C-API EXPORTS ---

#[no_mangle]
pub extern "C" fn mv_core_create(_config: MvClientConfig) -> *mut c_void {
    let runtime = Runtime::new().unwrap();
    let net_driver = runtime.block_on(async {
        NetworkDriver::new().expect("Failed to init network")
    });

    let client = CoreClient {
        runtime,
        net_driver: Arc::new(Mutex::new(net_driver)),
        local_position: MvVector3 { x: 0.0, y: 0.0, z: 0.0 },
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
        match d.connect(&url_str).await {
            Ok(_) => println!("[Rust] Connected to {}!", url_str),
            Err(e) => println!("[Rust] Connection Failed: {}", e),
        }
    });
}

#[no_mangle]
pub extern "C" fn mv_core_destroy(ptr: *mut c_void) {
    if ptr.is_null() { return; }
    unsafe { let _ = Box::from_raw(ptr as *mut CoreClient); }
}

#[no_mangle]
pub extern "C" fn mv_core_send_input(ptr: *mut c_void, input: MvInputCmd, dt: c_float) {
    let client = unsafe { &mut *(ptr as *mut CoreClient) };
    
    // Client-side Prediction
    let speed = 5.0;
    client.local_position.x += input.move_x * speed * dt;
    client.local_position.z += input.move_z * speed * dt;

    // Serialize
    let mut packet = Vec::with_capacity(13);
    packet.push(1);
    packet.extend_from_slice(&client.local_position.x.to_le_bytes());
    packet.extend_from_slice(&client.local_position.y.to_le_bytes());
    packet.extend_from_slice(&client.local_position.z.to_le_bytes());

    let driver = client.net_driver.clone();
    client.runtime.spawn(async move {
        let d = driver.lock().await;
        d.send_datagram(packet);
    });
}

#[no_mangle]
pub extern "C" fn mv_core_tick(_ptr: *mut c_void, _dt: c_float) {}

#[no_mangle]
pub extern "C" fn mv_core_get_entity_transform(ptr: *mut c_void, _id: u64, out: *mut MvTransform) -> bool {
    let client = unsafe { &mut *(ptr as *mut CoreClient) };
    unsafe {
        (*out).position = client.local_position;
        (*out).rotation = MvVector3 { x: 0.0, y: 0.0, z: 0.0 };
        (*out).scale = MvVector3 { x: 1.0, y: 1.0, z: 1.0 };
        (*out).velocity = MvVector3 { x: 0.0, y: 0.0, z: 0.0 };
    }
    true
}