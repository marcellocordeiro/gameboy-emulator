import Foundation
import GameBoyCore
import SDL

let filePath = CommandLine.arguments[1]

guard let data = NSData(contentsOfFile: filePath) else {
    exit(1)
}

let gb = gameboy_new()
gameboy_load_cartridge(gb, data.bytes, data.length)

let SCREEN_WIDTH = Int32(160)
let SCREEN_HEIGHT = Int32(144)

guard SDL_Init(SDL_INIT_VIDEO) == 0 else {
    fatalError("SDL could not initialize! SDL_Error: \(String(cString: SDL_GetError()))")
}

let window = SDL_CreateWindow(
    "gameboy-emulator",
    Int32(SDL_WINDOWPOS_CENTERED_MASK), Int32(SDL_WINDOWPOS_CENTERED_MASK),
    SCREEN_WIDTH * 3, SCREEN_HEIGHT * 3,
    SDL_WINDOW_SHOWN.rawValue
)

let renderer = SDL_CreateRenderer(
    window,
    -1,
    SDL_RENDERER_PRESENTVSYNC.rawValue | SDL_RENDERER_ACCELERATED.rawValue
)

SDL_RenderSetLogicalSize(renderer, SCREEN_WIDTH, SCREEN_HEIGHT)

let texture = SDL_CreateTexture(
    renderer,
    SDL_PIXELFORMAT_ABGR8888.rawValue,
    Int32(SDL_TEXTUREACCESS_STREAMING.rawValue),
    SCREEN_WIDTH, SCREEN_HEIGHT
)

var framebuffer = [UInt8](repeating: 0, count: Int(SCREEN_WIDTH * SCREEN_HEIGHT) * 4)

var quit = false
var event = SDL_Event()

while !quit {
    while SDL_PollEvent(&event) != 0 {
        switch SDL_EventType(event.type) {
        case SDL_QUIT,
             SDL_KEYUP where event.key.keysym.sym == SDLK_ESCAPE.rawValue:
            quit = true

        default:
            break
        }
    }

    gameboy_run_frame(gb)
    gameboy_draw(gb, &framebuffer)

    SDL_UpdateTexture(texture, nil, framebuffer, SCREEN_WIDTH * 4)
    SDL_RenderCopy(renderer, texture, nil, nil)
    SDL_RenderPresent(renderer)
}

SDL_DestroyRenderer(renderer)
SDL_DestroyTexture(texture)
SDL_DestroyWindow(window)

SDL_Quit()
