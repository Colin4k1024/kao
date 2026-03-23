import type { ReactNode } from "react";

type PageHeaderProps = {
  eyebrow: string;
  title: string;
  description: string;
  actions?: ReactNode;
};

type StatePanelProps = {
  title: string;
  description: string;
  tone?: "neutral" | "warning" | "error";
};

export function PageHeader({
  eyebrow,
  title,
  description,
  actions,
}: PageHeaderProps) {
  return (
    <div className="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
      <div className="space-y-2">
        <p className="text-sm font-medium uppercase tracking-[0.28em] text-slate-500">
          {eyebrow}
        </p>
        <h1 className="text-3xl font-semibold tracking-tight text-slate-950 md:text-4xl">
          {title}
        </h1>
        <p className="max-w-3xl text-sm leading-6 text-slate-600 md:text-base">
          {description}
        </p>
      </div>
      {actions ? <div className="flex flex-wrap gap-3">{actions}</div> : null}
    </div>
  );
}

export function StatePanel({
  title,
  description,
  tone = "neutral",
}: StatePanelProps) {
  const toneClasses = {
    neutral: "border-slate-200 bg-white text-slate-700",
    warning: "border-amber-200 bg-amber-50 text-amber-800",
    error: "border-rose-200 bg-rose-50 text-rose-700",
  } as const;

  return (
    <div className={`rounded-2xl border px-4 py-4 ${toneClasses[tone]}`}>
      <p className="font-medium">{title}</p>
      <p className="mt-1 text-sm leading-6">{description}</p>
    </div>
  );
}
