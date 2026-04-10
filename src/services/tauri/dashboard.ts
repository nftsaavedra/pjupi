import { invoke } from './client';
import type { DocenteProyectosCount, KpisDashboard } from './types';

export const getEstadisticasProyectosXDocente = async (): Promise<DocenteProyectosCount[]> => {
  return await invoke('get_estadisticas_proyectos_x_docente');
};

export const getKpisDashboard = async (): Promise<KpisDashboard> => {
  return await invoke('get_kpis_dashboard');
};