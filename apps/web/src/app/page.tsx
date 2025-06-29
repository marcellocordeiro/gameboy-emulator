"use client";

import { useRef, useState } from "react";
import { ModeToggle } from "@/components/mode-toggle";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useRenderer } from "@/hooks/use-renderer";
import { GameBoy, gameBoy } from "@/lib/game-boy";

export default function Home() {
  const [isLoaded, setIsLoaded] = useState(false);
  const [selectedRom, setSelectedRom] = useState<File | null>(null);

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const ctx = canvasRef.current?.getContext("2d");

  const { isRunning, setIsRunning } = useRenderer({ isLoaded, ctx });

  async function handleStart(file: File) {
    const buffer = await file.arrayBuffer();
    const data = new Uint8Array(buffer);

    gameBoy.load(data);
    setIsLoaded(true);
    setIsRunning(true);
  }

  function handleStop() {
    setIsRunning(false);
    setIsLoaded(false);
    gameBoy.reset();
  }

  function handleToggleExecution() {
    setIsRunning((oldValue) => !oldValue);
  }

  return (
    <main>
      <canvas
        ref={canvasRef}
        height={GameBoy.screenHeight()}
        width={GameBoy.screenWidth()}
      />

      <div>
        <Input
          type="file"
          accept=".gb,.gbc"
          onChange={async (event) => {
            const file = event.currentTarget.files?.[0];

            if (file == null) {
              return;
            }

            setSelectedRom(file);
            await handleStart(file);
          }}
        />

        <div>
          <Button
            disabled={isRunning || isLoaded || selectedRom == null}
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

          <ModeToggle />
        </div>
      </div>
    </main>
  );
}
