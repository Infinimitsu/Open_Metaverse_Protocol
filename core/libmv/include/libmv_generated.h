#ifndef LIBMV_GENERATED_H
#define LIBMV_GENERATED_H

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace mv_core {

constexpr static const float Geography_BLOCK_SIZE = 100.0;

constexpr static const float Geography_ROAD_HALF_WIDTH = 10.0;

struct MvClientConfig {
  const char *user_id;
  const char *auth_token;
  uint32_t vram_budget_mb;
};

struct MvInputCmd {
  float move_x;
  float move_z;
  float look_yaw;
  bool jump;
};

struct MvVector3 {
  float x;
  float y;
  float z;
};

struct MvTransform {
  MvVector3 position;
  MvVector3 rotation;
  MvVector3 scale;
  MvVector3 velocity;
  unsigned int color_hash;
};

extern "C" {

void *mv_core_create(MvClientConfig _config);

void mv_core_connect(void *ptr, const char *url);

void mv_core_destroy(void *ptr);

void mv_core_send_input(void *ptr, MvInputCmd input, float _dt);

void mv_core_tick(void *ptr, float dt);

bool mv_core_get_entity_transform(void *ptr, uint64_t id, MvTransform *out);

int mv_core_get_peer_ids(void *ptr, uint64_t *out_ids, int max_count);

bool mv_core_is_public_street(float x, float z);

} // extern "C"

} // namespace mv_core

#endif // LIBMV_GENERATED_H
