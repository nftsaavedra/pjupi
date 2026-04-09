import { getAllDocentes, type Docente } from '../api';
import { useStableFetch } from '../../../shared/hooks/useStableFetch';

export const useFetchDocentes = (refreshTrigger = 0) => {
  const { data, loading, refreshing, error, recargar } = useStableFetch<Docente[]>(
    getAllDocentes,
    refreshTrigger,
    'Error cargando docentes',
    [],
  );

  return { docentes: data, loading, refreshing, error, recargar };
};