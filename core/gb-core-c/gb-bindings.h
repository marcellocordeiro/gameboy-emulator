#pragma once

#ifdef __cplusplus
#include <cstdint>
#include <cstddef>
#else
#include <stdint.h>
#include <stddef.h>
#endif

#define SCREEN_WIDTH 160
#define SCREEN_HEIGHT 144
#define FRAMEBUFFER_SIZE ((SCREEN_WIDTH * SCREEN_HEIGHT) * sizeof(uint32_t))

#ifdef __cplusplus
extern "C" {
#endif
    struct GameBoy;

    struct GameBoy* gameboy_new();
    void gameboy_destroy(struct GameBoy* gb);
    void gameboy_reset(struct GameBoy* gb);
    void gameboy_load_cartridge(struct GameBoy* gb, const uint8_t* rom, size_t rom_size);
    void gameboy_run_frame(struct GameBoy* gb);
    void gameboy_draw_into_frame_rgba8888(struct GameBoy* gb, const uint8_t* frame);
#ifdef __cplusplus
}
#endif
