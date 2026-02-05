import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "../ui/button";
import { cn } from "@/lib/utils";
import { relaunch } from "@tauri-apps/plugin-process";
import { useEffect, useState } from "react";

interface RestartPendingDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  name?: string;
}

export function RestartPendingDialog({
  open,
  onOpenChange,
  name,
}: RestartPendingDialogProps) {
  const [countdown, setCountdown] = useState(5);
  const [notNowEnabled, setNotNowEnabled] = useState(false);

  useEffect(() => { // is this overengineered guys..
    if (!open) return;
    setCountdown(5);
    setNotNowEnabled(false);
    const interval = setInterval(() => {
      setCountdown((prev) => {
        if (prev <= 1) {
          clearInterval(interval);
          setNotNowEnabled(true);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
    return () => clearInterval(interval);
  }, [open]);

  const handleRestart = async () => {
    await relaunch();
  };

  const handleNotNow = () => {
    onOpenChange(false);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        onEscapeKeyDown={notNowEnabled ? undefined : (e) => e.preventDefault()} // shadcn should add these as a native feature like they did with `showCloseButton`
        onInteractOutside={
          notNowEnabled ? undefined : (e) => e.preventDefault()
        }
        showCloseButton={notNowEnabled}
      >
        <DialogHeader>
          <DialogTitle>Restart Required</DialogTitle>
        </DialogHeader>

        <p className="text-sm leading-relaxed text-muted-foreground">
          You need to restart the launcher for{" "}
          {name ? <strong>{name}</strong> : "changes"} to take effect.
        </p>

        <DialogFooter>
          <Button
            variant="outline"
            onClick={handleNotNow}
            disabled={!notNowEnabled}
          >
            {notNowEnabled ? "Not Now" : `Not Now (${countdown}s)`}
          </Button>
          <Button variant="input" onClick={handleRestart}>
            Restart Now
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
