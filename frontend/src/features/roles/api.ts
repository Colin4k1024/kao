import { http } from "@/lib/http";

import type {
  CreateRoleInput,
  RoleListItem,
  RoleListResponse,
  UpdateRoleInput,
} from "./types";

const rolesBasePath = "/api/v1/roles";

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

  for (const key of ["items", "rows", "roles", "list"] as const) {
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

export async function fetchRoles(): Promise<RoleListResponse> {
  const payload = await http.get<unknown>(rolesBasePath);
  return normalizeCollection<RoleListItem>(payload);
}

export async function createRole(input: CreateRoleInput) {
  return http.post<void>(rolesBasePath, input);
}

export async function updateRole(id: string, input: UpdateRoleInput) {
  return http.put<void>(`${rolesBasePath}/${id}`, input);
}
