import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { WarningIcon } from "@phosphor-icons/react";

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
    <div className="w-full py-2">
      <div className="flex items-center gap-2">
        <WarningIcon weight="fill" className="size-5 fill-amber-400" />
        <p className="text-sm text-muted-foreground">
          Some services are currently unavailable
        </p>
      </div>
    </div>
  );
}
