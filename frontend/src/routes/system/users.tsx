import { createRoute } from "@tanstack/react-router";

import { PageHeader, StatePanel } from "./shared";
import { systemRoute } from "./root";
import { useUsersQuery } from "@/features/users";

export const usersRoute = createRoute({
  getParentRoute: () => systemRoute,
  path: "users",
  component: UsersRoute,
});

function UsersRoute() {
  return <UsersManagementPage />;
}

export function UsersManagementPage() {
  const query = useUsersQuery();
  const users = query.data?.items ?? [];
  const total = query.data?.total ?? users.length;

  return (
    <section className="space-y-6">
      <PageHeader
        eyebrow="System Management"
        title="用户管理"
        description="查看用户、角色、部门关联和基础状态，后续可直接接入新增与编辑弹窗。"
        actions={
          <>
            <button
              className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
              type="button"
            >
              新增用户
            </button>
            <button
              className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
              type="button"
            >
              导入用户
            </button>
          </>
        }
      />

      <div className="grid gap-4 md:grid-cols-3">
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">用户总数</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">{total}</p>
        </article>
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">当前状态</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">
            {query.isLoading ? "同步中" : query.isError ? "异常" : "已就绪"}
          </p>
        </article>
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">可见记录</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">{users.length}</p>
        </article>
      </div>

      {query.isLoading ? (
        <StatePanel
          title="用户列表加载中"
          description="正在从后端同步用户数据和角色绑定。"
        />
      ) : query.isError ? (
        <StatePanel
          title="用户列表加载失败"
          description={query.error?.message ?? "无法获取用户列表，请稍后重试。"}
          tone="error"
        />
      ) : users.length === 0 ? (
        <StatePanel
          title="暂无用户数据"
          description="当前查询没有返回用户记录，可以先接入创建用户流程。"
        />
      ) : (
        <div className="overflow-hidden rounded-[28px] border border-white/70 bg-white/80 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <table className="w-full border-collapse text-left text-sm">
            <thead className="bg-slate-50 text-slate-500">
              <tr>
                <th className="px-5 py-4 font-medium">账号</th>
                <th className="px-5 py-4 font-medium">姓名</th>
                <th className="px-5 py-4 font-medium">部门</th>
                <th className="px-5 py-4 font-medium">角色</th>
                <th className="px-5 py-4 font-medium">状态</th>
                <th className="px-5 py-4 font-medium">操作</th>
              </tr>
            </thead>
            <tbody>
              {users.map((user) => (
                <tr key={user.id} className="border-t border-slate-100">
                  <td className="px-5 py-4">
                    <div className="font-medium text-slate-950">{user.username}</div>
                    <div className="text-xs text-slate-400">{user.email ?? "未填写邮箱"}</div>
                  </td>
                  <td className="px-5 py-4 text-slate-700">{user.displayName}</td>
                  <td className="px-5 py-4 text-slate-700">{user.deptName ?? "未分配"}</td>
                  <td className="px-5 py-4 text-slate-700">
                    {user.roles.length > 0
                      ? user.roles.map((role) => role.name).join("、")
                      : "无角色"}
                  </td>
                  <td className="px-5 py-4">
                    <span className="rounded-full bg-slate-100 px-3 py-1 text-xs font-medium text-slate-600">
                      {user.status}
                    </span>
                  </td>
                  <td className="px-5 py-4">
                    <div className="flex gap-2">
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
                        重置密码
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </section>
  );
}
