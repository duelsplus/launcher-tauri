import * as React from "react"
import { Slot } from "@radix-ui/react-slot"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@/lib/utils"

const badgeVariants = cva(
  "inline-flex items-center justify-center rounded-full border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden",
  {
    variants: {
      variant: {
        default:
          "border-transparent bg-primary text-primary-foreground [a&]:hover:bg-primary/90",
        secondary:
          "border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90",
        destructive:
          "border-transparent bg-destructive text-white [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
        outline:
          "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground",
        input: "border-transparent bg-input/50 dark:text-muted-foreground",
        divine:
          "border-transparent bg-purple-400/20 text-purple-600 dark:bg-purple-500/30 dark:text-purple-300 classic:bg-purple-500/30 classic:text-purple-300 black:bg-purple-500/30 black:text-purple-300",
        celestial:
          "border-transparent bg-blue-400/20 text-blue-600 dark:bg-blue-500/30 dark:text-blue-300 classic:bg-blue-500/30 classic:text-blue-300 black:bg-blue-500/30 black:text-blue-300",
        fallback:
          "border-transparent bg-zinc-200/50 text-zinc-600 dark:bg-zinc-700/50 dark:text-zinc-300 classic:bg-zinc-700/30 classic:text-zinc-300 black:bg-zinc-700/30 black:text-zinc-300",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
)

function Badge({
  className,
  variant,
  asChild = false,
  ...props
}: React.ComponentProps<"span"> &
  VariantProps<typeof badgeVariants> & { asChild?: boolean }) {
  const Comp = asChild ? Slot : "span"

  return (
    <Comp
      data-slot="badge"
      className={cn(badgeVariants({ variant }), className)}
      {...props}
    />
  )
}

export { Badge, badgeVariants }
