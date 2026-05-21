import React, { useCallback, useEffect, useState } from 'react';
import { ChevronDown, ChevronRight, Download } from 'lucide-react';
import ExcelJS from 'exceljs';
import { Document, Page, StyleSheet, Text, View, pdf } from '@react-pdf/renderer';
import {
  getDataExportacionAgrupada,
  getReporteDocenteIntegral,
  getReporteProyectoIntegral,
  getReportesDocentesIntegral,
  getTauriErrorMessage,
  type DatosExportDocenteAgrupado,
  type ReporteDocenteIntegral,
  type ReporteProyectoIntegral,
} from './api';
import { getAllProyectosDetalle } from '@/services/tauri/proyectos';
import { getAllDocentes } from '@/services/tauri/docentes';
import type { Docente, ProyectoDetalle } from '@/services/tauri/types';
import { toast } from '@/services/toast';
import { useStableFetchData } from '@/shared/hooks/useStableFetch';
import { useRefreshToast } from '@/shared/hooks/useRefreshToast';
import { AppIcon } from '@/shared/ui/AppIcon';
import { SkeletonTable } from '@/shared/ui/Skeleton';
import { saveDesktopFile } from '@/shared/utils/saveDesktopFile';
import { formatRenacytNivel, normalizeRenacytNivelSearch } from '@/shared/utils/renacyt';

type TipoReporte = 'agrupado_docente' | 'plano';

const normalizeText = (value: string | null | undefined) => (value ?? '').trim().toLowerCase();

const formatTimestamp = (ts?: number | null) =>
  ts ? new Date(ts).toLocaleDateString('es-PE', { dateStyle: 'medium' }) : '-';

const formatBool = (v: boolean | undefined | null) => (v ? 'Sí' : 'No');

const formatArray = (arr?: string[] | null) => (arr?.length ? arr.join(', ') : '-');

interface ReportesTabProps {
  canExport?: boolean;
  refreshTrigger?: number;
}

// ─── Shared sub-components ──────────────────────────────────────────────────

interface SectionHeaderProps {
  label: string;
  count?: number;
  open: boolean;
  onToggle: () => void;
}

const SectionHeader: React.FC<SectionHeaderProps> = ({ label, count, open, onToggle }) => (
  <summary
    style={{
      cursor: 'pointer',
      display: 'flex',
      alignItems: 'center',
      gap: '0.5rem',
      padding: '0.5rem 0.75rem',
      backgroundColor: 'var(--color-surface-alt, #f4f8fb)',
      borderRadius: '6px',
      marginBottom: open ? '0.5rem' : 0,
    }}
    onClick={(e) => {
      e.preventDefault();
      onToggle();
    }}
  >
    <AppIcon icon={open ? ChevronDown : ChevronRight} size={16} />
    <strong>{label}</strong>
    {count != null && <span className="badge badge-info">{count}</span>}
  </summary>
);

interface InfoRowProps {
  label: string;
  value: React.ReactNode;
  minWidth?: string;
}

const InfoRow: React.FC<InfoRowProps> = ({ label, value, minWidth = '160px' }) => (
  <div style={{ display: 'flex', marginBottom: '0.35rem' }}>
    <span style={{ fontWeight: 600, minWidth, color: 'var(--color-text-soft, #60758d)' }}>{label}:</span>
    <span>{value}</span>
  </div>
);

// ─── PDF document components ────────────────────────────────────────────────

const pdfDefaults = StyleSheet.create({
  page: { padding: 28, fontSize: 9, fontFamily: 'Helvetica', color: '#14213d' },
  title: { fontSize: 16, fontFamily: 'Helvetica-Bold', color: '#0b1f33', marginBottom: 6 },
  subtitle: { fontSize: 10, color: '#4a5d75', marginBottom: 14 },
  sectionTitle: {
    fontSize: 12,
    fontFamily: 'Helvetica-Bold',
    color: '#16324f',
    marginBottom: 6,
    borderBottomWidth: 1,
    borderBottomColor: '#d7e3f1',
    paddingBottom: 4,
  },
  table: { borderWidth: 1, borderColor: '#d7e3f1', borderRadius: 4, overflow: 'hidden', marginBottom: 10 },
  row: { flexDirection: 'row' },
  headerRow: { backgroundColor: '#16324f' },
  cell: {
    paddingHorizontal: 6,
    paddingVertical: 5,
    borderRightWidth: 1,
    borderRightColor: '#d7e3f1',
    justifyContent: 'center',
  },
  headerCellText: { color: '#fff', fontSize: 7, fontFamily: 'Helvetica-Bold' },
  cellText: { color: '#14213d', fontSize: 7 },
  infoRow: { flexDirection: 'row', marginBottom: 3 },
  infoLabel: { width: 120, fontFamily: 'Helvetica-Bold', fontSize: 8, color: '#4a5d75' },
  infoValue: { fontSize: 8, color: '#14213d', flex: 1 },
});

interface PdfTableProps {
  columns: string[];
  widths: string[];
  rows: string[][];
}

 
const PdfTable: React.FC<PdfTableProps> = ({ columns, widths, rows }) => (
  <View style={pdfDefaults.table}>
    <View style={[pdfDefaults.row, pdfDefaults.headerRow]} fixed>
      {columns.map((col, i) => (
        <View
          key={col}
          style={[pdfDefaults.cell, { width: widths[i] }, i === columns.length - 1 ? { borderRightWidth: 0 } : {}]}
        >
          <Text style={pdfDefaults.headerCellText}>{col}</Text>
        </View>
      ))}
    </View>
    {rows.map((row, ri) => (
      <View key={ri} style={pdfDefaults.row} wrap={false}>
        {row.map((cell, ci) => (
          <View
            key={ci}
            style={[pdfDefaults.cell, { width: widths[ci] }, ci === columns.length - 1 ? { borderRightWidth: 0 } : {}]}
          >
            <Text style={pdfDefaults.cellText}>{cell}</Text>
          </View>
        ))}
      </View>
    ))}
    {rows.length === 0 && (
      <View style={[pdfDefaults.row, { padding: 10 }]}>
        <Text style={pdfDefaults.cellText}>Sin datos</Text>
      </View>
    )}
  </View>
);

const ProyectoIntegralPdf = ({ report }: { report: ReporteProyectoIntegral }) => {
  const { cabecera, equipo, patentes, productos, equipamientos, financiamientos, resumen_financiero } = report;

  return (
    <Document>
      <Page size="A4" orientation="landscape" style={pdfDefaults.page}>
        <Text style={pdfDefaults.title}>Reporte Integral de Proyecto</Text>
        <Text style={pdfDefaults.subtitle}>{cabecera.titulo_proyecto}</Text>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Cabecera</Text>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>ID Proyecto:</Text><Text style={pdfDefaults.infoValue}>{cabecera.id_proyecto}</Text></View>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>Activo:</Text><Text style={pdfDefaults.infoValue}>{formatBool(cabecera.activo)}</Text></View>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>Campo OCDE:</Text><Text style={pdfDefaults.infoValue}>{cabecera.campo_ocde ?? '-'}</Text></View>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>Programas:</Text><Text style={pdfDefaults.infoValue}>{formatArray(cabecera.programas_relacionados)}</Text></View>
        </View>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Equipo ({report.total_docentes})</Text>
          <PdfTable
            columns={['Docente', 'DNI', 'Grado', 'RENACYT', 'Nivel', 'Grupo', 'Resp.', 'Pubs.']}
            widths={['17%', '12%', '12%', '15%', '10%', '12%', '10%', '12%']}
            rows={equipo.map(m => [
              m.nombres_apellidos, m.dni, m.grado_nombre,
              m.renacyt_codigo_registro ?? '-', m.renacyt_nivel ?? '-',
              m.grupo_nombre ?? '-', formatBool(m.es_responsable),
              String(m.publicaciones_count),
            ])}
          />
        </View>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Patentes ({report.total_patentes})</Text>
          <PdfTable
            columns={['Título', 'N° Patente', 'Tipo', 'Estado', 'País', 'F. Solicitud']}
            widths={['24%', '15%', '14%', '14%', '13%', '20%']}
            rows={patentes.map(p => [
              p.titulo, p.numero_patente ?? '-', p.tipo_nombre ?? '-',
              p.estado_nombre ?? '-', p.pais ?? '-', formatTimestamp(p.fecha_solicitud),
            ])}
          />
        </View>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Productos ({report.total_productos})</Text>
          <PdfTable
            columns={['Nombre', 'Tipo', 'Etapa', 'F. Registro']}
            widths={['40%', '22%', '18%', '20%']}
            rows={productos.map(p => [
              p.nombre, p.tipo_nombre ?? '-', p.etapa_nombre ?? '-', formatTimestamp(p.fecha_registro),
            ])}
          />
        </View>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Equipamientos ({report.total_equipamientos})</Text>
          <PdfTable
            columns={['Nombre', 'Valor Est.', 'Moneda', 'Proveedor', 'F. Adquisición']}
            widths={['28%', '20%', '14%', '20%', '18%']}
            rows={equipamientos.map(e => [
              e.nombre, e.valor_estimado != null ? String(e.valor_estimado) : '-',
              e.moneda_nombre ?? '-', e.proveedor ?? '-', formatTimestamp(e.fecha_adquisicion),
            ])}
          />
        </View>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Financiamiento ({report.total_financiamientos})</Text>
          <PdfTable
            columns={['Entidad', 'Tipo', 'Monto', 'Moneda', 'Estado', 'F. Inicio', 'F. Fin']}
            widths={['20%', '14%', '12%', '11%', '15%', '14%', '14%']}
            rows={financiamientos.map(f => [
              f.entidad_financiadora, f.tipo_nombre ?? '-',
              f.monto != null ? String(f.monto) : '-', f.moneda_nombre ?? '-',
              f.estado_financiero_nombre ?? '-', formatTimestamp(f.fecha_inicio),
              formatTimestamp(f.fecha_fin),
            ])}
          />
          <Text style={{ ...pdfDefaults.sectionTitle, marginTop: 10 }}>Resumen Financiero</Text>
          <Text style={pdfDefaults.infoValue}>
            Total financiamientos: {resumen_financiero.total_financiamientos}
          </Text>
        </View>
      </Page>
    </Document>
  );
};

const DocenteIntegralPdf = ({ report }: { report: ReporteDocenteIntegral }) => {
  const { perfil, proyectos, recursos, publicaciones, trazabilidad } = report;

  return (
    <Document>
      <Page size="A4" orientation="landscape" style={pdfDefaults.page}>
        <Text style={pdfDefaults.title}>Reporte Integral de Investigador</Text>
        <Text style={pdfDefaults.subtitle}>{perfil.nombres_apellidos}</Text>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Perfil</Text>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>DNI:</Text><Text style={pdfDefaults.infoValue}>{perfil.dni}</Text></View>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>Grado:</Text><Text style={pdfDefaults.infoValue}>{perfil.grado_nombre}</Text></View>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>RENACYT Nivel:</Text><Text style={pdfDefaults.infoValue}>{perfil.renacyt_nivel ?? '-'}</Text></View>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>Grupo:</Text><Text style={pdfDefaults.infoValue}>{perfil.grupo_nombre ?? '-'}</Text></View>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>ORCID:</Text><Text style={pdfDefaults.infoValue}>{perfil.renacyt_orcid ?? '-'}</Text></View>
        </View>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Proyectos ({report.total_proyectos})</Text>
          <PdfTable
            columns={['Proyecto', 'Responsable', 'Activo', 'OCDE', 'Colegas', 'Recursos']}
            widths={['30%', '12%', '8%', '14%', '22%', '14%']}
            rows={proyectos.map(p => [
              p.titulo_proyecto, formatBool(p.es_responsable), formatBool(p.activo),
              p.campo_ocde ?? '-',
              p.colegas.map(c => c.nombres_apellidos).join('; ') || '-',
              `P:${p.recursos_en_proyecto.patentes} PR:${p.recursos_en_proyecto.productos} E:${p.recursos_en_proyecto.equipamientos} F:${p.recursos_en_proyecto.financiamientos}`,
            ])}
          />
        </View>

        <View>
          <Text style={pdfDefaults.sectionTitle}>
            Recursos (P:{recursos.total_patentes} | PR:{recursos.total_productos} | E:{recursos.total_equipamientos})
          </Text>
        </View>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Publicaciones ({report.total_publicaciones})</Text>
          <PdfTable
            columns={['Título', 'Tipo', 'DOI', 'Año', 'Journal', 'ISSN']}
            widths={['30%', '16%', '18%', '6%', '18%', '12%']}
            rows={publicaciones.map(p => [
              p.titulo, p.tipo_publicacion ?? '-', p.doi ?? '-',
              p.anio_publicacion != null ? String(p.anio_publicacion) : '-',
              p.journal_titulo ?? '-', p.issn ?? '-',
            ])}
          />
        </View>

        <View>
          <Text style={pdfDefaults.sectionTitle}>Trazabilidad</Text>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>Actualizado:</Text><Text style={pdfDefaults.infoValue}>{formatTimestamp(trazabilidad.updated_at)}</Text></View>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>Sinc. RENACYT:</Text><Text style={pdfDefaults.infoValue}>{formatTimestamp(trazabilidad.fecha_ultima_sincronizacion_renacyt)}</Text></View>
          <View style={pdfDefaults.infoRow}><Text style={pdfDefaults.infoLabel}>Sinc. Pure:</Text><Text style={pdfDefaults.infoValue}>{formatTimestamp(trazabilidad.fecha_ultima_sincronizacion_pure)}</Text></View>
        </View>
      </Page>
    </Document>
  );
};

// ─── Sub-component: single docente report preview ───────────────────────────

interface SingleDocenteReportProps {
  report: ReporteDocenteIntegral;
  expandedSections: Record<string, boolean>;
  toggleSection: (key: string) => void;
  sectionKeyPrefix?: string;
  hideExportButtons?: boolean;
  exportingIntegral?: string | null;
  onExportXLSX?: () => void;
  onExportPDF?: () => void;
}

const SingleDocenteReport: React.FC<SingleDocenteReportProps> = ({
  report,
  expandedSections,
  toggleSection,
  sectionKeyPrefix = '',
  hideExportButtons = false,
  exportingIntegral,
  onExportXLSX,
  onExportPDF,
}) => {
  const { perfil, proyectos, recursos, publicaciones, trazabilidad } = report;
  const k = (s: string) => sectionKeyPrefix + s;

  return (
    <div
      style={{
        marginBottom: '1.5rem',
        border: '1px solid var(--color-border, #d7e3f1)',
        borderRadius: '8px',
        padding: '1rem',
      }}
    >
      <div className="section-header" style={{ marginBottom: '1rem' }}>
        <h3>{perfil.nombres_apellidos}</h3>
        {!hideExportButtons && onExportXLSX && onExportPDF && (
          <div className="section-header-actions" style={{ display: 'flex', gap: '0.5rem' }}>
            <button className="btn-primary" onClick={onExportXLSX} disabled={exportingIntegral !== null}>
              <span className="button-with-icon">
                <AppIcon icon={Download} size={16} />
                <span>{exportingIntegral === 'docente-xlsx' ? 'Exportando...' : 'Excel'}</span>
              </span>
            </button>
            <button className="btn-secondary" onClick={onExportPDF} disabled={exportingIntegral !== null}>
              <span className="button-with-icon">
                <AppIcon icon={Download} size={16} />
                <span>{exportingIntegral === 'docente-pdf' ? 'Exportando...' : 'PDF'}</span>
              </span>
            </button>
          </div>
        )}
      </div>

      <details open={expandedSections[k('doc-perfil')]}>
        <SectionHeader
          label="Perfil"
          open={expandedSections[k('doc-perfil')] ?? false}
          onToggle={() => { toggleSection(k('doc-perfil')); }}
        />
        <div style={{ padding: '0.75rem' }}>
          <InfoRow label="DNI" value={perfil.dni} />
          <InfoRow label="Grado Académico" value={perfil.grado_nombre} />
          <InfoRow label="Código RENACYT" value={perfil.renacyt_codigo_registro ?? '-'} />
          <InfoRow label="ID Investigador" value={perfil.renacyt_id_investigador ?? '-'} />
          <InfoRow label="Nivel RENACYT" value={perfil.renacyt_nivel ?? '-'} />
          <InfoRow label="Grupo RENACYT" value={perfil.renacyt_grupo ?? '-'} />
          <InfoRow label="Condición" value={perfil.renacyt_condicion ?? '-'} />
          <InfoRow label="ORCID" value={perfil.renacyt_orcid ?? '-'} />
          <InfoRow label="Scopus Author ID" value={perfil.renacyt_scopus_author_id ?? '-'} />
          <InfoRow label="Grupo Investigación" value={perfil.grupo_nombre ?? '-'} />
          <InfoRow
            label="Ficha RENACYT"
            value={
              perfil.renacyt_ficha_url ? (
                <a href={perfil.renacyt_ficha_url} target="_blank" rel="noreferrer">
                  Ver ficha
                </a>
              ) : (
                '-'
              )
            }
          />
        </div>
      </details>

      <details open={expandedSections[k('doc-proyectos')]}>
        <SectionHeader
          label="Proyectos"
          count={report.total_proyectos}
          open={expandedSections[k('doc-proyectos')] ?? false}
          onToggle={() => { toggleSection(k('doc-proyectos')); }}
        />
        {proyectos.length === 0 ? (
          <div className="empty-state">Sin proyectos registrados</div>
        ) : (
          <table className="table">
            <thead>
              <tr>
                <th>Proyecto</th>
                <th>Responsable</th>
                <th>Activo</th>
                <th>Campo OCDE</th>
                <th>Colegas</th>
                <th>Recursos (P|PR|E|F)</th>
              </tr>
            </thead>
            <tbody>
              {proyectos.map(p => (
                <tr key={p.id_proyecto}>
                  <td>{p.titulo_proyecto}</td>
                  <td>{formatBool(p.es_responsable)}</td>
                  <td>{formatBool(p.activo)}</td>
                  <td>{p.campo_ocde ?? '-'}</td>
                  <td>{p.colegas.map(c => c.nombres_apellidos).join('; ') || '-'}</td>
                  <td>
                    P:{p.recursos_en_proyecto.patentes} PR:{p.recursos_en_proyecto.productos} E:
                    {p.recursos_en_proyecto.equipamientos} F:{p.recursos_en_proyecto.financiamientos}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </details>

      <details open={expandedSections[k('doc-recursos')]}>
        <SectionHeader
          label="Recursos"
          open={expandedSections[k('doc-recursos')] ?? false}
          onToggle={() => { toggleSection(k('doc-recursos')); }}
        />
        <div style={{ padding: '0.75rem' }}>
          <p>
            <strong>Patentes ({recursos.total_patentes})</strong>
          </p>
          {recursos.patentes.length === 0 ? (
            <div className="empty-state">Sin patentes</div>
          ) : (
            <table className="table">
              <thead>
                <tr>
                  <th>Título</th>
                  <th>N° Patente</th>
                  <th>Tipo</th>
                  <th>Estado</th>
                  <th>País</th>
                </tr>
              </thead>
              <tbody>
                {recursos.patentes.map(p => (
                  <tr key={p.id_patente}>
                    <td>{p.titulo}</td>
                    <td>{p.numero_patente ?? '-'}</td>
                    <td>{p.tipo_nombre ?? '-'}</td>
                    <td>{p.estado_nombre ?? '-'}</td>
                    <td>{p.pais ?? '-'}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
          <p style={{ marginTop: '1rem' }}>
            <strong>Productos ({recursos.total_productos})</strong>
          </p>
          {recursos.productos.length === 0 ? (
            <div className="empty-state">Sin productos</div>
          ) : (
            <table className="table">
              <thead>
                <tr>
                  <th>Nombre</th>
                  <th>Tipo</th>
                  <th>Etapa</th>
                  <th>F. Registro</th>
                </tr>
              </thead>
              <tbody>
                {recursos.productos.map(p => (
                  <tr key={p.id_producto}>
                    <td>{p.nombre}</td>
                    <td>{p.tipo_nombre ?? '-'}</td>
                    <td>{p.etapa_nombre ?? '-'}</td>
                    <td>{formatTimestamp(p.fecha_registro)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
          <p style={{ marginTop: '1rem' }}>
            <strong>Equipamientos ({recursos.total_equipamientos})</strong>
          </p>
          {recursos.equipamientos.length === 0 ? (
            <div className="empty-state">Sin equipamientos</div>
          ) : (
            <table className="table">
              <thead>
                <tr>
                  <th>Nombre</th>
                  <th>Valor Est.</th>
                  <th>Moneda</th>
                  <th>Proveedor</th>
                </tr>
              </thead>
              <tbody>
                {recursos.equipamientos.map(e => (
                  <tr key={e.id_equipamiento}>
                    <td>{e.nombre}</td>
                    <td>{e.valor_estimado != null ? e.valor_estimado.toLocaleString('es-PE') : '-'}</td>
                    <td>{e.moneda_nombre ?? '-'}</td>
                    <td>{e.proveedor ?? '-'}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </details>

      <details open={expandedSections[k('doc-publicaciones')]}>
        <SectionHeader
          label="Publicaciones"
          count={report.total_publicaciones}
          open={expandedSections[k('doc-publicaciones')] ?? false}
          onToggle={() => { toggleSection(k('doc-publicaciones')); }}
        />
        {publicaciones.length === 0 ? (
          <div className="empty-state">Sin publicaciones registradas</div>
        ) : (
          <table className="table">
            <thead>
              <tr>
                <th>Título</th>
                <th>Tipo</th>
                <th>DOI</th>
                <th>Año</th>
                <th>Journal</th>
                <th>ISSN</th>
              </tr>
            </thead>
            <tbody>
              {publicaciones.map(p => (
                <tr key={p.id_publicacion}>
                  <td>{p.titulo}</td>
                  <td>{p.tipo_publicacion ?? '-'}</td>
                  <td>{p.doi ?? '-'}</td>
                  <td>{p.anio_publicacion ?? '-'}</td>
                  <td>{p.journal_titulo ?? '-'}</td>
                  <td>{p.issn ?? '-'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </details>

      <details open={expandedSections[k('doc-trazabilidad')]}>
        <SectionHeader
          label="Trazabilidad"
          open={expandedSections[k('doc-trazabilidad')] ?? false}
          onToggle={() => { toggleSection(k('doc-trazabilidad')); }}
        />
        <div style={{ padding: '0.75rem' }}>
          <InfoRow label="Última actualización" value={formatTimestamp(trazabilidad.updated_at)} />
          <InfoRow
            label="Sincronización RENACYT"
            value={formatTimestamp(trazabilidad.fecha_ultima_sincronizacion_renacyt)}
          />
          <InfoRow label="Sincronización Pure" value={formatTimestamp(trazabilidad.fecha_ultima_sincronizacion_pure)} />
        </div>
      </details>
    </div>
  );
};

// ─── Main component ─────────────────────────────────────────────────────────

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

  // ─── Entity-centric report state ─────────────────────────────────────────

  const [proyectos, setProyectos] = useState<ProyectoDetalle[]>([]);
  const [proyectosLoading, setProyectosLoading] = useState(true);
  const [docentes, setDocentes] = useState<Docente[]>([]);
  const [docentesLoading, setDocentesLoading] = useState(true);

  const [proyectoId, setProyectoId] = useState('');
  const [docenteId, setDocenteId] = useState('');

  const [proyectoReport, setProyectoReport] = useState<ReporteProyectoIntegral | null>(null);
  const [docenteReport, setDocenteReport] = useState<ReporteDocenteIntegral | null>(null);
  const [docenteReports, setDocenteReports] = useState<ReporteDocenteIntegral[]>([]);
  const [generatingProyecto, setGeneratingProyecto] = useState(false);
  const [generatingDocente, setGeneratingDocente] = useState(false);
  const [exportingIntegral, setExportingIntegral] = useState<
    'proyecto-xlsx' | 'proyecto-pdf' | 'docente-xlsx' | 'docente-pdf' | null
  >(null);

  const [expandedSections, setExpandedSections] = useState<Record<string, boolean>>({});

  const toggleSection = useCallback((key: string) => {
    setExpandedSections(prev => ({ ...prev, [key]: !prev[key] }));
  }, []);

  useEffect(() => {
    getAllProyectosDetalle()
      .then(setProyectos)
      .catch(() => { toast.error('Error cargando proyectos'); })
      .finally(() => { setProyectosLoading(false); });
  }, []);

  useEffect(() => {
    getAllDocentes()
      .then(setDocentes)
      .catch(() => { toast.error('Error cargando docentes'); })
      .finally(() => { setDocentesLoading(false); });
  }, []);

  // ─── Existing export ──────────────────────────────────────────────────────

  const exportar = async (format: 'xlsx' | 'pdf') => {
    setExportingFormat(format);

    try {
      const exportPayload =
        format === 'xlsx'
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
        mimeType:
          format === 'xlsx'
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

  // ─── Entity-centric report handlers ───────────────────────────────────────

  const handleGenerateProyecto = async () => {
    if (!proyectoId) {
      toast.info('Seleccione un proyecto');
      return;
    }
    setGeneratingProyecto(true);
    setProyectoReport(null);
    try {
      const report = await getReporteProyectoIntegral(proyectoId);
      setProyectoReport(report);
      setExpandedSections({ 'proy-cabecera': true });
      toast.success('Reporte de proyecto generado');
    } catch (err) {
      toast.error('Error generando reporte: ' + getTauriErrorMessage(err));
    } finally {
      setGeneratingProyecto(false);
    }
  };

  const handleGenerateDocente = async () => {
    if (!docenteId) {
      toast.info('Seleccione un investigador');
      return;
    }
    setGeneratingDocente(true);
    setDocenteReport(null);
    setDocenteReports([]);
    try {
      if (docenteId === '__todos__') {
        const reports = await getReportesDocentesIntegral();
        setDocenteReports(reports);
        setExpandedSections({ 'doc-perfil-0': true });
        toast.success(`${reports.length} reportes generados`);
      } else {
        const report = await getReporteDocenteIntegral(docenteId);
        setDocenteReport(report);
        setExpandedSections({ 'doc-perfil': true });
        toast.success('Reporte de investigador generado');
      }
    } catch (err) {
      toast.error('Error generando reporte: ' + getTauriErrorMessage(err));
    } finally {
      setGeneratingDocente(false);
    }
  };

  // ─── Entity-centric export handlers ───────────────────────────────────────

  const exportProyectoXLSX = async () => {
    if (!proyectoReport) return;
    setExportingIntegral('proyecto-xlsx');
    try {
      const wb = new ExcelJS.Workbook();

      const addSheet = (name: string, rows: object[]) => {
        const ws = wb.addWorksheet(name);
        if (rows.length === 0) {
          ws.addRow(['Sin datos']).commit();
          return;
        }
        ws.columns = Object.keys(rows[0]).map(k => ({ header: k, key: k, width: 24 }));
        for (const r of rows) {
          ws.addRow(r);
        }
      };

      addSheet('Cabecera', [
        {
          ...proyectoReport.cabecera,
          programas_relacionados: proyectoReport.cabecera.programas_relacionados.join(', '),
        },
      ]);
      addSheet('Equipo', proyectoReport.equipo);
      addSheet('Patentes', proyectoReport.patentes);
      addSheet('Productos', proyectoReport.productos);
      addSheet('Equipamientos', proyectoReport.equipamientos);
      addSheet('Financiamiento', proyectoReport.financiamientos);

      const wsResumen = wb.addWorksheet('Resumen Financiero');
      wsResumen.addRow(['Total Financiamientos', proyectoReport.resumen_financiero.total_financiamientos]).commit();
      wsResumen.addRow([]);
      wsResumen.addRow(['Desglose por Moneda']);
      wsResumen.addRow(['Moneda', 'Cantidad', 'Monto Total']);
      for (const d of proyectoReport.resumen_financiero.desglose_por_moneda) {
        wsResumen.addRow([d.moneda_nombre, d.cantidad, d.monto_total]);
      }
      wsResumen.addRow([]);
      wsResumen.addRow(['Desglose por Estado']);
      wsResumen.addRow(['Estado', 'Cantidad']);
      for (const d of proyectoReport.resumen_financiero.desglose_por_estado) {
        wsResumen.addRow([d.estado_nombre, d.cantidad]);
      }
      wsResumen.commit();

      const buffer = await wb.xlsx.writeBuffer();
      const date = new Date().toISOString().split('T')[0];
      await saveDesktopFile({
        suggestedName: `reporte-proyecto-integral-${date}.xlsx`,
        bytes: new Uint8Array(buffer),
        filters: [{ name: 'Archivo Excel', extensions: ['xlsx'] }],
        mimeType: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      });
      toast.success('Excel exportado exitosamente');
    } catch (err) {
      toast.error('Error exportando Excel: ' + getTauriErrorMessage(err));
    } finally {
      setExportingIntegral(null);
    }
  };

  const exportProyectoPDF = async () => {
    if (!proyectoReport) return;
    setExportingIntegral('proyecto-pdf');
    try {
      const blob = await pdf(<ProyectoIntegralPdf report={proyectoReport} />).toBlob();
      const bytes = new Uint8Array(await blob.arrayBuffer());
      const date = new Date().toISOString().split('T')[0];
      await saveDesktopFile({
        suggestedName: `reporte-proyecto-integral-${date}.pdf`,
        bytes,
        filters: [{ name: 'Documento PDF', extensions: ['pdf'] }],
        mimeType: 'application/pdf',
      });
      toast.success('PDF exportado exitosamente');
    } catch (err) {
      toast.error('Error exportando PDF: ' + getTauriErrorMessage(err));
    } finally {
      setExportingIntegral(null);
    }
  };

  const exportDocenteXLSX = async () => {
    if (!docenteReport && docenteReports.length === 0) return;
    setExportingIntegral('docente-xlsx');
    try {
      const wb = new ExcelJS.Workbook();
      const reports = docenteReport ? [docenteReport] : docenteReports;

      for (const rep of reports) {
        const name = rep.perfil.nombres_apellidos.substring(0, 31);
        const addSheet = (suffix: string, rows: object[]) => {
          const ws = wb.addWorksheet(`${name}_${suffix}`);
          if (rows.length === 0) {
            ws.addRow(['Sin datos']).commit();
            return;
          }
          ws.columns = Object.keys(rows[0]).map(k => ({ header: k, key: k, width: 24 }));
          for (const r of rows) {
            ws.addRow(r);
          }
        };

        addSheet('Perfil', [{ ...rep.perfil }]);
        addSheet(
          'Proyectos',
          rep.proyectos.map(p => ({
            ...p,
            colegas: p.colegas.map(c => c.nombres_apellidos).join('; '),
            programas_relacionados: p.programas_relacionados.join(', '),
          })),
        );
        addSheet('Patentes', rep.recursos.patentes);
        addSheet('Productos', rep.recursos.productos);
        addSheet('Equipamientos', rep.recursos.equipamientos);
        addSheet('Publicaciones', rep.publicaciones);
        addSheet('Trazabilidad', [{ ...rep.trazabilidad }]);
      }

      const buffer = await wb.xlsx.writeBuffer();
      const date = new Date().toISOString().split('T')[0];
      await saveDesktopFile({
        suggestedName: `reporte-docente-integral-${date}.xlsx`,
        bytes: new Uint8Array(buffer),
        filters: [{ name: 'Archivo Excel', extensions: ['xlsx'] }],
        mimeType: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      });
      toast.success('Excel exportado exitosamente');
    } catch (err) {
      toast.error('Error exportando Excel: ' + getTauriErrorMessage(err));
    } finally {
      setExportingIntegral(null);
    }
  };

  const exportDocentePDF = async () => {
    if (!docenteReport && docenteReports.length === 0) return;
    setExportingIntegral('docente-pdf');
    try {
      const reports = docenteReport ? [docenteReport] : docenteReports;
      for (const rep of reports) {
        const blob = await pdf(<DocenteIntegralPdf report={rep} />).toBlob();
        const bytes = new Uint8Array(await blob.arrayBuffer());
        const date = new Date().toISOString().split('T')[0];
        const name = rep.perfil.nombres_apellidos.replace(/\s+/g, '_').substring(0, 40);
        await saveDesktopFile({
          suggestedName:
            reports.length === 1
              ? `reporte-docente-integral-${date}.pdf`
              : `reporte-docente-${name}-${date}.pdf`,
          bytes,
          filters: [{ name: 'Documento PDF', extensions: ['pdf'] }],
          mimeType: 'application/pdf',
        });
      }
      toast.success('PDF exportado exitosamente');
    } catch (err) {
      toast.error('Error exportando PDF: ' + getTauriErrorMessage(err));
    } finally {
      setExportingIntegral(null);
    }
  };

  // ─── Computed ─────────────────────────────────────────────────────────────

  const normalizedQuery = normalizeText(query);
  const filtrados = preview.filter(
    d =>
      normalizeText(d.docente).includes(normalizedQuery) ||
      normalizeText(d.dni).includes(normalizedQuery) ||
      normalizeText(d.grado).includes(normalizedQuery) ||
      normalizeRenacytNivelSearch(d.renacyt_nivel).includes(normalizedQuery),
  );

  // ─── JSX ──────────────────────────────────────────────────────────────────

  return (
    <div className="tab-panel module-shell reportes-module">
      {/* ── Existing report section ── */}
      <div className="module-split-layout reportes-layout">
        <div className="form-card">
          <h2>Centro de Reportes</h2>
          <div className="form" style={{ gap: '1rem' }}>
            <div className="form-group">
              <label>Tipo de reporte</label>
              <select
                className="form-input"
                value={tipo}
                onChange={e => {
                  setTipo(e.target.value as TipoReporte);
                }}
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
          <p>Ajuste el formato antes de exportar.</p>
          <div className="module-aside-meta">
            <span className="badge badge-info">Consulta actual: {query ? 'Filtrada' : 'Completa'}</span>
            <span className={`badge ${canExport ? 'badge-success' : 'badge-warning'}`}>
              {canExport ? 'Exportación habilitada' : 'Solo vista previa'}
            </span>
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
            <span>
              Modo consulta: puede revisar la vista previa de reportes, pero la exportación a Excel está
              deshabilitada para su rol.
            </span>
          </div>
        )}
        <div className="form-group" style={{ marginBottom: '1rem' }}>
          <input
            className="form-input"
            placeholder="Buscar por docente, DNI, grado o nivel RENACYT"
            value={query}
            onChange={e => {
              setQuery(e.target.value);
            }}
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

      {/* ── Reporte Integral de Proyecto ── */}
      <div className="form-card" style={{ marginTop: '2rem' }}>
        <h2>Reporte Integral de Proyecto</h2>
        <div className="form" style={{ gap: '1rem' }}>
          <div className="form-group">
            <label>Seleccionar proyecto</label>
            <select
              className="form-input"
              value={proyectoId}
              onChange={e => { setProyectoId(e.target.value); }}
              disabled={proyectosLoading}
              aria-label="Seleccionar proyecto para reporte integral"
            >
              <option value="">{proyectosLoading ? 'Cargando...' : '-- Seleccionar --'}</option>
              {proyectos.map(p => (
                <option key={p.id_proyecto} value={p.id_proyecto}>
                  {p.titulo_proyecto}
                </option>
              ))}
            </select>
          </div>
          <div style={{ display: 'flex', gap: '0.75rem', flexWrap: 'wrap' }}>
            <button
              className="btn-primary"
              onClick={() => void handleGenerateProyecto()}
              disabled={!proyectoId || generatingProyecto}
            >
              {generatingProyecto ? 'Generando...' : 'Generar Reporte'}
            </button>
          </div>
        </div>
      </div>

      {proyectoReport && (
        <div className="table-container" style={{ marginTop: '1rem' }}>
          <div className="section-header">
            <h2>Resultado: {proyectoReport.cabecera.titulo_proyecto}</h2>
            <div className="section-header-actions" style={{ display: 'flex', gap: '0.5rem' }}>
              <button
                className="btn-primary"
                onClick={() => void exportProyectoXLSX()}
                disabled={exportingIntegral !== null}
              >
                <span className="button-with-icon">
                  <AppIcon icon={Download} size={16} />
                  <span>{exportingIntegral === 'proyecto-xlsx' ? 'Exportando...' : 'Excel'}</span>
                </span>
              </button>
              <button
                className="btn-secondary"
                onClick={() => void exportProyectoPDF()}
                disabled={exportingIntegral !== null}
              >
                <span className="button-with-icon">
                  <AppIcon icon={Download} size={16} />
                  <span>{exportingIntegral === 'proyecto-pdf' ? 'Exportando...' : 'PDF'}</span>
                </span>
              </button>
            </div>
          </div>

          <details open={expandedSections['proy-cabecera']}>
<SectionHeader
          label="Cabecera"
          open={expandedSections['proy-cabecera'] ?? false}
          onToggle={() => { toggleSection('proy-cabecera'); }}
        />
            <div style={{ padding: '0.75rem' }}>
              <InfoRow label="ID Proyecto" value={proyectoReport.cabecera.id_proyecto} />
              <InfoRow label="Título" value={proyectoReport.cabecera.titulo_proyecto} />
              <InfoRow label="Activo" value={formatBool(proyectoReport.cabecera.activo)} />
              <InfoRow label="Campo OCDE" value={proyectoReport.cabecera.campo_ocde ?? '-'} />
              <InfoRow label="Programas" value={formatArray(proyectoReport.cabecera.programas_relacionados)} />
              <InfoRow label="Creado" value={proyectoReport.cabecera.fecha_creacion ?? '-'} />
              <InfoRow label="Actualizado" value={proyectoReport.cabecera.fecha_actualizacion ?? '-'} />
            </div>
          </details>

          <details open={expandedSections['proy-equipo']}>
            <SectionHeader
              label="Equipo"
              count={proyectoReport.total_docentes}
              open={expandedSections['proy-equipo'] ?? false}
              onToggle={() => { toggleSection('proy-equipo'); }}
            />
            {proyectoReport.equipo.length === 0 ? (
              <div className="empty-state">Sin miembros registrados</div>
            ) : (
              <table className="table">
                <thead>
                  <tr>
                    <th>Docente</th>
                    <th>DNI</th>
                    <th>Grado</th>
                    <th>RENACYT Reg.</th>
                    <th>Nivel</th>
                    <th>Grupo</th>
                    <th>Responsable</th>
                    <th>Publicaciones</th>
                  </tr>
                </thead>
                <tbody>
                  {proyectoReport.equipo.map(m => (
                    <tr key={m.id_docente}>
                      <td>{m.nombres_apellidos}</td>
                      <td>{m.dni}</td>
                      <td>{m.grado_nombre}</td>
                      <td>{m.renacyt_codigo_registro ?? '-'}</td>
                      <td>{m.renacyt_nivel ?? '-'}</td>
                      <td>{m.grupo_nombre ?? '-'}</td>
                      <td>{formatBool(m.es_responsable)}</td>
                      <td>{m.publicaciones_count}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </details>

          <details open={expandedSections['proy-patentes']}>
            <SectionHeader
              label="Patentes"
              count={proyectoReport.total_patentes}
              open={expandedSections['proy-patentes'] ?? false}
              onToggle={() => { toggleSection('proy-patentes'); }}
            />
            {proyectoReport.patentes.length === 0 ? (
              <div className="empty-state">Sin patentes registradas</div>
            ) : (
              <table className="table">
                <thead>
                  <tr>
                    <th>Título</th>
                    <th>N° Patente</th>
                    <th>Tipo</th>
                    <th>Estado</th>
                    <th>País</th>
                    <th>Entidad</th>
                    <th>F. Solicitud</th>
                    <th>F. Concesión</th>
                  </tr>
                </thead>
                <tbody>
                  {proyectoReport.patentes.map(p => (
                    <tr key={p.id_patente}>
                      <td>{p.titulo}</td>
                      <td>{p.numero_patente ?? '-'}</td>
                      <td>{p.tipo_nombre ?? '-'}</td>
                      <td>{p.estado_nombre ?? '-'}</td>
                      <td>{p.pais ?? '-'}</td>
                      <td>{p.entidad_concedente ?? '-'}</td>
                      <td>{formatTimestamp(p.fecha_solicitud)}</td>
                      <td>{formatTimestamp(p.fecha_concesion)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </details>

          <details open={expandedSections['proy-productos']}>
            <SectionHeader
              label="Productos"
              count={proyectoReport.total_productos}
              open={expandedSections['proy-productos'] ?? false}
              onToggle={() => { toggleSection('proy-productos'); }}
            />
            {proyectoReport.productos.length === 0 ? (
              <div className="empty-state">Sin productos registrados</div>
            ) : (
              <table className="table">
                <thead>
                  <tr>
                    <th>Nombre</th>
                    <th>Tipo</th>
                    <th>Etapa</th>
                    <th>Descripción</th>
                    <th>F. Registro</th>
                  </tr>
                </thead>
                <tbody>
                  {proyectoReport.productos.map(p => (
                    <tr key={p.id_producto}>
                      <td>{p.nombre}</td>
                      <td>{p.tipo_nombre ?? '-'}</td>
                      <td>{p.etapa_nombre ?? '-'}</td>
                      <td>{p.descripcion ?? '-'}</td>
                      <td>{formatTimestamp(p.fecha_registro)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </details>

          <details open={expandedSections['proy-equipamientos']}>
            <SectionHeader
              label="Equipamientos"
              count={proyectoReport.total_equipamientos}
              open={expandedSections['proy-equipamientos'] ?? false}
              onToggle={() => { toggleSection('proy-equipamientos'); }}
            />
            {proyectoReport.equipamientos.length === 0 ? (
              <div className="empty-state">Sin equipamientos registrados</div>
            ) : (
              <table className="table">
                <thead>
                  <tr>
                    <th>Nombre</th>
                    <th>Valor Estimado</th>
                    <th>Moneda</th>
                    <th>Proveedor</th>
                    <th>F. Adquisición</th>
                  </tr>
                </thead>
                <tbody>
                  {proyectoReport.equipamientos.map(e => (
                    <tr key={e.id_equipamiento}>
                      <td>{e.nombre}</td>
                      <td>{e.valor_estimado != null ? e.valor_estimado.toLocaleString('es-PE') : '-'}</td>
                      <td>{e.moneda_nombre ?? '-'}</td>
                      <td>{e.proveedor ?? '-'}</td>
                      <td>{formatTimestamp(e.fecha_adquisicion)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </details>

          <details open={expandedSections['proy-financiamiento']}>
            <SectionHeader
              label="Financiamiento"
              count={proyectoReport.total_financiamientos}
              open={expandedSections['proy-financiamiento'] ?? false}
              onToggle={() => { toggleSection('proy-financiamiento'); }}
            />
            {proyectoReport.financiamientos.length === 0 ? (
              <div className="empty-state">Sin financiamientos registrados</div>
            ) : (
              <table className="table">
                <thead>
                  <tr>
                    <th>Entidad</th>
                    <th>Tipo</th>
                    <th>Monto</th>
                    <th>Moneda</th>
                    <th>Estado</th>
                    <th>F. Inicio</th>
                    <th>F. Fin</th>
                  </tr>
                </thead>
                <tbody>
                  {proyectoReport.financiamientos.map(f => (
                    <tr key={f.id_financiamiento}>
                      <td>{f.entidad_financiadora}</td>
                      <td>{f.tipo_nombre ?? '-'}</td>
                      <td>{f.monto != null ? f.monto.toLocaleString('es-PE') : '-'}</td>
                      <td>{f.moneda_nombre ?? '-'}</td>
                      <td>{f.estado_financiero_nombre ?? '-'}</td>
                      <td>{formatTimestamp(f.fecha_inicio)}</td>
                      <td>{formatTimestamp(f.fecha_fin)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
            <div
              style={{
                padding: '0.75rem',
                backgroundColor: 'var(--color-surface-alt, #f4f8fb)',
                borderRadius: '6px',
                marginTop: '0.5rem',
              }}
            >
              <strong>Resumen Financiero</strong>
              <div style={{ marginTop: '0.5rem' }}>
                <p>
                  Total financiamientos:{' '}
                  <span className="badge badge-info">{proyectoReport.resumen_financiero.total_financiamientos}</span>
                </p>
                <p style={{ marginTop: '0.5rem' }}>Desglose por moneda:</p>
                {proyectoReport.resumen_financiero.desglose_por_moneda.length > 0 ? (
                  <table className="table" style={{ marginTop: '0.25rem' }}>
                    <thead>
                      <tr>
                        <th>Moneda</th>
                        <th>Cantidad</th>
                        <th>Monto Total</th>
                      </tr>
                    </thead>
                    <tbody>
                      {proyectoReport.resumen_financiero.desglose_por_moneda.map((d, i) => (
                        <tr key={i}>
                          <td>{d.moneda_nombre}</td>
                          <td>{d.cantidad}</td>
                          <td>{d.monto_total.toLocaleString('es-PE')}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                ) : (
                  <p>-</p>
                )}
                <p style={{ marginTop: '0.5rem' }}>Desglose por estado:</p>
                {proyectoReport.resumen_financiero.desglose_por_estado.length > 0 ? (
                  <table className="table" style={{ marginTop: '0.25rem' }}>
                    <thead>
                      <tr>
                        <th>Estado</th>
                        <th>Cantidad</th>
                      </tr>
                    </thead>
                    <tbody>
                      {proyectoReport.resumen_financiero.desglose_por_estado.map((d, i) => (
                        <tr key={i}>
                          <td>{d.estado_nombre}</td>
                          <td>{d.cantidad}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                ) : (
                  <p>-</p>
                )}
              </div>
            </div>
          </details>
        </div>
      )}

      {/* ── Reporte Integral de Investigador ── */}
      <div className="form-card" style={{ marginTop: '2rem' }}>
        <h2>Reporte Integral de Investigador</h2>
        <div className="form" style={{ gap: '1rem' }}>
          <div className="form-group">
            <label>Seleccionar investigador</label>
            <select
              className="form-input"
              value={docenteId}
              onChange={e => { setDocenteId(e.target.value); }}
              disabled={docentesLoading}
              aria-label="Seleccionar investigador para reporte integral"
            >
              <option value="">{docentesLoading ? 'Cargando...' : '-- Seleccionar --'}</option>
              <option value="__todos__">— Todos los investigadores —</option>
              {docentes.map(d => (
                <option key={d.id_docente} value={d.id_docente}>
                  {d.nombres_apellidos}
                </option>
              ))}
            </select>
          </div>
          <div style={{ display: 'flex', gap: '0.75rem', flexWrap: 'wrap' }}>
            <button
              className="btn-primary"
              onClick={() => void handleGenerateDocente()}
              disabled={!docenteId || generatingDocente}
            >
              {generatingDocente ? 'Generando...' : 'Generar Reporte'}
            </button>
          </div>
        </div>
      </div>

      {(docenteReport || docenteReports.length > 0) && (
        <div className="table-container" style={{ marginTop: '1rem' }}>
          {docenteReport ? (
            <SingleDocenteReport
              report={docenteReport}
              expandedSections={expandedSections}
              toggleSection={toggleSection}
              exportingIntegral={exportingIntegral}
              onExportXLSX={() => void exportDocenteXLSX()}
              onExportPDF={() => void exportDocentePDF()}
            />
          ) : (
            <>
              <div className="section-header">
                <h2>Resultados ({docenteReports.length} investigadores)</h2>
                <div className="section-header-actions" style={{ display: 'flex', gap: '0.5rem' }}>
                  <button
                    className="btn-primary"
                    onClick={() => void exportDocenteXLSX()}
                    disabled={exportingIntegral !== null}
                  >
                    <span className="button-with-icon">
                      <AppIcon icon={Download} size={16} />
                      <span>{exportingIntegral === 'docente-xlsx' ? 'Exportando...' : 'Excel'}</span>
                    </span>
                  </button>
                  <button
                    className="btn-secondary"
                    onClick={() => void exportDocentePDF()}
                    disabled={exportingIntegral !== null}
                  >
                    <span className="button-with-icon">
                      <AppIcon icon={Download} size={16} />
                      <span>{exportingIntegral === 'docente-pdf' ? 'Exportando...' : 'PDF'}</span>
                    </span>
                  </button>
                </div>
              </div>
              {docenteReports.map((rep, idx) => (
                <SingleDocenteReport
                  key={rep.perfil.id_docente}
                  report={rep}
                  expandedSections={expandedSections}
                  toggleSection={toggleSection}
                  sectionKeyPrefix={`${idx}-`}
                  hideExportButtons
                />
              ))}
            </>
          )}
        </div>
      )}
    </div>
  );
};