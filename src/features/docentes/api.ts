export {
  buscarDocentePorDni,
  consultarDniReniec,
  crearDocente,
  eliminarDocente,
  getAllDocentes,
  getAllDocentesConProyectos,
  reactivarDocente,
} from '../../services/tauri/docentes';

export { getTauriErrorMessage } from '../../services/tauri/error';

export type {
  Docente,
  DocenteDetalle,
  EliminarDocenteResultado,
  ReniecDniLookupResult,
} from '../../services/tauri/types';