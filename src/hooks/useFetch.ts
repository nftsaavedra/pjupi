import { useEffect, useRef, useState } from 'react';
import { getAllGrados, getAllDocentes, getAllUsuarios, getTauriErrorMessage, type GradoAcademico, type Docente, type Usuario } from '../services/tauriApi';

interface StableFetchState<T> {
  data: T;
  loading: boolean;
  refreshing: boolean;
  error: string | null;
  recargar: () => Promise<void>;
}

const useStableFetch = <T,>(
  fetcher: () => Promise<T>,
  refreshTrigger: number,
  errorLabel: string,
  initialData: T,
): StableFetchState<T> => {
  const [data, setData] = useState<T>(initialData);
  const [loading, setLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const hasLoadedRef = useRef(false);

  const cargar = async () => {
    const isInitialLoad = !hasLoadedRef.current;

    if (isInitialLoad) {
      setLoading(true);
    } else {
      setRefreshing(true);
    }

    try {
      const nextData = await fetcher();
      setData(nextData);
      setError(null);
      hasLoadedRef.current = true;
    } catch (err) {
      const message = getTauriErrorMessage(err);
      setError(message);
      console.error(`${errorLabel}:`, err);
    } finally {
      if (isInitialLoad) {
        setLoading(false);
      } else {
        setRefreshing(false);
      }
    }
  };

  useEffect(() => {
    void cargar();
  }, [refreshTrigger]);

  return { data, loading, refreshing, error, recargar: cargar };
};

export const useFetchGrados = (refreshTrigger = 0) => {
  const { data, loading, refreshing, error, recargar } = useStableFetch<GradoAcademico[]>(
    getAllGrados,
    refreshTrigger,
    'Error cargando grados',
    [],
  );

  return { grados: data, loading, refreshing, error, recargar };
};

export const useFetchDocentes = (refreshTrigger = 0) => {
  const { data, loading, refreshing, error, recargar } = useStableFetch<Docente[]>(
    getAllDocentes,
    refreshTrigger,
    'Error cargando docentes',
    [],
  );

  return { docentes: data, loading, refreshing, error, recargar };
};

export const useFetchUsuarios = (refreshTrigger = 0) => {
  const { data, loading, refreshing, error, recargar } = useStableFetch<Usuario[]>(
    getAllUsuarios,
    refreshTrigger,
    'Error cargando usuarios',
    [],
  );

  return { usuarios: data, loading, refreshing, error, recargar };
};

export const useStableFetchData = useStableFetch;