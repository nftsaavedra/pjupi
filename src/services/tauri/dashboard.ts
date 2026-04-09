import { invoke } from './client';
import type { DocenteProyectosCount, KpisDashboard } from './types';

export const getEstadisticasProyectosXDocente = async (actor_user_id: string): Promise<DocenteProyectosCount[]> => {
  return await invoke('get_estadisticas_proyectos_x_docente', { actorUserId: actor_user_id });
};

export const getKpisDashboard = async (actor_user_id: string): Promise<KpisDashboard> => {
  return await invoke('get_kpis_dashboard', { actorUserId: actor_user_id });
};