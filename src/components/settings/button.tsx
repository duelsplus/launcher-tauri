import { CaretRightIcon } from "@phosphor-icons/react";
import { cn } from "@/lib/utils";

interface SettingButtonProps {
  title: string;
  description?: string;
  onClick?: () => void;
  disabled?: boolean;
}

export function SettingButton({
  title,
  description,
  onClick,
  disabled,
}: SettingButtonProps) {
  return (
    <button
      type="button"
      onClick={() => {
        if (!disabled && onClick) onClick();
      }}
      className={cn(
        "group/item w-full flex items-center justify-between text-left gap-4 rounded-2xl px-4 py-3",
        "bg-muted/70 hover:bg-muted transition-colors",
        disabled && "opacity-50 cursor-not-allowed",
      )}
    >
      <div className="space-y-0.5">
        <div className="text-sm font-medium text-foreground">{title}</div>
        {description && (
          <div className="text-xs text-muted-foreground/70 max-w-md">
            {description}
          </div>
        )}
      </div>

      <CaretRightIcon
        className="size-4 text-muted-foreground transition-transform group-hover/item:translate-x-0.5"
      />
    </button>
  );
}
