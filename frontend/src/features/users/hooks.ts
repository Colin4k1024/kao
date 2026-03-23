import { useMutation, useQuery } from "@tanstack/react-query";

import { hasAuthToken } from "@/lib/auth";
import type { HttpError } from "@/lib/http";

import { createUser, fetchUsers, updateUser } from "./api";
import type {
  CreateUserInput,
  UpdateUserInput,
  UserListResponse,
} from "./types";

export const usersQueryKeys = {
  list: ["users", "list"] as const,
};

export function useUsersQuery() {
  return useQuery<UserListResponse, HttpError>({
    queryKey: usersQueryKeys.list,
    queryFn: fetchUsers,
    enabled: hasAuthToken(),
  });
}

export function useCreateUserMutation() {
  return useMutation<void, HttpError, CreateUserInput>({
    mutationFn: createUser,
  });
}

export function useUpdateUserMutation() {
  return useMutation<void, HttpError, { id: string; input: UpdateUserInput }>({
    mutationFn: ({ id, input }) => updateUser(id, input),
  });
}
