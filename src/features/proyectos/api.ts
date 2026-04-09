export {
  crearProyectoConParticipantes,
  eliminarProyecto,
  eliminarRelacionProyectoDocente,
  eliminarRelacionesProyecto,
  getAllProyectosDetalle,
  reactivarProyecto,
} from '../../services/tauri/proyectos';

export { getTauriErrorMessage } from '../../services/tauri/error';

export type {
  EliminarProyectoResultado,
  Proyecto,
  ProyectoDetalle,
} from '../../services/tauri/types';