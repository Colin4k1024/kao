import { createRoute } from "@tanstack/react-router";

import { PageHeader, StatePanel } from "./shared";
import { systemRoute } from "./root";
import { useDepartmentsTreeQuery } from "@/features/departments";
import type { DepartmentTreeNode } from "@/features/departments";

export const departmentsRoute = createRoute({
  getParentRoute: () => systemRoute,
  path: "departments",
  component: DepartmentsRoute,
});

function DepartmentsRoute() {
  return <DepartmentsManagementPage />;
}

function DepartmentTreeNodeView({
  name,
  code,
  path,
  leader,
  childrenCount,
}: {
  name: string;
  code: string;
  path: string;
  leader: string | null;
  childrenCount: number;
}) {
  return (
    <article className="rounded-2xl border border-slate-100 bg-slate-50 px-4 py-3">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h3 className="font-medium text-slate-950">{name}</h3>
          <p className="mt-1 text-xs text-slate-400">{code}</p>
        </div>
        <span className="text-xs text-slate-400">{childrenCount} 个子部门</span>
      </div>
      <div className="mt-3 flex flex-wrap gap-2 text-xs text-slate-500">
        <span className="rounded-full bg-white px-2.5 py-1">路径 {path}</span>
        <span className="rounded-full bg-white px-2.5 py-1">
          负责人 {leader ?? "未设置"}
        </span>
      </div>
    </article>
  );
}

function DepartmentBranch({
  node,
  depth = 0,
}: {
  node: DepartmentTreeNode;
  depth?: number;
}) {
  return (
    <li className="space-y-3">
      <DepartmentTreeNodeView
        name={node.name}
        code={node.code}
        path={node.path}
        leader={node.leader}
        childrenCount={node.children.length}
      />
      {node.children.length > 0 ? (
        <ul className="space-y-3 pl-4">
          {node.children.map((child) => (
            <DepartmentBranch key={child.id} node={child} depth={depth + 1} />
          ))}
        </ul>
      ) : null}
    </li>
  );
}

export function DepartmentsManagementPage() {
  const query = useDepartmentsTreeQuery();
  const departments = query.data?.items ?? [];
  const total = departments.length;

  return (
    <section className="space-y-6">
      <PageHeader
        eyebrow="System Management"
        title="部门管理"
        description="按组织树维护部门结构，后续可直接接入用户归属和数据权限过滤。"
        actions={
          <button
            className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition hover:border-slate-300 hover:bg-slate-50"
            type="button"
          >
            新增部门
          </button>
        }
      />

      <div className="grid gap-4 md:grid-cols-3">
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">根部门数</p>
          <p className="mt-2 text-2xl font-semibold text-slate-950">{total}</p>
        </article>
        <article className="rounded-2xl border border-white/70 bg-white/80 p-5 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur">
          <p className="text-sm font-medium text-slate-500">树状结构</p>
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
          title="部门树加载中"
          description="正在从后端同步部门树与层级路径。"
        />
      ) : query.isError ? (
        <StatePanel
          title="部门树加载失败"
          description={query.error?.message ?? "无法获取部门树，请稍后重试。"}
          tone="error"
        />
      ) : departments.length === 0 ? (
        <StatePanel
          title="暂无部门数据"
          description="当前查询没有返回部门记录，可以先接入创建部门流程。"
        />
      ) : (
        <ul className="space-y-4">
          {departments.map((department) => (
            <DepartmentBranch key={department.id} node={department} />
          ))}
        </ul>
      )}
    </section>
  );
}
