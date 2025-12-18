import { ThemeProvider } from "@/components/theme-provider";
import { Shell } from "@/components/shell";

function App() {
  return (
    <ThemeProvider>
      <Shell />
    </ThemeProvider>
  );
}

export default App;
