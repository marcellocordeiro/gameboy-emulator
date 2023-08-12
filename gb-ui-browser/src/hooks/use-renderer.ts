import { useEffect, useRef, useState } from "react";

interface Props {
  loaded: boolean;
  ctx: CanvasRenderingContext2D | undefined | null;
  runFrame: () => void;
  draw: (ctx: CanvasRenderingContext2D) => void;
}

export function useRenderer({ loaded, ctx, runFrame, draw }: Props) {
  const [running, setRunning] = useState(false);
  const loopId = useRef(0);

  useEffect(() => {
    if (!loaded || !running || ctx == null) {
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
      setRunning(false);
      window.cancelAnimationFrame(loopId.current);
    };
  }, [ctx, draw, loaded, runFrame, running]);

  const value = {
    running,
    setRunning,
  };

  return value;
}
