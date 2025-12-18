import { Button } from "@/components/ui/button";
import { XIcon, MinusIcon } from "@phosphor-icons/react";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function Titlebar() {
  const minimize = async () => {
    await getCurrentWindow().minimize();
  };

  const close = async () => {
    await getCurrentWindow().close();
  };

  return (
    <header
      data-tauri-drag-region
      className="w-full flex items-center justify-end px-2"
    >
      <div className="flex items-center">
        <Button onClick={minimize} variant="ghost" size="sm">
          <MinusIcon />
        </Button>
        <Button onClick={close} variant="ghost" size="sm">
          <XIcon />
        </Button>
      </div>
    </header>
  );
}
