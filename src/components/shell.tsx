import { Titlebar } from "./titlebar";
import { ActionRail } from "./nav/action-rail";
import { MainView } from "./main-view";
import { useEffect } from "react";
import { useUpdater } from "@/lib/updater";
import { config } from "@/lib/config";
import { getBrand, applyBrand } from "@/lib/brand-color";
import Snowfall from "react-snowfall";

export function Shell() {
  const checkAndInstall = useUpdater((s) => s.checkAndInstall);
  const isDecember = (() => {
    const today = new Date();
    return today.getMonth() === 11;
  })();
  useEffect(() => {
    const update = async () => {
      const cfg = await config.get();
      if (cfg.autoUpdate) {
        await checkAndInstall();
      }
    };

    update();
  }, [checkAndInstall]);

  useEffect(() => {
    const saved = getBrand();
    if (saved) {
      applyBrand(saved);
    }
  }, []);

  return (
    <div className="w-screen h-screen bg-background text-foreground overflow-hidden flex flex-col select-none scroll-smooth">
      {isDecember && (
        <div className="fixed top-0 z-0 w-full h-full">
          <Snowfall
            style={{ zIndex: 0 }}
            snowflakeCount={100}
            opacity={[0.5, 0.7]}
            radius={[0.5, 1.5]}
          />
        </div>
      )}
      <Titlebar />

      <div className="flex flex-1 relative overflow-hidden">
        <ActionRail />
        <MainView className="flex-1" />
      </div>
    </div>
  );
}
