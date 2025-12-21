const key = "authToken";

export function getToken(): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

export function setToken(token: string | null): void {
  try {
    if (token) {
      localStorage.setItem(key, token);
    } else {
      localStorage.removeItem(key);
    }
  } catch {
    //
  }
}

export function hasToken(): boolean {
  const token = getToken();
  return typeof token === "string" && token.length > 0;
}
