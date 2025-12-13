The Metaverse Protocol (MVP)

Current Status: Pre-Alpha / Architectural Specification

License: Apache 2.0

Vision

The Metaverse Protocol (MVP) is an initiative to engineer "The Street"—a contiguous, decentralized, and persistent 3D spatial utility.

Unlike commercial "metaverse" platforms that function as walled garden applications, MVP is designed as a foundational civilization layer, analogous to the World Wide Web. It provides the protocol-level infrastructure (TCP/IP for 3D space) to allow independent servers ("Parcels") to co-exist within a single, shared coordinate system ("The Street").

This project rejects the hype-cycle definition of the metaverse (NFTs, forced scarcity, gamification) in favor of rigorous engineering, interoperability, and low-latency utility.

Core Pillars

The "Common Area" Mandate: The infrastructure supports a shared, public, procedurally generated thoroughfare that exists independently of any private entity.

Client Sovereignty: The user's software (The Browser) generates the world; it does not merely stream a video of it. The user retains control over visual themes and local rendering.

Distributed Telemetry: We solve the "C10K Problem" (10,000 users in one space) via distributed Relay Nodes and Peer-to-Peer offloading, rather than monolithic game servers.

Technical Architecture

The MVP Stack is built on open standards to ensure maximum compatibility:

Transport: QUIC (over UDP) for low-latency, encrypted data transfer.

Spatial Indexing: Z-Order Curve (Morton Codes) on a Dynamic Quadtree Grid for O(1) neighbor lookups.

Routing: Libp2p / GossipSub for decentralized message propagation.

Asset Composition: OpenUSD for scene hierarchy and MaterialX for engine-agnostic shading.

Base Layer: MPS (Metaverse Primitive Standard) — a fallback layer of geometric primitives ensuring the world is navigable even at zero bandwidth.

Repository Structure

/specs          # Protocol specifications, TDDs, and Whitepapers
/proto          # Protocol Buffer (.proto) definitions for the wire format
/core           # Reference implementation of the Core Logic (C++/Rust)
/relay          # The Street Relay Node (SRN) implementation
/client         # Reference implementation of the "Metaverse Browser"


Governance & Rights

The protocol encodes a "Bill of Rights" directly into the handshake process:

Right of Exit: Users effectively own a hardware-level "Kill Switch" to leave any server instantly.

The Glass Layer: A protected UI layer for personal notifications that servers cannot obscure.

Proof-of-Infrastructure: High-density commercial sectors must contribute compute resources (Relay Nodes) to support the public Street.

Getting Involved

This project is in the architectural design phase. We are currently defining the .proto wire formats and the Relay Node logic.

See specs/relay_node_spec_v1.md for the current Relay Node architecture.

Licensed under Apache License 2.0
