import React, { useEffect, useMemo, useState } from 'react';
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
import type { DocenteProyectosCount } from '../api';
import { useMeasuredChart } from '../hooks/useMeasuredChart';
import { SkeletonChart } from '@/shared/ui/Skeleton';

interface DashboardChartsProps {
  estadisticas: DocenteProyectosCount[];
  totalDocentes: number;
  totalProyectos: number;
}

export const DashboardCharts: React.FC<DashboardChartsProps> = ({ estadisticas, totalDocentes, totalProyectos }) => {
  const [viewportWidth, setViewportWidth] = useState(() => (
    typeof window !== 'undefined' ? window.innerWidth : 1280
  ));

  useEffect(() => {
    const handleResize = () => { setViewportWidth(window.innerWidth); };
    window.addEventListener('resize', handleResize);
    return () => { window.removeEventListener('resize', handleResize); };
  }, []);

  const totalAsignaciones = useMemo(
    () => estadisticas.reduce((acc, docente) => acc + docente.cantidad, 0),
    [estadisticas],
  );
  const hasProjectAssignments = totalAsignaciones > 0;
  const docentesConProyectos = useMemo(
    () => estadisticas.filter((docente) => docente.cantidad > 0).length,
    [estadisticas],
  );
  const docentesSinProyectos = Math.max(totalDocentes - docentesConProyectos, 0);
  const promedioProyectos = totalDocentes > 0 ? (totalProyectos / totalDocentes).toFixed(2) : '0.00';
  const topDocentes = useMemo(
    () => [...estadisticas]
      .filter((docente) => docente.cantidad > 0)
      .sort((a, b) => b.cantidad - a.cantidad)
      .slice(0, 8),
    [estadisticas],
  );
  const distribucion = useMemo(
    () => [
      { rango: '0', cantidad: estadisticas.filter((item) => item.cantidad === 0).length },
      { rango: '1', cantidad: estadisticas.filter((item) => item.cantidad === 1).length },
      { rango: '2', cantidad: estadisticas.filter((item) => item.cantidad === 2).length },
      { rango: '3+', cantidad: estadisticas.filter((item) => item.cantidad >= 3).length },
    ],
    [estadisticas],
  );
  const distribucionConDatos = distribucion.filter((item) => item.cantidad > 0);
  const pieData = useMemo(
    () => [
      { name: 'Con proyectos', value: docentesConProyectos },
      { name: 'Sin proyectos', value: docentesSinProyectos },
    ],
    [docentesConProyectos, docentesSinProyectos],
  );
  const pieColors = ['#10b981', '#f59e0b'];
  const isCompact = viewportWidth <= 768;
  const allDocentesTickInterval = isCompact ? Math.max(Math.ceil(estadisticas.length / 6) - 1, 0) : 0;
  const [topChartRef, topChart] = useMeasuredChart(320);
  const [distributionChartRef, distributionChart] = useMeasuredChart(280);
  const [pieChartRef, pieChart] = useMeasuredChart(280);
  const [allDocentesChartRef, allDocentesChart] = useMeasuredChart(300);
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

  return (
    <>
      <div className="dashboard-main-grid content-shell">
        <div className="chart-container dashboard-primary-chart">
          <h2>Top docentes por cantidad de proyectos</h2>
          <div ref={topChartRef} className="dashboard-chart-stage dashboard-chart-stage-lg">
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
          <div ref={distributionChartRef} className="dashboard-chart-stage dashboard-chart-stage-md">
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
          <div ref={pieChartRef} className="dashboard-chart-stage dashboard-chart-stage-md">
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
                      // eslint-disable-next-line @typescript-eslint/no-deprecated
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
        <div ref={allDocentesChartRef} className="dashboard-chart-stage dashboard-chart-stage-lg">
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
    </>
  );
};