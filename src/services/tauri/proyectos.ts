import { invoke } from './client';
import type { EliminarProyectoResultado, Proyecto, ProyectoDetalle } from './types';

export const crearProyectoConParticipantes = async (actor_user_id: string, titulo_proyecto: string, docentes_ids: string[]): Promise<Proyecto> => {
  return await invoke('crear_proyecto_con_participantes', { actorUserId: actor_user_id, request: { titulo_proyecto, docentes_ids } });
};

export const buscarProyectosPorDocente = async (actor_user_id: string, id_docente: string): Promise<Proyecto[]> => {
  return await invoke('buscar_proyectos_por_docente', { actorUserId: actor_user_id, idDocente: id_docente });
};

export const getAllProyectosDetalle = async (actor_user_id: string): Promise<ProyectoDetalle[]> => {
  return await invoke('get_all_proyectos_detalle', { actorUserId: actor_user_id });
};

export const eliminarRelacionProyectoDocente = async (actor_user_id: string, id_proyecto: string, id_docente: string): Promise<void> => {
  return await invoke('eliminar_relacion_proyecto_docente', { actorUserId: actor_user_id, idProyecto: id_proyecto, idDocente: id_docente });
};

export const eliminarRelacionesProyecto = async (actor_user_id: string, id_proyecto: string): Promise<void> => {
  return await invoke('eliminar_relaciones_proyecto', { actorUserId: actor_user_id, idProyecto: id_proyecto });
};

export const eliminarProyecto = async (actor_user_id: string, id_proyecto: string): Promise<EliminarProyectoResultado> => {
  return await invoke('eliminar_proyecto', { actorUserId: actor_user_id, idProyecto: id_proyecto });
};

export const reactivarProyecto = async (actor_user_id: string, id_proyecto: string): Promise<Proyecto> => {
  return await invoke('reactivar_proyecto', { actorUserId: actor_user_id, idProyecto: id_proyecto });
};