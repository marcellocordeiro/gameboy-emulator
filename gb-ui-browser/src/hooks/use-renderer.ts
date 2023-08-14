import { useEffect, useRef, useState } from "react";

interface Props {
  isLoaded: boolean;
  ctx: CanvasRenderingContext2D | undefined | null;
  runFrame: () => void;
  draw: (ctx: CanvasRenderingContext2D) => void;
}

export function useRenderer({ isLoaded, ctx, runFrame, draw }: Props) {
  const [isRunning, setIsRunning] = useState(false);
  const loopId = useRef(0);

  useEffect(() => {
    if (!isLoaded || !isRunning || ctx == null) {
      window.cancelAnimationFrame(loopId.current);
      return;
    }

    const loop = () => {
      runFrame();
      draw(ctx);

      loopId.current = window.requestAnimationFrame(loop);
    };

    loopId.current = window.requestAnimationFrame(loop);

    return () => {
      setIsRunning(false);
      window.cancelAnimationFrame(loopId.current);
    };
  }, [ctx, draw, isLoaded, runFrame, isRunning]);

  const value = {
    isRunning,
    setIsRunning,
  };

  return value;
}
