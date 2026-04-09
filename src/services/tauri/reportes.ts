import { invoke } from './client';
import type { DatosExportDocenteAgrupado, ExportData } from './types';

export const getDataExportacionPlana = async (): Promise<ExportData[]> => {
  return await invoke('get_data_exportacion_plana');
};

export const getDataExportacionAgrupada = async (): Promise<DatosExportDocenteAgrupado[]> => {
  return await invoke('get_data_exportacion_agrupada_docente');
};