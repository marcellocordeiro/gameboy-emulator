import { useRef, useState } from "react";

import { useRenderer } from "./hooks/use-renderer";
import {
  SCREEN_HEIGHT,
  SCREEN_WIDTH,
  draw,
  load,
  reset,
  runFrame,
} from "./lib/game-boy";
import { AppShell, Button } from "@mantine/core";

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

    load(data);
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
    <AppShell>
      <AppShell.Main>
        <canvas ref={canvasRef} height={SCREEN_HEIGHT} width={SCREEN_WIDTH} />

        <input
          type="file"
          accept=".gb"
          onChange={async (event) => {
            const file = event.currentTarget.files?.[0];

            if (file != null) {
              setSelectedRom(file);
              await handleStart(file);
            }
          }}
        />

        <Button
          disabled={
            isRunning || (isLoaded && !isRunning) || selectedRom == null
          }
          onClick={async () => {
            if (selectedRom != null) {
              await handleStart(selectedRom);
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
      </AppShell.Main>
    </AppShell>
  );
}
