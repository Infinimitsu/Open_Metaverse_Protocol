#include "raylib.h"
#include "raymath.h"
#include "libmv_generated.h"
#include <iostream>

int main() {
    InitWindow(1280, 720, "Metaverse Protocol - Rust Core Client");
    SetTargetFPS(60);

    Camera3D camera = { 0 };
    camera.position = { 10.0f, 10.0f, 10.0f };
    camera.target = { 0.0f, 0.0f, 0.0f };
    camera.up = { 0.0f, 1.0f, 0.0f };
    camera.fovy = 45.0f;
    camera.projection = CAMERA_PERSPECTIVE;

    // --- RUST INTEROP ---
    
    // 1. Create Config (Using new MvClientConfig struct)
    mv_core::MvClientConfig config;
    config.user_id = "Rust_User";
    config.auth_token = "token";
    config.vram_budget_mb = 1024;

    // 2. Initialize Core
    void* core_ptr = mv_core::mv_core_create(config);
    std::cout << "Rust Core Initialized: " << core_ptr << std::endl;

    // 3. Connect
    mv_core::mv_core_connect(core_ptr, "127.0.0.1:4433");

    while (!WindowShouldClose()) {
        float dt = GetFrameTime();
        
        // --- INPUT HANDLING ---
        mv_core::MvInputCmd input = { 0 };
        if (IsKeyDown(KEY_W)) input.move_z = 1.0f; // Forward
        if (IsKeyDown(KEY_S)) input.move_z = -1.0f; // Backward
        if (IsKeyDown(KEY_A)) input.move_x = 1.0f; // Left
        if (IsKeyDown(KEY_D)) input.move_x = -1.0f; // Right
        
        // 4. Send Input to Core
        mv_core::mv_core_send_input(core_ptr, input, dt);
        mv_core::mv_core_tick(core_ptr, dt);

        UpdateCamera(&camera, CAMERA_ORBITAL);

        // --- RENDER ---
        mv_core::MvTransform t;
        // 5. Get Entity Transform (ID 100)
        bool exists = mv_core::mv_core_get_entity_transform(core_ptr, 100, &t);

        BeginDrawing();
            ClearBackground(RAYWHITE);
            BeginMode3D(camera);
                DrawGrid(20, 1.0f);

                if (exists) {
                    // Map MvVector3 to Raylib Vector3
                    Vector3 pos = { t.position.x, t.position.y, t.position.z };
                    DrawCube(pos, 1.0f, 1.0f, 1.0f, BLUE);
                    DrawCubeWires(pos, 1.0f, 1.0f, 1.0f, DARKBLUE);
                }

            EndMode3D();
            DrawText("WASD to Move the Cube", 10, 10, 20, GRAY);
        EndDrawing();
    }

    // 6. Cleanup
    mv_core::mv_core_destroy(core_ptr);
    CloseWindow();

    return 0;
}