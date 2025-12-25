import * as React from "react"
import * as SwitchPrimitive from "@radix-ui/react-switch"

import { cn } from "@/lib/utils"

function Switch({
  className,
  ...props
}: React.ComponentProps<typeof SwitchPrimitive.Root>) {
  return (
    <SwitchPrimitive.Root
      data-slot="switch"
      className={cn(
        "peer group/switch data-[state=checked]:bg-input data-[state=unchecked]:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 inline-flex h-[1.25rem] w-9 shrink-0 items-center rounded-xl border border-transparent shadow-xs transition-all outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50",
        className
      )}
      {...props}
    >
      <SwitchPrimitive.Thumb
        data-slot="switch-thumb"
        className={cn(
          "bg-background dark:data-[state=unchecked]:bg-foreground classic:data-[state=unchecked]:bg-foreground/50 dark:data-[state=checked]:bg-primary-foreground classic:data-[state=checked]:bg-primary-foreground pointer-events-none block size-4 group-active/switch:w-5 group-active/item:w-5 group-hover/switch:w-5 group-hover/item:w-5  rounded-[3px] ring-0 transition-all data-[state=checked]:translate-x-full data-[state=unchecked]:translate-x-[2px] group-hover/switch:data-[state=checked]:translate-x-[calc(100%-8px)] group-hover/item:data-[state=checked]:translate-x-[calc(100%-8px)] group-active/switch:data-[state=checked]:translate-x-[calc(100%-8px)] group-active/item:data-[state=checked]:translate-x-[calc(100%-8px)]"
        )}
      />
    </SwitchPrimitive.Root>
  )
}

export { Switch }
