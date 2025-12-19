import clsx from "clsx";
import { useTabs } from "@/lib/tabs";
import { ReactNode } from "react";
import { Home } from "@/components/tabs/home-tab";

function TabPanel({
  active,
  children,
}: {
  active: boolean;
  children: ReactNode;
}) {
  return (
    <div
      className={clsx(
        "transition-opacity duration-200",
        active
          ? "opacity-100 pointer-events-auto"
          : "opacity-0 pointer-events-none",
      )}
    >
      {children}
    </div>
  );
}

export function MainView({ className }: { className?: string }) {
  const { activeTab } = useTabs();

  return (
    <div className={`ml-24 pb-12 px-8 h-full overflow-auto ${className ?? ""}`}>
      <TabPanel active={activeTab === "home"}>
        <Home />
      </TabPanel>
    </div>
  );
}
