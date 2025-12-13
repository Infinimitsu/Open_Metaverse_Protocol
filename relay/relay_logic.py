import time
from typing import Dict, Set, List

# Importing our Protocol Buffer definitions (Conceptual)
import mv_core_pb2 as Core
import mv_packets_pb2 as Packets

# --- CONSTANTS ---
SECTOR_SIZE = 10000.0 # 10km
BLOCK_SIZE = 1000.0   # 1km
PARCEL_SIZE = 32.0    # 32m (Base resolution for routing)

class Session:
    def __init__(self, connection_id, entity_id):
        self.connection_id = connection_id
        self.entity_id = entity_id
        self.current_address = None # Type: Core.GridAddress
        self.last_packet_time = time.time()

class GridManager:
    def __init__(self):
        # Map of SpatialHash (Morton Code) -> Set of Connection IDs
        self.topics: Dict[int, Set[str]] = {}

    def subscribe(self, session: Session, address: Core.GridAddress):
        """User wants to hear updates from this cell."""
        h = address.spatial_hash
        if h not in self.topics:
            self.topics[h] = set()
        self.topics[h].add(session.connection_id)

    def unsubscribe(self, session: Session, address: Core.GridAddress):
        """User left range of this cell."""
        h = address.spatial_hash
        if h in self.topics:
            self.topics[h].discard(session.connection_id)
            # Cleanup empty topics to save memory
            if not self.topics[h]:
                del self.topics[h]

    def get_subscribers(self, address: Core.GridAddress) -> Set[str]:
        """Who is listening to this cell?"""
        return self.topics.get(address.spatial_hash, set())

class Dispatcher:
    def __init__(self):
        self.sessions: Dict[str, Session] = {}
        self.grid = GridManager()

    def on_packet_received(self, connection_id: str, raw_data: bytes):
        """The Hot Loop: Entry Point"""
        packet = Packets.Packet()
        packet.ParseFromString(raw_data)

        # 1. Update Session Watchdog
        if connection_id in self.sessions:
            self.sessions[connection_id].last_packet_time = time.time()

        # 2. Route based on Payload Type
        if packet.HasField("telemetry"):
            self.handle_telemetry(connection_id, packet.telemetry)
        elif packet.HasField("grid_sub"):
            self.handle_grid_subscription(connection_id, packet.grid_sub)
        # ... handle other types

    def handle_telemetry(self, conn_id: str, telemetry: Packets.TelemetryUpdate):
        """
        Routing Logic:
        1. Calculate which cell the user is physically inside.
        2. Broadcast this packet to everyone SUBSCRIBED to that cell.
        """
        sender_session = self.sessions.get(conn_id)
        if not sender_session:
            return # Drop packet from unknown session

        # Calculate Z-Order Hash from Vector3 Position
        # Note: In real implementation, this math is bitwise and fast
        current_hash = self.calculate_spatial_hash(telemetry.position)
        
        # Build the address object to query the map
        target_address = Core.GridAddress(spatial_hash=current_hash)

        # GET SUBSCRIBERS
        # We broadcast to anyone listening to the cell the user IS IN.
        # (Subscribers engage the "9-Slice" rule client-side to listen 
        # to their cell + neighbors)
        recipients = self.grid.get_subscribers(target_address)

        # BROADCAST (Simulated)
        for recipient_id in recipients:
            if recipient_id == conn_id:
                continue # Don't echo back to sender
            
            self.send_datagram(recipient_id, telemetry)

    def handle_grid_subscription(self, conn_id: str, sub_req: Packets.GridSubscription):
        """
        Client says: "I moved. Subscribe me to new cells, unsubscribe old ones."
        """
        session = self.sessions.get(conn_id)
        
        # 1. Unsubscribe from old cells (behind the user)
        for old_cell in sub_req.unsubscribe_cells:
            self.grid.unsubscribe(session, old_cell)

        # 2. Subscribe to new cells (ahead of the user)
        for new_cell in sub_req.subscribe_cells:
            self.grid.subscribe(session, new_cell)

    # --- Helper Methods ---

    def calculate_spatial_hash(self, position: Core.Vector3) -> int:
        """
        Converts X,Z world coordinates to a Morton Code.
        See bit-interleaving logic in TDD Section 2.3.
        """
        # Simplistic implementation for demo:
        # Quantize position to Parcel Size
        qx = int(position.x // PARCEL_SIZE)
        qz = int(position.z // PARCEL_SIZE)
        
        # Interleave bits (Z-Order Curve)
        # ... bitwise magic goes here ...
        return interleave_bits(qx, qz)

    def send_datagram(self, target_id, data):
        # Wraps the UDP socket sendto() logic
        pass

def interleave_bits(x, y):
    # Standard Morton coding implementation
    # Maps 2D coords to 1D integer
    return 0 # Placeholder