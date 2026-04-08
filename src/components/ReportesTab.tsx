import React, { useState } from 'react';
import { Download } from 'lucide-react';
import {
  getDataExportacionPlana,
  getDataExportacionAgrupada,
  getTauriErrorMessage,
  type ExportData,
  type DatosExportDocenteAgrupado,
} from '../services/tauriApi';
import { toast } from '../services/toast';
import { useStableFetchData } from '../hooks/useFetch';
import { useRefreshToast } from '../hooks/useRefreshToast';
import { AppIcon } from './AppIcon';
import { SkeletonTable } from './Skeleton';

type TipoReporte = 'agrupado_docente' | 'plano';

interface ReportesTabProps {
  refreshTrigger?: number;
}

export const ReportesTab: React.FC<ReportesTabProps> = ({ refreshTrigger = 0 }) => {
  const [tipo, setTipo] = useState<TipoReporte>('agrupado_docente');
  const [query, setQuery] = useState('');
  const {
    data: preview,
    loading,
    refreshing,
    error,
    recargar: cargarPreview,
  } = useStableFetchData<DatosExportDocenteAgrupado[]>(
    getDataExportacionAgrupada,
    refreshTrigger,
    'Error cargando vista previa de reportes',
    [],
  );

  useRefreshToast({
    refreshing,
    message: 'Actualizando vista previa de reportes',
    toastKey: 'reportes-refresh',
    cooldownMs: 120000,
  });

  const exportar = async () => {
    try {
      const XLSX = await import('xlsx');
      let rows: Array<ExportData | DatosExportDocenteAgrupado>;
      let sheetName: string;
      if (tipo === 'agrupado_docente') {
        rows = await getDataExportacionAgrupada();
        sheetName = 'Docentes_Proyectos';
      } else {
        rows = await getDataExportacionPlana();
        sheetName = 'Detalle_Plano';
      }

      const ws = XLSX.utils.json_to_sheet(rows);
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, sheetName);
      XLSX.writeFile(wb, `reporte-${tipo}-${new Date().toISOString().split('T')[0]}.xlsx`);
      toast.success('Reporte exportado exitosamente');
    } catch (error) {
      toast.error('Error exportando reporte: ' + getTauriErrorMessage(error));
    }
  };

  const filtrados = preview.filter((d) =>
    d.docente.toLowerCase().includes(query.toLowerCase()) ||
    d.dni.includes(query) ||
    d.grado.toLowerCase().includes(query.toLowerCase())
  );

  return (
    <div className="tab-panel module-shell reportes-module">
      <div className="module-split-layout reportes-layout">
        <div className="form-card">
          <h2>Centro de Reportes</h2>
          <div className="form" style={{ gap: '1rem' }}>
            <div className="form-group">
              <label>Tipo de reporte</label>
              <select
                className="form-input"
                value={tipo}
                onChange={(e) => setTipo(e.target.value as TipoReporte)}
                aria-label="Seleccionar tipo de reporte"
              >
                <option value="agrupado_docente">Docentes con proyectos (agrupado)</option>
                <option value="plano">Detalle plano (proyecto-docente)</option>
              </select>
            </div>

            <button className="btn-primary" onClick={exportar}>
              <span className="button-with-icon">
                <AppIcon icon={Download} size={18} />
                <span>Exportar Excel</span>
              </span>
            </button>
          </div>
        </div>

        <aside className="module-aside-card reportes-aside">
          <span className="module-aside-kicker">Resumen rápido</span>
          <strong>{tipo === 'agrupado_docente' ? 'Agrupado' : 'Plano'}</strong>
          <p>
            Ajuste el formato antes de exportar.
          </p>
          <div className="module-aside-meta">
            <span className="badge badge-info">Consulta actual: {query ? 'Filtrada' : 'Completa'}</span>
            <span className="badge badge-success">Actualización activa</span>
          </div>
        </aside>
      </div>

      <div className="table-container">
        <div className="section-header">
          <h2>Vista previa: Docentes y trazabilidad de proyectos</h2>
        </div>
        {error && (
          <div className="inline-feedback inline-feedback-warning">
            <span>No se pudo refrescar la vista previa. Se mantienen los datos actuales.</span>
            <button type="button" className="btn-secondary" onClick={() => void cargarPreview()}>
              Reintentar
            </button>
          </div>
        )}
        <div className="form-group" style={{ marginBottom: '1rem' }}>
          <input
            className="form-input"
            placeholder="Buscar por docente, DNI o grado"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            aria-label="Buscar en la vista previa por docente, DNI o grado"
          />
        </div>

        {loading ? (
          <SkeletonTable columns={5} rows={6} />
        ) : filtrados.length === 0 ? (
          <div className="empty-state">No hay datos para mostrar</div>
        ) : (
          <table className="table" aria-label="Tabla de vista previa de reportes">
            <thead>
              <tr>
                <th>Docente</th>
                <th>DNI</th>
                <th>Grado</th>
                <th>Cantidad Proyectos</th>
                <th>Proyectos</th>
              </tr>
            </thead>
            <tbody>
              {filtrados.map((row, idx) => (
                <tr key={`${row.dni}-${idx}`}>
                  <td>{row.docente}</td>
                  <td>{row.dni}</td>
                  <td>{row.grado}</td>
                  <td>{row.cantidad_proyectos}</td>
                  <td>{row.proyectos || '-'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
};