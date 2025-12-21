import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  SpinnerIcon,
  CloudArrowDownIcon,
  HeartbeatIcon,
  PlayCircleIcon,
  StopCircleIcon,
  HeartBreakIcon,
} from "@phosphor-icons/react";
import { Button } from "@/components/ui/button";
import { motion, AnimatePresence } from "framer-motion";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";

type ProxyStatusEvent =
  | { status: "checking" }
  | { status: "downloading"; version: string }
  | { status: "launching" }
  | { status: "launched" }
  | { status: "error" };

type DownloadProgress = {
  downloaded: number;
  total: number;
  speed: number;
};

type ProxyState =
  | "unknown"
  | "running"
  | "checking"
  | "downloading"
  | "error"
  | "stopping"
  | "stopped";

export function LaunchButton() {
  const [state, setState] = useState<ProxyState>("unknown");
  const [busy, setBusy] = useState(false);
  const [hovered, setHovered] = useState(false);
  const [statusText, setStatusText] = useState<string | null>(null);
  const [progress, setProgress] = useState<DownloadProgress | null>(null);

  useEffect(() => {
    invoke<boolean>("get_proxy_status")
      .then((running) => {
        setState(running ? "running" : "stopped");
      })
      .catch(() => setState("stopped"));
  }, []);

  useEffect(() => {
    const unlistenStatus = listen<ProxyStatusEvent>(
      "updater:status",
      (event) => {
        const status = event.payload.status;
        //setStatusText(status);

        if (status === "checking") {
          setState("checking");
          setProgress(null);
        }

        if (status === "downloading") {
          setState("downloading");
          setStatusText("Downloading");
          setProgress(null);
        }

        if (status === "launched") {
          setState("running");
          setStatusText("Launched");
          setBusy(false);
          setProgress(null);
        }

        if (status === "error") {
          setState("error");
          setStatusText("Error");
          setBusy(false);
        }
      },
    );

    const unlistenProgress = listen<DownloadProgress>(
      "updater:progress",
      (event) => {
        setProgress(event.payload);
      },
    );

    return () => {
      unlistenStatus.then((u) => u());
      unlistenProgress.then((u) => u());
    };
  }, []);

  async function handle() {
    if (busy) return;
    setBusy(true);
    setStatusText(null);

    try {
      const running = await invoke<boolean>("get_proxy_status");
      if (!running) {
        setState("checking");
        await invoke("launch_proxy");
      } else {
        setState("stopping");
        await invoke("stop_proxy");
        setState("stopped");
        setBusy(false);
        setStatusText(null);
      }
    } catch (err) {
      console.error(err);
      setBusy(false);
      setState("error");
      setStatusText("Error");
    }
  }

  const isDisabled = state === "unknown" || busy;
  const isRunning = state === "running";

  const speed = progress
    ? progress.speed > 1_000_000
      ? `${(progress.speed / 1_000_000).toFixed(1)} MB/s`
      : `${(progress.speed / 1_000).toFixed(1)} KB/s`
    : null;

  const percent = progress
    ? Math.round((progress.downloaded / progress.total) * 100).toString()
    : null;

  return (
    <div className="relative w-fit">
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="inline-flex">
            <Button
              onClick={handle}
              disabled={isDisabled}
              variant={
                (isRunning || state === "error") && !hovered
                  ? "default"
                  : "input"
              }
              size="pill-lg"
              className="z-10 gap-2 [&_svg:not([class*='size-'])]:size-7 relative overflow-hidden"
              onMouseEnter={() => setHovered(true)}
              onMouseLeave={() => setHovered(false)}
            >
              {state === "downloading" ? (
                <>
                  <CloudArrowDownIcon weight="fill" />
                  <div className="flex flex-col text-left">
                    Downloading...
                    {speed && (
                      <p className="text-xs text-muted-foreground -mt-0.5">
                        {speed}
                      </p>
                    )}
                  </div>
                  {percent && (
                    <div
                      className="absolute inset-0 bg-primary/30 -z-10 transition-all duration-300"
                      style={{ width: `${percent}%`, left: 0 }}
                    />
                  )}
                </>
              ) : state === "checking" ? (
                <>
                  <SpinnerIcon weight="regular" className="animate-spin" />
                  Preparing...
                </>
              ) : state === "stopping" ? (
                <>
                  <SpinnerIcon weight="regular" className="animate-spin" />
                  Stopping...
                </>
              ) : state === "error" ? (
                <>
                  <>
                    <motion.div
                      key={hovered ? "stop" : "error"}
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      exit={{ opacity: 0 }}
                      transition={{ duration: 0.15 }}
                      className="flex items-center gap-2"
                    >
                      {hovered ? (
                        <StopCircleIcon weight="fill" />
                      ) : (
                        <HeartBreakIcon weight="fill" />
                      )}
                      {hovered ? "Stop" : "Error"}
                    </motion.div>
                  </>
                </>
              ) : isRunning ? (
                <>
                  <>
                    <motion.div
                      key={hovered ? "stop" : "running"}
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      exit={{ opacity: 0 }}
                      transition={{ duration: 0.15 }}
                      className="flex items-center gap-2"
                    >
                      {hovered ? (
                        <StopCircleIcon weight="fill" />
                      ) : (
                        <HeartbeatIcon weight="fill" />
                      )}
                      <div className="flex flex-col text-left">
                        {hovered ? "Stop" : "Running"}
                        {statusText && !hovered && (
                          <p className="text-xs text-white/70 -mt-0.5">
                            {statusText}
                          </p>
                        )}
                      </div>
                    </motion.div>
                  </>
                </>
              ) : (
                <>
                  <PlayCircleIcon weight="fill" />
                  Launch
                </>
              )}
            </Button>
          </span>
        </TooltipTrigger>
        {state === "downloading" && (
          <TooltipContent side="right">
            <p>The proxy cannot be stopped while downloading.</p>
          </TooltipContent>
        )}
        <AnimatePresence>
          {progress && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 1 }}
            >
              <div className="absolute z-0 -inset-1 border-2 border-primary shadow-[0_0_8px_oklch(0.6822_0.2063_24.4310)] animate-[pulse_3s_cubic-bezier(0.4,0,0.6,1)_infinite] rounded-full pointer-events-none" />
            </motion.div>
          )}
        </AnimatePresence>
      </Tooltip>
    </div>
  );
}
