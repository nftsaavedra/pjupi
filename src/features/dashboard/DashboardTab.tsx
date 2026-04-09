import React, { useEffect, useMemo, useState } from 'react';
import { FolderOpen, RotateCcw, TrendingUp, TriangleAlert, Users } from 'lucide-react';
import {
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  Legend,
  Pie,
  PieChart,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';
import { useStableFetchData } from '../../hooks/useFetch';
import { useRefreshToast } from '../../hooks/useRefreshToast';
import { getEstadisticasProyectosXDocente, getKpisDashboard, type DocenteProyectosCount, type KpisDashboard } from '../../services/tauriApi';
import { AppIcon } from '../../shared/ui/AppIcon';
import { SkeletonChart, SkeletonKpiGrid } from '../../shared/ui/Skeleton';
import { KPICard } from './components/KPICard';

interface DashboardTabProps {
  refreshTrigger?: number;
}

const useMeasuredChart = (minHeight: number) => {
  const [node, setNode] = useState<HTMLDivElement | null>(null);
  const [width, setWidth] = useState(0);

  useEffect(() => {
    if (!node) {
      setWidth(0);
      return;
    }

    const element = node;

    const updateWidth = (nextWidth: number) => {
      setWidth(Math.max(Math.floor(nextWidth), 0));
    };

    const measure = () => updateWidth(element.getBoundingClientRect().width);

    measure();
    const rafId = window.requestAnimationFrame(measure);

    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) return;
      updateWidth(entry.contentRect.width);
    });

    observer.observe(element);
    return () => {
      window.cancelAnimationFrame(rafId);
      observer.disconnect();
    };
  }, [node]);

  return {
    ref: setNode,
    width,
    height: minHeight,
    ready: width > 0,
  };
};

export const DashboardTab: React.FC<DashboardTabProps> = ({ refreshTrigger = 0 }) => {
  const [viewportWidth, setViewportWidth] = useState(() => (
    typeof window !== 'undefined' ? window.innerWidth : 1280
  ));

  useEffect(() => {
    const handleResize = () => setViewportWidth(window.innerWidth);
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

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
        getKpisDashboard(),
        getEstadisticasProyectosXDocente(),
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
  const totalAsignaciones = estadisticas.reduce((acc, docente) => acc + docente.cantidad, 0);
  const hasProjectAssignments = totalAsignaciones > 0;

  const docentesConProyectos = estadisticas.filter((d) => d.cantidad > 0).length;
  const docentesSinProyectos = Math.max(totalDocentes - docentesConProyectos, 0);
  const promedioProyectos = totalDocentes > 0
    ? (totalProyectos / totalDocentes).toFixed(2)
    : '0.00';

  const topDocentes = [...estadisticas]
    .filter((docente) => docente.cantidad > 0)
    .sort((a, b) => b.cantidad - a.cantidad)
    .slice(0, 8);

  const distribucion = [
    { rango: '0', cantidad: estadisticas.filter((x) => x.cantidad === 0).length },
    { rango: '1', cantidad: estadisticas.filter((x) => x.cantidad === 1).length },
    { rango: '2', cantidad: estadisticas.filter((x) => x.cantidad === 2).length },
    { rango: '3+', cantidad: estadisticas.filter((x) => x.cantidad >= 3).length },
  ];
  const distribucionConDatos = distribucion.filter((item) => item.cantidad > 0);

  const pieData = [
    { name: 'Con proyectos', value: docentesConProyectos },
    { name: 'Sin proyectos', value: docentesSinProyectos },
  ];
  const pieColors = ['#10b981', '#f59e0b'];
  const isCompact = viewportWidth <= 768;
  const allDocentesTickInterval = isCompact ? Math.max(Math.ceil(estadisticas.length / 6) - 1, 0) : 0;
  const topChart = useMeasuredChart(320);
  const distributionChart = useMeasuredChart(280);
  const pieChart = useMeasuredChart(280);
  const allDocentesChart = useMeasuredChart(300);
  const pieHasVisibleData = pieData.some((item) => item.value > 0);
  const showTopRanking = topDocentes.length > 0 && hasProjectAssignments;
  const showAllDocentes = estadisticas.length > 0 && hasProjectAssignments;
  const chartMargin = useMemo(() => ({
    top: 8,
    right: isCompact ? 8 : 20,
    left: isCompact ? -18 : 0,
    bottom: isCompact ? 24 : 8,
  }), [isCompact]);
  const pieOuterRadius = Math.max(Math.min((pieChart.width - (isCompact ? 40 : 72)) / 2, isCompact ? 78 : 98), 42);
  const chartLoadingState = <SkeletonChart titleWidth="md" height="md" />;

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
          <SkeletonChart titleWidth="md" height="lg" />
          <div className="two-col-charts">
            <SkeletonChart titleWidth="md" height="md" />
            <SkeletonChart titleWidth="md" height="md" />
          </div>
          <SkeletonChart titleWidth="lg" height="md" />
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

          <div className="dashboard-main-grid content-shell">
            <div className="chart-container dashboard-primary-chart">
              <h2>Top docentes por cantidad de proyectos</h2>
              <div ref={topChart.ref} className="dashboard-chart-stage dashboard-chart-stage-lg">
                {showTopRanking ? (
                  topChart.ready ? (
                    <BarChart width={topChart.width} height={topChart.height} data={topDocentes} margin={chartMargin}>
                      <CartesianGrid stroke="#dbe7f5" strokeDasharray="3 3" vertical={false} />
                      <XAxis
                        dataKey="nombre"
                        angle={isCompact ? -18 : 0}
                        textAnchor={isCompact ? 'end' : 'middle'}
                        height={isCompact ? 58 : 40}
                        tick={{ fontSize: isCompact ? 11 : 12, fill: '#64748b' }}
                        interval={0}
                      />
                      <YAxis allowDecimals={false} tick={{ fontSize: 12, fill: '#64748b' }} />
                      <Tooltip cursor={{ fill: 'rgba(148, 163, 184, 0.12)' }} />
                      <Legend wrapperStyle={{ fontSize: 12 }} />
                      <Bar dataKey="cantidad" fill="#2196F3" name="Cantidad de Proyectos" radius={[8, 8, 0, 0]} />
                    </BarChart>
                  ) : chartLoadingState
                ) : (
                  <div className="empty-state">Aun no hay proyectos asignados a docentes para este ranking.</div>
                )}
              </div>
            </div>

            <div className="dashboard-side-panel">
              <div className="dashboard-insight-card">
                <span className="dashboard-insight-label">Docentes con proyectos</span>
                <strong>{docentesConProyectos}</strong>
                <p>Participan actualmente en al menos un proyecto.</p>
              </div>
              <div className="dashboard-insight-card">
                <span className="dashboard-insight-label">Carga media</span>
                <strong>{promedioProyectos}</strong>
                <p>Promedio de proyectos asignados por docente registrado.</p>
              </div>
            </div>
          </div>

          <div className="dashboard-secondary-grid content-shell">
            <div className="chart-container">
              <h2>Distribución de carga por docente</h2>
              <div ref={distributionChart.ref} className="dashboard-chart-stage dashboard-chart-stage-md">
                {distribucionConDatos.length > 0 ? (
                  distributionChart.ready ? (
                    <BarChart width={distributionChart.width} height={distributionChart.height} data={distribucionConDatos} margin={chartMargin}>
                      <CartesianGrid stroke="#dbe7f5" strokeDasharray="3 3" vertical={false} />
                      <XAxis dataKey="rango" tick={{ fontSize: 12, fill: '#64748b' }} />
                      <YAxis allowDecimals={false} tick={{ fontSize: 12, fill: '#64748b' }} />
                      <Tooltip cursor={{ fill: 'rgba(148, 163, 184, 0.12)' }} />
                      <Legend wrapperStyle={{ fontSize: 12 }} />
                      <Bar dataKey="cantidad" fill="#0ea5e9" name="Docentes" radius={[8, 8, 0, 0]} />
                    </BarChart>
                  ) : chartLoadingState
                ) : (
                  <div className="empty-state">No hay docentes activos registrados para calcular la distribución.</div>
                )}
              </div>
            </div>

            <div className="chart-container">
              <h2>Docentes con y sin proyectos</h2>
              <div ref={pieChart.ref} className="dashboard-chart-stage dashboard-chart-stage-md">
                {pieHasVisibleData ? (
                  pieChart.ready ? (
                    <PieChart width={pieChart.width} height={pieChart.height}>
                      <Pie
                        data={pieData}
                        dataKey="value"
                        nameKey="name"
                        cx="50%"
                        cy="50%"
                        innerRadius={Math.max(pieOuterRadius - 28, 24)}
                        outerRadius={pieOuterRadius}
                        paddingAngle={pieData.filter((item) => item.value > 0).length > 1 ? 2 : 0}
                        minAngle={pieData.filter((item) => item.value > 0).length > 1 ? 4 : 0}
                        labelLine={false}
                        label={({ name, value }) => value ? `${name}: ${value}` : ''}
                      >
                        {pieData.map((entry, idx) => (
                          <Cell key={`${entry.name}-${idx}`} fill={pieColors[idx % pieColors.length]} stroke="#ffffff" strokeWidth={2} />
                        ))}
                      </Pie>
                      <Tooltip formatter={(value) => [value ?? 0, 'Docentes']} />
                      <Legend wrapperStyle={{ fontSize: 12 }} />
                    </PieChart>
                  ) : chartLoadingState
                ) : (
                  <div className="empty-state">No hay docentes activos para comparar asignaciones.</div>
                )}
              </div>
            </div>
          </div>

          <div className="chart-container content-shell dashboard-wide-chart">
            <h2>Todos los docentes: proyectos asignados</h2>
            <div ref={allDocentesChart.ref} className="dashboard-chart-stage dashboard-chart-stage-lg">
              {showAllDocentes ? (
                allDocentesChart.ready ? (
                  <BarChart width={allDocentesChart.width} height={allDocentesChart.height} data={estadisticas} margin={chartMargin}>
                    <CartesianGrid stroke="#dbe7f5" strokeDasharray="3 3" vertical={false} />
                    <XAxis
                      dataKey="nombre"
                      interval={allDocentesTickInterval}
                      angle={isCompact ? -20 : 0}
                      textAnchor={isCompact ? 'end' : 'middle'}
                      height={isCompact ? 62 : 40}
                      tick={{ fontSize: isCompact ? 11 : 12, fill: '#64748b' }}
                    />
                    <YAxis allowDecimals={false} tick={{ fontSize: 12, fill: '#64748b' }} />
                    <Tooltip cursor={{ fill: 'rgba(148, 163, 184, 0.12)' }} />
                    <Legend wrapperStyle={{ fontSize: 12 }} />
                    <Bar dataKey="cantidad" fill="#6366f1" name="Cantidad" radius={[8, 8, 0, 0]} />
                  </BarChart>
                ) : chartLoadingState
              ) : (
                <div className="empty-state">Los docentes existen, pero todavia no tienen proyectos activos asignados.</div>
              )}
            </div>
          </div>

          <button className="btn-secondary dashboard-refresh-action" onClick={() => void cargarDatos()}>
            Actualizar
          </button>
        </>
      )}
    </div>
  );
};