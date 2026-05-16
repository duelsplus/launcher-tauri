import { ReactNode } from "react";

interface SettingsSectionProps {
  title: string;
  children: ReactNode;
}

export function SettingsSection({ title, children }: SettingsSectionProps) {
  return (
    <section className="rounded-3xl p-1.5 bg-muted/70">
      <h2 className="text-xs font-bold tracking-widest uppercase text-primary/70 px-2 pt-1 pb-2">
        {title}
      </h2>
      <div className="rounded-2xl bg-background p-1">
        <div className="space-y-1">{children}</div>
      </div>
    </section>
  );
}
