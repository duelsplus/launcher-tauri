import { cn } from "@/lib/utils";
import { CheckIcon } from "@phosphor-icons/react";
import { Ripple } from "m3-ripple";

interface SettingThemeProps {
  title: string;
  value: "dark" | "light" | "classic";
  onChange: (theme: "dark" | "light" | "classic") => void;
}

const THEMES = ["dark", "classic", "light"] as const;
const COLORS: Record<(typeof THEMES)[number], string> = {
  dark: "bg-[#262624]",
  classic: "bg-[#18181B]",
  light: "bg-[#FAF9F5]",
};

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

      <div className="flex gap-1.5 shrink-0">
        {THEMES.map((t) => {
          const active = value === t;
          return (
            <button
              key={t}
              onClick={() => onChange(t)}
              className={cn(
                "group relative size-8 rounded-full border flex items-center justify-center transition-colors",
                COLORS[t],
                active && "border-primary z-10",
              )}
            >
              <Ripple hoverOpacity={0} />
              <CheckIcon
                weight="bold"
                className={cn(
                  "absolute size-4 transition-opacity",
                  active ? "text-primary opacity-100" : "text-foreground opacity-0 group-hover:opacity-50",
                )}
              />
            </button>
          );
        })}
      </div>
    </div>
  );
}
