#pragma once

#ifdef __cplusplus
#include <cstddef>
#include <cstdint>
#else
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#endif

#define SCREEN_WIDTH ((size_t)160)
#define SCREEN_HEIGHT ((size_t)144)
#define FRAMEBUFFER_SIZE ((SCREEN_WIDTH * SCREEN_HEIGHT) * sizeof(uint32_t))

#ifdef __cplusplus
extern "C" {
#endif
enum Button {
  A = 0,
  B = 1,
  SELECT = 2,
  START = 3,
  RIGHT = 4,
  LEFT = 5,
  UP = 6,
  DOWN = 7,
};

struct GameBoy;

struct Bootrom {
  const uint8_t* data;
  size_t size;
};

struct Rom {
  const uint8_t* data;
  size_t size;
};

struct GameBoy* gameboy_new(bool is_cgb);
void gameboy_destroy(struct GameBoy* gb_ptr);
void gameboy_reset(struct GameBoy* gb_ptr);
bool gameboy_load(struct GameBoy* gb_ptr, struct Bootrom bootrom, struct Rom rom);
void gameboy_run_frame(struct GameBoy* gb_ptr);
void gameboy_set_joypad_button(struct GameBoy* gb_ptr, enum Button button, bool value);
void gameboy_joypad_button_up(struct GameBoy* gb_ptr, enum Button button);
void gameboy_joypad_button_down(struct GameBoy* gb_ptr, enum Button button);
void gameboy_draw_into_frame_rgba8888(struct GameBoy* gb_ptr, uint8_t* frame);
#ifdef __cplusplus
}
#endif
