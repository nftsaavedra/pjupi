import { useEffect, useRef } from 'react';
import { toast } from '../../services/toast';

interface UseRefreshToastOptions {
  refreshing: boolean;
  message: string;
  toastKey: string;
  duration?: number;
  delayMs?: number;
  cooldownMs?: number;
}

const lastToastAtByKey = new Map<string, number>();

export const useRefreshToast = ({
  refreshing,
  message,
  toastKey,
  duration = 1800,
  delayMs = 450,
  cooldownMs = 90000,
}: UseRefreshToastOptions) => {
  const wasRefreshingRef = useRef(refreshing);
  const timerRef = useRef<number | null>(null);

  useEffect(() => {
    const clearTimer = () => {
      if (timerRef.current !== null) {
        window.clearTimeout(timerRef.current);
        timerRef.current = null;
      }
    };

    if (refreshing && !wasRefreshingRef.current) {
      clearTimer();
      timerRef.current = window.setTimeout(() => {
        const now = Date.now();
        const lastToastAt = lastToastAtByKey.get(toastKey) ?? 0;

        if (now - lastToastAt >= cooldownMs) {
          toast.info(message, duration);
          lastToastAtByKey.set(toastKey, now);
        }
      }, delayMs);
    }

    if (!refreshing) {
      clearTimer();
    }

    wasRefreshingRef.current = refreshing;

    return clearTimer;
  }, [cooldownMs, delayMs, duration, message, refreshing, toastKey]);
};