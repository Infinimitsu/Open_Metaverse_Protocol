# **Component 5: Authority & Sovereignty Specification**

## **1\. Overview**

The Authority Protocol manages the transition of control from the Public Utility ("The Street") to a Private Server ("The Parcel"). It implements the "Federated Sovereignty" model, where a user voluntarily suspends local simulation authority in exchange for entry into a curated experience (Holodeck Mode), subject to specific inalienable rights.

## **2\. The Handoff State Machine**

The Client maintains an AuthorityState enum:

1. **STREET\_MODE (Default):** Client simulates physics; Relay Node routes traffic. Visuals are user-themed.  
2. **NEGOTIATING:** Client is shaking hands with a Parcel Server.  
3. **PARCEL\_MODE (Sovereign):** Parcel Server dictates physics and visuals. Client acts as a "Dumb Terminal" for logic, but retains UI control.

## **3\. The Negotiation Handshake**

When a user crosses a PRIM\_GATEWAY threshold:

1. **Trigger:** Client sends HandoffRequest (UDP) to Parcel IP.  
   * *Payload:* "I am User X. I am willing to accept: Visual Overrides=YES, Physics Overrides=YES, Script Blocking=NO."  
2. **Challenge:** Server responds with HandoffResponse.  
   * *Payload:* "Entry Approved. REQUIRED: Gravity=0.5, Shader=Cartoon. WARNING: This server blocks active scripts."  
3. **Acceptance:**  
   * If User Settings match Server Requirements \-\> **Enter**.  
   * If Mismatch (e.g., Server demands script blocking, User refuses) \-\> **Bounce** (Invisible Wall physics applied).

## **4\. The Bill of Rights (Technical Enforcement)**

### **4.1 The Kill Switch (Right of Exit)**

* **Requirement:** The Client MUST maintain a hardware-level input (e.g., Esc or specific Gesture) that immediately terminates the Parcel Session.  
* **Logic:** Upon trigger, the Client immediately drops the Parcel Connection, resets AuthorityState to STREET\_MODE, and respawns the user at the Parcel's entrance on the Street.

### **4.2 The Glass Layer (UI Sovereignty)**

* **Requirement:** The Rendering Engine must reserve a distinct Z-Order layer for User UI (System Clock, Notifications, Wallet) that acts as an overlay.  
* **Constraint:** Parcel Asset Streamers are strictly forbidden from rendering into this layer.

### **4.3 Data Isolation**

* **Requirement:** The Handshake only transmits the Public Entity ID and Visual Manifest.  
* **Constraint:** The Client rejects any Server RPC that attempts to read the local file system or query the user's full inventory without explicit, per-request authorization.

## **5\. Security & Anti-Cheat**

* **Server Right:** The Server may request a "Process Integrity Check" during handshake.  
* **Client Obligation:** The Client reports status of "Active Daemons" (e.g., "Aimbot running: NO", "Librarian running: YES").  
* **Result:** Server may deny entry if prohibited Daemons are active.