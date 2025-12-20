import { createContext, useContext, useEffect, useState } from "react";

export type TabId = "home" | "logs" | "stats" | "settings";

type TabsContextValue = {
  activeTab: TabId;
  setActiveTab: (tab: TabId) => void;
};

const TabsContext = createContext<TabsContextValue | null>(null);

export function TabsProvider({ children }: { children: React.ReactNode }) {
  const [activeTab, setActiveTab] = useState<TabId>(() => {
    const saved = localStorage.getItem("active-tab");
    return (saved as TabId) || "home";
  });

  useEffect(() => {
    localStorage.setItem("active-tab", activeTab);
  }, [activeTab]);

  return (
    <TabsContext.Provider value={{ activeTab, setActiveTab }}>
      {children}
    </TabsContext.Provider>
  );
}

export function useTabs() {
  const ctx = useContext(TabsContext);
  if (!ctx) {
    throw new Error("useTabs must be used within <TabsProvider>");
  }
  return ctx;
}
