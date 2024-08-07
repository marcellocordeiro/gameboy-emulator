// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import init, { GameBoy } from "gb-core-wasm";

await init();

const gameBoy = new GameBoy(false);

export const SCREEN_WIDTH = 160;
export const SCREEN_HEIGHT = 144;

export function load(rom: Uint8Array) {
  gameBoy.load(rom, null);
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
