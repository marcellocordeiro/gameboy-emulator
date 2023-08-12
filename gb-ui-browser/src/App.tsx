import { useRef, useState } from "react";

import { Button } from "@/components/ui/button";
import { HEIGHT, WIDTH, useGameBoy } from "@/hooks/use-game-boy";
import { useRenderer } from "@/hooks/use-renderer";

export function App() {
  const [selectedRom, setSelectedRom] = useState<File | null>(null);

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const ctx = canvasRef?.current?.getContext("2d");

  const { loaded, loadCartridge, reset, runFrame, draw } = useGameBoy();
  const { running, setRunning } = useRenderer({ loaded, ctx, runFrame, draw });

  const handleStart = async (file: File) => {
    const buffer = await file.arrayBuffer();
    const data = new Uint8Array(buffer);

    loadCartridge(data);
    setRunning(true);
  };

  const handleStop = () => {
    setRunning(false);
    reset();
  };

  const handleToggleExecution = () => {
    setRunning((oldValue) => !oldValue);
  };

  return (
    <main className="container flex flex-col items-center justify-center gap-4 pt-4">
      <canvas
        ref={canvasRef}
        className="border"
        height={HEIGHT}
        width={WIDTH}
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
          disabled={running || (loaded && !running) || selectedRom == null}
          onClick={() => {
            if (selectedRom != null) {
              handleStart(selectedRom);
            }
          }}
        >
          Start
        </Button>

        <Button disabled={!loaded} onClick={handleStop}>
          Stop
        </Button>

        <Button disabled={!loaded} onClick={handleToggleExecution}>
          {running ? "Pause" : "Resume"}
        </Button>
      </div>
    </main>
  );
}
