import init, { GameBoyWasm } from "core-wasm";
import { useCallback, useState } from "react";

await init();
const gameBoy = new GameBoyWasm();

export const WIDTH = 160;
export const HEIGHT = 144;

export function useGameBoy() {
  const [loaded, setLoaded] = useState(false);

  const loadCartridge = useCallback((rom: Uint8Array) => {
    gameBoy.load_cartridge(rom);
    setLoaded(true);
  }, []);

  const reset = useCallback(() => {
    gameBoy.reset();
    setLoaded(false);
  }, []);

  const runFrame = useCallback(() => {
    gameBoy.run_frame();
  }, []);

  const draw = useCallback((ctx: CanvasRenderingContext2D) => {
    gameBoy.draw(ctx);
  }, []);

  const value = {
    loaded,
    loadCartridge,
    reset,
    runFrame,
    draw,
  };

  return value;
}
