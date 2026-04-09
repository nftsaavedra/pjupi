import { invoke } from './client';
import type { DatosExportDocenteAgrupado, ExportData } from './types';

export const getDataExportacionPlana = async (actor_user_id: string): Promise<ExportData[]> => {
  return await invoke('get_data_exportacion_plana', { actorUserId: actor_user_id });
};

export const getDataExportacionAgrupada = async (actor_user_id: string): Promise<DatosExportDocenteAgrupado[]> => {
  return await invoke('get_data_exportacion_agrupada_docente', { actorUserId: actor_user_id });
};