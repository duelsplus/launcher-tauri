export interface ProxyError {
  code: string;
  title: string;
  message: string;
  suggestion: string;
  severity: "info" | "warning" | "error" | "critical"; // inline enums LOL
  category: "network" | "authentication" | "hypixel" | "proxy" | "api" | "unknown";
  originalMessage: string;
  context: string | null;
  timestamp: number;
}
