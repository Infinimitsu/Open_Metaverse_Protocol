#include "raylib.h"
#include "rlgl.h" 
#include "raymath.h"
#include "libmv_generated.h"
#include <iostream>
#include <vector>
#include <cmath> 

Color HashToColor(unsigned int hash) {
    unsigned char r = (hash >> 16) & 0xFF;
    unsigned char g = (hash >> 8) & 0xFF;
    unsigned char b = hash & 0xFF;

    Color c;
    c.r = r; c.g = g; c.b = b; c.a = 255;
    return c;
}

int main() {
    InitWindow(1280, 720, "Metaverse Protocol - The Street");
    SetTargetFPS(60);

    DisableCursor();
    bool cursorLocked = true;

    float camDist = 15.0f;
    float camYaw = 0.0f;
    float camPitch = 0.5f;

    Camera3D camera = { 0 };
    camera.up = { 0.0f, 1.0f, 0.0f };
    camera.fovy = 45.0f;
    camera.projection = CAMERA_PERSPECTIVE;

    // --- RUST INTEROP ---
    mv_core::MvClientConfig config;
    config.user_id = "User";
    config.auth_token = "token";
    config.vram_budget_mb = 1024;

    void* core_ptr = mv_core::mv_core_create(config);
    mv_core::mv_core_connect(core_ptr, "127.0.0.1:4433");

    uint64_t peer_ids[100];

    while (!WindowShouldClose()) {
        float dt = GetFrameTime();

        // --- INPUT ---
        if (IsMouseButtonPressed(MOUSE_BUTTON_RIGHT)) {
            cursorLocked = !cursorLocked;
            if (cursorLocked) DisableCursor();
            else EnableCursor();
        }

        if (cursorLocked) {
            Vector2 mouseDelta = GetMouseDelta();
            camYaw -= mouseDelta.x * 0.003f;
            camPitch += mouseDelta.y * 0.003f;

            if (camPitch < 0.1f) camPitch = 0.1f;
            if (camPitch > 1.5f) camPitch = 1.5f;

            camDist -= GetMouseWheelMove() * 1.0f;
            if (camDist < 2.0f) camDist = 2.0f;
            if (camDist > 50.0f) camDist = 50.0f;
        }

        float sinYaw = sinf(camYaw);
        float cosYaw = cosf(camYaw);

        float local_x = 0.0f;
        float local_z = 0.0f;

        if (IsKeyDown(KEY_W)) local_z = -1.0f;
        if (IsKeyDown(KEY_S)) local_z = 1.0f;
        if (IsKeyDown(KEY_A)) local_x = -1.0f;
        if (IsKeyDown(KEY_D)) local_x = 1.0f;

        float fwdX = -sinYaw;
        float fwdZ = -cosYaw;
        float rightX = cosYaw;
        float rightZ = -sinYaw;

        mv_core::MvInputCmd input = { 0 };
        input.move_x = (fwdX * -local_z) + (rightX * local_x);
        input.move_z = (fwdZ * -local_z) + (rightZ * local_x);
        input.look_yaw = camYaw;
        input.jump = IsKeyDown(KEY_SPACE);

        mv_core::mv_core_send_input(core_ptr, input, dt);
        mv_core::mv_core_tick(core_ptr, dt);

        // --- CAMERA ---
        mv_core::MvTransform localTransform;
        Vector3 playerPos = { 0,0,0 };

        if (mv_core::mv_core_get_entity_transform(core_ptr, 0, &localTransform)) {
            playerPos = { localTransform.position.x, localTransform.position.y, localTransform.position.z };
            camera.target = playerPos;

            float offsetX = camDist * sinf(camYaw) * cosf(camPitch);
            float offsetZ = camDist * cosf(camYaw) * cosf(camPitch);
            float offsetY = camDist * sinf(camPitch);

            camera.position = {
                playerPos.x + offsetX,
                playerPos.y + offsetY,
                playerPos.z + offsetZ
            };
        }

        // --- RENDER ---
        BeginDrawing();
        ClearBackground(RAYWHITE);
        BeginMode3D(camera);

        // DRAW THE STREET (Procedural Floor)
        int drawDist = 100;
        int startX = (int)playerPos.x - drawDist;
        int endX = (int)playerPos.x + drawDist;
        int startZ = (int)playerPos.z - drawDist;
        int endZ = (int)playerPos.z + drawDist;

        // ALIGNMENT FIX:
        // We want tiles centered at ... -5, 5, 15, 25 ...
        // This ensures the edges fall on ... -10, 0, 10, 20 ... which matches the math.
        // floor() used to handle negative coordinates correctly.
        startX = (int)floor(startX / 10.0f) * 10 + 5;
        startZ = (int)floor(startZ / 10.0f) * 10 + 5;

        for (int x = startX; x < endX; x += 10) {
            for (int z = startZ; z < endZ; z += 10) {
                // Rust Logic determines zoning
                bool isStreet = mv_core::mv_core_is_public_street((float)x, (float)z);

                Vector3 tilePos = { (float)x, -0.1f, (float)z };
                Color tileColor = isStreet ? DARKGRAY : LIGHTGRAY;

                DrawCube(tilePos, 10.0f, 0.1f, 10.0f, tileColor);
                DrawCubeWires(tilePos, 10.0f, 0.1f, 10.0f, GRAY);
            }
        }

        // Entities
        mv_core::MvTransform t;

        // Local
        if (mv_core::mv_core_get_entity_transform(core_ptr, 0, &t)) {
            Vector3 pos = { t.position.x, t.position.y, t.position.z };
            float rotationDeg = camYaw * (180.0f / PI);
            rlPushMatrix();
            rlTranslatef(pos.x, pos.y, pos.z);
            rlRotatef(rotationDeg, 0, 1, 0);
            DrawCube(Vector3{ 0,0,0 }, 1.0f, 1.0f, 1.0f, BLUE);
            DrawCubeWires(Vector3{ 0,0,0 }, 1.0f, 1.0f, 1.0f, DARKBLUE);
            DrawCube(Vector3{ 0, 0.5f, -0.5f }, 0.2f, 0.2f, 0.2f, YELLOW);
            rlPopMatrix();
        }

        // Peers
        int count = mv_core::mv_core_get_peer_ids(core_ptr, peer_ids, 100);
        for (int i = 0; i < count; i++) {
            uint64_t id = peer_ids[i];
            if (mv_core::mv_core_get_entity_transform(core_ptr, id, &t)) {
                Vector3 pos = { t.position.x, t.position.y, t.position.z };
                Color idColor = HashToColor(t.color_hash);
                DrawCube(pos, 1.0f, 1.0f, 1.0f, idColor);
                DrawCubeWires(pos, 1.0f, 1.0f, 1.0f, BLACK);
            }
        }

        EndMode3D();

        // HUD
        DrawText(TextFormat("Pos: %.1f, %.1f, %.1f", playerPos.x, playerPos.y, playerPos.z), 10, 10, 20, BLACK);
        bool onStreet = mv_core::mv_core_is_public_street(playerPos.x, playerPos.z);
        DrawText(onStreet ? "Zone: PUBLIC STREET" : "Zone: PRIVATE PARCEL", 10, 35, 20, onStreet ? GREEN : ORANGE);

        EndDrawing();
    }

    mv_core::mv_core_destroy(core_ptr);
    CloseWindow();

    return 0;
}