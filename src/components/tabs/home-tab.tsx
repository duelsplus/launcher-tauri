import { Logo, LogoBeta } from "@/components/logo";
import { WhatsNew } from "@/components/whats-new";
import { LaunchButton } from "@/components/proxy/launch-button";
import ServiceStatus from "@/components/service-status";
import { Button } from "../ui/button";
import { SiDiscord } from "@icons-pack/react-simple-icons";
import { config as configApi } from "@/lib/config";
import { useEffect, useState } from "react";

export function Home() {
  const [isBeta, setIsBeta] = useState(false);

  useEffect(() => {
    configApi.get().then((cfg) => {
      setIsBeta(!!cfg.receiveBetaReleases);
    });
  }, []);

  return (
    <div className="relative">
      <div className="pointer-events-none fixed top-0 h-16 w-full bg-linear-to-b from-background/70 to-transparent z-20" />
      <div className="min-h-[20vh]" />

      <div className="space-y-6">
        <div className="flex items-center gap-6">
          <Logo className="h-16 w-auto text-primary dark:text-foreground classic:text-foreground" />
          {isBeta && <LogoBeta className="h-16 w-auto" />}
        </div>
        <div className="flex justify-between items-center gap-4">
          <div className="flex items-center gap-4">
            <LaunchButton isBeta={isBeta} />
            <ServiceStatus />
          </div>
          <div>
            <a
              href="https://discord.gg/YD4JZnuGYv"
              target="_blank"
              rel="noopener noreferrer"
            >
              <Button variant="muted" size="pill" className="z-10">
                <SiDiscord />
                Support server
              </Button>
            </a>
          </div>
        </div>
        <div className="space-y-3">
          <h2 className="text-base font-medium">What's new</h2>
          <WhatsNew />
        </div>
      </div>
    </div>
  );
}
