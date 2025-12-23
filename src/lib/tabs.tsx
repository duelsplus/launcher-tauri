import { createContext, useContext, useEffect, useState } from "react";

export type TabId = "home" | "logs" | "stats" | "settings";

type TabsContextValue = {
  activeTab: TabId;
  isDrawerOpen: boolean;
  setActiveTab: (tab: TabId) => void;
  toggleTab: (tab: TabId) => void;
};

const TabsContext = createContext<TabsContextValue | null>(null);

export function TabsProvider({ children }: { children: React.ReactNode }) {
  const [activeTab, setActiveTab] = useState<TabId>(() => {
    const saved = localStorage.getItem("active-tab");
    return (saved as TabId) || "home";
  });

  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(
    activeTab !== "home",
  );

  useEffect(() => {
    localStorage.setItem("active-tab", activeTab);
    setIsDrawerOpen(activeTab !== "home");
  }, [activeTab]);

  const toggleTab = (tab: TabId) => {
    if (tab === "home") {
      setActiveTab("home");
    } else if (activeTab === tab) {
      setActiveTab("home");
    } else {
      setActiveTab(tab);
    }
  };

  return (
    <TabsContext.Provider
      value={{ activeTab, isDrawerOpen, setActiveTab, toggleTab }}
    >
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
