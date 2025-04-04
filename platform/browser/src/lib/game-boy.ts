import { GameBoy } from "./gb-core-wasm";

const gameBoy = new GameBoy();

export const SCREEN_WIDTH = 160;
export const SCREEN_HEIGHT = 144;

export function load(rom: Uint8Array) {
  gameBoy.load(rom);
}

export function reset() {
  gameBoy.reset();
}

export function runFrame() {
  gameBoy.run_frame();
}

export function draw(ctx: CanvasRenderingContext2D) {
  gameBoy.draw(ctx);
}
