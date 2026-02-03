import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { cn } from "@/lib/utils";
import { Button } from "../ui/button";
import { ArrowUpRightIcon } from "@phosphor-icons/react";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";

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

interface DonationsDialogProps {
  user?: User | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

interface GridItemProps {
  name: string;
  image?: string;
  url: string;
  disabled?: boolean;
}

export function GridItem({
  name,
  image,
  url,
  disabled = false,
}: GridItemProps) {
  return (
    <Tooltip>
      <TooltipTrigger>
        <a
          href={url}
          target="_blank"
          rel="noopener noreferrer"
          className={cn(
            "flex gap-2 rounded-xl border p-3 items-center hover:bg-accent transition cursor-default",
            disabled && "opacity-50 pointer-events-none",
          )}
        >
          {image && (
            <div className="shrink-0">
              <img src={image} className="rounded-full size-6" />
            </div>
          )}
          <div className="flex flex-col">
            <p className="text-sm font-medium">{name}</p>
          </div>
        </a>
      </TooltipTrigger>
      {disabled && (
        <TooltipContent side="top">
          <p>This dude doesn't accept donations. :(</p>
        </TooltipContent>
      )}
    </Tooltip>
  );
}

export function DonationsDialog({
  user,
  open,
  onOpenChange,
}: DonationsDialogProps) {
  const isVeryCool =
    user?.tier === "support" ||
    user?.tier === "supportcombo" ||
    user?.tier === "developer" ||
    user?.tier === "admin";

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Support the Development of Duels+</DialogTitle>
        </DialogHeader>

        <div className="space-y-6">
          <section className="space-y-3 text-sm leading-relaxed text-muted-foreground">
            {user?.id && isVeryCool && (
              <Button size="sm" variant="rose" className="pointer-events-none">
                YOOO, you're already part of the cool club!
              </Button>
            )}

            <p>
              Duels+ is built and maintained by a small independent team of 6.
              There is no company backing it; everything runs on our own money.
            </p>

            <p>
              <strong>100% of donations go toward development.</strong> Nothing
              is split for personal use within the team.
            </p>

            <p>
              Right now, that means covering server costs, hosting, data
              storage, and the tooling we use behind-the-scenes to build, test,
              and ship updates to <strong>you</strong>.
            </p>

            <div className="w-full flex justify-center">
              <a
                href="https://ko-fi.com/duels"
                target="_blank"
                rel="noopener noreferrer"
                className="w-full" /* ts pmo */
              >
                <Button variant="input" size="sm" className="w-full">
                  Donate to Duels+
                  <ArrowUpRightIcon />
                </Button>
              </a>
            </div>
          </section>

          <section className="space-y-3">
            <h3 className="text-xs font-semibold uppercase tracking-widest text-muted-foreground">
              Support the Devs Individually
            </h3>

            <div className="grid grid-cols-1 sm:grid-cols-3 gap-2">
              <GridItem
                name="lee"
                image="https://avatars.githubusercontent.com/u/74821251"
                url="https://ko-fi.com/wyzux"
              />
              <GridItem
                name="Hachem"
                image="https://avatars.githubusercontent.com/u/83752039"
                url="https://ko-fi.com/hxchem"
              />
              <GridItem
                name="Venxm"
                image="https://avatars.githubusercontent.com/u/96634931"
                url=""
              />
              <GridItem
                name="Noowz"
                image="https://avatars.githubusercontent.com/u/44096711"
                url=""
              />
              <GridItem
                name="Stu"
                image="https://avatars.githubusercontent.com/u/73356864"
                url=""
              />
              <GridItem
                name="ref"
                image="https://avatars.githubusercontent.com/u/104659005"
                url=""
                disabled
              />
            </div>

            <p className="text-sm leading-relaxed text-muted-foreground">
              All funds will go <strong>directly to the developers</strong>.
              Every contribution supports them personally for their time and
              work.
            </p>

            {!isVeryCool && (
              <p className="text-xs leading-relaxed text-muted-foreground/70">
                Note: Donating to a developer directly does <strong>not</strong>{" "}
                grant supporter perks.
              </p>
            )}
          </section>
        </div>
      </DialogContent>
    </Dialog>
  );
}
