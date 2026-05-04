import type { ProyectoParticipanteResumen } from './api';

export const parseParticipantesProyecto = (value?: string | null): ProyectoParticipanteResumen[] => {
  if (!value) {
    return [];
  }

  try {
    const parsed: unknown = JSON.parse(value);
    return Array.isArray(parsed) ? parsed as ProyectoParticipanteResumen[] : [];
  } catch {
    return [];
  }
};

export const getResponsableProyecto = (participantes: ProyectoParticipanteResumen[]) => {
  return participantes.find((participante) => participante.es_responsable) ?? null;
};