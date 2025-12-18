import { MoonIcon, SunIcon } from "@phosphor-icons/react"
import { Button } from "@/components/ui/button";
import { useTheme } from "@/components/theme-provider";

export function ThemeSwitcher() {
  const { theme, setTheme } = useTheme();

  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark");
  };

  return (
    <Button
      variant="muted"
      size="icon-lg"
      className="relative rounded-[32px] hover:rounded-3xl p-5.5 [&_svg:not([class*='size-'])]:size-6"
      onClick={toggleTheme}
    >
      <SunIcon className="rotate-0 scale-100 transition-all dark:-rotate-95 dark:scale-0" />
      <MoonIcon className="absolute rotate-95 scale-0 transition-all dark:rotate-0 dark:scale-100" />
    </Button>
  );
}
