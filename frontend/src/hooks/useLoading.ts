import { useState, useEffect } from 'react';

export interface LoadingState {
  loading: boolean;
  loadingCount: number;
}

export const useLoading = (initialValue: boolean = false) => {
  const [loading, setLoading] = useState<boolean>(initialValue);

  const showLoading = () => {
    setLoading(true);
  };

  const hideLoading = () => {
    setLoading(false);
  };

  const resetLoading = () => {
    setLoading(false);
  };

  return {
    loading,
    showLoading,
    hideLoading,
    resetLoading,
  };
};

// Hook for API loading state
export const useApiLoading = <T extends (...args: any[]) => Promise<any>>(
  apiFn: T
): {
  loading: boolean;
  run: (...args: Parameters<T>) => Promise<void>;
  data: Awaited<ReturnType<T>> | null;
  error: Error | null;
} => {
  const [data, setData] = useState<Awaited<ReturnType<T>> | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const { loading: apiLoading, showLoading, hideLoading } = useLoading(false);

  const run = async (...args: Parameters<T>) => {
    showLoading();
    try {
      const result = await apiFn(...args);
      setData(result);
      setError(null);
      return result;
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
      throw err;
    } finally {
      hideLoading();
    }
  };

  return {
    loading: apiLoading,
    run,
    data,
    error,
  };
};

// Hook for Debounced Loading
export const useDebouncedLoading = (delay: number = 300) => {
  const { loading, showLoading, hideLoading } = useLoading(false);
  const [debouncedLoading, setDebouncedLoading] = useState(false);

  useEffect(() => {
    let timeoutId: ReturnType<typeof setTimeout> | undefined;
    if (loading) {
      timeoutId = setTimeout(() => {
        setDebouncedLoading(true);
      }, delay);
    } else {
      setDebouncedLoading(false);
    }
    return () => {
      if (timeoutId) clearTimeout(timeoutId);
    };
  }, [loading, delay]);

  return {
    loading,
    debouncedLoading,
    showLoading,
    hideLoading,
  };
};

export default useLoading;
