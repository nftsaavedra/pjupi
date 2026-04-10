import { formatRenacytNivel } from '../../shared/utils/renacyt';
import { getDataExportacionAgrupada, getDataExportacionPlana } from './api';

type TipoReporte = 'agrupado_docente' | 'plano';

interface ReportExportPayload {
  bytes: Uint8Array;
  suggestedName: string;
}

const getReportBaseName = (tipo: TipoReporte) => {
  return tipo === 'agrupado_docente' ? 'docentes-proyectos' : 'detalle-plano';
};

const getSuggestedFileName = (tipo: TipoReporte, extension: 'xlsx' | 'pdf') => {
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

const getSheetName = (tipo: TipoReporte) => {
  return tipo === 'agrupado_docente' ? 'Docentes_Proyectos' : 'Detalle_Plano';
};

export const buildExcelReport = async (tipo: TipoReporte): Promise<ReportExportPayload> => {
  const XLSX = await import('xlsx');
  const rows = await normalizeRows(tipo);
  const worksheet = XLSX.utils.json_to_sheet(rows);
  const workbook = XLSX.utils.book_new();

  XLSX.utils.book_append_sheet(workbook, worksheet, getSheetName(tipo));

  return {
    bytes: new Uint8Array(XLSX.write(workbook, { bookType: 'xlsx', type: 'array' })),
    suggestedName: getSuggestedFileName(tipo, 'xlsx'),
  };
};

