import { Titlebar } from "./titlebar"
import { ActionRail } from "./action-rail";
import { MainView } from "./main-view";

export function Shell() {
  return (
    <div className="w-screen h-screen bg-background text-foreground relative overflow-hidden flex flex-col">
      <Titlebar />

      <div className="flex items-center flex-1">
        <ActionRail />
        <MainView className="flex-1" />
      </div>
    </div>
  );
}
