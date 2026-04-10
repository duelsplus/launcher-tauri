import { parse, formatHex } from "culori";

export function toHex(color: string): string {
  const parsed = parse(color);
  if (!parsed) return "#000000";
  return formatHex(parsed);
}
