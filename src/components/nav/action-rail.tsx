import { ReactNode } from "react";
import { Button } from "@/components/ui/button";
import {
  HouseIcon,
  ListHeartIcon,
  ChartLineIcon,
  GearFineIcon,
  SpinnerIcon,
  DownloadSimpleIcon,
  ArrowsClockwiseIcon,
  WarningIcon,
} from "@phosphor-icons/react";
import type { IconWeight } from "@phosphor-icons/react";
import clsx from "clsx";
import { ThemeSwitcher } from "./theme-switcher";
import { useTabs } from "@/lib/tabs";
import { useUpdater } from "@/lib/updater";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Stats } from "../tabs/stats-tab";
import { Settings } from "../tabs/settings-tab";
import { Logs } from "../tabs/logs-tab";

interface ActionButtonProps {
  icon: "home" | "logs" | "stats" | "settings";
  active?: boolean;
  onClick?: () => void;
}

function ActionButton({ icon, active, onClick }: ActionButtonProps) {
  const iconWeight: IconWeight = active ? "fill" : "regular";
  const iconsMap = {
    home: <HouseIcon weight={iconWeight} />,
    logs: <ListHeartIcon weight={iconWeight} />,
    stats: <ChartLineIcon weight={iconWeight} />,
    settings: <GearFineIcon weight={iconWeight} />,
  };

  return (
    <Button
      variant="muted"
      size="icon-lg"
      className={clsx(
        "rounded-[32px] hover:rounded-3xl p-5.5 [&_svg:not([class*='size-'])]:size-6",
        active &&
          "rounded-3xl bg-popover text-popover-foreground hover:bg-popover/70",
      )}
      onClick={onClick}
    >
      {iconsMap[icon]}
    </Button>
  );
}

type ActionCategoryProps = {
  children: ReactNode;
};

function ActionCategory({ children }: ActionCategoryProps) {
  return (
    <div className="flex flex-col items-center gap-1.5 h-fit bg-muted rounded-nav p-2">
      {children}
    </div>
  );
}

function UpdateButton() {
  const status = useUpdater((s) => s.status);
  const icon = (() => {
    switch (status.state) {
      case "downloading":
        return <DownloadSimpleIcon weight="fill" />;
      case "pending-restart":
        return <ArrowsClockwiseIcon weight="fill" />;
      case "error":
        return <WarningIcon weight="fill" />;
      default:
        return <SpinnerIcon className="animate-spin" />;
    }
  })();

  const tooltipText = (() => {
    switch (status.state) {
      case "checking":
        return "Checking for updates...";
      case "downloading":
        return "Downloading update...";
      /*return `Downloading update... (+${Math.round(
          status.chunkLength / 1024,
        )} KB)`;*/
      case "pending-restart":
        return "Restart the launcher to install update.";
      case "error":
        if (import.meta.env.DEV)
          return "Update failed. This is normal in dev environment.";
        return "Update failed.";
      default:
        return "";
    }
  })();

  if (
    status.state === "downloading" ||
    status.state === "pending-restart" ||
    status.state === "error"
  ) {
    return (
      <ActionCategory>
        <Tooltip delayDuration={0}>
          <TooltipTrigger asChild>
            <Button
              variant="warning"
              size="icon-lg"
              className="rounded-[32px] hover:rounded-3xl p-5.5 [&_svg:not([class*='size-'])]:size-6"
            >
              {icon}
            </Button>
          </TooltipTrigger>
          <TooltipContent side="right">{tooltipText}</TooltipContent>
        </Tooltip>
      </ActionCategory>
    );
  }

  return null;
}

export function ActionRail() {
  const { activeTab, toggleTab } = useTabs();
  return (
    <>
      <nav className="fixed top-0 bottom-0 left-0 w-24 z-10 flex flex-col justify-center items-center bg-semimuted gap-3">
        <ActionCategory>
          <ActionButton
            icon="home"
            active={activeTab === "home"}
            onClick={() => toggleTab("home")}
          />
          <ActionButton
            icon="logs"
            active={activeTab === "logs"}
            onClick={() => toggleTab("logs")}
          />
          <ActionButton
            icon="stats"
            active={activeTab === "stats"}
            onClick={() => toggleTab("stats")}
          />
          <ActionButton
            icon="settings"
            active={activeTab === "settings"}
            onClick={() => toggleTab("settings")}
          />
        </ActionCategory>
        <UpdateButton />
      </nav>

      <Drawer />
    </>
  );
}

function Drawer() {
  const { activeTab } = useTabs();
  const isOpen = activeTab !== "home";

  return (
    <div
      className={clsx(
        "fixed top-0 left-24 h-full w-80 bg-semimuted z-0 transition-transform duration-300 flex flex-col",
        isOpen ? "translate-x-0" : "-translate-x-full",
      )}
    >
      <div className="p-6 pl-0">
        {activeTab === "logs" && <Logs />}
        {activeTab === "stats" && <Stats />}
        {activeTab === "settings" && <Settings />}
      </div>
    </div>
  );
}
