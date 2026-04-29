import ExcelJS from 'exceljs';
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
  const rows = await normalizeRows(tipo);
  const workbook = new ExcelJS.Workbook();
  const worksheet = workbook.addWorksheet(getSheetName(tipo));

  if (rows.length > 0) {
    worksheet.columns = Object.keys(rows[0]).map((key) => ({
      header: key,
      key,
      width: 24,
    }));
    rows.forEach((row) => worksheet.addRow(row));
  }

  const buffer = await workbook.xlsx.writeBuffer();
  return {
    bytes: new Uint8Array(buffer as ArrayBuffer),
    suggestedName: getSuggestedFileName(tipo, 'xlsx'),
  };
};

