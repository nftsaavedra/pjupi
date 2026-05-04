export {
  getAllGrupos,
  getGrupo,
  createGrupo,
  updateGrupo,
  deleteGrupo,
  type CreateGrupoPayload,
  type UpdateGrupoPayload,
} from '@/services/tauri/grupos';

export { getTauriErrorMessage } from '@/services/tauri/error';

export type { GrupoInvestigacion } from '@/services/tauri/types';
