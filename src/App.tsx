import { ThemeProvider } from "@/components/theme-provider";
import { Shell } from "@/components/shell";
import { OnboardingProvider, useOnboarding } from "@/lib/onboarding";
import { Onboarding } from "@/components/onboarding";
import { TabsProvider } from "@/lib/tabs";
import { useEffect } from "react";
import { showWindow } from "@/lib/window";

function Content() {
  const { isOnboarded, setOnboarded } = useOnboarding();

  useEffect(() => {
    showWindow();
  }, []);

  if (!isOnboarded) {
    return <Onboarding open onFinish={() => setOnboarded(true)} />;
  }

  return <Shell />;
}

function App() {
  return (
    <OnboardingProvider>
      <TabsProvider>
        <ThemeProvider>
          <Content />
        </ThemeProvider>
      </TabsProvider>
    </OnboardingProvider>
  );
}

export default App;
