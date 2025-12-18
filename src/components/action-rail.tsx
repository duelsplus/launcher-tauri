import { Button } from "@/components/ui/button";
import {
  HouseIcon,
  ListHeartIcon,
  ChartLineIcon,
  GearFineIcon,
} from "@phosphor-icons/react";
import clsx from "clsx";

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
      variant={active ? "default" : "ghost"}
      size="icon-lg"
      className={clsx(
        "rounded-full p-4 [&_svg:not([class*='size-'])]:size-4.5",
        active && "bg-white text-black hover:bg-white/80",
      )}
      onClick={onClick}
    >
      {iconsMap[icon]}
    </Button>
  );
}

export function ActionRail() {
  return (
    <nav className="flex flex-col items-center h-fit bg-muted rounded-full p-1.5 m-2">
      <ActionButton icon="home" active />
      <ActionButton icon="logs" onClick={() => console.log("Open Logs")} />
      <ActionButton icon="stats" onClick={() => console.log("Open Stats")} />
      <ActionButton
        icon="settings"
        onClick={() => console.log("Open Settings")}
      />
    </nav>
  );
}
