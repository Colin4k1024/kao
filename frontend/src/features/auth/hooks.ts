import {
  useMutation,
  useQuery,
  type UseMutationResult,
} from "@tanstack/react-query";

import { hasAuthToken } from "@/lib/auth";
import type { HttpError } from "@/lib/http";

import {
  fetchCurrentMenus,
  fetchCurrentPermissions,
  fetchCurrentProfile,
  login,
} from "./api";
import type {
  AuthLoginInput,
  AuthLoginResponse,
  AuthMenuNode,
  AuthSessionUser,
  PermissionRequirement,
} from "./types";

export const authQueryKeys = {
  profile: ["auth", "profile"] as const,
  permissions: ["auth", "permissions"] as const,
  menus: ["auth", "menus"] as const,
};

function useAuthedQuery<TData>(
  queryKey: readonly unknown[],
  queryFn: () => Promise<TData>,
) {
  return useQuery<TData, HttpError>({
    queryKey,
    queryFn,
    enabled: hasAuthToken(),
  });
}

export function useLoginMutation(): UseMutationResult<
  AuthLoginResponse,
  HttpError,
  AuthLoginInput
> {
  return useMutation<AuthLoginResponse, HttpError, AuthLoginInput>({
    mutationFn: login,
  });
}

export function useCurrentProfile() {
  return useAuthedQuery<AuthSessionUser>(authQueryKeys.profile, fetchCurrentProfile);
}

export function useCurrentPermissions() {
  return useAuthedQuery<string[]>(
    authQueryKeys.permissions,
    fetchCurrentPermissions,
  );
}

export function useCurrentMenus() {
  return useAuthedQuery<AuthMenuNode[]>(authQueryKeys.menus, fetchCurrentMenus);
}

export function useCurrentSession() {
  const profileQuery = useCurrentProfile();
  const permissionsQuery = useCurrentPermissions();
  const menusQuery = useCurrentMenus();

  return {
    profile: profileQuery.data ?? null,
    permissions: permissionsQuery.data ?? [],
    menus: menusQuery.data ?? [],
    isLoading:
      profileQuery.isLoading || permissionsQuery.isLoading || menusQuery.isLoading,
    isFetching:
      profileQuery.isFetching ||
      permissionsQuery.isFetching ||
      menusQuery.isFetching,
    isError: profileQuery.isError || permissionsQuery.isError || menusQuery.isError,
    error:
      profileQuery.error ?? permissionsQuery.error ?? menusQuery.error ?? null,
  };
}

export function usePermissions() {
  const session = useCurrentSession();

  const hasPermission = (required: PermissionRequirement) => {
    if (Array.isArray(required)) {
      return required.every((permission) =>
        session.permissions.includes(permission),
      );
    }

    return session.permissions.includes(required);
  };

  return {
    ...session,
    hasPermission,
  };
}
