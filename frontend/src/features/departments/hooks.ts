import { useMutation, useQuery } from "@tanstack/react-query";

import { hasAuthToken } from "@/lib/auth";
import type { HttpError } from "@/lib/http";

import {
  createDepartment,
  fetchDepartmentsTree,
  updateDepartment,
} from "./api";
import type {
  CreateDepartmentInput,
  DepartmentTreeResponse,
  UpdateDepartmentInput,
} from "./types";

export const departmentsQueryKeys = {
  tree: ["departments", "tree"] as const,
};

export function useDepartmentsTreeQuery() {
  return useQuery<DepartmentTreeResponse, HttpError>({
    queryKey: departmentsQueryKeys.tree,
    queryFn: fetchDepartmentsTree,
    enabled: hasAuthToken(),
  });
}

export function useCreateDepartmentMutation() {
  return useMutation<void, HttpError, CreateDepartmentInput>({
    mutationFn: createDepartment,
  });
}

export function useUpdateDepartmentMutation() {
  return useMutation<
    void,
    HttpError,
    { id: string; input: UpdateDepartmentInput }
  >({
    mutationFn: ({ id, input }) => updateDepartment(id, input),
  });
}
