import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "../ui/button";
import { ProxyError } from "@/types/proxy";
import { cn } from "@/lib/utils";

interface ProxyErrorDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  error: ProxyError | null;
}

export function ProxyErrorDialog({
  open,
  onOpenChange,
  error,
}: ProxyErrorDialogProps) {
  if (!error) return null;
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Uh oh!</DialogTitle> {/* {error.title} */}
        </DialogHeader>

        <section className="space-y-3 text-sm leading-relaxed text-muted-foreground">
          <p className="font-medium">
            The proxy has crashed unexpectedly. Here's what we know:
          </p>
          <p>{error.message}</p>
          <div>
            <Button
              size="sm"
              variant="rose"
              className="pointer-events-none rounded-b-none"
              ripple={false}
            >
              Suggestion
            </Button>
            <div className="p-4 rounded-xl rounded-tl-none bg-rose-200/50 text-rose-600 dark:bg-rose-950/50 dark:text-rose-400 classic:bg-rose-950/50 classic:text-rose-400">
              <p>{error.suggestion}</p>
            </div>
            {/* i replicated the rose button variant */}
          </div>
        </section>
      </DialogContent>
    </Dialog>
  );
}
