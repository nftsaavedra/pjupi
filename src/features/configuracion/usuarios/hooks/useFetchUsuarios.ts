import { getAllUsuarios, type Usuario } from '../../api';
import { useStableFetch } from '../../../../shared/hooks/useStableFetch';

export const useFetchUsuarios = (actorUserId: string, refreshTrigger = 0) => {
  const { data, loading, refreshing, error, recargar } = useStableFetch<Usuario[]>(
    () => getAllUsuarios(actorUserId),
    refreshTrigger,
    'Error cargando usuarios',
    [],
  );

  return { usuarios: data, loading, refreshing, error, recargar };
};