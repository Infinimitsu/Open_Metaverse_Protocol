# **Component 4: Asset Streaming Pipeline**

## **1\. Overview**

The Asset Pipeline ensures the Metaverse is navigable instantly, regardless of connection speed. It prioritizes **Semantic Structure** over **Visual Fidelity**. Unlike a video game that pre-loads levels, the Metaverse streams assets based on visual priority (Screen Space Error).

## **2\. The Cascading Level of Detail (LOD)**

The client must implement a three-stage loading process for every entity in the world.

### **Stage 1: The Primitive (T=0ms)**

* **Source:** TelemetryUpdate packet (UDP).  
* **Data:** PrimitiveID (e.g., PRIM\_CYLINDER), Scale, Color.  
* **Visual:** The client renders a generic, untextured geometry immediately.  
* **Physics:** Collision is active immediately.

### **Stage 2: The Manifest (T=100ms)**

* **Source:** **OpenUSD** file (hosted on the Parcel Server).  
* **Data:** Scene Hierarchy. "This Cylinder is actually the root bone of a character. It has a hat, a jacket, and uses Animation Set B."  
* **Visual:** The client prepares the skeleton and dependency graph.

### **Stage 3: The Leaf Assets (T=500ms+)**

* **Source:** **glTF (Binary glB)** and **MaterialX (.mtlx)** files.  
* **Data:** The actual triangle meshes and texture maps.  
* **Visual:** The generic cylinder is replaced by the high-fidelity character model.

## **3\. The "Budget" System (Anti-Crash)**

To prevent a malicious server from crashing a client with 8K textures:

* **VRAM Budget:** The client sets a hard limit (e.g., 4GB).  
* **Priority Queue:** Assets are loaded based on **Distance** and **Screen Size**.  
* **Culling:** If the budget is full, distant objects remain in "Stage 1" (Primitives). The server cannot force the client to download assets that exceed the budget.

## **4\. Caching & Content Addressing**

* **Hashing:** All assets are requested via Content Hash (SHA-256), not just filename.  
* **The Global Cache:** If "Server A" and "Server B" both use the same concrete\_wall.glb, the client only downloads it once. The asset is stored in a local LRU (Least Recently Used) cache.

## **5\. File Formats**

| Component | Standard | Why? |
| :---- | :---- | :---- |
| **Composition** | **OpenUSD (.usdc)** | Best-in-class for layering and overrides. |
| **Geometry** | **glTF 2.0 (.glb)** | The "JPEG of 3D." Highly compressed, fast to parse. |
| **Shading** | **MaterialX** | Engine-agnostic. Looks the same in Unreal, Unity, and WebGPU. |

