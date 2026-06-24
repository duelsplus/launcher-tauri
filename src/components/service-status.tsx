import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { WarningIcon, InfoIcon, SpinnerIcon } from "@phosphor-icons/react";

import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "./ui/button";
import { getToken } from "@/lib/token";

type ApiResponse<T> = {
  success: boolean;
  data: T;
};

type User = {
  id: string;
  username: string;
  isBanned: boolean;
};

type StatusType = "error" | "warning" | "info";

interface StatusMessage {
  type: StatusType;
  message: string;
  createdAt: string;
}

interface StatusResponse {
  status: string;
  messages: StatusMessage[];
}

const PRIORITY: Record<StatusType, number> = {
  error: 0,
  warning: 1,
  info: 2,
};

const STATUS = {
  error: {
    label: "Critical",
    Icon: WarningIcon,
    iconClass: "text-red-500 dark:text-red-400 classic:text-red-400 black:text-red-400",
  },
  warning: {
    label: "Degraded",
    Icon: WarningIcon,
    iconClass: "text-amber-500 dark:text-amber-400 classic:text-amber-400 black:text-amber-400",
  },
  info: {
    label: "Notice",
    Icon: InfoIcon,
    iconClass: "text-sky-500 dark:text-sky-400 classic:text-sky-400 black:text-sky-400",
  },
} as const;

const COLOR = {
  error: {
    variant: "rose",
    button: "rose",
    bg: "bg-rose-200/50 dark:bg-rose-950/50 classic:bg-rose-950/50 black:bg-rose-950/50",
    text: "text-rose-600 dark:text-rose-400 classic:text-rose-400 black:text-rose-400",
  },
  warning: {
    variant: "warning",
    button: "warning",
    bg: "bg-amber-100/50 dark:bg-amber-950/50 classic:bg-amber-950/50 black:bg-amber-950/50",
    text: "text-amber-600 dark:text-amber-500 classic:text-amber-500 black:text-amber-500",
  },
  info: {
    variant: "info",
    button: "info",
    bg: "bg-sky-200/50 dark:bg-sky-950/50 classic:bg-sky-950/50 black:bg-sky-950/50",
    text: "text-sky-600 dark:text-sky-400 classic:text-sky-400 black:text-sky-400",
  },
} as const;

export default function ServiceStatus() {
  const [user, setUser] = useState<User | null>(null);
  const [messages, setMessages] = useState<StatusMessage[]>([]);
  const [open, setOpen] = useState(false);

  useEffect(() => {
    invoke<ApiResponse<User>>("get_user", {
      token: getToken(),
    })
      .then((u) => setUser(u.data))
      .catch(() => setUser(null));
  }, []);

  useEffect(() => {
    const check = async () => {
      try {
        const { messages } = await invoke<StatusResponse>("get_status");
        setMessages(messages);
      } catch {
        //
      }
    };
    check();
    const interval = setInterval(check, 5 * 60 * 1000); //5min
    return () => clearInterval(interval);
  }, []);

  const sorted = useMemo(
    () => [...messages].sort((a, b) => PRIORITY[a.type] - PRIORITY[b.type]),
    [messages],
  );
  const notices = useMemo(() => {
    if (!user?.isBanned) return sorted;
    return [
      {
        type: "error" as StatusType,
        message: "Your account was banned for breaching the Duels+ ToS.",
        createdAt: new Date().toISOString(),
      },
      ...sorted,
    ];
  }, [user, sorted]);

  const severity = useMemo<StatusType | null>(() => {
    switch (true) {
      case notices.some((m) => m.type === "error"):
        return "error";
      case notices.some((m) => m.type === "warning"):
        return "warning";
      case notices.some((m) => m.type === "info"):
        return "info";
      default:
        return null;
    }
  }, [notices]);
  if (!severity) {
    return null;
  }

  const meta = STATUS[severity];
  const metaClr = COLOR[severity];
  const StatusIcon = meta.Icon;

  if (!user && !notices) {
    return (
      <Button
        size="pill-lg"
        variant="outline"
        className="z-10 gap-2 [&_svg:not([class*='size-'])]:size-6 relative overflow-hidden pointer-events-none"
      >
        <SpinnerIcon className="animate-spin" />
        <span>Connecting...</span>
      </Button>
    )
  }

  return (
    <>
      <Button
        size="pill-lg"
        variant={metaClr.variant}
        className="z-10 gap-2 [&_svg:not([class*='size-'])]:size-6 relative overflow-hidden"
        onClick={() => setOpen(true)}
      >
        <StatusIcon weight="fill" className={`${meta.iconClass}`} />
        <span>{meta.label}</span>
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Service updates</DialogTitle>
          </DialogHeader>

          <section className="space-y-3">
            {notices.map((message, index) => {
              const color = COLOR[message.type];
              const { Icon, label, iconClass } = STATUS[message.type];

              return (
                <div className="space-y-0">
                  <Button
                    size="sm"
                    variant={color.variant}
                    className="pointer-events-none rounded-b-none"
                    ripple={false}
                  >
                    {label}
                  </Button>
                  <div
                    key={`${message.createdAt}-${index}`}
                    className={`p-3 flex gap-3 rounded-xl rounded-tl-none ${color.bg} ${color.text}`}
                  >
                    <Icon
                      weight="fill"
                      className={`mt-0.5 size-4 shrink-0 ${iconClass}`}
                    />
                    <p className="text-sm opacity-80">{message.message}</p>
                  </div>
                </div>
              );
            })}
          </section>
        </DialogContent>
      </Dialog>
    </>
  );
}
