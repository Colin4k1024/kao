import { useState, useEffect } from 'react';

export interface LoadingState {
  loading: boolean;
  loadingCount: number;
}

export const useLoading = (initialValue: boolean = false) => {
  const [loading, setLoading] = useState<boolean>(initialValue);
  const [loadingCount, setLoadingCount] = useState<number>(0);

  const showLoading = () => {
    setLoadingCount((prev) => {
      const newCount = prev + 1;
      if (newCount === 1) {
        setLoading(true);
      }
      return newCount;
    });
  };

  const hideLoading = () => {
    setLoadingCount((prev) => {
      const newCount = prev - 1;
      if (newCount <= 0) {
        setLoading(false);
        return 0;
      }
      return newCount;
    });
  };

  const resetLoading = () => {
    setLoading(false);
    setLoadingCount(0);
  };

  return {
    loading,
    loadingCount,
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
  const { loading, showLoading, hideLoading } = useLoading(false);

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
    loading,
    run,
    data,
    error,
  };
};

// Hook for Debounced Loading
export const useDebouncedLoading = (delay: number = 300) => {
  const { loading, showLoading, hideLoading, loadingCount } = useLoading(false);
  const [debouncedLoading, setDebouncedLoading] = useState(false);

  useEffect(() => {
    let timeoutId: number | undefined;
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
