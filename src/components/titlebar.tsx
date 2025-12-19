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
        <Button className="rounded-none!" onClick={minimize} variant="ghost">
          <MinusIcon />
        </Button>
        <Button className="rounded-none!" onClick={close} variant="ghost">
          <XIcon />
        </Button>
      </div>
    </header>
  );
}
