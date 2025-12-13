# **Component 2: Street Relay Node (SRN) Specification**

## **1\. Overview**

The Street Relay Node (SRN) is a high-performance, distributed packet switch. It does not calculate physics, collisions, or game state. It strictly enforces the **Spatial Pub/Sub Protocol**.

**Role:** To ingest telemetry from a Publisher (User A), determine which Grid Cells need that data, and blast it out via QUIC Datagrams to Subscribers (Users B, C, D).

## **2\. Core Modules**

### **2.1 The Connection Manager (QUIC)**

* **Listener:** Listens on UDP Port 443 (or configured port).  
* **Handshake:** Validates HandshakeRequest (from mv\_packets.proto).  
* **Session Map:** Maintains a map of ConnectionID \-\> SessionData.  
  * SessionData contains: EntityID, CurrentGridAddress, BandwidthBudget.

### **2.2 The Grid Manager (The "Brain")**

This module maintains the in-memory state of the Quadtree for the Sectors this node is responsible for.

* **Topic Map:** Map\<MortonCode, Set\<ConnectionID\>\>  
  * Keys are 64-bit Spatial Hashes (Z-Order Curves).  
  * Values are lists of connected clients subscribed to that cell.  
* **Density Monitor:** Tracks UsersPerCell.  
  * *Logic:* If UsersPerCell \> SPLIT\_THRESHOLD, the node signals the protocol to subdivide the cell (increasing spatial resolution).

### **2.3 The Dispatcher (The "Hot Loop")**

The critical path for latency. Must run on the main thread or highly optimized worker threads.

1. **Ingest:** Pop TelemetryUpdate packet from QUIC buffer.  
2. **Validate:** Check if EntityID matches the Connection (Anti-Spoofing).  
3. **Lookup:** Calculate SpatialHash of the user's position.  
4. **Route:**  
   * Find subscribers for the user's *current* cell.  
   * Find subscribers for the *neighboring* cells (Moore Neighborhood).  
5. **Broadcast:** Send copy of packet to all found connections.

### **2.4 The Metrics Agent (Economic Enforcement)**

To satisfy the "Proof-of-Infrastructure" requirement for corporate owners:

* **Heartbeat:** Every 60 seconds, reports PacketsProcessed, AverageLatency, and ActiveUsers to the distributed ledger or auditing authority.  
* **Uptime Check:** Responds to random "Ping" challenges from the network to prove the node is actually relaying traffic and not just squatting.

## **3\. Data Flow Architecture**

graph TD  
    User\[User Client\] \--\>|QUIC Datagram (Telemetry)| SRN\[Relay Node\]  
    SRN \--\>|Validate Packet| Logic\[Dispatcher\]  
    Logic \--\>|Get Cell ID| Grid\[Grid Manager\]  
    Grid \--\>|Return Subscriber List| Logic  
    Logic \--\>|QUIC Datagram| Neighbors\[Neighbor Clients 1..N\]

## **4\. Safety & Security Logic**

### **4.1 Rate Limiting (The "Spam" Filter)**

* **Rule:** A client typically sends 20-60 updates per second.  
* **Enforcement:** If a client sends \>100 packets/sec, the SRN drops excess packets. If \>500/sec, the connection is terminated (DDoS protection).

### **4.2 Spatial Sanity Check**

* **Teleport Detection:** If a user moves from Sector 1 to Sector 99 in 16ms (physically impossible speed), the SRN flags the movement.  
  * *Note:* The SRN does *not* correct the position (that's client-side prediction), but it tags the packet as Suspicious so other clients can choose to ignore it.

### **4.3 The "Ghost" Cleanup**

* If a client disconnects ungracefully (crash), the Connection Manager must immediately remove their ID from all Grid Topic maps to prevent "Ghost Avatars" from lingering in the world.