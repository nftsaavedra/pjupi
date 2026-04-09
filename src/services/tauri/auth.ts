import { invoke } from './client';
import type { AuthStatus, Usuario } from './types';

export const getAuthStatus = async (): Promise<AuthStatus> => {
  return await invoke('get_auth_status');
};

export const registrarPrimerUsuario = async (
  username: string,
  nombre_completo: string,
  password: string,
): Promise<Usuario> => {
  return await invoke('registrar_primer_usuario', { request: { username, nombre_completo, password } });
};

export const loginUsuario = async (
  username: string,
  password: string,
): Promise<Usuario> => {
  return await invoke('login_usuario', { request: { username, password } });
};