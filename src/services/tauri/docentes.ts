import { invoke } from './client';
import type { Docente, DocenteDetalle, EliminarDocenteResultado, RefreshDocenteRenacytFormacionResultado, RenacytLookupResult, ReniecDniLookupResult } from './types';

interface CreateDocenteRenacytPayload {
  codigo_registro: string;
  id_investigador: string;
  nivel?: string | null;
  grupo?: string | null;
  condicion?: string | null;
  fecha_informe_calificacion?: number | null;
  fecha_registro?: number | null;
  fecha_ultima_revision?: number | null;
  orcid?: string | null;
  scopus_author_id?: string | null;
  ficha_url: string;
  formaciones_academicas_json?: string | null;
}

export const crearDocente = async (
  actor_user_id: string,
  dni: string,
  id_grado: string,
  nombres: string,
  apellido_paterno: string,
  apellido_materno?: string,
  renacyt?: CreateDocenteRenacytPayload | null,
): Promise<Docente> => {
  return await invoke('crear_docente', {
    actorUserId: actor_user_id,
    request: {
      dni,
      id_grado,
      nombres,
      apellido_paterno,
      apellido_materno: apellido_materno?.trim() ? apellido_materno : null,
      renacyt: renacyt ?? null,
    },
  });
};

export const getAllDocentes = async (actor_user_id: string): Promise<Docente[]> => {
  return await invoke('get_all_docentes', { actorUserId: actor_user_id });
};

export const buscarDocentePorDni = async (actor_user_id: string, dni: string): Promise<Docente | null> => {
  return await invoke('buscar_docente_por_dni', { actorUserId: actor_user_id, dni });
};

export const consultarDniReniec = async (actor_user_id: string, numero: string): Promise<ReniecDniLookupResult> => {
  return await invoke('consultar_dni_reniec', { actorUserId: actor_user_id, numero });
};

export const consultarRenacytDocente = async (actor_user_id: string, codigo_o_id: string): Promise<RenacytLookupResult> => {
  return await invoke('consultar_renacyt_docente', { actorUserId: actor_user_id, codigoOId: codigo_o_id });
};

export const getAllDocentesConProyectos = async (actor_user_id: string): Promise<DocenteDetalle[]> => {
  return await invoke('get_all_docentes_con_proyectos', { actorUserId: actor_user_id });
};

export const eliminarDocente = async (actor_user_id: string, id_docente: string): Promise<EliminarDocenteResultado> => {
  return await invoke('eliminar_docente', { actorUserId: actor_user_id, idDocente: id_docente });
};

export const reactivarDocente = async (actor_user_id: string, id_docente: string): Promise<Docente> => {
  return await invoke('reactivar_docente', { actorUserId: actor_user_id, idDocente: id_docente });
};

export const refrescarFormacionAcademicaRenacytDocente = async (actor_user_id: string, id_docente: string): Promise<RefreshDocenteRenacytFormacionResultado> => {
  return await invoke('refrescar_formacion_academica_renacyt_docente', { actorUserId: actor_user_id, idDocente: id_docente });
};