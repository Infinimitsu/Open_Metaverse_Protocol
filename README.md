# **🌐 The Metaverse Protocol (MVP)**

**Current Status:** MVP v1.0 (Proof of Concept)

**License:** [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)

## **💡 Vision**

The **Metaverse Protocol (MVP)** is an initiative to engineer **"The Street"**—a contiguous, decentralized, and persistent 3D spatial utility.

Unlike commercial "metaverse" platforms that function as walled garden applications, MVP is designed as a foundational civilization layer, analogous to the World Wide Web. It provides the **protocol-level infrastructure** (think TCP/IP for 3D space) to allow independent servers (**"Parcels"**) to co-exist within a single, shared coordinate system (**"The Street"**).

This project rejects the hype-cycle definition of the metaverse (NFTs, forced scarcity, gamification) in favor of **rigorous engineering, interoperability, and low-latency utility.**

## **🚀 MVP v1.0 Features**

We have successfully released the first working prototype of the protocol stack.

* **Rust Core:** High-performance logic library (LibMV) handling physics and networking.  
* **QUIC Transport:** Low-latency UDP networking (via Quinn) for real-time movement.  
* **Physics:** Rapier3D integration for gravity, collision, and jumping.  
* **Identity:** Cryptographic key generation determines your unique avatar color.  
* **Geography:** Procedural generation of "The Street" vs "Private Parcels."

## **🎮 How to Run the Demo**

You can download the latest release from the [Releases Page](https://github.com/Infinimitsu/Open_Metaverse_Protocol/releases).

### **1\. Start the Server**

The server acts as the relay node. It must be running for clients to see each other.

1. Open the Server folder.  
2. Double-click mv\_server.exe.  
3. You should see: \[Server\] Listening on 0.0.0.0:4433.

### **2\. Start the Client(s)**

1. Open the Client folder.  
2. Double-click mv\_client.exe.  
3. A window will open. You are the **Blue Cube**.  
4. Launch mv\_client.exe again to spawn a second window.  
5. Arrange windows side-by-side. Move the second client; you will see it appear as a **Colored Cube** in the first window.

### **Controls**

* **W, A, S, D:** Move  
* **Space:** Jump  
* **Mouse:** Look  
* **Right Click:** Toggle Mouse Capture (Lock/Unlock cursor)

## **🛠️ Core Pillars**

* **The "Common Area" Mandate:** The infrastructure supports a shared, public, procedurally generated thoroughfare that exists independently of any private entity.  
* **Client Sovereignty:** The user's software (**The Browser**) generates the world; it does not merely stream a video of it. The user retains control over visual themes and local rendering.  
* **Distributed Telemetry:** We solve the **"C10K Problem"** (10,000 users in one space) via distributed **Relay Nodes** and **Peer-to-Peer offloading**, rather than monolithic game servers.

## **📐 Technical Architecture**

The MVP Stack is built on open standards to ensure maximum compatibility:

| Component | Standard/Technology | Purpose |
| :---- | :---- | :---- |
| **Transport** | **QUIC** (over UDP) | Low-latency, encrypted data transfer. |
| **Spatial Indexing** | **Z-Order Curve** (Morton Codes) on a Dynamic Quadtree Grid | $O(1)$ neighbor lookups for culling and proximity. |
| **Routing** | **Libp2p / GossipSub** | Decentralized message propagation. |
| **Asset Composition** | **OpenUSD** (Universal Scene Description) | Scene hierarchy and interchange format. |
| **Shading** | **MaterialX** | Engine-agnostic shading and material definition. |
| **Base Layer** | **MPS** (Metaverse Primitive Standard) | A fallback layer ensuring navigability even at zero bandwidth. |

## **📂 Repository Structure**

* /specs → Protocol specifications, TDDs, and Whitepapers  
* /proto → Protocol Buffer (.proto) definitions for the wire format  
* /core → Reference implementation of the Core Logic (Rust/LibMV)  
* /server → The Relay Server implementation (Rust)  
* /client → Reference implementation of the "Metaverse Browser" (C++/Raylib)

## **⚖️ Governance & Rights**

The protocol encodes a **"Bill of Rights"** directly into the handshake process:

* **Right of Exit:** Users effectively own a hardware-level **"Kill Switch"** to leave any server instantly.  
* **The Glass Layer:** A protected UI layer for personal notifications that servers cannot obscure.  
* **Proof-of-Infrastructure:** High-density commercial sectors must contribute compute resources (Relay Nodes) to support the public Street.

## **👋 Building from Source**

**Prerequisites:**

* **Rust:** [Install Rustup](https://rustup.rs/) (Ensure cargo is in your PATH).  
* **Visual Studio 2022:** With "Desktop development with C++" workload.  
* **CMake:** Included with Visual Studio.

**Steps:**

1. Clone the repository.  
2. Open the folder in Visual Studio 2022\.  
3. Wait for CMake to configure (check Output window).  
4. **Run Server:** Open terminal in /server and run cargo run.  
5. **Run Client:** Select mv\_client.exe from the Startup Item dropdown and press **F5**.

Licensed under Apache License 2.0