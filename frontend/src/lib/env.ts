const DEFAULT_API_BASE_URL = "http://127.0.0.1:3001";

export const env = {
  appName: "AI Coding Project",
  apiBaseUrl:
    import.meta.env.VITE_API_BASE_URL?.trim() || DEFAULT_API_BASE_URL,
} as const;
