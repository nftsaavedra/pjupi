export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface ToastItem {
  id: string;
  type: ToastType;
  message: string;
  duration?: number;
}

type ToastListener = (toast: ToastItem) => void;

const listeners = new Set<ToastListener>();

const emit = (toast: ToastItem) => {
  for (const listener of listeners) {
    listener(toast);
  }
};

const show = (type: ToastType, message: string, duration = 3500) => {
  emit({
    id: `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`,
    type,
    message,
    duration,
  });
};

export const toast = {
  success: (message: string, duration?: number) => { show('success', message, duration); },
  error: (message: string, duration?: number) => { show('error', message, duration); },
  info: (message: string, duration?: number) => { show('info', message, duration); },
  warning: (message: string, duration?: number) => { show('warning', message, duration); },
  subscribe: (listener: ToastListener) => {
    listeners.add(listener);
    return () => {
      listeners.delete(listener);
    };
  },
};
