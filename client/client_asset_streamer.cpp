#include <string>
#include <queue>
#include <unordered_map>
#include <cmath>

// --- TYPES ---

// Unique ID for an asset (Hash of the file content)
using AssetHash = std::string;

enum class LoadState {
    Unloaded,
    LoadingManifest,
    LoadingGeometry,
    Complete
};

struct AssetRequest {
    AssetHash id;
    float priority; // Calculated based on distance/size
    std::string url;

    // Operator for Priority Queue sorting (Higher priority first)
    bool operator<(const AssetRequest& other) const {
        return priority < other.priority;
    }
};

// --- THE STREAMER CLASS ---

class AssetStreamer {
private:
    // How much VRAM allows? (Simulated)
    size_t vram_budget_mb = 4096;
    size_t current_vram_usage = 0;

    // Queue of assets waiting to download
    std::priority_queue<AssetRequest> load_queue;
    
    // Track state of all known assets
    std::unordered_map<AssetHash, LoadState> asset_states;

public:
    void request_asset(AssetHash id, std::string url, float distance_to_camera, float object_scale) {
        // 1. Check if already loaded
        if (asset_states[id] == LoadState::Complete) return;
        if (asset_states[id] == LoadState::LoadingGeometry) return;

        // 2. Calculate Priority
        // Simple logic: Large objects close to camera = High Priority
        float apparent_size = object_scale / (distance_to_camera + 0.1f);
        float priority = apparent_size;

        // 3. Add to Queue
        AssetRequest req;
        req.id = id;
        req.url = url;
        req.priority = priority;

        load_queue.push(req);
    }

    void update_loading_loop() {
        // Process a few items per frame
        int max_downloads = 3; 

        while (max_downloads > 0 && !load_queue.empty()) {
            AssetRequest req = load_queue.top();
            load_queue.pop();

            // Budget Check
            if (!can_afford_vram(req.id)) {
                // If we can't afford it, we skip it. 
                // The object stays as a Primitive (Cylinder/Box).
                continue; 
            }

            start_download(req);
            max_downloads--;
        }
    }

private:
    bool can_afford_vram(AssetHash id) {
        // In a real engine, we'd check the header of the file 
        // to see estimated texture size.
        size_t estimated_size = 50 * 1024 * 1024; // Assume 50MB for demo
        return (current_vram_usage + estimated_size) <= vram_budget_mb;
    }

    void start_download(AssetRequest req) {
        asset_states[req.id] = LoadState::LoadingGeometry;
        
        // --- REAL IMPLEMENTATION NOTES ---
        // 1. Http Get (req.url)
        // 2. On Success: Parse glTF/USD
        // 3. Upload to GPU
        // 4. asset_states[req.id] = LoadState::Complete;
        
        current_vram_usage += 50; // Mock increment
    }
};