import React, { useEffect, useState } from 'react';
import { CheckCircle2, Info, TriangleAlert, XCircle } from 'lucide-react';
import { toast, type ToastItem } from '../../services/toast';
import { AppIcon } from '../ui/AppIcon';

export const ToastContainer: React.FC = () => {
  const [toasts, setToasts] = useState<ToastItem[]>([]);

  useEffect(() => {
    const unsubscribe = toast.subscribe((item) => {
      setToasts((prev) => [...prev, item]);
      const timeout = item.duration ?? 3500;
      window.setTimeout(() => {
        setToasts((prev) => prev.filter((t) => t.id !== item.id));
      }, timeout);
    });

    return unsubscribe;
  }, []);

  return (
    <div className="toast-container" aria-live="polite" aria-atomic="true">
      {toasts.map((item) => (
        <div key={item.id} className={`toast toast-${item.type}`}>
          <span className="toast-icon">
            {item.type === 'success' && <AppIcon icon={CheckCircle2} size={18} />}
            {item.type === 'error' && <AppIcon icon={XCircle} size={18} />}
            {item.type === 'warning' && <AppIcon icon={TriangleAlert} size={18} />}
            {item.type === 'info' && <AppIcon icon={Info} size={18} />}
          </span>
          <span className="toast-message">{item.message}</span>
        </div>
      ))}
    </div>
  );
};