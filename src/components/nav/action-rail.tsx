import { ReactNode } from "react";
import { Button } from "@/components/ui/button";
import {
  HouseIcon,
  ListHeartIcon,
  ChartLineIcon,
  GearFineIcon,
} from "@phosphor-icons/react";
import clsx from "clsx";
import { ThemeSwitcher } from "./theme-switcher";

interface ActionButtonProps {
  icon: "home" | "logs" | "stats" | "settings";
  active?: boolean;
  onClick?: () => void;
}

function ActionButton({ icon, active, onClick }: ActionButtonProps) {
  const iconsMap = {
    home: <HouseIcon />,
    logs: <ListHeartIcon />,
    stats: <ChartLineIcon />,
    settings: <GearFineIcon />,
  };

  return (
    <Button
      variant="muted"
      size="icon-lg"
      className={clsx(
        "rounded-[32px] hover:rounded-3xl p-5.5 [&_svg:not([class*='size-'])]:size-6",
        active && "rounded-3xl bg-popover text-popover-foreground hover:bg-popover/70",
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

export function ActionRail() {
  return (
    <nav className="flex flex-col justify-center items-center h-full bg-muted/50 gap-3 p-3">
      <ActionCategory>
        <ActionButton icon="home" active />
        <ActionButton icon="logs" onClick={() => console.log("Open Logs")} />
        <ActionButton icon="stats" onClick={() => console.log("Open Stats")} />
        <ActionButton
          icon="settings"
          onClick={() => console.log("Open Settings")}
        />
      </ActionCategory>
      <ActionCategory>
        <ThemeSwitcher />
      </ActionCategory>
    </nav>
  );
}
