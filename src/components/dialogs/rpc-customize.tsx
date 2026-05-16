import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { config as configApi } from "@/lib/config";
import { User, hasPerm } from "@/lib/perm";
import { getToken } from "@/lib/token";
import { defaultSettings } from "@/settings/defaults";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { cn } from "@/lib/utils";
import { LockSimpleIcon } from "@phosphor-icons/react";

type ApiResponse<T> = {
  success: boolean;
  data: T;
};

type RpcImageItem = {
  key: string;
  label: string;
  image: string;
};

const FILL_IMAGES: RpcImageItem[] = [
  {
    key: "logo-v1",
    label: "Classic",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1391867456661880884.png",
  },
  {
    key: "logo-v1-purple",
    label: "Purple",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1404368523651842179.png",
  },
  {
    key: "logo-emerald",
    label: "Emerald",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143122220253297.png",
  },
  {
    key: "logo-golden-mark",
    label: "Golden",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143122627104778.png",
  },
  {
    key: "logo-green",
    label: "Green",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143121742106824.png",
  },
  {
    key: "logo-blue",
    label: "Blue",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1461068864094998623.png",
  },
  {
    key: "logo-white",
    label: "White",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1461068864048726153.png",
  },
  {
    key: "logo-gray",
    label: "Gray",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1461068175725564048.png",
  },
];

const PLUS_IMAGES: RpcImageItem[] = [
  {
    key: "logo-shiny",
    label: "Shiny",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143121972789382.png",
  },
  {
    key: "logo-emerald-plus",
    label: "Emerald +",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143123096600698.png",
  },
  {
    key: "logo-golden-plus",
    label: "Golden +",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1454143123474354400.png",
  },
  {
    key: "logo-blue-plus",
    label: "Blue +",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1461068864409309284.png",
  },
];

const SP_IMAGES: RpcImageItem[] = [
  {
    key: "nerd",
    label: "Nerd",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1505142506503798824.png",
  },
  {
    key: "i-paid-for-this-btw",
    label: "paid for this",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1505141712408805386.png",
  },
  {
    key: "i-own-u",
    label: "i own u",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1505141712509468802.png",
  },
  {
    key: "esex",
    label: "esex?",
    image:
      "https://cdn.discordapp.com/app-assets/1391866803889770526/1505142195936563281.png",
  },
];

interface RpcCustomizeDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const getTab = (imageKey: string): string => {
  //for making sure it opens on the tab of active image
  if (FILL_IMAGES.some((img) => img.key === imageKey)) {
    return "fill";
  }
  if (PLUS_IMAGES.some((img) => img.key === imageKey)) {
    return "plus";
  }
  if (SP_IMAGES.some((img) => img.key === imageKey)) {
    return "supporterplus";
  }
  return "fill"; // fallback??
};

export function RpcCustomizeDialog({
  open,
  onOpenChange,
}: RpcCustomizeDialogProps) {
  const [activeKey, setActiveKey] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<string>("fill");
  const [user, setUser] = useState<User | null>(null);

  const isVeryCool =
    hasPerm(user, "supporter") ||
    hasPerm(user, "supporterplus") ||
    hasPerm(user, "developer") ||
    hasPerm(user, "admin");

  const isVeryVeryCool =
    hasPerm(user, "supporterplus") ||
    hasPerm(user, "developer") ||
    hasPerm(user, "admin");

  useEffect(() => {
    invoke<ApiResponse<User>>("get_user", {
      token: getToken(),
    })
      .then((u) => setUser(u.data))
      .catch(() => setUser(null));
  }, []);

  useEffect(() => {
    if (!open) return;
    configApi
      .get()
      .then((cfg) => {
        setActiveKey(cfg.rpcImage);
        setActiveTab(getTab(cfg.rpcImage));
      })
      .catch(() => {
        setActiveKey(defaultSettings.rpcImage);
        setActiveTab(getTab(defaultSettings.rpcImage));
      });
  }, [open]);

  const selectImage = async (item: RpcImageItem) => {
    setActiveKey(item.key);
    setActiveTab(getTab(item.key));

    try {
      await invoke("rpc_set_image", {
        imageKey: item.key,
      });
    } catch {
      // optional: rollback or toast
    }
  };

  const renderGrid = (items: RpcImageItem[], disabled = false) => (
    <div className="grid grid-cols-4 gap-1.5">
      {items.map((item) => {
        const active = item.key === activeKey;

        return (
          <button
            key={item.key}
            onClick={() => !disabled && selectImage(item)}
            disabled={disabled}
            className={cn(
              "group relative aspect-square rounded-xl size-27 overflow-hidden bg-muted/70",
              "hover:ring-2 hover:ring-foreground/10 text-white/70 transition-all focus:outline-none",
              active &&
                "ring-3 hover:ring-3 ring-foreground/20 text-white hover:ring-foreground/20",
              disabled && "pointer-events-none ring-2 ring-red-950/20",
            )}
          >
            <img
              src={item.image}
              alt=""
              draggable={false}
              className={cn(
                "h-full w-full object-cover select-none pointer-events-none transition-opacity",
                //disabled && "opacity-60",
              )}
            />

            <div
              className={cn(
                "absolute inset-0 flex items-end justify-center",
                "bg-gradient-to-t from-black/60 via-black/0 to-black/0",
                disabled && "from-red-950/60",
              )}
            >
              <span
                className={cn(
                  "relative text-sm group-hover:text-white transition font-medium z-20 pb-1",
                  disabled && "text-red-400",
                )}
              >
                {item.label}
              </span>
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

        <div className="rounded-3xl p-1.5 bg-muted/70">
          <Tabs
            className="gap-1"
            value={activeTab}
            onValueChange={setActiveTab}
          >
            <TabsList>
              <TabsTrigger value="fill">Free</TabsTrigger>
              <TabsTrigger
                value="plus"
                className={cn(
                  !isVeryCool &&
                    "data-[state=active]:bg-red-200/70! data-[state=active]:text-red-800! dark:data-[state=active]:bg-red-950/70! dark:data-[state=active]:text-red-300! classic:data-[state=active]:bg-red-950/70! classic:data-[state=active]:text-red-300!",
                )}
              >
                Supporter
              </TabsTrigger>
              <TabsTrigger
                value="supporterplus"
                className={cn(
                  !isVeryVeryCool &&
                    "data-[state=active]:bg-red-200/70! data-[state=active]:text-red-800! dark:data-[state=active]:bg-red-950/70! dark:data-[state=active]:text-red-300! classic:data-[state=active]:bg-red-950/70! classic:data-[state=active]:text-red-300!",
                )}
              >
                Supporter+
              </TabsTrigger>
            </TabsList>

            <div className="rounded-2xl bg-background p-1">
              <TabsContent value="fill">{renderGrid(FILL_IMAGES)}</TabsContent>

              <TabsContent value="plus">
                {renderGrid(PLUS_IMAGES, !isVeryCool)}
              </TabsContent>

              <TabsContent value="supporterplus">
                {renderGrid(SP_IMAGES, !isVeryVeryCool)}
              </TabsContent>
            </div>
          </Tabs>
        </div>
      </DialogContent>
    </Dialog>
  );
}
