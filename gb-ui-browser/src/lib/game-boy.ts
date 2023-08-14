// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import init, { init_logging, GameBoy } from "gb-core-wasm";

await init();
init_logging();
const gameBoy = new GameBoy();

export const WIDTH = 160;
export const HEIGHT = 144;

export function loadCartridge(rom: Uint8Array) {
  gameBoy.load_cartridge(rom);
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
