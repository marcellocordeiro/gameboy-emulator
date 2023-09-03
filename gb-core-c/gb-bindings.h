#pragma once

#ifdef __cplusplus
#include <cstdint>
#include <cstddef>
#else
#include <stdint.h>
#include <stddef.h>
#endif

#define WIDTH 160
#define HEIGHT 144
#define FRAMEBUFFER_SIZE ((WIDTH * HEIGHT) * sizeof(uint32_t))

#ifdef __cplusplus
extern "C" {
#endif
    struct GameBoy;

    struct GameBoy* gameboy_new();
    void gameboy_reset(struct GameBoy* gb);
    void gameboy_load_cartridge(struct GameBoy* gb, const uint8_t* rom, size_t rom_size);
    void gameboy_run_frame(struct GameBoy* gb);
    void gameboy_draw(struct GameBoy* gb, const uint8_t* frame);
#ifdef __cplusplus
}
#endif
