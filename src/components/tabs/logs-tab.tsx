import { useEffect, useRef } from "react";
import { useLogs } from "@/lib/proxy-logs";
import { FunnelXIcon } from "@phosphor-icons/react";

export function Logs() {
  const logs = useLogs((s) => s.logs);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;
    el.scrollTop = el.scrollHeight;
  }, [logs]);

  return (
    <div className="flex flex-col space-y-4">
      <h2 className="text-base font-medium">Logs</h2>

      <div
        style={{ height: "calc(100vh - 5.5rem)" }}
        className="overflow-auto p-3 rounded-2xl bg-muted font-mono text-xs leading-relaxed text-muted-foreground"
        ref={containerRef}
      >
        {logs.length === 0 ? (
          <div className="flex flex-col gap-2 justify-center items-center text-center h-full text-muted-foreground/50">
            <FunnelXIcon className="size-6" weight="light" />
            No logs yet
          </div>
        ) : (
          logs.map((line, i) => (
            <div key={i} className="whitespace-pre-wrap break-words select-text">
              {line}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
