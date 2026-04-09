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
} from '../../services/tauri/docentes';

export { getTauriErrorMessage } from '../../services/tauri/error';

export type {
  Docente,
  DocenteDetalle,
  EliminarDocenteResultado,
  RefreshDocenteRenacytFormacionResultado,
  RenacytFormacionAcademicaResumen,
  RenacytLookupResult,
  ReniecDniLookupResult,
} from '../../services/tauri/types';