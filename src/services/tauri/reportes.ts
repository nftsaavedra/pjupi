import { invoke } from './client';
import type { DatosExportDocenteAgrupado, ExportData, ReporteDocenteIntegral, ReporteProyectoIntegral } from './types';

export const getDataExportacionPlana = async (): Promise<ExportData[]> => {
  return await invoke('get_data_exportacion_plana');
};

export const getDataExportacionAgrupada = async (): Promise<DatosExportDocenteAgrupado[]> => {
  return await invoke('get_data_exportacion_agrupada_docente');
};

export const getReporteProyectoIntegral = async (id_proyecto: string): Promise<ReporteProyectoIntegral> =>
  await invoke('get_reporte_proyecto_integral', { idProyecto: id_proyecto });

export const getReporteDocenteIntegral = async (id_docente: string): Promise<ReporteDocenteIntegral> =>
  await invoke('get_reporte_docente_integral', { idDocente: id_docente });

export const getReportesDocentesIntegral = async (): Promise<ReporteDocenteIntegral[]> =>
  await invoke('get_reportes_docentes_integral');