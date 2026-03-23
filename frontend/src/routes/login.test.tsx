import { describe, expect, it } from "vitest";

import { screen } from "@testing-library/react";

import { renderWithAppRouter } from "@/test/render-with-app-router";

describe("router bootstrap", () => {
  it("renders the login route through the real app router", async () => {
    renderWithAppRouter("/login");

    expect(
      await screen.findByRole("heading", { name: "登录" }),
    ).toBeInTheDocument();
    expect(await screen.findByRole("button", { name: "登录" })).toBeInTheDocument();
    expect(await screen.findByLabelText("用户名")).toBeInTheDocument();
  });
});
