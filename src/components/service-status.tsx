import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { WarningIcon } from "@phosphor-icons/react";
import { Tooltip, TooltipContent, TooltipTrigger } from "./ui/tooltip";

export default function ServiceStatus() {
  const [healthy, setHealthy] = useState(true);

  const check = () => {
    invoke<boolean>("check_api_status")
      .then((healthy) => {
        setHealthy(healthy ? true : false);
      })
      .catch(() => setHealthy(false));
  };

  useEffect(() => {
    check();
    const interval = setInterval(check, 5 * 60 * 1000); // every 5min
    return () => clearInterval(interval);
  }, []);

  if (healthy) return null;

  return (
    <div className="py-2">
      <Tooltip>
        <TooltipTrigger className="flex items-center">
          <WarningIcon weight="fill" className="size-5 fill-amber-400" />
        </TooltipTrigger>
        <TooltipContent>
          <p>Some services are currently unavailable.</p>
        </TooltipContent>
      </Tooltip>
    </div>
  );
}
