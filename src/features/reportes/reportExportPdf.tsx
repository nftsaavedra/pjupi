import { Document, Page, StyleSheet, Text, View, pdf } from '@react-pdf/renderer';
import { formatRenacytNivel } from '../../shared/utils/renacyt';
import { getDataExportacionAgrupada, getDataExportacionPlana } from './api';

type TipoReporte = 'agrupado_docente' | 'plano';

interface ReportExportColumn {
  key: string;
  label: string;
  width: string;
}

type ReportExportRow = Record<string, string | number>;

interface ReportExportPayload {
  bytes: Uint8Array;
  suggestedName: string;
}

const REPORT_DATE_FORMATTER = new Intl.DateTimeFormat('es-PE', {
  dateStyle: 'medium',
  timeStyle: 'short',
});

const pdfStyles = StyleSheet.create({
  page: {
    padding: 28,
    fontSize: 9,
    fontFamily: 'Helvetica',
    color: '#14213d',
    backgroundColor: '#ffffff',
  },
  header: {
    marginBottom: 18,
    paddingBottom: 12,
    borderBottomWidth: 1,
    borderBottomColor: '#d7e3f1',
  },
  title: {
    fontSize: 18,
    fontFamily: 'Helvetica-Bold',
    color: '#0b1f33',
    marginBottom: 4,
  },
  subtitle: {
    fontSize: 10,
    color: '#4a5d75',
  },
  summaryRow: {
    flexDirection: 'row',
    gap: 10,
    marginBottom: 14,
  },
  summaryCard: {
    flexGrow: 1,
    padding: 10,
    borderRadius: 6,
    backgroundColor: '#f4f8fb',
    borderWidth: 1,
    borderColor: '#d7e3f1',
  },
  summaryLabel: {
    fontSize: 8,
    textTransform: 'uppercase',
    color: '#60758d',
    marginBottom: 3,
  },
  summaryValue: {
    fontSize: 14,
    fontFamily: 'Helvetica-Bold',
    color: '#0b1f33',
  },
  table: {
    borderWidth: 1,
    borderColor: '#d7e3f1',
    borderRadius: 6,
    overflow: 'hidden',
  },
  row: {
    flexDirection: 'row',
  },
  headerRow: {
    backgroundColor: '#16324f',
  },
  bodyRowEven: {
    backgroundColor: '#ffffff',
  },
  bodyRowOdd: {
    backgroundColor: '#f7fafc',
  },
  cell: {
    paddingHorizontal: 8,
    paddingVertical: 7,
    borderRightWidth: 1,
    borderRightColor: '#d7e3f1',
    justifyContent: 'center',
  },
  headerCellText: {
    color: '#ffffff',
    fontSize: 8,
    fontFamily: 'Helvetica-Bold',
  },
  bodyCellText: {
    color: '#14213d',
    fontSize: 8,
  },
  footer: {
    marginTop: 12,
    fontSize: 8,
    color: '#60758d',
    textAlign: 'right',
  },
});

const getColumns = (tipo: TipoReporte): ReportExportColumn[] => {
  if (tipo === 'agrupado_docente') {
    return [
      { key: 'docente', label: 'Docente', width: '23%' },
      { key: 'dni', label: 'DNI', width: '11%' },
      { key: 'grado', label: 'Grado', width: '16%' },
      { key: 'renacyt_nivel', label: 'RENACYT', width: '12%' },
      { key: 'cantidad_proyectos', label: 'Proyectos', width: '10%' },
      { key: 'proyectos', label: 'Detalle', width: '28%' },
    ];
  }

  return [
    { key: 'proyecto', label: 'Proyecto', width: '31%' },
    { key: 'docente', label: 'Docente', width: '24%' },
    { key: 'dni', label: 'DNI', width: '12%' },
    { key: 'grado', label: 'Grado', width: '17%' },
    { key: 'renacyt_nivel', label: 'RENACYT', width: '16%' },
  ];
};

const ReportePdfDocument = ({
  tipo,
  rows,
}: {
  tipo: TipoReporte;
  rows: ReportExportRow[];
}) => {
  const columns = getColumns(tipo);
  const generatedAt = REPORT_DATE_FORMATTER.format(new Date());

  return (
    <Document>
      <Page size="A4" orientation="landscape" style={pdfStyles.page}>
        <View style={pdfStyles.header}>
          <Text style={pdfStyles.title}>Centro de Reportes PJUPI</Text>
          <Text style={pdfStyles.subtitle}>
            {tipo === 'agrupado_docente'
              ? 'Relación agrupada de docentes con trazabilidad de proyectos'
              : 'Detalle plano proyecto-docente para análisis y cruces operativos'}
          </Text>
        </View>

        <View style={pdfStyles.summaryRow}>
          <View style={pdfStyles.summaryCard}>
            <Text style={pdfStyles.summaryLabel}>Formato</Text>
            <Text style={pdfStyles.summaryValue}>{tipo === 'agrupado_docente' ? 'Agrupado' : 'Plano'}</Text>
          </View>
          <View style={pdfStyles.summaryCard}>
            <Text style={pdfStyles.summaryLabel}>Registros</Text>
            <Text style={pdfStyles.summaryValue}>{rows.length}</Text>
          </View>
          <View style={pdfStyles.summaryCard}>
            <Text style={pdfStyles.summaryLabel}>Generado</Text>
            <Text style={pdfStyles.summaryValue}>{generatedAt}</Text>
          </View>
        </View>

        <View style={pdfStyles.table}>
          <View style={[pdfStyles.row, pdfStyles.headerRow]} fixed>
            {columns.map((column, index) => (
              <View
                key={column.key}
                style={[
                  pdfStyles.cell,
                  { width: column.width },
                  ...(index === columns.length - 1 ? [{ borderRightWidth: 0 }] : []),
                ]}
              >
                <Text style={pdfStyles.headerCellText}>{column.label}</Text>
              </View>
            ))}
          </View>

          {rows.map((row, rowIndex) => (
            <View
              key={`${tipo}-${rowIndex}`}
              style={[pdfStyles.row, rowIndex % 2 === 0 ? pdfStyles.bodyRowEven : pdfStyles.bodyRowOdd]}
              wrap={false}
            >
              {columns.map((column, columnIndex) => (
                <View
                  key={column.key}
                  style={[
                    pdfStyles.cell,
                    { width: column.width },
                    ...(columnIndex === columns.length - 1 ? [{ borderRightWidth: 0 }] : []),
                  ]}
                >
                  <Text style={pdfStyles.bodyCellText}>{String(row[column.key] ?? '-')}</Text>
                </View>
              ))}
            </View>
          ))}
        </View>

        <Text style={pdfStyles.footer} fixed>
          Exportado desde PJUPI
        </Text>
      </Page>
    </Document>
  );
};

export const buildPdfDocumentBytes = async (tipo: TipoReporte, rows: ReportExportRow[]) => {
  const blob = await pdf(<ReportePdfDocument tipo={tipo} rows={rows} />).toBlob();
  return new Uint8Array(await blob.arrayBuffer());
};

const getReportBaseName = (tipo: TipoReporte) => {
  return tipo === 'agrupado_docente' ? 'docentes-proyectos' : 'detalle-plano';
};

const getSuggestedFileName = (tipo: TipoReporte, extension: 'pdf') => {
  const date = new Date().toISOString().split('T')[0];
  return `reporte-${getReportBaseName(tipo)}-${date}.${extension}`;
};

const normalizeRows = async (tipo: TipoReporte) => {
  if (tipo === 'agrupado_docente') {
    const rows = await getDataExportacionAgrupada();
    return rows.map((row) => ({
      docente: row.docente,
      dni: row.dni,
      grado: row.grado,
      renacyt_nivel: formatRenacytNivel(row.renacyt_nivel) ?? row.renacyt_nivel,
      cantidad_proyectos: row.cantidad_proyectos,
      proyectos: row.proyectos ?? '-',
    }));
  }

  const rows = await getDataExportacionPlana();
  return rows.map((row) => ({
    proyecto: row.proyecto,
    docente: row.docente,
    dni: row.dni,
    grado: row.grado,
    renacyt_nivel: formatRenacytNivel(row.renacyt_nivel) ?? row.renacyt_nivel,
  }));
};

export const buildPdfReport = async (tipo: TipoReporte): Promise<ReportExportPayload> => {
  const rows = await normalizeRows(tipo);

  return {
    bytes: await buildPdfDocumentBytes(tipo, rows),
    suggestedName: getSuggestedFileName(tipo, 'pdf'),
  };
};