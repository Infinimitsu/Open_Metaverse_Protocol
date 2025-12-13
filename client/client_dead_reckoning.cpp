#include <vector>
#include <cmath>
#include <map>

// --- CORE DATA STRUCTURES (Mirroring the Proto) ---

struct Vector3 { float x, y, z; };
struct Quaternion { float x, y, z, w; };

struct StateSnapshot {
    double timestamp;   // Time this packet was SENT by the relay
    Vector3 position;
    Vector3 velocity;
    Quaternion rotation;
};

// --- THE INTERPOLATOR CLASS ---

class EntityInterpolator {
private:
    // We store a history of state updates received from the network
    std::vector<StateSnapshot> state_buffer;
    
    // Config: How far behind real-time do we render? (e.g., 100ms)
    // This allows us to smoothly blend between known points.
    double interpolation_delay = 0.1; 

public:
    void on_packet_received(StateSnapshot new_state) {
        // Add new packet to buffer
        state_buffer.push_back(new_state);
        
        // Cleanup old history (keep last 1 second)
        // ... (Cleanup logic omitted for brevity) ...
    }

    void get_render_transform(double current_client_time, Vector3& out_pos, Quaternion& out_rot) {
        // 1. Calculate the "Render Time" (Past)
        double render_time = current_client_time - interpolation_delay;

        // 2. Find the two snapshots that sandwich our render_time
        StateSnapshot* prev = nullptr;
        StateSnapshot* next = nullptr;

        for (int i = state_buffer.size() - 1; i >= 0; i--) {
            if (state_buffer[i].timestamp <= render_time) {
                prev = &state_buffer[i];
                if (i + 1 < state_buffer.size()) {
                    next = &state_buffer[i + 1];
                }
                break;
            }
        }

        // 3. LOGIC BRANCH: INTERPOLATION vs EXTRAPOLATION

        if (prev && next) {
            // CASE A: INTERPOLATION (We have a start and end point)
            // Calculate how far we are between Prev and Next (0.0 to 1.0)
            double total_time = next->timestamp - prev->timestamp;
            double current_delta = render_time - prev->timestamp;
            float t = (float)(current_delta / total_time);

            // Linear Interpolation (Lerp) for position
            // (In production, use Hermite Spline for curves)
            out_pos = lerp(prev->position, next->position, t);
            
            // Spherical Interpolation (Slerp) for rotation
            out_rot = slerp(prev->rotation, next->rotation, t);

        } else if (prev && !next) {
            // CASE B: EXTRAPOLATION (We ran out of data!)
            // We have to guess where they are going based on velocity.
            double time_since_update = render_time - prev->timestamp;
            
            // Limit extrapolation to avoid "shooting off into space"
            if (time_since_update > 0.5) time_since_update = 0.5;

            // Project: P_new = P_old + (Velocity * Time)
            out_pos.x = prev->position.x + (prev->velocity.x * time_since_update);
            out_pos.y = prev->position.y + (prev->velocity.y * time_since_update);
            out_pos.z = prev->position.z + (prev->velocity.z * time_since_update);
            
            out_rot = prev->rotation; // Keep rotation same (risky to extrapolate rotation)
        }
    }

    // --- MATH HELPERS ---
    
    Vector3 lerp(Vector3 a, Vector3 b, float t) {
        return {
            a.x + (b.x - a.x) * t,
            a.y + (b.y - a.y) * t,
            a.z + (b.z - a.z) * t
        };
    }

    Quaternion slerp(Quaternion a, Quaternion b, float t) {
        // ... Standard Quaternion Slerp implementation ...
        return a; // Placeholder
    }
};