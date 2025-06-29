import { useEffect, useRef, useState } from "react";
import { gameBoy } from "@/lib/game-boy";

interface Props {
  isLoaded: boolean;
  ctx: CanvasRenderingContext2D | null | undefined;
}

export function useRenderer({ isLoaded, ctx }: Props) {
  const [isRunning, setIsRunning] = useState(false);
  const loopId = useRef(0);

  useEffect(() => {
    if (!isLoaded || !isRunning || ctx == null) {
      window.cancelAnimationFrame(loopId.current);
      return;
    }

    const loop = () => {
      gameBoy.runFrame();
      gameBoy.draw(ctx);

      loopId.current = window.requestAnimationFrame(loop);
    };

    loopId.current = window.requestAnimationFrame(loop);

    return () => {
      setIsRunning(false);
      window.cancelAnimationFrame(loopId.current);
    };
  }, [ctx, isLoaded, isRunning]);

  return { isRunning, setIsRunning };
}
