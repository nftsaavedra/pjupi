export {
  getAuthStatus,
  getCurrentSession,
  loginUsuario,
  logoutUsuario,
  registrarPrimerUsuario,
} from '@/services/tauri/auth';

export { getTauriErrorMessage } from '@/services/tauri/error';

export type { AuthStatus, Usuario } from '@/services/tauri/types';