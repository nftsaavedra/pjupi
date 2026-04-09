import { invoke } from './client';
import type { Docente, DocenteDetalle, EliminarDocenteResultado, ReniecDniLookupResult } from './types';

export const crearDocente = async (
  dni: string,
  id_grado: string,
  nombres: string,
  apellido_paterno: string,
  apellido_materno?: string,
): Promise<Docente> => {
  return await invoke('crear_docente', {
    request: { dni, id_grado, nombres, apellido_paterno, apellido_materno: apellido_materno?.trim() ? apellido_materno : null },
  });
};

export const getAllDocentes = async (): Promise<Docente[]> => {
  return await invoke('get_all_docentes');
};

export const buscarDocentePorDni = async (dni: string): Promise<Docente | null> => {
  return await invoke('buscar_docente_por_dni', { dni });
};

export const consultarDniReniec = async (numero: string): Promise<ReniecDniLookupResult> => {
  return await invoke('consultar_dni_reniec', { numero });
};

export const getAllDocentesConProyectos = async (): Promise<DocenteDetalle[]> => {
  return await invoke('get_all_docentes_con_proyectos');
};

export const eliminarDocente = async (id_docente: string): Promise<EliminarDocenteResultado> => {
  return await invoke('eliminar_docente', { idDocente: id_docente });
};

export const reactivarDocente = async (id_docente: string): Promise<Docente> => {
  return await invoke('reactivar_docente', { idDocente: id_docente });
};