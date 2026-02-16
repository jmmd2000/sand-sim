import { useRef } from "react";

export function usePainting(canvasRef: React.RefObject<HTMLCanvasElement | null>, W: number, H: number, onPaint: (x: number, y: number) => void) {
  const isPainting = useRef(false);
  const mousePos = useRef({ x: 0, y: 0 });
  const paintTimer = useRef<number | null>(null);

  const getPos = (e: React.PointerEvent) => {
    const r = canvasRef.current!.getBoundingClientRect();
    return {
      x: Math.floor(((e.clientX - r.left) / r.width) * W),
      y: Math.floor(((e.clientY - r.top) / r.height) * H),
    };
  };

  const onPointerDown = (e: React.PointerEvent) => {
    canvasRef.current!.setPointerCapture(e.pointerId);
    isPainting.current = true;

    const pos = getPos(e);
    mousePos.current = pos;

    onPaint(pos.x, pos.y);

    paintTimer.current = window.setInterval(() => {
      if (isPainting.current) onPaint(mousePos.current.x, mousePos.current.y);
    }, 16);
  };

  const onPointerMove = (e: React.PointerEvent) => {
    if (!isPainting.current) return;

    const pos = getPos(e);
    mousePos.current = pos;

    onPaint(pos.x, pos.y);
  };

  const onPointerUp = () => {
    isPainting.current = false;

    if (paintTimer.current) {
      clearInterval(paintTimer.current);
      paintTimer.current = null;
    }
  };

  return { onPointerDown, onPointerMove, onPointerUp };
}
