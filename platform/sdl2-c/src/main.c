#include <SDL.h>
#include <gb-bindings.h>

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

  struct GameBoy* gb = gameboy_new();
  gameboy_insert_cartridge(gb, rom, rom_size);

  free(rom);

  uint8_t framebuffer[FRAMEBUFFER_SIZE];

  for (size_t i = 0; i < FRAMEBUFFER_SIZE; ++i) {
    framebuffer[i] = 0;
  }

  SDL_Init(SDL_INIT_VIDEO);

  SDL_Window* window = SDL_CreateWindow(
    "gameboy-emulator",
    SDL_WINDOWPOS_CENTERED,
    SDL_WINDOWPOS_CENTERED,
    SCREEN_WIDTH * 3,
    SCREEN_HEIGHT * 3,
    0
  );

  SDL_Renderer* renderer =
    SDL_CreateRenderer(window, -1, SDL_RENDERER_PRESENTVSYNC | SDL_RENDERER_ACCELERATED);

  SDL_RenderSetLogicalSize(renderer, SCREEN_WIDTH, SCREEN_HEIGHT);

  SDL_Texture* texture = SDL_CreateTexture(
    renderer, SDL_PIXELFORMAT_ABGR8888, SDL_TEXTUREACCESS_STREAMING, SCREEN_WIDTH, SCREEN_HEIGHT
  );

  bool quit = false;

  while (!quit) {
    SDL_Event event;

    while (SDL_PollEvent(&event)) {
      switch (event.type) {
        case SDL_QUIT: quit = true; break;

        default: break;
      }
    }

    gameboy_run_frame(gb);
    gameboy_draw_into_frame_rgba8888(gb, framebuffer);

    SDL_UpdateTexture(texture, nullptr, framebuffer, SCREEN_WIDTH * sizeof(uint32_t));
    SDL_RenderCopy(renderer, texture, nullptr, nullptr);
    SDL_RenderPresent(renderer);
  }

  SDL_DestroyRenderer(renderer);
  SDL_DestroyTexture(texture);
  SDL_DestroyWindow(window);

  gameboy_destroy(gb);

  return 0;
}
