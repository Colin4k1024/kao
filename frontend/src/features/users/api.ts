import { http } from "@/lib/http";

import type {
  CreateUserInput,
  UpdateUserInput,
  UserListItem,
  UserListResponse,
} from "./types";

const usersBasePath = "/api/v1/users";

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function readNumber(value: unknown, fallback: number) {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function normalizeCollection<T>(payload: unknown): { items: T[]; total: number } {
  if (Array.isArray(payload)) {
    return {
      items: payload as T[],
      total: payload.length,
    };
  }

  if (!isRecord(payload)) {
    return { items: [], total: 0 };
  }

  for (const key of ["items", "rows", "users", "list"] as const) {
    const value = payload[key];

    if (Array.isArray(value)) {
      return {
        items: value as T[],
        total: readNumber(payload.total, value.length),
      };
    }
  }

  return { items: [], total: readNumber(payload.total, 0) };
}

export async function fetchUsers(): Promise<UserListResponse> {
  const payload = await http.get<unknown>(usersBasePath);
  return normalizeCollection<UserListItem>(payload);
}

export async function createUser(input: CreateUserInput) {
  return http.post<void>(usersBasePath, input);
}

export async function updateUser(id: string, input: UpdateUserInput) {
  return http.put<void>(`${usersBasePath}/${id}`, input);
}
