import { invoke } from './client';
import type { EliminarGradoResultado, GradoAcademico, Usuario } from './types';

export const getAllGrados = async (actor_user_id: string): Promise<GradoAcademico[]> => {
  return await invoke('get_all_grados', { actorUserId: actor_user_id });
};

export const crearGrado = async (actor_user_id: string, nombre: string, descripcion?: string): Promise<GradoAcademico> => {
  return await invoke('crear_grado', { actorUserId: actor_user_id, request: { nombre, descripcion } });
};

export const actualizarGrado = async (actor_user_id: string, id_grado: string, nombre: string, descripcion?: string): Promise<GradoAcademico> => {
  return await invoke('actualizar_grado', { actorUserId: actor_user_id, idGrado: id_grado, request: { nombre, descripcion } });
};

export const eliminarGrado = async (actor_user_id: string, id_grado: string): Promise<EliminarGradoResultado> => {
  return await invoke('eliminar_grado', { actorUserId: actor_user_id, idGrado: id_grado });
};

export const reactivarGrado = async (actor_user_id: string, id_grado: string): Promise<GradoAcademico> => {
  return await invoke('reactivar_grado', { actorUserId: actor_user_id, idGrado: id_grado });
};

export const crearUsuario = async (
  username: string,
  nombre_completo: string,
  rol: string,
  password: string,
  actor_user_id: string,
): Promise<Usuario> => {
  return await invoke('crear_usuario', { actorUserId: actor_user_id, request: { username, nombre_completo, rol, password } });
};

export const getAllUsuarios = async (actor_user_id: string): Promise<Usuario[]> => {
  return await invoke('get_all_usuarios', { actorUserId: actor_user_id });
};

export const actualizarUsuario = async (
  id_usuario: string,
  username: string,
  nombre_completo: string,
  rol: string,
  actor_user_id: string,
  password?: string,
): Promise<Usuario> => {
  return await invoke('actualizar_usuario', {
    actorUserId: actor_user_id,
    idUsuario: id_usuario,
    request: { username, nombre_completo, rol, password: password?.trim() ? password : null },
  });
};

export const desactivarUsuario = async (id_usuario: string, actor_user_id: string): Promise<Usuario> => {
  return await invoke('desactivar_usuario', { idUsuario: id_usuario, actorUserId: actor_user_id });
};

export const reactivarUsuario = async (id_usuario: string, actor_user_id: string): Promise<Usuario> => {
  return await invoke('reactivar_usuario', { idUsuario: id_usuario, actorUserId: actor_user_id });
};