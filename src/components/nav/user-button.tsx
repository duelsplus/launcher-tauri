import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  UserIcon,
  SignOutIcon,
  CopyIcon,
  CheckIcon,
  SpinnerIcon,
  UserGearIcon,
  ShieldCheckIcon,
  WrenchIcon,
  TestTubeIcon,
  BracketsCurlyIcon,
  StarFourIcon,
  TrophyIcon,
  HeartIcon,
  ClockIcon,
} from "@phosphor-icons/react";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from "@/components/ui/popover";
import { cn } from "@/lib/utils";
import { getToken, setToken } from "@/lib/token";
import { useOnboarding } from "@/lib/onboarding";
import { Skeleton } from "../ui/skeleton";
import { User, Perm } from "@/lib/perm";
import { Ripple } from "m3-ripple";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";

type ApiResponse<T> = {
  success: boolean;
  data: T;
};

const permLabels: Record<Perm, string> = {
  admin: "Admin",
  developer: "Developer",
  moderator: "Moderator",
  tester: "Tester",
  partner: "Partner",
  leaderboard: "Leaderboard",
  supporter: "Supporter",
  combo: "Combo",
  standard: "Standard", // to satisfy typescript
};

const permIcons: Record<Perm, any> = {
  admin: UserGearIcon,
  developer: BracketsCurlyIcon,
  moderator: ShieldCheckIcon,
  tester: TestTubeIcon,
  partner: StarFourIcon,
  leaderboard: TrophyIcon,
  supporter: HeartIcon,
  combo: ClockIcon,
  standard: WrenchIcon, // to satisfy typescript
};

export function PermIcon({ perm }: { perm: Perm }) {
  const Icon = permIcons[perm];
  return <Icon className="text-muted-foreground/30 size-4" weight="fill" />;
}

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

  /*if (!user)
    return (
      <Button
        variant="muted"
        size="icon-lg"
        className="rounded-[32px] hover:rounded-3xl p-5.5 [&_svg:not([class*='size-'])]:size-6"
      >
        <UserIcon />
      </Button>
      );*/

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="muted"
          size="icon-lg"
          className="rounded-[32px] hover:rounded-3xl p-5.5 [&_svg:not([class*='size-'])]:size-6"
          ripple={false}
        >
          <UserIcon />
        </Button>
      </PopoverTrigger>

      <PopoverContent side="right" align="start" className="select-none">
        <div className="flex flex-col p-2 pr-1">
          <div className="flex items-center gap-2">
            {user ? (
              <>
                <div className="text-sm font-medium truncate">
                  {user.username}
                </div>
                <div className="flex items-center gap-1">
                  {(user.perms || [])
                    .filter((p) => p !== "standard") //skip standard perm
                    .map((p) => (
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <span className="inline-flex">
                            <PermIcon key={p} perm={p} />
                          </span>
                        </TooltipTrigger>
                        <TooltipContent side="top" className="text-xs font-medium">
                          {permLabels[p]}
                        </TooltipContent>
                      </Tooltip>
                    ))}
                </div>
              </>
            ) : (
              <>
                <Skeleton className="bg-foreground/10 h-6.5 w-16 rounded-xl" />
                <Skeleton className="bg-foreground/10 h-6.5 w-8 rounded-xl" />
              </>
            )}
          </div>
          <div className="flex justify-between items-center">
            {user ? (
              <>
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
              </>
            ) : (
              <>
                <Skeleton className="bg-foreground/10 h-4 w-38 rounded-xl mt-1" />
              </>
            )}
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
        "relative overflow-hidden flex items-center gap-3 rounded-2xl px-2 py-2 text-sm transition-colors",
        destructive && "text-destructive",
      )}
    >
      <Ripple />
      {isLoading ? (
        <SpinnerIcon className="size-4 animate-spin" />
      ) : (
        <Icon className="size-4" />
      )}
      {label}
    </button>
  );
}
