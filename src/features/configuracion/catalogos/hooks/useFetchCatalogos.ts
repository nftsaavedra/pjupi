import { getAllCatalogosAdmin, getCatalogos, type CatalogoItem } from '../../api';
import { useStableFetch } from '@/shared/hooks/useStableFetch';

export const useFetchCatalogos = (tipo: string, admin: boolean, refreshTrigger = 0) => {
  const { data, loading, refreshing, error, recargar } = useStableFetch<CatalogoItem[]>(
    () => (admin ? getAllCatalogosAdmin(tipo) : getCatalogos(tipo)),
    refreshTrigger,
    `Error cargando catálogo "${tipo}"`,
    [],
  );

  return { catalogos: data, loading, refreshing, error, recargar };
};
