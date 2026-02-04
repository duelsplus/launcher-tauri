export type Perm =
  | "admin"
  | "developer"
  | "moderator"
  | "tester"
  | "partner"
  | "leaderboard"
  | "supporter"
  | "combo"
  | "standard";

export interface User {
  id: string;
  discordId: string;
  username: string;
  perms: Perm[];
  isBanned: boolean;
}

export function hasPerm(user: User | undefined | null, perm: Perm) {
  if (!user || !user.perms) return false;
  return user.perms.includes(perm);
}
