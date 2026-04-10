const key = "brandColor";

export function getBrand(): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

export function setBrand(color: string | null): void {
  try {
    if (color) {
      localStorage.setItem(key, color);
    } else {
      localStorage.removeItem(key);
    }
  } catch {
    //
  }
}

export function hasBrand(): boolean {
  const brand = getBrand();
  return typeof brand === "string" && brand.length > 0;
}

export function applyBrand(color: string | null) {
  if (!color) return;
  document.documentElement.style.setProperty("--brand", color);
}
