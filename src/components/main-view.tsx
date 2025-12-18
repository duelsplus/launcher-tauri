import { PlayCircleIcon } from "@phosphor-icons/react";
import { Logo } from "./logo";
import { Button } from "./ui/button";

export function MainView({ className }: { className?: string }) {
  return (
    <div className={`ml-24 px-8 h-full overflow-auto relative ${className ?? ""}`}>
      <div className="pointer-events-none sticky top-0 h-16 w-full bg-linear-to-b from-background/70 to-transparent z-20" />
      <div className="min-h-[20vh]" />

      <div className="space-y-5">
        <Logo className="h-16 w-auto" />
        <Button className="gap-2! [&_svg:not([class*='size-'])]:size-7" variant="input" size="pill-lg">
          <PlayCircleIcon weight="fill" />
          Launch
        </Button>

        <div className="space-y-4">
          {Array.from({ length: 30 }).map((_, i) => (
            <div key={i} className="h-20 bg-muted" />
          ))}
        </div>
      </div>
    </div>
  );
}
