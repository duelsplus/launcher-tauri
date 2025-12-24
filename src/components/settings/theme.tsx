import { cn } from "@/lib/utils";
import { CheckIcon } from "@phosphor-icons/react";

interface SettingThemeProps {
  title: string;
  value: "dark" | "light";
  onChange: (theme: "dark" | "light") => void;
}

const THEMES = ["dark", "light"] as const;

export function SettingTheme({
  title,
  value,
  onChange,
}: SettingThemeProps) {
  return (
    <div
      className={cn(
        "w-full flex items-center justify-between gap-4 rounded-2xl px-4 py-3 transition-colors",
        "bg-muted/70",
      )}
    >
      <div className="space-y-0.5">
        <div className="text-sm font-medium text-foreground">{title}</div>
      </div>

      <div className="flex h-10 shrink-0">
        {THEMES.map((t, idx, arr) => {
          const active = value === t;
          return (
            <button
              key={t}
              onClick={() => onChange(t)}
              className={cn(
                "relative w-12 text-[11px] capitalize border transition-colors",
                "flex justify-center items-center",
                active
                  ? "bg-primary/10 border-primary z-10"
                  : "bg-background border-border dark:border-input hover:bg-muted",
                idx === 0 ? "rounded-l-xl" : "rounded-r-xl",
              )}
            >
              <span>{t}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}
