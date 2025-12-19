import { ThemeProvider } from "@/components/theme-provider";
import { Shell } from "@/components/shell";
import { TabsProvider } from "@/lib/tabs";

function App() {
  return (
    <TabsProvider>
      <ThemeProvider>
        <Shell />
      </ThemeProvider>
    </TabsProvider>
  );
}

export default App;
