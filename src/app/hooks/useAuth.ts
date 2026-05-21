import { useEffect, useState } from 'react';
import { getAuthStatus, getCurrentSession, logoutUsuario, type Usuario } from '@/features/auth/api';
import { toast } from '@/services/toast';
import { getTauriErrorMessage } from '@/services/tauri/error';

export interface UseAuthReturn {
  authLoading: boolean;
  requiresSetup: boolean;
  currentUser: Usuario | null;
  handleAuthenticated: (usuario: Usuario) => void;
  handleLogout: () => Promise<void>;
}

export function useAuth(): UseAuthReturn {
  const [authLoading, setAuthLoading] = useState(true);
  const [requiresSetup, setRequiresSetup] = useState(false);
  const [currentUser, setCurrentUser] = useState<Usuario | null>(null);

  const cargarAuthStatus = async () => {
    try {
      const [status, session] = await Promise.all([getAuthStatus(), getCurrentSession()]);
      setRequiresSetup(status.requires_setup);
      setCurrentUser(session);
    } catch (error) {
      toast.error('Error verificando autenticación: ' + getTauriErrorMessage(error));
    } finally {
      setAuthLoading(false);
    }
  };

  const handleAuthenticated = (usuario: Usuario) => {
    setCurrentUser(usuario);
    setRequiresSetup(false);
  };

  const handleLogout = async () => {
    try {
      await logoutUsuario();
      setCurrentUser(null);
    } catch (error) {
      toast.error('Error al cerrar sesión: ' + getTauriErrorMessage(error));
    }
  };

  useEffect(() => {
    const init = async () => {
      try { await cargarAuthStatus(); }
      catch { /* auth init error handled internally */ }
    };
    void init();
  }, []);

  return { authLoading, requiresSetup, currentUser, handleAuthenticated, handleLogout };
}
