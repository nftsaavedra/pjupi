export {
  getDataExportacionAgrupada,
  getDataExportacionPlana,
  getReporteProyectoIntegral,
  getReporteDocenteIntegral,
  getReportesDocentesIntegral,
} from '@/services/tauri/reportes';

export { getTauriErrorMessage } from '@/services/tauri/error';

export type {
  DatosExportDocenteAgrupado,
  ExportData,
  ColegaProyecto,
  EquipamientoConEtiquetas,
  FinanciamientoConEtiquetas,
  MiembroProyectoReporte,
  PatenteConEtiquetas,
  PerfilDocenteReporte,
  ProductoConEtiquetas,
  ProyectoCabeceraReporte,
  ProyectoDocenteDetalle,
  PublicacionConEtiquetas,
  RecursosDocenteResumen,
  RecursosProyectoResumen,
  ReporteDocenteIntegral,
  ReporteProyectoIntegral,
  ResumenFinanciero,
  TrazabilidadDocente,
} from '@/services/tauri/types';