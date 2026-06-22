import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  UserIcon,
  SignOutIcon,
  CopyIcon,
  CheckIcon,
  SpinnerIcon,
  HeartIcon,
  CaretRightIcon,
  ArrowRightIcon,
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
import { User, hasPerm } from "@/lib/perm";
import { SubscriptionsDialog } from "../dialogs/subscriptions";

type ApiResponse<T> = {
  success: boolean;
  data: T;
};

export function SubscribeBanner() {
  const [user, setUser] = useState<User | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);

  useEffect(() => {
    invoke<ApiResponse<User>>("get_user", {
      token: getToken(),
    })
      .then((u) => setUser(u.data))
      .catch(() => setUser(null));
  }, []);

  const isVeryCool = hasPerm(user, "celestial") || hasPerm(user, "divine");

  return (
    <>
      <div
        onClick={() => setDialogOpen(true)}
        className={`group sticky top-0 z-30 gap-1.5 flex justify-center items-center text-center bg-muted/70 rounded-xl text-sm font-medium py-2 w-full transition-opacity ${!isVeryCool ? "opacity-100" : "opacity-0 pointer-events-none"}`}
      >
        <p>Never refresh your API key again with Celestial</p>
        <span className="relative size-3">
          <CaretRightIcon
            weight="bold"
            className="absolute inset-0 size-3 opacity-50 transition-all duration-200 group-hover:opacity-0 group-hover:translate-x-1"
          />
          <ArrowRightIcon
            weight="bold"
            className="absolute inset-0 size-3 opacity-0 transition-all duration-200 group-hover:opacity-60 group-hover:translate-x-0.5"
          />
        </span>
      </div>

      <SubscriptionsDialog
        user={user}
        open={dialogOpen}
        onOpenChange={setDialogOpen}
      />
    </>
  );
}
