import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "../ui/button";
import { cn } from "@/lib/utils";
import { useEffect, useState } from "react";

interface EnableBetaDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onContinue?: () => void | Promise<void>;
}

export function EnableBetaDialog({
  open,
  onOpenChange,
  onContinue,
}: EnableBetaDialogProps) {
  const [countdown, setCountdown] = useState(15);
  const [continueEnabled, setContinueEnabled] = useState(false);

  useEffect(() => {
    // is this overengineered guys..
    if (!open) return;
    setCountdown(15);
    setContinueEnabled(false);
    const interval = setInterval(() => {
      setCountdown((prev) => {
        if (prev <= 1) {
          clearInterval(interval);
          setContinueEnabled(true);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
    return () => clearInterval(interval);
  }, [open]);

  const handleContinue = async () => {
    onOpenChange(false);
    if (onContinue) {
      await onContinue();
    }
  };

  const handleClose = () => {
    onOpenChange(false);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Enable Beta Releases</DialogTitle>
        </DialogHeader>

        <section className="space-y-3 text-sm leading-relaxed text-muted-foreground">
          <p>
            Beta releases are pre-release builds of the Duels+ proxy and have
            features that are unfinished, unstable, or subject to change.
          </p>
          <p>
            Expect bugs, crashes, and <strong>BREAKING CHANGES</strong>!! Some
            features might not work as intended but that's the fun part of being
            a tester.
          </p>
          <p>
            Continue at your own risk. We ask that you share feedback and bug
            reports. Stability isn't guaranteed.
          </p>

          <div>
            <Button
              size="sm"
              variant="rose"
              className="pointer-events-none rounded-b-none"
              ripple={false}
            >
              Terms
            </Button>
            <div className="p-3 rounded-xl rounded-tl-none bg-rose-200/50 text-rose-600 dark:bg-rose-950/50 dark:text-rose-400 classic:bg-rose-950/50 classic:text-rose-400">
              <ul className="list-disc list-inside space-y-1 marker:text-current/30 text-sm">
                <li>Avoid exposure of beta builds in public environments.</li>
                <li>Don't stream, record, or share your gameplay.</li>
                <li>
                  Play on beta builds using alt accounts (if possible) so you
                  don't risk your main account.
                </li>
              </ul>
            </div>
            {/* i replicated the rose button variant */}
          </div>
        </section>

        <DialogFooter>
          <Button variant="outline" onClick={handleClose}>
            Nevermind
          </Button>
          <Button
            variant="input"
            onClick={handleContinue}
            disabled={!continueEnabled}
          >
            {continueEnabled ? "Continue" : `Continue (${countdown}s)`}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
