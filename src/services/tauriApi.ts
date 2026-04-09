export { getTauriErrorMessage } from './tauri/error';

export type {
  AuthStatus,
  DatosExportDocenteAgrupado,
  Docente,
  DocenteDetalle,
  DocenteProyectosCount,
  EliminarDocenteResultado,
  EliminarGradoResultado,
  EliminarProyectoResultado,
  ExportData,
  GradoAcademico,
  KpisDashboard,
  Proyecto,
  ProyectoDetalle,
  ReniecDniLookupResult,
  Usuario,
} from './tauri/types';

export {
  buscarDocentePorDni,
  consultarDniReniec,
  crearDocente,
  eliminarDocente,
  getAllDocentes,
  getAllDocentesConProyectos,
  reactivarDocente,
} from './tauri/docentes';

export {
  buscarProyectosPorDocente,
  crearProyectoConParticipantes,
  eliminarProyecto,
  eliminarRelacionProyectoDocente,
  eliminarRelacionesProyecto,
  getAllProyectosDetalle,
  reactivarProyecto,
} from './tauri/proyectos';

export {
  actualizarGrado,
  actualizarUsuario,
  crearGrado,
  crearUsuario,
  desactivarUsuario,
  eliminarGrado,
  getAllGrados,
  getAllUsuarios,
  reactivarGrado,
  reactivarUsuario,
} from './tauri/configuracion';

export {
  getEstadisticasProyectosXDocente,
  getKpisDashboard,
} from './tauri/dashboard';

export {
  getDataExportacionAgrupada,
  getDataExportacionPlana,
} from './tauri/reportes';

export {
  getAuthStatus,
  loginUsuario,
  registrarPrimerUsuario,
} from './tauri/auth';