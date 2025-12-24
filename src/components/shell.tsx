import { Titlebar } from "./titlebar";
import { ActionRail } from "./nav/action-rail";
import { MainView } from "./main-view";
import { useEffect } from "react";
import { useUpdater } from "@/lib/updater";
import { config } from "@/lib/config";

export function Shell() {
  const checkAndInstall = useUpdater((s) => s.checkAndInstall);
  useEffect(() => {
    const update = async () => {
      const cfg = await config.get();
      if (cfg.autoUpdate) {
        await checkAndInstall();
      }
    };

    update();
  }, [checkAndInstall]);

  return (
    <div className="w-screen h-screen bg-background text-foreground overflow-hidden flex flex-col select-none scroll-smooth">
      <Titlebar />

      <div className="flex flex-1 relative overflow-hidden">
        <ActionRail />
        <MainView className="flex-1" />
      </div>
    </div>
  );
}
