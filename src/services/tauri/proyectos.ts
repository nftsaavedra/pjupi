import { invoke } from './client';
import type { EliminarProyectoResultado, Proyecto, ProyectoDetalle } from './types';

export interface ProyectoParticipantesPayload {
  titulo_proyecto: string;
  docentes_ids: string[];
  docente_responsable_id?: string | null;
}

export const crearProyectoConParticipantes = async (
  titulo_proyecto: string,
  docentes_ids: string[],
  docente_responsable_id?: string | null,
): Promise<Proyecto> => {
  return await invoke('crear_proyecto_con_participantes', { request: { titulo_proyecto, docentes_ids, docente_responsable_id } });
};

export const actualizarProyectoConParticipantes = async (
  id_proyecto: string,
  payload: ProyectoParticipantesPayload,
): Promise<Proyecto> => {
  return await invoke('actualizar_proyecto_con_participantes', { idProyecto: id_proyecto, request: payload });
};

export const buscarProyectosPorDocente = async (id_docente: string): Promise<Proyecto[]> => {
  return await invoke('buscar_proyectos_por_docente', { idDocente: id_docente });
};

export const getAllProyectosDetalle = async (): Promise<ProyectoDetalle[]> => {
  return await invoke('get_all_proyectos_detalle');
};

export const eliminarRelacionProyectoDocente = async (id_proyecto: string, id_docente: string): Promise<void> => {
  return await invoke('eliminar_relacion_proyecto_docente', { idProyecto: id_proyecto, idDocente: id_docente });
};

export const eliminarRelacionesProyecto = async (id_proyecto: string): Promise<void> => {
  return await invoke('eliminar_relaciones_proyecto', { idProyecto: id_proyecto });
};

export const eliminarProyecto = async (id_proyecto: string): Promise<EliminarProyectoResultado> => {
  return await invoke('eliminar_proyecto', { idProyecto: id_proyecto });
};

export const reactivarProyecto = async (id_proyecto: string): Promise<Proyecto> => {
  return await invoke('reactivar_proyecto', { idProyecto: id_proyecto });
};