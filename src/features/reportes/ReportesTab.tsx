import React, { useState } from 'react';
import { Download } from 'lucide-react';
import {
  getDataExportacionAgrupada,
  getTauriErrorMessage,
  type DatosExportDocenteAgrupado,
} from './api';
import { toast } from '@/services/toast';
import { useStableFetchData } from '@/shared/hooks/useStableFetch';
import { useRefreshToast } from '@/shared/hooks/useRefreshToast';
import { AppIcon } from '@/shared/ui/AppIcon';
import { SkeletonTable } from '@/shared/ui/Skeleton';
import { saveDesktopFile } from '@/shared/utils/saveDesktopFile';
import { formatRenacytNivel, normalizeRenacytNivelSearch } from '@/shared/utils/renacyt';

type TipoReporte = 'agrupado_docente' | 'plano';

const normalizeText = (value: string | null | undefined) => (value ?? '').trim().toLowerCase();

interface ReportesTabProps {
  canExport?: boolean;
  refreshTrigger?: number;
}

export const ReportesTab: React.FC<ReportesTabProps> = ({ canExport = true, refreshTrigger = 0 }) => {
  const [tipo, setTipo] = useState<TipoReporte>('agrupado_docente');
  const [query, setQuery] = useState('');
  const [exportingFormat, setExportingFormat] = useState<'xlsx' | 'pdf' | null>(null);
  const {
    data: preview,
    loading,
    refreshing,
    error,
    recargar: cargarPreview,
  } = useStableFetchData<DatosExportDocenteAgrupado[]>(
    () => getDataExportacionAgrupada(),
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

  const exportar = async (format: 'xlsx' | 'pdf') => {
    setExportingFormat(format);

    try {
      const exportPayload = format === 'xlsx'
        ? await import('./reportExport').then(({ buildExcelReport }) => buildExcelReport(tipo))
        : await import('./reportExportPdf').then(({ buildPdfReport }) => buildPdfReport(tipo));

      const savedFilePath = await saveDesktopFile({
        suggestedName: exportPayload.suggestedName,
        bytes: exportPayload.bytes,
        filters: [
          {
            name: format === 'xlsx' ? 'Archivo Excel' : 'Documento PDF',
            extensions: [format],
          },
        ],
        mimeType: format === 'xlsx'
          ? 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
          : 'application/pdf',
      });

      if (!savedFilePath) {
        toast.info('Exportación cancelada');
        return;
      }

      toast.success(`Reporte ${format === 'xlsx' ? 'Excel' : 'PDF'} exportado exitosamente`);
    } catch (error) {
      toast.error('Error exportando reporte: ' + getTauriErrorMessage(error));
    } finally {
      setExportingFormat(null);
    }
  };

  const normalizedQuery = normalizeText(query);
  const filtrados = preview.filter((d) =>
    normalizeText(d.docente).includes(normalizedQuery) ||
    normalizeText(d.dni).includes(normalizedQuery) ||
    normalizeText(d.grado).includes(normalizedQuery) ||
    normalizeRenacytNivelSearch(d.renacyt_nivel).includes(normalizedQuery)
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
                onChange={(e) => { setTipo(e.target.value as TipoReporte); }}
                aria-label="Seleccionar tipo de reporte"
              >
                <option value="agrupado_docente">Docentes con proyectos (agrupado)</option>
                <option value="plano">Detalle plano (proyecto-docente)</option>
              </select>
            </div>

            {canExport && (
              <div style={{ display: 'flex', gap: '0.75rem', flexWrap: 'wrap' }}>
                <button
                  className="btn-primary"
                  onClick={() => void exportar('xlsx')}
                  disabled={exportingFormat !== null}
                >
                  <span className="button-with-icon">
                    <AppIcon icon={Download} size={18} />
                    <span>{exportingFormat === 'xlsx' ? 'Exportando Excel...' : 'Exportar Excel'}</span>
                  </span>
                </button>
                <button
                  className="btn-secondary"
                  onClick={() => void exportar('pdf')}
                  disabled={exportingFormat !== null}
                >
                  <span className="button-with-icon">
                    <AppIcon icon={Download} size={18} />
                    <span>{exportingFormat === 'pdf' ? 'Exportando PDF...' : 'Exportar PDF'}</span>
                  </span>
                </button>
              </div>
            )}
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
            <span className={`badge ${canExport ? 'badge-success' : 'badge-warning'}`}>{canExport ? 'Exportación habilitada' : 'Solo vista previa'}</span>
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
        {!canExport && (
          <div className="inline-feedback inline-feedback-info">
            <span>Modo consulta: puede revisar la vista previa de reportes, pero la exportación a Excel está deshabilitada para su rol.</span>
          </div>
        )}
        <div className="form-group" style={{ marginBottom: '1rem' }}>
          <input
            className="form-input"
            placeholder="Buscar por docente, DNI, grado o nivel RENACYT"
            value={query}
            onChange={(e) => { setQuery(e.target.value); }}
            aria-label="Buscar en la vista previa por docente, DNI, grado o nivel RENACYT"
          />
        </div>

        {loading ? (
          <SkeletonTable columns={6} rows={6} />
        ) : filtrados.length === 0 ? (
          <div className="empty-state">No hay datos para mostrar</div>
        ) : (
          <table className="table" aria-label="Tabla de vista previa de reportes">
            <thead>
              <tr>
                <th>Docente</th>
                <th>DNI</th>
                <th>Grado</th>
                <th>Nivel RENACYT</th>
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
                  <td>{formatRenacytNivel(row.renacyt_nivel) ?? 'No disponible'}</td>
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