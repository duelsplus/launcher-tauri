import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  UserIcon,
  SignOutIcon,
  CopyIcon,
  CheckIcon,
  SpinnerIcon,
} from "@phosphor-icons/react";
import { Button } from "@/components/ui/button";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from "@/components/ui/popover";
import { cn } from "@/lib/utils";
import { getToken, setToken } from "@/lib/token";
import { useOnboarding } from "@/lib/onboarding";

type ApiResponse<T> = {
  success: boolean;
  data: T;
};

type User = {
  id: string;
  discordId: string;
  username: string;
  isBanned: boolean;
};

export function UserButton() {
  const [user, setUser] = useState<User | null>(null);
  const [copied, setCopied] = useState(false);
  const [isLoggingOut, setIsLoggingOut] = useState(false);
  const { setOnboarded } = useOnboarding();

  useEffect(() => {
    invoke<ApiResponse<User>>("get_user", {
      token: getToken(),
    })
      .then((u) => setUser(u.data))
      .catch(() => setUser(null));
  }, []);

  const handleCopy = () => {
    if (!user) return;
    navigator.clipboard.writeText(user.discordId);
    setCopied(true);
    setTimeout(() => setCopied(false), 3000);
  };

  const handleLogout = async () => {
    setIsLoggingOut(true);
    try {
      const running = await invoke<boolean>("get_proxy_status");
      if (running) {
        await invoke("stop_proxy");
      }
      await invoke("delete_token");
      setToken(null);
      setOnboarded(false);
    } catch (err) {
      console.error(err);
    } finally {
      setIsLoggingOut(false);
    }
  };

  if (!user)
    return (
      <Button
        variant="muted"
        size="icon-lg"
        className="rounded-[32px] hover:rounded-3xl p-5.5 [&_svg:not([class*='size-'])]:size-6"
      >
        <UserIcon />
      </Button>
    );

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="muted"
          size="icon-lg"
          className="rounded-[32px] hover:rounded-3xl p-5.5 [&_svg:not([class*='size-'])]:size-6"
        >
          <UserIcon />
        </Button>
      </PopoverTrigger>

      <PopoverContent side="right" align="start" className="select-none">
        <div className="flex flex-col p-2 pr-1">
          <div className="text-sm font-medium truncate">{user.username}</div>
          <div className="flex justify-between items-center">
            <span className="text-xs font-mono truncate text-muted-foreground mt-1">
              {user.discordId}
            </span>
            <Button
              variant="ghost"
              size="icon-sm"
              className="size-6 text-muted-foreground hover:text-foreground"
              onClick={handleCopy}
            >
              {copied ? <CheckIcon /> : <CopyIcon />}
            </Button>
          </div>
        </div>

        <div className="h-px bg-border mx-2 my-1" />

        <div className="flex flex-col gap-1">
          <PanelButton
            icon={SignOutIcon}
            label="Log out"
            onClick={handleLogout}
            isLoading={isLoggingOut}
            destructive
          />
        </div>
      </PopoverContent>
    </Popover>
  );
}

function PanelButton({
  icon: Icon,
  label,
  destructive,
  onClick,
  isLoading,
}: {
  icon: any;
  label: string;
  destructive?: boolean;
  onClick?: () => void | Promise<void>;
  isLoading?: boolean;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "flex items-center gap-3 rounded-2xl px-2 py-2 text-sm transition-colors",
        "hover:bg-muted",
        destructive && "text-destructive hover:bg-destructive/10",
      )}
    >
      {isLoading ? (
        <SpinnerIcon className="size-4 animate-spin" />
      ) : (
        <Icon className="size-4" />
      )}
      {label}
    </button>
  );
}
