import { Logo } from "@/components/logo";
import { WhatsNew } from "@/components/whats-new";
import { LaunchButton } from "@/components/proxy/launch-button";
import ServiceStatus from "@/components/service-status";
import { Button } from "../ui/button";
import { SiDiscord } from "@icons-pack/react-simple-icons";

export function Home() {
  return (
    <div className="relative">
      <div className="pointer-events-none fixed top-0 h-16 w-full bg-linear-to-b from-background/70 to-transparent z-20" />
      <div className="min-h-[20vh]" />

      <div className="space-y-6">
        <Logo className="h-16 w-auto text-primary dark:text-foreground classic:text-foreground" />
        <div className="flex justify-between items-center gap-4">
          <div className="flex items-center gap-4">
            <LaunchButton />
            <ServiceStatus />
          </div>
          <div>
            <Button
              variant="muted"
              size="pill"
              className="z-10"
            >
              <SiDiscord />
              Support server
            </Button>
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
