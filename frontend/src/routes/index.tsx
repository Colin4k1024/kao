import { Link, createRoute } from "@tanstack/react-router";

import { AuthGuard } from "@/components/guards/auth-guard";
import { PermissionGuard } from "@/components/guards/permission-guard";
import { AppShell } from "@/components/layout/app-shell";
import { buildNavigationSections, useCurrentSession } from "@/features/auth";

import { rootRoute } from "./__root";

export const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: IndexRoute,
});

function IndexRoute() {
  const session = useCurrentSession();
  const navigationSections = buildNavigationSections(session.menus);
  const hasRemoteMenus = session.menus.length > 0;

  const summaryCards = [
    {
      label: "当前用户",
      value: session.profile?.displayName ?? "待同步",
      description: session.profile?.username ?? "登录后同步账号资料",
    },
    {
      label: "菜单分组",
      value: hasRemoteMenus ? `${navigationSections.length}` : "本地骨架",
      description: hasRemoteMenus ? "直接来自后端菜单树" : "等待后端菜单接口接入",
    },
    {
      label: "权限标识",
      value: `${session.permissions.length}`,
      description: "按钮级权限控制已接通",
    },
  ];

  return (
    <AuthGuard>
      <AppShell>
        <section className="space-y-6">
          <div className="space-y-2">
            <p className="text-sm font-medium uppercase tracking-[0.28em] text-slate-500">
              Admin Framework
            </p>
            <h1 className="text-3xl font-semibold tracking-tight text-slate-950 md:text-4xl">
              企业后台框架基座
            </h1>
            <p className="max-w-2xl text-sm leading-6 text-slate-600 md:text-base">
              这是前端重构后的起点：登录、路由守卫、应用壳、请求层和会话数据已经
              连起来，后续可以平滑接入用户、角色、部门与菜单模块。
            </p>
          </div>

          {session.isError ? (
            <div className="rounded-2xl border border-amber-200 bg-amber-50 px-4 py-3 text-sm text-amber-800">
              会话数据同步失败，页面已回退到本地菜单骨架。
            </div>
          ) : null}

          <div className="grid gap-4 md:grid-cols-3">
            {summaryCards.map((card) => (
              <article
                key={card.label}
                className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur"
              >
                <p className="text-sm font-medium text-slate-500">{card.label}</p>
                <p className="mt-2 text-2xl font-semibold text-slate-950">
                  {card.value}
                </p>
                <p className="mt-3 text-sm leading-6 text-slate-600">
                  {card.description}
                </p>
              </article>
            ))}
          </div>

          <div className="rounded-[28px] border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
            <div className="flex items-center justify-between gap-4">
              <div>
                <h2 className="text-lg font-semibold text-slate-950">系统管理入口</h2>
                <p className="mt-1 text-sm text-slate-500">
                  这些路由已经接入 TanStack Router，可直接进入用户、角色、部门和菜单页面。
                </p>
              </div>
            </div>

            <div className="mt-4 flex flex-wrap gap-3">
              <Link
                to="/system/users"
                className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
              >
                用户管理
              </Link>
              <Link
                to="/system/roles"
                className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
              >
                角色管理
              </Link>
              <Link
                to="/system/departments"
                className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
              >
                部门管理
              </Link>
              <Link
                to="/system/menus"
                className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
              >
                菜单管理
              </Link>
            </div>
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h2 className="text-lg font-semibold text-slate-950">菜单概览</h2>
              <PermissionGuard required="system:user:add">
                <button
                  className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
                  type="button"
                >
                  新增用户
                </button>
              </PermissionGuard>
            </div>

            <div className="grid gap-4 lg:grid-cols-2">
              {navigationSections.map((section) => (
                <article
                  key={section.id}
                  className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur"
                >
                  <div className="flex items-center justify-between">
                    <h3 className="text-base font-semibold text-slate-950">
                      {section.label}
                    </h3>
                    <span className="text-xs text-slate-400">
                      {section.items.length} 项
                    </span>
                  </div>
                  <ul className="mt-4 space-y-2">
                    {section.items.length > 0 ? (
                      section.items.map((item) => (
                        <li
                          key={item.id}
                          className="flex items-center justify-between rounded-xl border border-slate-100 bg-slate-50 px-3 py-2 text-sm text-slate-700"
                        >
                          <span>{item.label}</span>
                          <span className="text-xs text-slate-400">
                            {item.permission ?? "menu"}
                          </span>
                        </li>
                      ))
                    ) : (
                      <li className="rounded-xl border border-dashed border-slate-200 px-3 py-4 text-sm text-slate-500">
                        {session.isLoading ? "菜单同步中..." : "暂无可见菜单"}
                      </li>
                    )}
                  </ul>
                </article>
              ))}
            </div>
          </div>
        </section>
      </AppShell>
    </AuthGuard>
  );
}
