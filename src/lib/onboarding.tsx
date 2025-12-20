import { createContext, useContext, useEffect, useState } from "react";

type OnboardingContextValue = {
  isOnboarded: boolean;
  setOnboarded: (value: boolean) => void;
};

const OnboardingContext = createContext<OnboardingContextValue | null>(null);
const storageKey = "isOnboarded";

export function OnboardingProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [isOnboarded, setIsOnboarded] = useState<boolean>(() => {
    return localStorage.getItem(storageKey) === "true";
  });

  useEffect(() => {
    localStorage.setItem(storageKey, String(isOnboarded));
  }, [isOnboarded]);

  return (
    <OnboardingContext.Provider
      value={{
        isOnboarded,
        setOnboarded: setIsOnboarded,
      }}
    >
      {children}
    </OnboardingContext.Provider>
  );
}

export function useOnboarding() {
  const ctx = useContext(OnboardingContext);
  if (!ctx) {
    throw new Error("useOnboarding must be used within <OnboardingProvider>");
  }
  return ctx;
}
