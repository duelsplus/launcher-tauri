import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  UserIcon,
  SignOutIcon,
  CopyIcon,
  CheckIcon,
  SpinnerIcon,
  HeartIcon,
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
import { DonationsDialog } from "../dialogs/donations";

type ApiResponse<T> = {
  success: boolean;
  data: T;
};

export type Tier =
  | "admin"
  | "developer"
  | "moderator"
  | "tester"
  | "partner"
  | "leaderboard"
  | "supportcombo"
  | "support"
  | "combo"
  | "standard";

type User = {
  id: string;
  tier: Tier;
};

export function DonateButton() {
  const [user, setUser] = useState<User | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);

  useEffect(() => {
    invoke<ApiResponse<User>>("get_user", {
      token: getToken(),
    })
      .then((u) => setUser(u.data))
      .catch(() => setUser(null));
  }, []);

  return (
    <>
      <Button
        variant="rose"
        size="icon-lg"
        onClick={() => setDialogOpen(true)}
        className="rounded-[32px] hover:rounded-3xl p-5.5 [&_svg:not([class*='size-'])]:size-6"
      >
        <HeartIcon />
      </Button>

      <DonationsDialog user={user} open={dialogOpen} onOpenChange={setDialogOpen} />
    </>
  );
}
