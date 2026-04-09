import React, { Suspense, lazy } from 'react';
import { FolderOpen, RotateCcw, TrendingUp, TriangleAlert, Users } from 'lucide-react';
import { getEstadisticasProyectosXDocente, getKpisDashboard, type DocenteProyectosCount, type KpisDashboard } from './api';
import { useRefreshToast } from '../../shared/hooks/useRefreshToast';
import { useStableFetchData } from '../../shared/hooks/useStableFetch';
import { AppIcon } from '../../shared/ui/AppIcon';
import { SkeletonChart, SkeletonKpiGrid } from '../../shared/ui/Skeleton';
import { KPICard } from './components/KPICard';

const DashboardCharts = lazy(async () => {
  const module = await import('./components/DashboardCharts');
  return { default: module.DashboardCharts };
});

const DashboardChartsFallback = () => (
  <>
    <SkeletonChart titleWidth="md" height="lg" />
    <div className="two-col-charts">
      <SkeletonChart titleWidth="md" height="md" />
      <SkeletonChart titleWidth="md" height="md" />
    </div>
    <SkeletonChart titleWidth="lg" height="md" />
  </>
);

interface DashboardTabProps {
  currentUserId: string;
  refreshTrigger?: number;
}

export const DashboardTab: React.FC<DashboardTabProps> = ({ currentUserId, refreshTrigger = 0 }) => {
  const {
    data: dashboardData,
    loading,
    refreshing,
    error,
    recargar: cargarDatos,
  } = useStableFetchData<{
    kpis: KpisDashboard | null;
    estadisticas: DocenteProyectosCount[];
  }>(
    async () => {
      const [kpisRes, estadisticasRes] = await Promise.all([
        getKpisDashboard(currentUserId),
        getEstadisticasProyectosXDocente(currentUserId),
      ]);

      return { kpis: kpisRes, estadisticas: estadisticasRes };
    },
    refreshTrigger,
    'Error al cargar datos del dashboard',
    { kpis: null, estadisticas: [] },
  );

  useRefreshToast({
    refreshing,
    message: 'Actualizando indicadores del dashboard',
    toastKey: 'dashboard-refresh',
    cooldownMs: 120000,
  });

  const { kpis, estadisticas } = dashboardData;
  const totalDocentes = kpis?.total_docentes ?? 0;
  const totalProyectos = kpis?.total_proyectos ?? 0;
  const docentesConProyectos = estadisticas.filter((docente) => docente.cantidad > 0).length;
  const docentesSinProyectos = Math.max(totalDocentes - docentesConProyectos, 0);
  const promedioProyectos = totalDocentes > 0
    ? (totalProyectos / totalDocentes).toFixed(2)
    : '0.00';

  if (error && !kpis && estadisticas.length === 0) {
    return (
      <div className="tab-panel error">
        <p>{error}</p>
        <button onClick={() => void cargarDatos()}>
          <span className="button-with-icon">
            <AppIcon icon={RotateCcw} size={16} />
            <span>Reintentar</span>
          </span>
        </button>
      </div>
    );
  }

  return (
    <div className="tab-panel dashboard">
      {error && !loading && (
        <div className="inline-feedback inline-feedback-warning">
          <span>No se pudo refrescar el dashboard. Se mantienen los indicadores anteriores.</span>
          <button type="button" className="btn-secondary" onClick={() => void cargarDatos()}>
            Reintentar
          </button>
        </div>
      )}

      {loading ? (
        <>
          <SkeletonKpiGrid />
          <DashboardChartsFallback />
        </>
      ) : (
        <>
          <div className="kpi-grid dashboard-kpi-grid content-shell">
            {kpis && (
              <>
                <KPICard label="Total Docentes" value={kpis.total_docentes} icon={Users} />
                <KPICard label="Total Proyectos" value={kpis.total_proyectos} icon={FolderOpen} />
                <KPICard label="Docentes Sin Proyectos" value={docentesSinProyectos} icon={TriangleAlert} />
                <KPICard label="Promedio Proyectos/Docente" value={promedioProyectos} icon={TrendingUp} />
              </>
            )}
          </div>

          <Suspense fallback={<DashboardChartsFallback />}>
            <DashboardCharts
              estadisticas={estadisticas}
              totalDocentes={totalDocentes}
              totalProyectos={totalProyectos}
            />
          </Suspense>

          <button className="btn-secondary dashboard-refresh-action" onClick={() => void cargarDatos()}>
            Actualizar
          </button>
        </>
      )}
    </div>
  );
};