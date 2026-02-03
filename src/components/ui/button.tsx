import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-xl text-sm font-medium transition-all duration-200 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none aria-invalid:border-destructive",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground hover:bg-primary/90",
        destructive:
          "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
        outline:
          "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50",
        secondary:
          "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        warning:
          "bg-amber-100/50 text-amber-600 hover:bg-amber-100 dark:bg-amber-950/50 dark:text-amber-500 dark:hover:bg-amber-950 classic:bg-amber-950/50 classic:text-amber-500 classic:hover:bg-amber-950",
        rose: "bg-rose-200/50 text-rose-600 hover:bg-rose-200 dark:bg-rose-950/50 dark:text-rose-400 dark:hover:bg-rose-950 classic:bg-rose-950/50 classic:text-rose-400 classic:hover:bg-rose-950",
        input: "bg-input/50 dark:text-muted-foreground hover:bg-input/80",
        muted: "bg-card text-muted-foreground hover:bg-popover",
        ghost:
          "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
        link: "text-primary underline-offset-4 hover:underline",
      },
      size: {
        default: "h-9 px-4 py-2 has-[>svg]:px-3",
        sm: "h-8 gap-1.5 px-3 has-[>svg]:px-2.5",
        lg: "h-10 px-6 has-[>svg]:px-4",
        icon: "size-9",
        "icon-sm": "size-8",
        "icon-lg": "size-10",
        "pill-lg":
          "h-12 rounded-[32px] hover:rounded-3xl gap-3 font-medium text-base px-6 [&_svg:not([class*='size-'])]:size-5 has-[>svg]:px-4",
        "pill":
          "h-9 rounded-[24px] hover:rounded-2xl gap-2 font-medium text-sm px-4 [&_svg:not([class*='size-'])]:size-5.5 has-[>svg]:px-3",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
);

function Button({
  className,
  variant = "default",
  size = "default",
  asChild = false,
  ...props
}: React.ComponentProps<"button"> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : "button";

  return (
    <Comp
      data-slot="button"
      data-variant={variant}
      data-size={size}
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  );
}

export { Button, buttonVariants };
