import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { cn } from "@/lib/utils";
import { Button } from "../ui/button";
import { ArrowUpRightIcon, CheckIcon } from "@phosphor-icons/react";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";
import { User, hasPerm } from "@/lib/perm";
import { Ripple } from "m3-ripple";

interface SubscriptionsDialogProps {
  user?: User | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function SubscriptionsDialog({
  user,
  open,
  onOpenChange,
}: SubscriptionsDialogProps) {
  const isVeryCool = hasPerm(user, "celestial") || hasPerm(user, "divine");
  const isCele = hasPerm(user, "celestial");
  const isDivine = hasPerm(user, "divine");

  const bodyCopy = isDivine
    ? "You’re on Divine, a monthly subscription that keeps Duels+ evolving with exclusive features, and early access drops, and everything else Celestial has to offer."
    : isCele
      ? "You're on Celestial, a one-time unlock that permanently grants API-Keyless and other neat perks."
      : "Subscribe to a paid tier for extra features that expand what you can do with Duels+ and make your experience feel complete.";

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Get more out of Duels+</DialogTitle>
        </DialogHeader>

        <div className="space-y-6">
          <section className="space-y-3 text-sm leading-relaxed text-muted-foreground">
            <p>{bodyCopy}</p>

            <p>
              We've got many options, and you&apos;re free to stay on the base
              experience or switch tiers whenever it makes sense for you.
            </p>

            <div className="w-full flex justify-center">
              <a
                href="https://dash.duelsplus.com/?tab=subscriptions"
                target="_blank"
                rel="noopener noreferrer"
                className="w-full" /* ts pmo */
              >
                <Button variant="input" size="sm" className="w-full">
                  {isVeryCool ? "Manage subscription" : "Subscribe now"}
                  <ArrowUpRightIcon />
                </Button>
              </a>
            </div>
          </section>
        </div>
      </DialogContent>
    </Dialog>
  );
}
