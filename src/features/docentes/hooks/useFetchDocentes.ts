import { getAllDocentesConProyectos, type DocenteDetalle } from '../api';
import { useStableFetch } from '../../../shared/hooks/useStableFetch';

export const useFetchDocentes = (refreshTrigger = 0) => {
  const { data, loading, refreshing, error, recargar } = useStableFetch<DocenteDetalle[]>(
    () => getAllDocentesConProyectos(),
    refreshTrigger,
    'Error cargando docentes',
    [],
  );

  return {
    docentes: data.filter((docente) => docente.activo === 1),
    loading,
    refreshing,
    error,
    recargar,
  };
};