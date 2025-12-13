# **Component 6: Distributed Spatial Index Specification**

## **1\. Overview**

The Spatial Index serves as the "DNS" of the Metaverse. It resolves a **Grid Address** (Sector-Block-Parcel) into a **Network Connection** (IP Address \+ Port).

To satisfy the requirement of "No Central Controller," this index is stored in a **Distributed Hash Table (DHT)** maintained by the federation of Relay Nodes and authorized clients.

## **2\. The Addressing Schema**

### **2.1 The Key (The "URL")**

In the Web, you look up google.com. In the Metaverse, you look up a **Spatial Hash**.

* **Format:** 64-bit Integer (Morton Code / Z-Order Curve).  
* **Granularity:** The DHT stores records at the **Block Level** (1km squares).  
* **Derivation:** Key \= InterleaveBits(SectorID, BlockID)

### **2.2 The Value (The "A Record")**

The data stored in the DHT is a Protocol Buffer message: SpatialRecord.

message SpatialRecord {  
  // Who manages the public street here?  
  ConnectionInfo public\_relay\_node \= 1;  
    
  // Is there a private sovereign server here?  
  ConnectionInfo private\_parcel\_server \= 2;  
    
  // SECURITY: Proof of Ownership  
  // Signed by the Root Registry (The Non-Profit DAO)  
  bytes lease\_signature \= 3;   
  uint64 lease\_expiration \= 4;  
}

## **3\. The Resolution Process**

1. **Calculate:** Client computes the Morton Code for its target destination.  
2. **Query:** Client asks the DHT: "Get Record for Key 0x4A1...".  
3. **Verify:** Client checks lease\_signature.  
   * If valid: Connect.  
   * If invalid/expired: Treat as "Unclaimed Land" (Render procedural wilderness).  
4. **Connect:** Client opens QUIC connection to the IP in public\_relay\_node.

## **4\. The "Gravity" Handover (Dynamic DNS)**

This mechanism handles the "Times Square" scenario where a corporate entity takes over processing duties.

1. **Standard State:** The DHT points to a generic ISP-hosted Relay Node.  
2. **High-Traffic Event:** The "Gravity" algorithm flags the sector.  
3. **Update:** The Corporate Owner publishes a **DHT Update**.  
   * *New Record:* "Public Relay is now amazon-metaverse-node.aws.com".  
   * *Authority:* Validated by the lease\_signature.  
4. **Propagation:** The network updates within seconds. All new users connect to the high-power corporate node.

## **5\. Caching Strategy**

* **Hot Path:** Clients cache lookups for the 9-slice neighborhood they are currently in.  
* **TTL:** Records have a short Time-To-Live (e.g., 60 seconds) to allow for rapid infrastructure changes (failover).