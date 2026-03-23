import { Link, useNavigate } from "@tanstack/react-router";
import { useQueryClient } from "@tanstack/react-query";
import type { PropsWithChildren } from "react";

import { buildNavigationSections, useCurrentSession } from "@/features/auth";
import { clearAuthToken } from "@/lib/auth";

export function AppShell({ children }: PropsWithChildren) {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const session = useCurrentSession();
  const navigationSections = buildNavigationSections(session.menus);
  const hasRemoteMenus = session.menus.length > 0;

  const handleLogout = () => {
    clearAuthToken();
    queryClient.clear();
    navigate({ to: "/login", replace: true });
  };

  return (
    <div className="min-h-screen px-4 py-4 text-slate-950 md:px-6 md:py-6">
      <div className="mx-auto grid min-h-[calc(100vh-2rem)] max-w-7xl gap-6 md:grid-cols-[280px_1fr]">
        <aside className="rounded-[28px] border border-white/70 bg-white/80 p-6 shadow-[0_30px_90px_rgba(15,23,42,0.12)] backdrop-blur">
          <div className="space-y-3">
            <p className="text-sm font-medium uppercase tracking-[0.3em] text-slate-500">
              AI Coding Project
            </p>
            <h2 className="text-2xl font-semibold tracking-tight">
              企业后台框架
            </h2>
            <p className="text-sm leading-6 text-slate-600">
              Vite + React + TanStack Router 的前端壳子已经启动，后续会直接消费后端菜单和权限会话。
            </p>
          </div>

          <div className="mt-8 rounded-2xl border border-slate-100 bg-slate-50 p-4">
            <p className="text-sm font-medium text-slate-500">当前会话</p>
            <div className="mt-2 space-y-1">
              <p className="text-base font-semibold text-slate-950">
                {session.profile?.displayName ?? "待同步用户"}
              </p>
              <p className="text-sm text-slate-600">
                {session.profile?.username ?? "登录后读取账号信息"}
              </p>
            </div>
            <div className="mt-4 flex flex-wrap gap-2">
              <span className="rounded-full bg-slate-900 px-3 py-1 text-xs font-medium text-white">
                {hasRemoteMenus
                  ? `${navigationSections.length} 个菜单分组`
                  : "本地菜单骨架"}
              </span>
              <span className="rounded-full bg-white px-3 py-1 text-xs font-medium text-slate-700">
                {session.permissions.length} 个权限
              </span>
            </div>
          </div>

          <div className="mt-8 space-y-5">
            {navigationSections.map((section) => (
              <div key={section.id} className="space-y-3">
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium text-slate-500">
                    {section.label}
                  </span>
                  <span className="text-xs text-slate-400">
                    {section.items.length > 0
                      ? `${section.items.length} 项`
                      : session.isLoading
                        ? "同步中"
                        : "暂无"}
                  </span>
                </div>
                <div className="space-y-2">
                  {section.items.length > 0 ? (
                    section.items.map((item) => (
                      hasRemoteMenus ? (
                        <Link
                          key={item.id}
                          to={item.path}
                          className="flex items-center justify-between rounded-2xl border border-slate-100 bg-white px-4 py-3 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
                          activeProps={{
                            className:
                              "border-slate-900 bg-slate-950 text-white hover:border-slate-900 hover:bg-slate-950",
                          }}
                        >
                          <span>{item.label}</span>
                          <span className="text-xs opacity-70">
                            {item.permission ?? "menu"}
                          </span>
                        </Link>
                      ) : (
                        <div
                          key={item.id}
                          className="flex items-center justify-between rounded-2xl border border-slate-100 bg-white px-4 py-3 text-sm font-medium text-slate-700"
                        >
                          <span>{item.label}</span>
                          <span className="text-xs opacity-70">
                            {item.permission ?? "menu"}
                          </span>
                        </div>
                      )
                    ))
                  ) : (
                    <div className="rounded-2xl border border-dashed border-slate-200 bg-white px-4 py-4 text-sm text-slate-500">
                      {session.isLoading
                        ? "菜单树同步中..."
                        : "当前角色暂无可见菜单"}
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>

          <button
            className="mt-8 inline-flex w-full items-center justify-center rounded-2xl border border-slate-200 bg-white px-4 py-3 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
            type="button"
            onClick={handleLogout}
          >
            退出登录
          </button>
        </aside>

        <main className="rounded-[28px] border border-white/70 bg-white/75 p-6 shadow-[0_30px_90px_rgba(15,23,42,0.12)] backdrop-blur md:p-8">
          {children}
        </main>
      </div>
    </div>
  );
}
