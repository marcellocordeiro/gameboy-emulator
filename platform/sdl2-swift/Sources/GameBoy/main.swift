import Foundation
import GameBoyCore
import SDL

let filePath = CommandLine.arguments[1]
let url = URL(filePath: filePath)
let rom = try [UInt8](Data(contentsOf: url))

let gb = GameBoy()
gb.load(rom)

guard SDL_Init(SDL_INIT_VIDEO) == 0 else {
    fatalError("SDL could not initialize! SDL_Error: \(String(cString: SDL_GetError()))")
}

let window = SDL_CreateWindow(
    "gameboy-emulator",
    Int32(SDL_WINDOWPOS_CENTERED_MASK), Int32(SDL_WINDOWPOS_CENTERED_MASK),
    Int32(GameBoy.width) * 3, Int32(GameBoy.height) * 3,
    SDL_WINDOW_SHOWN.rawValue
)

let renderer = SDL_CreateRenderer(
    window,
    -1,
    SDL_RENDERER_PRESENTVSYNC.rawValue | SDL_RENDERER_ACCELERATED.rawValue
)

SDL_RenderSetLogicalSize(renderer, Int32(GameBoy.width), Int32(GameBoy.height))

let texture = SDL_CreateTexture(
    renderer,
    SDL_PIXELFORMAT_ABGR8888.rawValue,
    Int32(SDL_TEXTUREACCESS_STREAMING.rawValue),
    Int32(GameBoy.width), Int32(GameBoy.height)
)

var frame = [UInt8](repeating: 0, count: GameBoy.width * GameBoy.height * 4)

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
    
    gb.runFrame()
    gb.draw(frame: &frame)

    SDL_UpdateTexture(texture, nil, frame, Int32(GameBoy.width) * 4)
    SDL_RenderCopy(renderer, texture, nil, nil)
    SDL_RenderPresent(renderer)
}

SDL_DestroyRenderer(renderer)
SDL_DestroyTexture(texture)
SDL_DestroyWindow(window)

SDL_Quit()
