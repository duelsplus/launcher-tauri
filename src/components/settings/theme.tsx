import { cn } from "@/lib/utils";
import { CheckIcon } from "@phosphor-icons/react";

interface SettingThemeProps {
  title: string;
  value: "dark" | "light" | "classic";
  onChange: (theme: "dark" | "light" | "classic") => void;
}

const THEMES = ["dark", "classic", "light"] as const;

export function SettingTheme({ title, value, onChange }: SettingThemeProps) {
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
          const isFirst = idx === 0;
          const isLast = idx === arr.length - 1;
          return (
            <button
              key={t}
              onClick={() => onChange(t)}
              className={cn(
                "relative w-12 text-[11px] capitalize border transition-colors",
                "flex justify-center items-center",
                active
                  ? "bg-primary/10 border-primary z-10"
                  : "bg-background border-border dark:border-input classic:border-input hover:bg-muted",
                isFirst && "rounded-l-xl",
                isLast && "rounded-r-xl",
                !isFirst && "-ml-px",
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
