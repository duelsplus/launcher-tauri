import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { config as configApi } from "@/lib/config";
import { defaultSettings } from "@/settings/defaults";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { cn } from "@/lib/utils";

type RpcImageItem = {
  key: string;
  label: string;
  image: string;
};

const FILL_IMAGES: RpcImageItem[] = [
  { key: "logo-v1", label: "Classic", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1391867456661880884.png" },
  { key: "logo-v1-purple", label: "Purple", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1404368523651842179.png" },
  { key: "logo-emerald", label: "Emerald", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143122220253297.png" },
  { key: "logo-golden-mark", label: "Golden", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143122627104778.png" },
  { key: "logo-green", label: "Green", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143121742106824.png" },
  { key: "logo-shiny", label: "Shiny", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143121972789382.png" },
  { key: "logo-blue", label: "Blue", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1461068864094998623.png" },
  { key: "logo-white", label: "White", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1461068864048726153.png" },
  { key: "logo-gray", label: "Gray", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1461068175725564048.png" },
];

const PLUS_IMAGES: RpcImageItem[] = [
  { key: "logo-emerald-plus", label: "Emerald", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143123096600698.png" },
  { key: "logo-golden-plus", label: "Golden", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143123474354400.png" },
  { key: "logo-blue-plus", label: "Blue", image: "https://cdn.discordapp.com/app-assets/1391866803889770526/1461068864409309284.png" },
];

interface RpcCustomizeDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function RpcCustomizeDialog({
  open,
  onOpenChange,
}: RpcCustomizeDialogProps) {
  const [activeKey, setActiveKey] = useState<string | null>(null);

  useEffect(() => {
    if (!open) return;
    configApi
      .get()
      .then((cfg) => {
        setActiveKey(cfg.rpcImage);
      })
      .catch(() => {
        setActiveKey(defaultSettings.rpcImage);
      });
  }, [open]);

  const selectImage = async (item: RpcImageItem) => {
    setActiveKey(item.key);

    try {
      await invoke("rpc_set_image", {
        imageKey: item.key,
      });
    } catch {
      // optional: rollback or toast
    }
  };

  const renderGrid = (items: RpcImageItem[]) => (
    <div className="grid grid-cols-4 gap-2.5">
      {items.map((item) => {
        const active = item.key === activeKey;

        return (
          <button
            key={item.key}
            onClick={() => selectImage(item)}
            className={cn(
              "group relative aspect-square rounded-xl size-27 overflow-hidden bg-muted",
              "hover:ring-2 hover:ring-foreground/10 text-white/70 transition-all focus:outline-none",
              active && "ring-3 hover:ring-3 ring-foreground/20 text-white hover:ring-foreground/20",
            )}
          >
            <img
              src={item.image}
              alt=""
              draggable={false}
              className="h-full w-full object-cover select-none pointer-events-none"
            />

            <div
              className={cn(
                "absolute inset-0 flex items-end justify-center",
                "bg-gradient-to-t from-black/60 via-black/0 to-black/0",
              )}
            >
              <span className="text-sm group-hover:text-white transition font-medium pb-1">{item.label}</span>
            </div>
          </button>
        );
      })}
    </div>
  );

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Customize Rich Presence</DialogTitle>
        </DialogHeader>

        <Tabs defaultValue="fill">
          <TabsList>
            <TabsTrigger value="fill">Fill</TabsTrigger>
            <TabsTrigger value="plus">Plus Only</TabsTrigger>
          </TabsList>

          <TabsContent value="fill">
            {renderGrid(FILL_IMAGES)}
          </TabsContent>

          <TabsContent value="plus">
            {renderGrid(PLUS_IMAGES)}
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  );
}
