import { useEffect, useRef, useState } from 'react';

export const useMeasuredChart = (minHeight: number): [React.RefObject<HTMLDivElement | null>, { width: number; height: number; ready: boolean }] => {
  const chartRef = useRef<HTMLDivElement>(null);
  const [width, setWidth] = useState(0);
  const [ready, setReady] = useState(false);

  useEffect(() => {
    const node = chartRef.current;
    if (!node) return;

    const measure = () => { setWidth(Math.max(Math.floor(node.getBoundingClientRect().width), 0)); };
    measure();
    setReady(true);

    const observer = new ResizeObserver((entries) => {
      const width = entries[0]?.contentRect.width ?? 0;
      setWidth(Math.max(Math.floor(width), 0));
    });

    observer.observe(node);
    return () => { observer.disconnect(); };
  }, []);

  return [chartRef, { width, height: minHeight, ready }];
};
