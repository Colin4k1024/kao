import { render, screen } from "@testing-library/react";
import { describe, expect, beforeEach, it, vi } from "vitest";

const permissionState = vi.hoisted(() => ({
  permissions: [] as string[],
  isLoading: false,
}));

vi.mock("@/features/auth", () => ({
  usePermissions: () => ({
    permissions: permissionState.permissions,
    isLoading: permissionState.isLoading,
    hasPermission: (required: string | string[]) => {
      if (Array.isArray(required)) {
        return required.every((item) =>
          permissionState.permissions.includes(item),
        );
      }

      return permissionState.permissions.includes(required);
    },
  }),
}));

import { PermissionGuard } from "./permission-guard";

describe("PermissionGuard", () => {
  beforeEach(() => {
    permissionState.permissions = [];
    permissionState.isLoading = false;
  });

  it("renders children when the permission is granted", () => {
    permissionState.permissions = ["system:user:add"];

    render(
      <PermissionGuard required="system:user:add">
        <span>新增用户</span>
      </PermissionGuard>,
    );

    expect(screen.getByText("新增用户")).toBeInTheDocument();
  });

  it("hides children when the permission is missing", () => {
    render(
      <PermissionGuard required="system:user:add">
        <span>新增用户</span>
      </PermissionGuard>,
    );

    expect(screen.queryByText("新增用户")).toBeNull();
  });
});
