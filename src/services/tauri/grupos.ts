import { invoke } from './client';
import type { GrupoInvestigacion } from './types';

export interface CreateGrupoPayload {
  nombre: string;
  descripcion?: string | null;
  coordinador_id?: string | null;
  lineas_investigacion: string[];
}

export interface UpdateGrupoPayload {
  nombre: string;
  descripcion?: string | null;
  coordinador_id?: string | null;
  lineas_investigacion: string[];
}

export const getAllGrupos = async (): Promise<GrupoInvestigacion[]> => {
  return await invoke('get_all_grupos');
};

export const getGrupo = async (id_grupo: string): Promise<GrupoInvestigacion> => {
  return await invoke('get_grupo', { idGrupo: id_grupo });
};

export const createGrupo = async (request: CreateGrupoPayload): Promise<GrupoInvestigacion> => {
  return await invoke('create_grupo', { request });
};

export const updateGrupo = async (id_grupo: string, request: UpdateGrupoPayload): Promise<GrupoInvestigacion> => {
  return await invoke('update_grupo', { idGrupo: id_grupo, request });
};

export const deleteGrupo = async (id_grupo: string): Promise<void> => {
  await invoke('delete_grupo', { idGrupo: id_grupo });
};
