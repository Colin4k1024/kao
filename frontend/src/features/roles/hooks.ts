import { useMutation, useQuery } from "@tanstack/react-query";

import { hasAuthToken } from "@/lib/auth";
import type { HttpError } from "@/lib/http";

import { createRole, fetchRoles, updateRole } from "./api";
import type { CreateRoleInput, RoleListResponse, UpdateRoleInput } from "./types";

export const rolesQueryKeys = {
  list: ["roles", "list"] as const,
};

export function useRolesQuery() {
  return useQuery<RoleListResponse, HttpError>({
    queryKey: rolesQueryKeys.list,
    queryFn: fetchRoles,
    enabled: hasAuthToken(),
  });
}

export function useCreateRoleMutation() {
  return useMutation<void, HttpError, CreateRoleInput>({
    mutationFn: createRole,
  });
}

export function useUpdateRoleMutation() {
  return useMutation<void, HttpError, { id: string; input: UpdateRoleInput }>({
    mutationFn: ({ id, input }) => updateRole(id, input),
  });
}
