import { Logo } from "@/components/logo";
import { WhatsNew } from "@/components/whats-new";
import { LaunchButton } from "@/components/proxy/launch-button";

export function Home() {
  return (
    <div className="relative">
      <div className="pointer-events-none sticky top-0 h-16 w-full bg-linear-to-b from-background/70 to-transparent z-20" />
      <div className="min-h-[20vh]" />

      <div className="space-y-6">
        <Logo className="h-16 w-auto text-primary dark:text-foreground" />
        <LaunchButton />

        <div className="space-y-3">
          <h2 className="text-base font-medium">What's new</h2>
          <WhatsNew />
        </div>
      </div>
    </div>
  );
}
