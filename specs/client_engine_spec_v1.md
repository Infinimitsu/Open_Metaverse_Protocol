# **Component 3: Client Engine Architecture**

## **1\. Overview**

The "Metaverse Browser" is the user-facing application. Unlike a standard game client, it is **Engine-Agnostic**. This specification defines how a client must interpret the protocol to ensure visual consistency across different rendering engines (e.g., Unreal vs. Godot).

## **2\. The Core Loop**

The Browser does not run a "Game Loop" in the traditional sense. It runs a **Simulation Loop** that is decoupled from the Rendering Loop.

1. **Network Thread (IO):** Receives TelemetryUpdate packets via QUIC. Pushes them into a "State Buffer."  
2. **Simulation Thread (Physics):** Reads the State Buffer, applies Dead Reckoning (prediction), and updates the transform of every entity.  
3. **Render Thread (Visuals):** Draws the entities using the current interpolated state.

## **3\. The "MPS" Fallback Renderer**

To satisfy the "zero-bandwidth navigation" requirement, the client must implement the **Metaverse Primitive Standard (MPS)**.

* **Rule:** Every entity has a RootPrimitive defined in the protocol (e.g., PRIM\_CAPSULE).  
* **Loading State:**  
  1. **T=0:** Packet received. Client spawns a generic white Capsule.  
  2. **T+1s:** Asset Manifest received. Client starts downloading avatar.glb.  
  3. **T+5s:** Download complete. Client hides the Capsule and attaches the avatar.glb mesh.  
* **Benefit:** The user never waits at a loading screen. They see a world of geometric shapes that progressively resolve into high-fidelity assets.

## **4\. Dead Reckoning & Interpolation**

The most critical logic in the client is masking network latency. We use **Entity Interpolation** with **Cubic Hermite Splines**.

* **The Buffer:** The client keeps a history of the last 20 packets for every entity.  
* **The Delay:** The client renders the world **100ms in the past**.  
  * *Why?* By rendering the past, we always have a "Start Packet" and an "End Packet" to interpolate between. We never have to guess (extrapolate) unless packets are lost.  
* **The Math:**  
  * $P(t)$ \= Interpolate(Pos\_A, Pos\_B, t)  
  * If a packet is missed, we switch to **Extrapolation** (Predicting the future based on velocity).

## **5\. The "9-Slice" Grid Manager**

The Client is responsible for telling the Relay Node which cells it needs.

* **Position Check:** Every 1 second, the client calculates its current Morton Code.  
* **Boundary Check:** If the client crosses into a new Quadtree Node:  
  1. Calculate the 8 neighbors of the new cell.  
  2. Diff against the old list.  
  3. Send GridSubscription packet: "Unsub from \[Old\], Sub to \[New\]."