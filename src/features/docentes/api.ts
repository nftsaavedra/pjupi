export {
  buscarDocentePorDni,
  consultarRenacytDocente,
  consultarDniReniec,
  crearDocente,
  eliminarDocente,
  getAllDocentes,
  getAllDocentesConProyectos,
  refrescarFormacionAcademicaRenacytDocente,
  reactivarDocente,
} from '@/services/tauri/docentes';

export {
  sincronizarPublicacionesPure,
  getPublicacionesDocente,
} from '@/services/tauri/pure';

export { getTauriErrorMessage } from '@/services/tauri/error';

export type {
  Docente,
  DocenteDetalle,
  EliminarDocenteResultado,
  Publicacion,
  RefreshDocenteRenacytFormacionResultado,
  RenacytFormacionAcademicaResumen,
  RenacytLookupResult,
  ReniecDniLookupResult,
  SyncPublicacionesResult,
} from '@/services/tauri/types';