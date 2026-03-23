import { getAuthHeaders } from "./auth";
import { env } from "./env";

export class HttpError extends Error {
  status: number;
  payload: unknown;

  constructor(status: number, message: string, payload: unknown = null) {
    super(message);
    this.name = "HttpError";
    this.status = status;
    this.payload = payload;
  }
}

export interface ApiEnvelope<T> {
  code: number;
  message: string;
  data: T;
}

function buildUrl(path: string) {
  const normalizedPath = path.startsWith("/") ? path : `/${path}`;
  return new URL(normalizedPath, env.apiBaseUrl).toString();
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function isApiEnvelope<T>(value: unknown): value is ApiEnvelope<T> {
  return (
    isRecord(value) &&
    typeof value.code === "number" &&
    typeof value.message === "string" &&
    "data" in value
  );
}

function normalizePayload<T>(response: Response, payload: unknown): T {
  if (isApiEnvelope<T>(payload)) {
    if (payload.code !== 0) {
      throw new HttpError(response.status, payload.message, payload);
    }

    return payload.data;
  }

  return payload as T;
}

async function request<T>(path: string, init: RequestInit = {}) {
  const headers = new Headers(init.headers);

  if (!headers.has("Content-Type") && init.body !== undefined) {
    headers.set("Content-Type", "application/json");
  }

  const authHeaders = getAuthHeaders();

  for (const [key, value] of Object.entries(authHeaders)) {
    headers.set(key, value);
  }

  const response = await fetch(buildUrl(path), {
    ...init,
    headers,
  });

  if (!response.ok) {
    let payload: unknown = null;

    try {
      payload = await response.json();
    } catch {
      payload = await response.text();
    }

    throw new HttpError(
      response.status,
      `Request failed with status ${response.status}`,
      payload,
    );
  }

  if (response.status === 204) {
    return undefined as T;
  }

  const text = await response.text();

  if (!text) {
    return undefined as T;
  }

  const payload = JSON.parse(text) as unknown;

  return normalizePayload<T>(response, payload);
}

export const http = {
  get<T>(path: string) {
    return request<T>(path, { method: "GET" });
  },
  post<T>(path: string, body?: unknown) {
    return request<T>(path, {
      method: "POST",
      body: body === undefined ? undefined : JSON.stringify(body),
    });
  },
  put<T>(path: string, body?: unknown) {
    return request<T>(path, {
      method: "PUT",
      body: body === undefined ? undefined : JSON.stringify(body),
    });
  },
  patch<T>(path: string, body?: unknown) {
    return request<T>(path, {
      method: "PATCH",
      body: body === undefined ? undefined : JSON.stringify(body),
    });
  },
  delete<T>(path: string) {
    return request<T>(path, { method: "DELETE" });
  },
};
