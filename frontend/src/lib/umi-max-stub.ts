// Stub for @umijs/max - this project uses Vite + React Router, not UmiJS
// These are passthrough exports to actual implementations

export const history = {
  push: (path: string | { pathname: string; search?: string; query?: Record<string, string> }) => {
    if (typeof path === 'string') {
      window.location.href = path;
    } else {
      const query = path.query ? '?' + new URLSearchParams(path.query).toString() : '';
      window.location.href = path.pathname + (path.search || '') + query;
    }
  },
  replace: (path: string | { pathname: string; search?: string; query?: Record<string, string> }) => {
    if (typeof path === 'string') {
      window.location.href = path;
    } else {
      const query = path.query ? '?' + new URLSearchParams(path.query).toString() : '';
      window.location.href = path.pathname + (path.search || '') + query;
    }
  },
  goBack: () => window.history.back(),
  location: window.location,
};

export interface InitialState {
  currentUser?: {
    name?: string;
    [key: string]: any;
  };
  [key: string]: any;
}

export const useModel = (model: string): { initialState: InitialState | null; setInitialState: (fn: (s: InitialState) => InitialState) => void } => {
  console.warn(`@umijs/max useModel("${model}") is stubbed`);
  return {
    initialState: null,
    setInitialState: (fn: (s: InitialState) => InitialState) => fn({ currentUser: undefined }),
  };
};

export const useIntl = () => ({
  t: (id: string) => id,
  formatMessage: (params: { id: string; defaultMessage?: string }) => params.defaultMessage || params.id,
});

export const SelectLang = () => null;
export const FormattedMessage = (props: { id: string; defaultMessage?: string }) => props.defaultMessage || props.id;
export const Helmet = (props: { children?: React.ReactNode }) => props.children;
export const useRequest = (service: (...args: any[]) => Promise<any>, options?: any) => {
  return {
    data: null,
    loading: false,
    run: (...args: any[]) => service(...args),
  };
};

export type RequestConfig = {
  errorConfig?: {
    errorHandler?: (error: any) => void;
  };
  requestInterceptors?: any[];
  responseInterceptors?: any[];
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const request = async (options: any) => {
  const { url, method, data } = options;
  const response = await fetch(url, {
    method: method?.toUpperCase() || 'GET',
    headers: { 'Content-Type': 'application/json' },
    body: data ? JSON.stringify(data) : undefined,
  });
  return response.json();
};
