import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

vi.mock("@/features/users", () => ({
  useUsersQuery: () => ({
    data: undefined,
    isLoading: false,
    isError: false,
    error: null,
  }),
}));

import { UsersManagementPage } from "./users";

describe("UsersManagementPage", () => {
  it("renders an empty state when no users are returned", () => {
    render(<UsersManagementPage />);

    expect(screen.getByRole("heading", { name: "用户管理" })).toBeInTheDocument();
    expect(screen.getByText("暂无用户数据")).toBeInTheDocument();
  });
});
