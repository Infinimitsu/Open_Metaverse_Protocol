#include <string>
#include <functional>

// --- MOCK TYPES (Assuming Proto Headers) ---
struct HandoffRequest { 
    std::string user_id; 
    bool allow_visual_override; 
    bool allow_physics_override; 
    bool allow_script_blocking; 
};

struct HandoffResponse { 
    bool approved; 
    std::string rejection_reason; 
    bool requires_script_blocking;
    // ... other server mandates
};

enum class AuthorityState {
    STREET_MODE,
    NEGOTIATING,
    PARCEL_MODE
};

// --- USER PREFERENCES ---
struct UserPrivacySettings {
    bool allow_visual_overrides = true;  // "Let the server change my skybox?"
    bool allow_physics_overrides = true; // "Let the server change gravity?"
    bool allow_drm_scripts = false;      // "Let server block my tools?"
};

// --- THE MANAGER CLASS ---

class AuthorityManager {
private:
    AuthorityState current_state = AuthorityState::STREET_MODE;
    UserPrivacySettings user_settings;
    std::string current_parcel_id;

    // Callback to Engine to switch physics modes
    std::function<void(bool)> set_street_physics_enabled;

public:
    AuthorityManager(UserPrivacySettings settings) : user_settings(settings) {}

    // Called when player hits a PRIM_GATEWAY
    void request_entry(std::string parcel_id, std::string server_ip) {
        if (current_state != AuthorityState::STREET_MODE) return;

        current_state = AuthorityState::NEGOTIATING;
        current_parcel_id = parcel_id;

        // 1. Build Request based on User Rights
        HandoffRequest req;
        req.user_id = "USER_123"; // Retrieve from Identity Manager
        req.allow_visual_override = user_settings.allow_visual_overrides;
        req.allow_physics_override = user_settings.allow_physics_overrides;
        req.allow_script_blocking = user_settings.allow_drm_scripts;

        // 2. Send via QUIC (Simulated)
        send_packet_to_server(server_ip, req);
    }

    // Called when Server responds
    void on_handoff_response(HandoffResponse res) {
        if (current_state != AuthorityState::NEGOTIATING) return;

        if (res.approved) {
            // 3. Final Compatibility Check
            // Did the server demand something we refused?
            if (res.requires_script_blocking && !user_settings.allow_drm_scripts) {
                // AUTO-REJECT: User rights violation
                abort_entry("Server requires DRM, but User refused.");
                return;
            }

            // 4. Enter Parcel Mode
            current_state = AuthorityState::PARCEL_MODE;
            set_street_physics_enabled(false); // Handover control to server
            // trigger_asset_load(res.manifest);

        } else {
            abort_entry("Server denied entry: " + res.rejection_reason);
        }
    }

    // THE BILL OF RIGHTS: "Kill Switch"
    // This must be bound to a hardware key (e.g., ESC or Panic Button)
    void emergency_exit() {
        if (current_state == AuthorityState::STREET_MODE) return;

        // 1. Sever Connection immediately
        disconnect_from_parcel();

        // 2. Reset State
        current_state = AuthorityState::STREET_MODE;
        current_parcel_id = "";

        // 3. Restore Local Physics
        set_street_physics_enabled(true);

        // 4. UI Feedback
        show_notification("Emergency Exit Activated. Returned to Street.");
    }

private:
    void abort_entry(std::string reason) {
        current_state = AuthorityState::STREET_MODE;
        current_parcel_id = "";
        show_notification("Entry Failed: " + reason);
        // Apply "Bounce" force to player controller to push them out of gateway
    }

    // --- Mock Engine Hooks ---
    void send_packet_to_server(std::string ip, HandoffRequest req) {}
    void disconnect_from_parcel() {}
    void show_notification(std::string msg) {}
};