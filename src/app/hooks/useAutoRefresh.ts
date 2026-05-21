import { useEffect } from 'react';
import { type Usuario } from '@/features/auth/api';

export function useAutoRefresh(currentUser: Usuario | null, onRefresh: () => void): void {
  useEffect(() => {
    if (!currentUser) return;

    const triggerRefresh = () => {
      if (document.visibilityState === 'visible') {
        onRefresh();
      }
    };

    const intervalId = window.setInterval(triggerRefresh, 15000);
    window.addEventListener('focus', triggerRefresh);
    document.addEventListener('visibilitychange', triggerRefresh);

    return () => {
      window.clearInterval(intervalId);
      window.removeEventListener('focus', triggerRefresh);
      document.removeEventListener('visibilitychange', triggerRefresh);
    };
  }, [currentUser, onRefresh]);
}
