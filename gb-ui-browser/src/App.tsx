import { useRef, useState } from "react";

import { Button } from "@/components/ui/button";
import { useRenderer } from "@/hooks/use-renderer";
import {
  SCREEN_HEIGHT,
  SCREEN_WIDTH,
  draw,
  loadCartridge,
  reset,
  runFrame,
} from "@/lib/game-boy";

export function App() {
  const [isLoaded, setIsLoaded] = useState(false);
  const [selectedRom, setSelectedRom] = useState<File | null>(null);

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const ctx = canvasRef?.current?.getContext("2d");

  const { isRunning, setIsRunning } = useRenderer({
    isLoaded,
    ctx,
    runFrame,
    draw,
  });

  const handleStart = async (file: File) => {
    const buffer = await file.arrayBuffer();
    const data = new Uint8Array(buffer);

    loadCartridge(data);
    setIsLoaded(true);
    setIsRunning(true);
  };

  const handleStop = () => {
    setIsRunning(false);
    setIsLoaded(false);
    reset();
  };

  const handleToggleExecution = () => {
    setIsRunning((oldValue) => !oldValue);
  };

  return (
    <main className="container flex flex-col items-center justify-center gap-4 pt-4">
      <canvas
        ref={canvasRef}
        className="border"
        height={SCREEN_HEIGHT}
        width={SCREEN_WIDTH}
      />

      <input
        type="file"
        accept=".gb"
        onChange={(event) => {
          const file = event.currentTarget.files?.[0];

          if (file != null) {
            setSelectedRom(file);
            handleStart(file);
          }
        }}
      />

      <div className="flex gap-4">
        <Button
          disabled={
            isRunning || (isLoaded && !isRunning) || selectedRom == null
          }
          onClick={() => {
            if (selectedRom != null) {
              handleStart(selectedRom);
            }
          }}
        >
          Start
        </Button>

        <Button disabled={!isLoaded} onClick={handleStop}>
          Stop
        </Button>

        <Button disabled={!isLoaded} onClick={handleToggleExecution}>
          {isRunning ? "Pause" : "Resume"}
        </Button>
      </div>
    </main>
  );
}
