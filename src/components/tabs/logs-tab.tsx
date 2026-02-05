import { useEffect, useRef, useState } from "react";
import { useLogs } from "@/lib/proxy-logs";
import { BackspaceIcon, FunnelXIcon, PaletteIcon } from "@phosphor-icons/react";

import AnsiToHtml from "ansi-to-html";
import { Button } from "../ui/button";
const ansiConvert = new AnsiToHtml({ escapeXML: true });

export function Logs() {
  const logs = useLogs((s) => s.logs);
  const containerRef = useRef<HTMLDivElement>(null);
  const [colors, setColors] = useState(() => {
    const saved = localStorage.getItem("logs-colors");
    return saved === null ? true : saved === "true";
  });

  useEffect(() => {
    localStorage.setItem("logs-colors", colors.toString());
  }, [colors]);

  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;
    el.scrollTop = el.scrollHeight;
  }, [logs]);

  function renderLog(text: string) {
    let escaped = text
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;");

    if (colors) {
      escaped = ansiConvert.toHtml(escaped);
    } else {
      escaped = strip(escaped);
    }

    escaped = escaped.replace(
      /(https?:\/\/[^\s]+)/g,
      '<a href="$1" target="_blank" class="underline hover:no-underline" rel="noopener noreferrer">$1</a>',
    );

    return escaped;
  }

  function strip(text: string) {
    return text.replace(/\x1B\[[0-9;]*m/g, "");
  }

  return (
    <div className="flex flex-col space-y-4">
      <div className="flex justify-between items-center gap-2">
        <h2 className="text-base font-medium">Logs</h2>
        <div className="flex items-center gap-1.5">
          <Button
            size="icon-xs"
            onClick={() => useLogs.getState().clear()}
            variant="input"
          >
            <BackspaceIcon weight="fill" />
          </Button>
          <Button
            size="icon-xs"
            onClick={() => setColors((prev) => !prev)}
            variant={colors ? "input" : "outline"}
          >
            <PaletteIcon weight={colors ? "fill" : "regular"} />
          </Button>
        </div>
      </div>

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
            <div
              key={i}
              className="whitespace-pre-wrap break-words select-text"
              dangerouslySetInnerHTML={{ __html: renderLog(line) }}
            />
          ))
        )}
      </div>
    </div>
  );
}
