#include <stdio.h>
#include <stdlib.h>

#include <SDL3/SDL.h>

#include <gb/gb.h>

uint64_t get_file_size(FILE* file) {
  fseek(file, 0, SEEK_END);

  uint64_t rom_size = ftell(file);

  fseek(file, 0, SEEK_SET);

  return rom_size;
}

int main(int argc, char* argv[]) {
  if (argc < 2) {
    return 1;
  }

  FILE* file = fopen(argv[1], "rb");

  uint64_t rom_size = get_file_size(file);
  uint8_t* rom = malloc(rom_size);

  fread(rom, sizeof(uint8_t), rom_size, file);
  fclose(file);

  struct Bootrom gbBootrom = {.data = nullptr, .size = 0};

  struct Rom gbRom = {.data = rom, .size = rom_size};

  struct GameBoy* gb = gameboy_new(true);
  gameboy_load(gb, gbBootrom, gbRom);

  free(rom);

  uint8_t framebuffer[FRAMEBUFFER_SIZE];

  for (size_t i = 0; i < FRAMEBUFFER_SIZE; ++i) {
    framebuffer[i] = 0;
  }

  //SDL_SetAppMetadata("gb", "1.0.0", "com.emulator.gb");

  if (!SDL_Init(SDL_INIT_VIDEO)) {
    SDL_Log("SDL_Init: %s", SDL_GetError());
    return -1;
  }

  SDL_Window* window = nullptr;
  SDL_Renderer* renderer = nullptr;

  if (!SDL_CreateWindowAndRenderer(
        "gameboy-emulator",
        SCREEN_WIDTH * 3,
        SCREEN_HEIGHT * 3,
        0,
        &window,
        &renderer
      ))
  {
    SDL_Log("SDL_CreateWindowAndRenderer: %s", SDL_GetError());
    return -1;
  }

  if (!SDL_SetRenderVSync(renderer, 1)) {
    SDL_Log("SDL_SetRenderVSync: %s", SDL_GetError());
    return -1;
  }

  if (!SDL_SetRenderLogicalPresentation(
        renderer,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        SDL_LOGICAL_PRESENTATION_INTEGER_SCALE
      ))
  {
    SDL_Log("SDL_SetRenderLogicalPresentation: %s", SDL_GetError());
    return -1;
  }

  SDL_Texture* texture = SDL_CreateTexture(
    renderer,
    SDL_PIXELFORMAT_ABGR8888,
    SDL_TEXTUREACCESS_STREAMING,
    SCREEN_WIDTH,
    SCREEN_HEIGHT
  );

  bool quit = false;

  while (!quit) {
    SDL_Event event;

    while (SDL_PollEvent(&event)) {
      switch (event.type) {
      case SDL_EVENT_QUIT: quit = true; break;

      default: break;
      }
    }

    gameboy_run_frame(gb);
    gameboy_draw_into_frame_rgba8888(gb, framebuffer);

    SDL_UpdateTexture(texture, nullptr, framebuffer, SCREEN_WIDTH * sizeof(uint32_t));
    SDL_RenderTexture(renderer, texture, nullptr, nullptr);
    SDL_RenderPresent(renderer);
  }

  SDL_DestroyRenderer(renderer);
  SDL_DestroyTexture(texture);
  SDL_DestroyWindow(window);

  gameboy_destroy(gb);

  return 0;
}
