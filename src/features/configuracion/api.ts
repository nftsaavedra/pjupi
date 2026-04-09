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
} from '../../services/tauri/configuracion';

export { getTauriErrorMessage } from '../../services/tauri/error';

export type { GradoAcademico, Usuario } from '../../services/tauri/types';