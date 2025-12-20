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
        "w-full h-full",
        active
          ? "inline-block"
          : "hidden",
      )}
    >
      {children}
    </div>
  );
}

export function MainView({ className }: { className?: string }) {
  const { activeTab } = useTabs();

  return (
    <div className={`ml-24 py-12 px-8 h-full overflow-auto ${className ?? ""}`}>
      <TabPanel active={activeTab === "home"}>
        <Home />
      </TabPanel>
    </div>
  );
}
