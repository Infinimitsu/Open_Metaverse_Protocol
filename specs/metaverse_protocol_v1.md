# **The Metaverse Protocol**

### **Technical Design Document v1.0**

Abstract  
This document outlines the architectural specification for "The Street"—a decentralized, persistent, and interoperable 3D spatial utility. Unlike contemporary "metaverse" platforms which function as walled garden applications, this protocol describes a foundational civilization layer analogous to the World Wide Web, but for spatial existence. The architecture prioritizes low-latency telemetry, engine-agnostic rendering, and a federated sovereignty model that balances user rights with server authority.

## **1\. Philosophy & Core Vision**

The current definition of "Metaverse" has been co-opted by commercial interests to describe isolated video games or social platforms. We reject this definition.

**The Street** is defined as:

1. **A Protocol, Not a Platform:** It is a set of rules (TCP/IP for 3D space) that allows independent servers to present a contiguous world.  
2. **The "Common Area" Mandate:** The infrastructure must support a shared, public, procedurally generated thoroughfare that exists independently of any private commercial entity.  
3. **Client Sovereignty:** The user's software (The Browser) generates the world; it does not merely stream a video of it.

## **2\. Spatial Architecture (The Grid)**

To support a contiguous, infinite world without "sharding" users into parallel instances, the protocol utilizes a hierarchical spatial partitioning system.

### **2.1 The Geometry**

* **Coordinate System:** Infinite Cartesian Grid $(x, y, z)$.  
* **Unit Scale:** 1 Unit \= 1 Meter.  
* **Orientation:** Y-Up (Standard engineering/architectural alignment).

### **2.2 Partitioning: The Dynamic Quadtree**

The world is not a fixed grid of static servers. It is a fluid, recursive grid that adapts to population density.

* **Root Sectors:** 10km x 10km base squares.  
* **Dynamic Subdivision:** When a sector exceeds a defined user density (e.g., \>100 users), it automatically subdivides into 4 child quadrants. This recursion continues down to meter-level granularity for high-density events (e.g., "Times Square").  
* **Re-unification:** When density drops, quadrants merge back into parent sectors to save resources.

### **2.3 Indexing: Morton Codes (Z-Order Curve)**

To enable instant database lookups of spatial position, coordinates are hashed using a Z-Order Curve.

* **Function:** Interleaves the bits of the X and Z coordinates to create a single 64-bit integer ID (The "Spatial Hash").  
* **Benefit:** Preserves **Data Locality**. Users with numerically similar Spatial Hashes are physically close in the virtual world, allowing databases to query neighbors without scanning the entire world map.

### **2.4 Addressing: Sector-Block-Parcel**

Human-readable addresses map directly to the Quadtree hierarchy:

* **Format:** \[Sector ID\] \- \[Block ID\] \- \[Parcel ID\]  
* **Example:** 8-40-12 (Sector 8, Block 40, Parcel 12).

## **3\. The Network Stack**

The "C10K Problem" (10,000 concurrent users) is solved by moving away from monolithic game servers toward a distributed, relay-based mesh.

### **3.1 Transport Layer: QUIC (over UDP)**

* **Protocol:** QUIC (Quick UDP Internet Connections).  
* **Rationale:** Eliminates TCP "Head-of-Line Blocking." A lost packet containing a texture does not stall the packet containing user movement data.  
* **Encryption:** TLS 1.3 built-in by default.

### **3.2 Routing: The Distributed Mesh (Libp2p)**

* **Discovery:** Uses **DHT (Distributed Hash Tables)** to find peers.  
* **Message Propagation:** Uses **GossipSub** (a Pub/Sub protocol).  
  * **Topics:** Each Quadtree Cell is a "Topic."  
  * **Subscription:** A user's client subscribes to the Topic of their current Cell \+ the 8 surrounding neighbors (Moore Neighborhood).  
  * **The Mesh:** Messages are propagated via a mesh of peers, reducing the load on any single server.

### **3.3 The "Signal Towers" (Relay Nodes)**

* **Function:** Dumb, high-speed nodes that route QUIC packets. They perform no game logic, physics, or rendering.  
* **Operators:** Distributed mix of ISPs, NGOs, Universities, and Corporate Sponsors (see Section 7).

### **3.4 Crowd Offloading (WebRTC)**

To reduce infrastructure costs, non-critical data is offloaded to Peer-to-Peer (P2P) connections.

* **High Priority (Position/State):** Routed via Relay Nodes (Authoritative).  
* **Low Priority (Voice/Gesture):** Routed via **WebRTC Data Channels** directly between users.

## **4\. The Rendering Stack (The Browser)**

The "Metaverse Browser" is an engine-agnostic client (Unity, Unreal, Godot, Custom C++) that interprets the protocol.

### **4.1 Layer 0: The Metaverse Primitive Standard (MPS)**

The "Fallback Universe." Every asset must inherit from a lightweight mathematical primitive hard-coded into the browser.

* **Geometry:** PRIM\_BOX, PRIM\_SPHERE, PRIM\_CYLINDER, PRIM\_CAPSULE, PRIM\_PLANE.  
* **Logic:** PRIM\_GATEWAY (Portal boundaries), PRIM\_DAEMON (Script indicators).  
* **Behavior:** Even if a client has 0 bandwidth and cannot download assets, they can navigate the world using these untextured primitives.

### **4.2 Asset Composition: OpenUSD**

* **Standard:** **OpenUSD (Universal Scene Description)**.  
* **Usage:** Handles the hierarchy and composition of the scene (e.g., "This building contains these walls and these lights").

### **4.3 Meshes & Materials**

* **Leaf Assets:** **glTF 2.0** (Binary glB) for highly compressed geometry delivery.  
* **Surfaces:** **MaterialX** for engine-agnostic shaders. Defines the *math* of the surface (Roughness, Metallic, Emissive) rather than the compiled shader code, ensuring a gold wall looks like gold in both Unity and Unreal.

## **5\. Authority & Sovereignty Model**

The protocol distinguishes between "The Street" (Public Utility) and "The Parcel" (Private Application).

### **5.1 The Street (Client Sovereignty)**

* **Visuals:** Controlled by the **User**. The user applies a "Theme" (e.g., Cyberpunk, Nature, Minimalist) that styles the procedural common areas.  
* **Physics:** Controlled by the **Protocol**. Standard gravity and collision rules apply to everyone.

### **5.2 The Parcel (Server Sovereignty)**

* **The Threshold:** Crossing a PRIM\_GATEWAY triggers a REQUEST\_HANDOFF.  
* **The Holodeck Mode:** Upon entry, the Server assumes **Total Authority**.  
  * **Visual Override:** The Server dictates the rendering pipeline (Visuals/Audio).  
  * **Physics Override:** The Server can change gravity, speed, or locomotion inputs.  
  * **Logic:** The Parcel effectively functions as a separate video game engine streaming data to the client.

### **5.3 Non-Euclidean Geography**

* **External:** A parcel has a fixed, finite address on the Street (e.g., 20m wide).  
* **Internal:** Once inside, the coordinate system resets relative to the Server. The interior can be infinite, looped, or functionally distinct from the exterior dimensions.

## **6\. Rights & Governance (The Bill of Rights)**

To balance Server Sovereignty with User Safety, hard-coded rights are baked into the browser.

### **6.1 User Rights**

1. **Right of Exit (The Kill Switch):** A hardware/OS-level UI overlay that instantly severs the connection to a Parcel and returns the user to the Street. This cannot be disabled by a Server.  
2. **The Glass Layer:** A personal UI layer (Notifications, System Clock, Wallet) rendered *over* the world. The Server cannot draw on or obscure this layer.  
3. **Data Isolation:** Entry into a Parcel does not grant the Server read-access to the user's local file system or full inventory, only their Public Profile.

### **6.2 Server Rights**

1. **Integrity Check:** Right to scan the Client for unauthorized "Active Daemons" (cheats/scripts) upon entry.  
2. **Admission Control:** Right to deny entry based on age, token possession, or ban lists.  
3. **Immersion Control:** Right to override user cosmetic avatars with server-mandated avatars (e.g., forcing a "Soldier" avatar in a war simulation).

## **7\. The Economic Model: Proof-of-Infrastructure**

A centralized tax is replaced by a resource-sharing obligation for prime real estate.

### **7.1 "Gravity" Zoning**

* **High Density Cells:** Any cell exceeding a traffic threshold (e.g., "Times Square") is flagged.  
* **The Obligation:** Owners of Parcels within High Density cells are **required** to run public Relay Nodes to support the Street's traffic in that sector.  
* **The Benefit:** Corporations (Amazon, Disney) cluster together to share the compute cost of the "Downtown" districts.  
* **The Suburbs:** Low-traffic areas do not require public node operation, making them accessible to individual users and small creators.

### **7.2 Anti-Squatting / Forfeiture**

* **The Audit:** The Protocol performs automated uptime and latency checks on Corporate Relay Nodes.  
* **Eviction:** Failure to maintain the required node performance results in a **Coordinate Forfeiture**. The Parcel is unlinked from the location, ensuring prime coordinates are always powered by active, capable infrastructure.

## **8\. Daemon Architecture**

* **Type A (Active):** Scripts that manipulate world state (Bots, Traders). **Server-Blocked** by default inside Parcels.  
* **Type B (Passive):** Scripts that analyze local data (Notes, Price Checkers, Recording). **User-Controlled**, generally protected, with exceptions for DRM-flagged zones (e.g., Virtual Cinemas).