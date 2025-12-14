# **Parallel Development Roadmap: Core vs. Flagship**

## **1\. The Strategic Conflict**

* **Goal A (Open Standard):** A lightweight, license-free reference client to prove the protocol is accessible.  
* **Goal B (Flagship Demo):** A photorealistic Unreal Engine 5 client to prove the protocol is commercially viable and "Metaverse-ready."

**Verdict:** Highly Feasible, provided we strictly separate **Logic** from **Rendering**.

## **2\. The Architecture: "LibMV"**

To avoid duplicating work, 80% of the code must live in a shared C++ dynamic library (libmv.dll / libmv.so).

### **2.1 What lives in LibMV? (Shared Code)**

* **Network Stack:** QUIC/UDP socket management.  
* **Protocol Parsers:** Protobuf serialization/deserialization.  
* **Spatial Logic:** Quadtree management, Z-Order hashing, 9-Slice neighbor calculation.  
* **State Machine:** Dead Reckoning (Interpolation), Prediction buffers.  
* **Asset Logic:** Priority queues for downloading USD/glTF (but *not* the rendering).

### **2.2 The Integration Layers**

* **Unreal Integration:** A C++ UActorComponent that wraps LibMV. It maps LibMV Entity IDs to Unreal AActor instances.  
* **Barebones Integration:** A simple main.cpp loop using Raylib/Sokol that maps LibMV Entity IDs to draw calls.

## **3\. Phased Execution Plan**

### **Phase 1: The "Headless" Core (Weeks 1-4)**

**Goal:** Prove the math works without graphics.

* **Team Focus:** 100% C++.  
* **Output:** A Unit Test where a "Virtual Client" connects to a Relay Node, moves around, and receives updates about neighbors.  
* **Success Metric:** Two headless clients prints "I see Neighbor X at distance Y" to the console.

### **Phase 2: The Barebones Visualizer (Weeks 5-6)**

**Goal:** Visual debugging and low-end testing.

* **Tool:** **Raylib** (C).  
* **Work:** Hook LibMV up to a window. Draw colored cubes for entities.  
* **Why first?** It compiles in seconds. It allows rapid iteration on physics smoothing and grid logic without waiting for the Unreal Editor to load.  
* **Value:** This becomes the "Developer Kit" for the open-source community.

### **Phase 3: The Unreal Flagship (Weeks 7-12)**

**Goal:** The Investor Demo.

* **Tool:** **Unreal Engine 5**.  
* Work: 1\. Create an Unreal Plugin (MVP\_NetDriver).  
  2\. Bind LibMV data to Nanite meshes.  
  3\. Implement the "Holodeck" asset streamer using Unreal's generic mesh component.  
* **Value:** Shows high-fidelity lighting, avatars, and massive scale.

## **4\. Resource Allocation (Hypothetical Team)**

If you have **3 Developers**:

* **Dev A (The Architect):** Works solely on LibMV (C++). Optimizes the QUIC stack and prediction math.  
* **Dev B (The Engine Dev):** Builds the Unreal Plugin. Focuses on translating LibMV coordinates into Unreal World Space and handling Asset streaming.  
* **Dev C (The Toolsmith):** Builds the Barebones client. Focuses on testing, stress-testing (spawning 1,000 bots), and ensuring the protocol actually works on low-end hardware.

## **5\. Risk Analysis**

| Risk | Mitigation |
| :---- | :---- |
| **Logic Drift** | **Strict Rule:** No game logic allowed in Unreal Blueprints. All positioning logic MUST come from LibMV. Unreal is for display only. |
| **Coordinate Precision** | Unreal uses double (Large World Coordinates), Raylib often uses float. Ensure LibMV uses double internally for the infinite grid. |
| **Asset Compatibility** | Ensure the "Asset Pipeline" delivers standard files (USD/glTF) that both engines can read. Do not use .uasset files for the network stream. |

## **6\. Conclusion**

By treating the specific engine (Unreal vs. Custom) as a "swappable frontend," you protect the project. If Unreal changes its license or terms, your protocol survives because the core logic is in LibMV. The Barebones client proves openness; the Unreal client proves power.