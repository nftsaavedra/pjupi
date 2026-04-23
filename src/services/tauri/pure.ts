import { invoke } from './client';
import type { Publicacion, SyncPublicacionesResult } from './types';

export const sincronizarPublicacionesPure = async (
  docente_id: string,
): Promise<SyncPublicacionesResult> => {
  return await invoke('sincronizar_publicaciones_pure', { docenteId: docente_id });
};

export const getPublicacionesDocente = async (
  docente_id: string,
): Promise<Publicacion[]> => {
  return await invoke('get_publicaciones_docente', { docenteId: docente_id });
};
