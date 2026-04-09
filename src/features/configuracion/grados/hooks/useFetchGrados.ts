import { getAllGrados, type GradoAcademico } from '../../api';
import { useStableFetch } from '../../../../shared/hooks/useStableFetch';

export const useFetchGrados = (actorUserId: string, refreshTrigger = 0) => {
  const { data, loading, refreshing, error, recargar } = useStableFetch<GradoAcademico[]>(
    () => getAllGrados(actorUserId),
    refreshTrigger,
    'Error cargando grados',
    [],
  );

  return { grados: data, loading, refreshing, error, recargar };
};