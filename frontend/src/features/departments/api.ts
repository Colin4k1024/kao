import { http } from "@/lib/http";

import type {
  CreateDepartmentInput,
  DepartmentTreeNode,
  DepartmentTreeResponse,
  UpdateDepartmentInput,
} from "./types";

const departmentsBasePath = "/api/v1/departments";

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function normalizeCollection<T>(payload: unknown): { items: T[] } {
  if (Array.isArray(payload)) {
    return { items: payload as T[] };
  }

  if (!isRecord(payload)) {
    return { items: [] };
  }

  for (const key of ["items", "rows", "departments", "tree", "list"] as const) {
    const value = payload[key];

    if (Array.isArray(value)) {
      return { items: value as T[] };
    }
  }

  return { items: [] };
}

export async function fetchDepartmentsTree(): Promise<DepartmentTreeResponse> {
  const payload = await http.get<unknown>(`${departmentsBasePath}/tree`);
  return normalizeCollection<DepartmentTreeNode>(payload);
}

export async function createDepartment(input: CreateDepartmentInput) {
  return http.post<void>(departmentsBasePath, input);
}

export async function updateDepartment(id: string, input: UpdateDepartmentInput) {
  return http.put<void>(`${departmentsBasePath}/${id}`, input);
}
