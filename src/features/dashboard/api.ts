export {
  getEstadisticasProyectosXDocente,
  getKpisDashboard,
  getProyectosTrend,
  getRenacytDistribucion,
} from '@/services/tauri/dashboard';

export type { DocenteProyectosCount, KpisDashboard, ProyectosTrendItem, RenacytDistribucionItem } from '@/services/tauri/types';
