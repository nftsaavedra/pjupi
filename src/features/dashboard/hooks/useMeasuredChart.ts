import { useEffect, useState } from 'react';

export const useMeasuredChart = (minHeight: number) => {
  const [node, setNode] = useState<HTMLDivElement | null>(null);
  const [width, setWidth] = useState(0);

  useEffect(() => {
    if (!node) {
      setWidth(0);
      return;
    }

    const element = node;

    const updateWidth = (nextWidth: number) => {
      setWidth(Math.max(Math.floor(nextWidth), 0));
    };

    const measure = () => updateWidth(element.getBoundingClientRect().width);

    measure();
    const rafId = window.requestAnimationFrame(measure);

    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) return;
      updateWidth(entry.contentRect.width);
    });

    observer.observe(element);
    return () => {
      window.cancelAnimationFrame(rafId);
      observer.disconnect();
    };
  }, [node]);

  return {
    ref: setNode,
    width,
    height: minHeight,
    ready: width > 0,
  };
};