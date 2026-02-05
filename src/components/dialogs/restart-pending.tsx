import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "../ui/button";
import { cn } from "@/lib/utils";
import { invoke } from "@tauri-apps/api/core";
import { relaunch } from "@tauri-apps/plugin-process";
import { useEffect, useState } from "react";
import { SpinnerIcon } from "@phosphor-icons/react";

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
  const [loading, setLoading] = useState(false);
  const [countdown, setCountdown] = useState(5);
  const [notNowEnabled, setNotNowEnabled] = useState(false);

  const canClose = notNowEnabled && !loading;

  useEffect(() => {
    // is this overengineered guys..
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
    setLoading(true);
    const running = await invoke<boolean>("get_proxy_status");
    if (running) {
      await invoke("stop_proxy");
    }
    await relaunch();
  };

  const handleNotNow = () => {
    onOpenChange(false);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        onEscapeKeyDown={canClose ? undefined : (e) => e.preventDefault()} // shadcn should add these as a native feature like they did with `showCloseButton`
        onInteractOutside={
          canClose ? undefined : (e) => e.preventDefault()
        }
        showCloseButton={canClose}
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
            disabled={!notNowEnabled || loading}
          >
            {notNowEnabled ? "Not Now" : `Not Now (${countdown}s)`}
          </Button>
          <Button variant="input" disabled={loading} onClick={handleRestart}>
            {loading && <SpinnerIcon className="animate-spin" />}
            Restart Now
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
