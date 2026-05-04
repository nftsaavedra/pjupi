import type { RenacytFormacionAcademicaResumen } from '../../features/docentes/api';

export const parseFormacionesAcademicas = (value?: string | null): RenacytFormacionAcademicaResumen[] => {
  if (!value) return [];
  try {
    const parsed: unknown = JSON.parse(value);
    return Array.isArray(parsed) ? parsed as RenacytFormacionAcademicaResumen[] : [];
  } catch {
    return [];
  }
};

export const hasFormacionDate = (value?: number | null): boolean => {
  return Boolean(value && value > 0);
};

export const parseAutores = (value?: string | null): string[] => {
  if (!value) return [];
  try {
    const parsed: unknown = JSON.parse(value);
    return Array.isArray(parsed) ? parsed as string[] : [];
  } catch {
    return [];
  }
};

export const formatDate = (value?: number | null): string => {
  if (!value) return 'No disponible';
  return new Intl.DateTimeFormat('es-PE', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
  }).format(value);
};
