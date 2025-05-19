#include "app.hpp"

#include <array>
#include <span>
#include <stdexcept>
#include <string_view>
#include <vector>

#include "gb/gb.h"
#include "imgui/imgui.hpp"
#include "sdl/sdl.hpp"
#include "utils/common.hpp"
#include "utils/files.hpp"
#include "utils/scaling.hpp"

namespace {
void render_display(const SDL::Renderer& renderer, const SDL::Texture& texture) {
  const auto availableSize = renderer.get_current_render_output_size();
  const auto rect =
    integer_scale_centered_rect(availableSize, {.width = SCREEN_WIDTH, .height = SCREEN_HEIGHT});

  SDL_RenderTexture(renderer.get(), texture.get(), nullptr, &rect);
}
} // namespace

App::App(std::span<std::string_view> args) : args(args) {
  if (args.size() == 1) {
    throw std::invalid_argument("Too few arguments");
  }
}

auto App::run() -> void {
  auto rom = read_binary_file(args[1]);

  auto* gb = gameboy_new(true);
  gameboy_load(gb, {.data = nullptr, .size = 0}, {.data = rom.data(), .size = rom.size()});

  std::array<u8, FRAMEBUFFER_SIZE> framebuffer = {};

  auto context = SDL::Context{SDL_INIT_VIDEO | SDL_INIT_GAMEPAD};

  const auto window_flags = SDL_WindowFlags{SDL_WINDOW_RESIZABLE | SDL_WINDOW_HIDDEN};
  const auto window = SDL::Window("gameboy-emulator", 1280, 720, window_flags);
  const auto renderer = SDL::Renderer(window);

  renderer.enable_vsync();
  window.set_window_position(SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED);
  window.show_window();

  const auto texture = SDL::Texture(
    renderer,
    SDL_PIXELFORMAT_ABGR8888,
    SDL_TEXTUREACCESS_STREAMING,
    SCREEN_WIDTH,
    SCREEN_HEIGHT
  );

  texture.set_scale_mode(SDL_SCALEMODE_NEAREST);

  // Setup Dear ImGui context
  IMGUI_CHECKVERSION();
  ImGui::CreateContext();
  ImGuiIO& io = ImGui::GetIO();
  (void)io;
  io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard; // Enable Keyboard Controls
  io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;  // Enable Gamepad Controls
  io.IniFilename = nullptr;

  // Setup Dear ImGui style
  ImGui::StyleColorsDark();
  //ImGui::StyleColorsLight();

  // Setup Platform/Renderer backends
  ImGui_ImplSDL3_InitForSDLRenderer(window.get(), renderer.get());
  ImGui_ImplSDLRenderer3_Init(renderer.get());

  // Load Fonts
  // - If no fonts are loaded, dear imgui will use the default font. You can also load multiple fonts and use ImGui::PushFont()/PopFont() to select them.
  // - AddFontFromFileTTF() will return the ImFont* so you can store it if you need to select the font among multiple.
  // - If the file cannot be loaded, the function will return a nullptr. Please handle those errors in your application (e.g. use an assertion, or display an error and quit).
  // - The fonts will be rasterized at a given size (w/ oversampling) and stored into a texture when calling ImFontAtlas::Build()/GetTexDataAsXXXX(), which ImGui_ImplXXXX_NewFrame below will call.
  // - Use '#define IMGUI_ENABLE_FREETYPE' in your imconfig file to use Freetype for higher quality font rendering.
  // - Read 'docs/FONTS.md' for more instructions and details.
  // - Remember that in C/C++ if you want to include a backslash \ in a string literal you need to write a double backslash \\ !
  // - Our Emscripten build process allows embedding fonts to be accessible at runtime from the "fonts/" folder. See Makefile.emscripten for details.
  //io.Fonts->AddFontDefault();
  //io.Fonts->AddFontFromFileTTF("c:\\Windows\\Fonts\\segoeui.ttf", 18.0f);
  //io.Fonts->AddFontFromFileTTF("../../misc/fonts/DroidSans.ttf", 16.0f);
  //io.Fonts->AddFontFromFileTTF("../../misc/fonts/Roboto-Medium.ttf", 16.0f);
  //io.Fonts->AddFontFromFileTTF("../../misc/fonts/Cousine-Regular.ttf", 15.0f);
  //ImFont* font = io.Fonts->AddFontFromFileTTF("c:\\Windows\\Fonts\\ArialUni.ttf", 18.0f, nullptr, io.Fonts->GetGlyphRangesJapanese());
  //IM_ASSERT(font != nullptr);

  // Our state
  bool show_demo_window = true;

  // Main loop
  bool done = false;
  while (!done) {
    // Poll and handle events (inputs, window resize, etc.)
    // You can read the io.WantCaptureMouse, io.WantCaptureKeyboard flags to tell if dear imgui wants to use your inputs.
    // - When io.WantCaptureMouse is true, do not dispatch mouse input data to your main application, or clear/overwrite your copy of the mouse data.
    // - When io.WantCaptureKeyboard is true, do not dispatch keyboard input data to your main application, or clear/overwrite your copy of the keyboard data.
    // Generally you may always pass all inputs to dear imgui, and hide them from your application based on those two flags.
    // [If using SDL_MAIN_USE_CALLBACKS: call ImGui_ImplSDL3_ProcessEvent() from your SDL_AppEvent() function]
    SDL_Event event;
    while (SDL_PollEvent(&event)) {
      ImGui_ImplSDL3_ProcessEvent(&event);

      if (event.type == SDL_EVENT_QUIT) {
        done = true;
      }

      if (event.type == SDL_EVENT_WINDOW_CLOSE_REQUESTED
          && event.window.windowID == SDL_GetWindowID(window.get()))
      {
        done = true;
      }
    }

    // [If using SDL_MAIN_USE_CALLBACKS: all code below would likely be your SDL_AppIterate() function]
    if ((SDL_GetWindowFlags(window.get()) & SDL_WINDOW_MINIMIZED) != 0) {
      SDL_Delay(10);
      continue;
    }

    // Start the Dear ImGui frame
    ImGui_ImplSDLRenderer3_NewFrame();
    ImGui_ImplSDL3_NewFrame();
    ImGui::NewFrame();

    // 1. Show the big demo window (Most of the sample code is in ImGui::ShowDemoWindow()! You can browse its code to learn more about Dear ImGui!).
    if (show_demo_window) {
      ImGui::ShowDemoWindow(&show_demo_window);
    }

    // 2. Show a simple window that we create ourselves. We use a Begin/End pair to create a named window.
    {
      static int counter = 0;

      // Create a window called "Hello, world!" and append into it.
      ImGui::Begin("Hello, world!");

      // Display some text (you can use a format strings too)
      ImGui::Text("This is some useful text.");

      // Edit bools storing our window open/close state
      ImGui::Checkbox("Demo Window", &show_demo_window);

      // Buttons return true when clicked (most widgets return true when edited/activated)
      if (ImGui::Button("Button")) {
        counter++;
      }
      ImGui::SameLine();
      ImGui::Text("counter = %d", counter);

      auto framerate = static_cast<double>(io.Framerate);
      ImGui::Text("Application average %.3f ms/frame (%.1f FPS)", 1000.0 / framerate, framerate);
      ImGui::End();
    }

    gameboy_run_frame(gb);
    gameboy_draw_into_frame_rgba8888(gb, framebuffer.data());
    SDL_UpdateTexture(texture.get(), nullptr, framebuffer.data(), SCREEN_WIDTH * sizeof(u32));

    // Rendering
    ImGui::Render();
    SDL_RenderClear(renderer.get());

    render_display(renderer, texture);

    ImGui_ImplSDLRenderer3_RenderDrawData(ImGui::GetDrawData(), renderer.get());

    SDL_RenderPresent(renderer.get());
  }

  // Cleanup
  // [If using SDL_MAIN_USE_CALLBACKS: all code below would likely be your SDL_AppQuit() function]
  ImGui_ImplSDLRenderer3_Shutdown();
  ImGui_ImplSDL3_Shutdown();
  ImGui::DestroyContext();
}
