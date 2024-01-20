#pragma once

#ifdef __cplusplus
#include <cstddef>
#include <cstdint>
#else
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#endif

#define SCREEN_WIDTH 160
#define SCREEN_HEIGHT 144
#define FRAMEBUFFER_SIZE ((SCREEN_WIDTH * SCREEN_HEIGHT) * sizeof(uint32_t))

#ifdef __cplusplus
extern "C" {
#endif
enum GameBoyButton {
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

struct GameBoy* gameboy_new();
void gameboy_destroy(struct GameBoy* gb_ptr);
void gameboy_reset(struct GameBoy* gb_ptr);
void gameboy_load_cartridge(struct GameBoy* gb_ptr, const uint8_t* rom, uintptr_t rom_size);
void gameboy_run_frame(struct GameBoy* gb_ptr);
void gameboy_set_key(struct GameBoy* gb_ptr, enum GameBoyButton button, bool value);
void gameboy_key_up(struct GameBoy* gb_ptr, enum GameBoyButton button);
void gameboy_key_down(struct GameBoy* gb_ptr, enum GameBoyButton button);
void gameboy_draw_into_frame_rgba8888(struct GameBoy* gb_ptr, uint8_t* frame);
#ifdef __cplusplus
}
#endif
