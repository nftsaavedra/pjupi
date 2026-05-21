import { invoke } from './client';
import type { DocenteProyectosCount, KpisDashboard, ProyectosTrendItem, RenacytDistribucionItem } from './types';

export const getEstadisticasProyectosXDocente = async (): Promise<DocenteProyectosCount[]> => {
  return await invoke('get_estadisticas_proyectos_x_docente');
};

export const getKpisDashboard = async (): Promise<KpisDashboard> => {
  return await invoke('get_kpis_dashboard');
};

export const getProyectosTrend = async (): Promise<ProyectosTrendItem[]> => {
  return await invoke('get_proyectos_trend');
};

export const getRenacytDistribucion = async (): Promise<RenacytDistribucionItem[]> => {
  return await invoke('get_renacyt_distribucion');
};