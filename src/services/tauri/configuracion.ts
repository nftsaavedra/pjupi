import { invoke } from './client';
import type { EliminarGradoResultado, GradoAcademico, Usuario } from './types';

export const getAllGrados = async (): Promise<GradoAcademico[]> => {
  return await invoke('get_all_grados');
};

export const crearGrado = async (nombre: string, descripcion?: string): Promise<GradoAcademico> => {
  return await invoke('crear_grado', { request: { nombre, descripcion } });
};

export const actualizarGrado = async (id_grado: string, nombre: string, descripcion?: string): Promise<GradoAcademico> => {
  return await invoke('actualizar_grado', { idGrado: id_grado, request: { nombre, descripcion } });
};

export const eliminarGrado = async (id_grado: string): Promise<EliminarGradoResultado> => {
  return await invoke('eliminar_grado', { idGrado: id_grado });
};

export const reactivarGrado = async (id_grado: string): Promise<GradoAcademico> => {
  return await invoke('reactivar_grado', { idGrado: id_grado });
};

export const crearUsuario = async (
  username: string,
  nombre_completo: string,
  rol: string,
  password: string,
): Promise<Usuario> => {
  return await invoke('crear_usuario', { request: { username, nombre_completo, rol, password } });
};

export const getAllUsuarios = async (): Promise<Usuario[]> => {
  return await invoke('get_all_usuarios');
};

export const actualizarUsuario = async (
  id_usuario: string,
  username: string,
  nombre_completo: string,
  rol: string,
  password?: string,
): Promise<Usuario> => {
  return await invoke('actualizar_usuario', {
    idUsuario: id_usuario,
    request: { username, nombre_completo, rol, password: password?.trim() ? password : null },
  });
};

export const desactivarUsuario = async (id_usuario: string): Promise<Usuario> => {
  return await invoke('desactivar_usuario', { idUsuario: id_usuario });
};

export const reactivarUsuario = async (id_usuario: string): Promise<Usuario> => {
  return await invoke('reactivar_usuario', { idUsuario: id_usuario });
};