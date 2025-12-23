import { Switch } from "@/components/ui/switch";
import { cn } from "@/lib/utils";

interface SettingSwitchProps {
  title: string;
  description?: string;
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  disabled?: boolean;
}

export function SettingSwitch({
  title,
  description,
  checked,
  onCheckedChange,
  disabled,
}: SettingSwitchProps) {
  const toggle = () => {
    if (disabled) return;
    onCheckedChange(!checked);
  };

  return (
    <button
      onClick={toggle}
      className={cn(
        " w-full flex items-center justify-between text-left gap-4 rounded-xl px-4 py-3",
        "bg-muted/50 hover:bg-muted transition-colors",
        disabled && "opacity-50",
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

      <Switch
        checked={checked}
        onCheckedChange={onCheckedChange}
        disabled={disabled}
      />
    </button>
  );
}
