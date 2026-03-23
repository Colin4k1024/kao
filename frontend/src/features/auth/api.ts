import { http } from "@/lib/http";

import type {
  AuthLoginInput,
  AuthLoginResponse,
  AuthMenuNode,
  AuthMenusResponse,
  AuthPermissionsResponse,
  AuthProfileResponse,
  AuthSessionUser,
} from "./types";

const authBasePath = "/api/v1/auth";

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function unwrapField<T>(value: unknown, field: string): T {
  if (isRecord(value) && field in value) {
    return value[field] as T;
  }

  return value as T;
}

export async function login(input: AuthLoginInput): Promise<AuthLoginResponse> {
  return http.post<AuthLoginResponse>(`${authBasePath}/login`, input);
}

export async function fetchCurrentProfile(): Promise<AuthSessionUser> {
  const payload = await http.get<AuthProfileResponse | AuthSessionUser>(
    `${authBasePath}/profile`,
  );

  return unwrapField<AuthSessionUser>(payload, "user");
}

export async function fetchCurrentPermissions(): Promise<string[]> {
  const payload = await http.get<AuthPermissionsResponse | string[]>(
    `${authBasePath}/permissions`,
  );

  return unwrapField<string[]>(payload, "permissions");
}

export async function fetchCurrentMenus(): Promise<AuthMenuNode[]> {
  const payload = await http.get<AuthMenusResponse | AuthMenuNode[]>(
    `${authBasePath}/menus`,
  );

  return unwrapField<AuthMenuNode[]>(payload, "menus");
}
