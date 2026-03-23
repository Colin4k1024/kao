import { zodResolver } from "@hookform/resolvers/zod";
import { Navigate, createRoute, useNavigate } from "@tanstack/react-router";
import { useForm } from "react-hook-form";
import { z } from "zod";

import { useLoginMutation } from "@/features/auth";
import { hasAuthToken, setAuthToken } from "@/lib/auth";
import { HttpError } from "@/lib/http";

import { rootRoute } from "./__root";

const loginSchema = z.object({
  username: z.string().min(2, "请输入用户名"),
  password: z.string().min(6, "请输入密码"),
});

type LoginFormValues = z.infer<typeof loginSchema>;

export const loginRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/login",
  component: LoginPage,
});

export function LoginPage() {
  const navigate = useNavigate();
  const loginMutation = useLoginMutation();
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      username: "admin",
      password: "",
    },
  });

  const onSubmit = handleSubmit(async (values) => {
    try {
      const response = await loginMutation.mutateAsync(values);
      setAuthToken(response.access_token);
      navigate({ to: "/", replace: true });
    } catch {
      // Mutation state carries the failure details for the UI.
    }
  });

  if (hasAuthToken()) {
    return <Navigate to="/" replace />;
  }

  const errorMessage = loginMutation.error
    ? loginMutation.error instanceof HttpError
      ? loginMutation.error.message
      : "登录失败，请稍后重试。"
    : null;

  return (
    <main className="flex min-h-screen items-center justify-center px-4 py-10">
      <section className="w-full max-w-md rounded-[28px] border border-white/70 bg-white/85 p-8 shadow-[0_30px_90px_rgba(15,23,42,0.14)] backdrop-blur">
        <div className="space-y-2">
          <p className="text-sm font-medium uppercase tracking-[0.3em] text-slate-500">
            AI Coding Project
          </p>
          <h1 className="text-3xl font-semibold tracking-tight text-slate-950">
            登录
          </h1>
          <p className="text-sm leading-6 text-slate-600">
            使用管理员账号进入系统，认证结果会同步到后端会话中心。
          </p>
        </div>

        <form className="mt-8 space-y-4" onSubmit={onSubmit}>
          <label className="block space-y-2">
            <span className="text-sm font-medium text-slate-700">用户名</span>
            <input
              autoComplete="username"
              className="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3 text-sm outline-none transition focus:border-sky-400 focus:ring-4 focus:ring-sky-100"
              placeholder="admin"
              {...register("username")}
            />
            {errors.username ? (
              <span className="text-xs text-rose-600">
                {errors.username.message}
              </span>
            ) : null}
          </label>

          <label className="block space-y-2">
            <span className="text-sm font-medium text-slate-700">密码</span>
            <input
              autoComplete="current-password"
              className="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3 text-sm outline-none transition focus:border-sky-400 focus:ring-4 focus:ring-sky-100"
              type="password"
              placeholder="••••••••"
              {...register("password")}
            />
            {errors.password ? (
              <span className="text-xs text-rose-600">
                {errors.password.message}
              </span>
            ) : null}
          </label>

          {errorMessage ? (
            <div className="rounded-2xl border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700">
              {errorMessage}
            </div>
          ) : null}

          <button
            className="inline-flex w-full items-center justify-center rounded-2xl bg-slate-950 px-4 py-3 text-sm font-medium text-white transition hover:bg-slate-800 disabled:cursor-not-allowed disabled:opacity-60"
            type="submit"
            disabled={isSubmitting || loginMutation.isPending}
          >
            {isSubmitting || loginMutation.isPending ? "登录中..." : "登录"}
          </button>
        </form>
      </section>
    </main>
  );
}
