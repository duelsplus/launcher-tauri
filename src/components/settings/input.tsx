import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

interface SettingInputProps {
  title: string;
  description?: string;
  value: string | number;
  onChange: (value: string) => void;
  type?: React.HTMLInputTypeAttribute;
  placeholder?: string;
  disabled?: boolean;
  //input constraints
  min?: number;
  max?: number;
  step?: number;
  maxLength?: number;
}

export function SettingInput({
  title,
  description,
  value,
  onChange,
  type = "text",
  placeholder,
  disabled,
  min,
  max,
  step,
  maxLength,
}: SettingInputProps) {
  return (
    <div
      className={cn(
        "w-full flex items-center justify-between gap-4 rounded-2xl px-4 py-3",
        "bg-muted/70 focus-within:bg-muted transition-colors",
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

      <Input
        type={type}
        value={value}
        placeholder={placeholder}
        disabled={disabled}
        min={min}
        max={max}
        step={step}
        maxLength={maxLength}
        onChange={(e) => onChange(e.target.value)}
        className={cn("w-16 h-8 p-0 text-center text-sm", "bg-background")}
      />
    </div>
  );
}
