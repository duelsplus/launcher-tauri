import { Titlebar } from "./titlebar"
import { ActionRail } from "./nav/action-rail";
import { MainView } from "./main-view";

export function Shell() {
  return (
    <div className="w-screen h-screen bg-background text-foreground overflow-hidden flex flex-col">
      <Titlebar />

      <div className="flex flex-1 relative overflow-hidden">
        <ActionRail />
        <MainView className="flex-1" />
      </div>
    </div>
  );
}
