import { useEffect, useRef, useState } from 'react';
import { getTauriErrorMessage } from '../../services/tauri/error';

export interface StableFetchState<T> {
  data: T;
  loading: boolean;
  refreshing: boolean;
  error: string | null;
  recargar: () => Promise<void>;
}

export const useStableFetch = <T,>(
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
  const fetcherRef = useRef(fetcher);
  const errorLabelRef = useRef(errorLabel);

  useEffect(() => {
    fetcherRef.current = fetcher;
    errorLabelRef.current = errorLabel;
  }, [errorLabel, fetcher]);

  const cargar = async () => {
    const isInitialLoad = !hasLoadedRef.current;

    if (isInitialLoad) {
      setLoading(true);
    } else {
      setRefreshing(true);
    }

    try {
      const nextData = await fetcherRef.current();
      setData(nextData);
      setError(null);
      hasLoadedRef.current = true;
    } catch (err) {
      const message = getTauriErrorMessage(err);
      setError(message);
      console.error(`${errorLabelRef.current}:`, err);
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

export const useStableFetchData = useStableFetch;