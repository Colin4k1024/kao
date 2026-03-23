import { createRoute } from "@tanstack/react-router";

import { PageHeader, StatePanel } from "./shared";
import { systemRoute } from "./root";
import { useRolesQuery } from "@/features/roles";

export const rolesRoute = createRoute({
  getParentRoute: () => systemRoute,
  path: "roles",
  component: RolesRoute,
});

function RolesRoute() {
  return <RolesManagementPage />;
}

export function RolesManagementPage() {
  const query = useRolesQuery();
  const roles = query.data?.items ?? [];
  const total = query.data?.total ?? roles.length;

  return (
    <section className="space-y-6">
      <PageHeader
        eyebrow="System Management"
        title="角色管理"
        description="管理角色、数据范围和权限绑定，后续可直接接入菜单与部门关联。"
        actions={
          <button
            className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
            type="button"
          >
            新增角色
          </button>
        }
      />

      <div className="grid gap-4 md:grid-cols-3">
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">角色总数</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">{total}</p>
        </article>
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">数据范围</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">已建模</p>
        </article>
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">当前状态</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">
            {query.isLoading ? "同步中" : query.isError ? "异常" : "已就绪"}
          </p>
        </article>
      </div>

      {query.isLoading ? (
        <StatePanel
          title="角色列表加载中"
          description="正在从后端同步角色、权限与数据范围。"
        />
      ) : query.isError ? (
        <StatePanel
          title="角色列表加载失败"
          description={query.error?.message ?? "无法获取角色列表，请稍后重试。"}
          tone="error"
        />
      ) : roles.length === 0 ? (
        <StatePanel
          title="暂无角色数据"
          description="当前查询没有返回角色记录，可以先接入创建角色流程。"
        />
      ) : (
        <div className="grid gap-4 lg:grid-cols-2">
          {roles.map((role) => (
            <article
              key={role.id}
              className="rounded-[28px] border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur"
            >
              <div className="flex items-start justify-between gap-4">
                <div>
                  <h2 className="text-lg font-semibold text-slate-950">{role.name}</h2>
                  <p className="mt-1 text-sm text-slate-500">{role.code}</p>
                </div>
                <span className="rounded-full bg-slate-100 px-3 py-1 text-xs font-medium text-slate-600">
                  {role.status}
                </span>
              </div>

              <div className="mt-4 grid gap-3 text-sm text-slate-600 sm:grid-cols-2">
                <p>数据范围: {role.dataScope}</p>
                <p>用户数: {role.userCount}</p>
                <p>权限数: {role.permissionCount}</p>
                <p>系统角色: {role.isSystem ? "是" : "否"}</p>
              </div>

              <p className="mt-4 text-sm leading-6 text-slate-600">
                {role.description ?? "暂无角色说明。"}
              </p>

              <div className="mt-4 flex gap-2">
                <button
                  className="rounded-full border border-slate-200 px-3 py-1.5 text-xs font-medium text-slate-700"
                  type="button"
                >
                  编辑
                </button>
                <button
                  className="rounded-full border border-slate-200 px-3 py-1.5 text-xs font-medium text-slate-700"
                  type="button"
                >
                  分配菜单
                </button>
              </div>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
