import { createRoute } from "@tanstack/react-router";

import { PageHeader, StatePanel } from "./shared";
import { systemRoute } from "./root";
import { buildNavigationSections, useCurrentMenus } from "@/features/auth";

export const menusRoute = createRoute({
  getParentRoute: () => systemRoute,
  path: "menus",
  component: MenusRoute,
});

function MenusRoute() {
  return <MenusManagementPage />;
}

export function MenusManagementPage() {
  const menusQuery = useCurrentMenus();
  const menuSections = buildNavigationSections(menusQuery.data ?? []);
  const menuCount = menusQuery.data?.length ?? 0;

  return (
    <section className="space-y-6">
      <PageHeader
        eyebrow="System Management"
        title="菜单管理"
        description="展示后端菜单树、目录和按钮权限，为路由和权限控制提供统一入口。"
        actions={
          <button
            className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
            type="button"
          >
            同步菜单
          </button>
        }
      />

      <div className="grid gap-4 md:grid-cols-3">
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">菜单节点</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">{menuCount}</p>
        </article>
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">分组</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">
            {menuSections.length}
          </p>
        </article>
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">当前状态</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">
            {menusQuery.isLoading ? "同步中" : menusQuery.isError ? "异常" : "已就绪"}
          </p>
        </article>
      </div>

      {menusQuery.isLoading ? (
        <StatePanel
          title="菜单树加载中"
          description="正在从会话中心同步菜单和权限节点。"
        />
      ) : menusQuery.isError ? (
        <StatePanel
          title="菜单树加载失败"
          description={menusQuery.error?.message ?? "无法获取菜单树，请稍后重试。"}
          tone="error"
        />
      ) : menuSections.length === 0 ? (
        <StatePanel
          title="暂无菜单数据"
          description="当前角色没有可见菜单，后续可以由后端权限接口填充。"
        />
      ) : (
        <div className="grid gap-4 lg:grid-cols-2">
          {menuSections.map((section) => (
            <article
              key={section.id}
              className="rounded-[28px] border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur"
            >
              <div className="flex items-center justify-between gap-4">
                <div>
                  <h2 className="text-lg font-semibold text-slate-950">{section.label}</h2>
                  <p className="text-xs text-slate-400">
                    {section.items.length} 个可见入口
                  </p>
                </div>
                <span className="rounded-full bg-slate-100 px-3 py-1 text-xs font-medium text-slate-600">
                  {section.icon ?? "menu"}
                </span>
              </div>

              <ul className="mt-4 space-y-2">
                {section.items.map((item) => (
                  <li
                    key={item.id}
                    className="flex items-center justify-between rounded-xl border border-slate-100 bg-slate-50 px-3 py-2 text-sm text-slate-700"
                  >
                    <span>{item.label}</span>
                    <span className="text-xs text-slate-400">
                      {item.permission ?? item.path}
                    </span>
                  </li>
                ))}
              </ul>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
