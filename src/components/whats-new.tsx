import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Skeleton } from "./ui/skeleton";
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";
import { motion, AnimatePresence } from "framer-motion";
import { getToken } from "@/lib/token";

import {
  BugIcon,
  PlusIcon,
  SparkleIcon,
  WrenchIcon,
  InfoIcon,
  CircleIcon,
} from "@phosphor-icons/react";
import { Tooltip, TooltipContent, TooltipTrigger } from "./ui/tooltip";
type Release = {
  id: string;
  version: string;
  releaseDate: string;
  isBeta: boolean;
  isLatest: boolean;
  changelog: string;
  whatsNew: string[];
};

type ApiResponse<T> = {
  success: boolean;
  data: T;
};

type User = {
  id: string;
  username: string;
  isBanned: boolean;
};

export function renderMarkdown(text: string) {
  // i did not want to install another dependency for md
  let escaped = text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
  //code
  escaped = escaped.replace(/`([^`]+)`/g, "<code>$1</code>");
  //bold
  escaped = escaped.replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
  //italic
  escaped = escaped.replace(/\*([^*]+)\*/g, "<em>$1</em>");
  //masked link
  escaped = escaped.replace(
    /\[([^\]]+)\]\((https?:\/\/[^\s)]+)\)/g,
    '<a href="$2" target="_blank" class="text-primary no-underline hover:underline" rel="noopener noreferrer">$1</a>',
  );
  //link
  escaped = escaped.replace(
    /(^|\s)((https?:\/\/|www\.)[^\s<]+)/gi,
    (_, prefix, url) => {
      const href = url.startsWith("http") ? url : `https://${url}`;
      return `${prefix}<a href="${href}" target="_blank" class="text-primary no-underline hover:underline" rel="noopener noreferrer">${url}</a>`;
    },
  );

  return escaped;
}

function formatDate(date: string) {
  const now = new Date();
  const then = new Date(date);
  const diffMs = now.getTime() - then.getTime();

  const seconds = Math.floor(diffMs / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);
  const weeks = Math.floor(days / 7);
  const months = Math.floor(days / 30);
  const years = Math.floor(days / 365);

  if (seconds < 30) return "just now";
  if (seconds < 60) return `${seconds} seconds ago`;
  if (minutes < 60) return `${minutes} minute${minutes === 1 ? "" : "s"} ago`;
  if (hours < 24) return `${hours} hour${hours === 1 ? "" : "s"} ago`;
  if (days === 1) return "yesterday";
  if (days < 7) return `${days} days ago`;
  if (weeks < 5) return `${weeks} week${weeks === 1 ? "" : "s"} ago`;
  if (months < 12) return `${months} month${months === 1 ? "" : "s"} ago`;
  return `${years} year${years === 1 ? "" : "s"} ago`;
}

function changeMeta(item: string) {
  const lower = item.toLowerCase();

  if (lower.startsWith("fix") || lower.startsWith("fixed"))
    return {
      type: "fix",
      Icon: BugIcon,
      className: "text-rose-500 dark:text-rose-400 classic:text-rose-400 black:text-rose-400",
    };

  if (
    lower.startsWith("add") ||
    lower.startsWith("added") ||
    lower.startsWith("new") ||
    lower.startsWith("enable") ||
    lower.startsWith("enabled")
  )
    return {
      type: "new",
      Icon: PlusIcon,
      className: "text-green-600 dark:text-green-400 classic:text-green-400 black:text-green-400",
    };

  if (
    lower.startsWith("improve") ||
    lower.startsWith("improved") ||
    lower.startsWith("rework") ||
    lower.startsWith("reworked") ||
    lower.startsWith("optimise") ||
    lower.startsWith("optimised")
  )
    return {
      type: "improve",
      Icon: SparkleIcon,
      className: "text-sky-500 dark:text-sky-400 classic:text-sky-400 black:text-sky-400",
    };

  if (
    lower.startsWith("change") ||
    lower.startsWith("update") ||
    lower.startsWith("revert") ||
    lower.startsWith("reverted") ||
    lower.startsWith("rename") ||
    lower.startsWith("renamed")
  )
    return {
      type: "change",
      Icon: WrenchIcon,
      className: "text-amber-500 dark:text-amber-400 classic:text-amber-400 black:text-amber-400",
    };

  return { type: "info", Icon: InfoIcon, className: "text-muted-foreground" };
}

export function WhatsNew() {
  const [releases, setReleases] = useState<Release[]>([]);
  const [loading, setLoading] = useState(true);
  const [selected, setSelected] = useState<Release | null>(null);
  const [page, setPage] = useState(0); //zero‑based
  const itemsPerPage = 6;
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    invoke<ApiResponse<User>>("get_user", {
      token: getToken(),
    })
      .then((u) => setUser(u.data))
      .catch(() => setUser(null));
  }, []);

  const totalPages = Math.ceil(releases.length / itemsPerPage);
  const pageItems = releases.slice(
    page * itemsPerPage,
    page * itemsPerPage + itemsPerPage,
  );

  useEffect(() => {
    invoke<Release[]>("fetch_releases")
      .then(setReleases)
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="rounded-3xl p-1.5 bg-muted/70">
      <h2 className="text-xs font-bold tracking-widest uppercase text-primary/70 px-2 pt-1 pb-2">
        Latest Updates
      </h2>
      <div className="rounded-2xl bg-background p-1">
        {loading || !releases.length ? (
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-1.5">
            {Array.from({ length: 6 }).map((_, i) => (
              <Skeleton key={i} className="h-44" />
            ))}
          </div>
        ) : (
          <>
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-1.5">
              {pageItems.map((release) => {
                const maxBullets = 2;
                const whatsNew = release.whatsNew ?? [];
                const showMore = release.whatsNew.length > maxBullets;
                return (
                  <motion.div
                    key={release.id}
                    layoutId={`release-${release.id}`}
                    onClick={() => setSelected(release)}
                    className="p-4 rounded-xl bg-muted/70"
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.97 }}
                    transition={{
                      type: "spring",
                      stiffness: 250,
                      damping: 22,
                      mass: 0.7,
                    }}
                  >
                    <h3 className="text-lg font-semibold mb-2">
                      {release.version}
                    </h3>
                    <ul className="list-disc list-inside space-y-1 text-sm">
                      {whatsNew.slice(0, maxBullets).map((item, idx) => {
                        const meta = changeMeta(item);
                        return (
                          <li
                            key={idx}
                            className="flex gap-2 items-center text-sm"
                          >
                            <CircleIcon
                              className={`shrink-0 size-2 ${meta.className}`}
                              weight="bold"
                            />
                            <span
                              className="line-clamp-1 font-medium"
                              dangerouslySetInnerHTML={{
                                __html: renderMarkdown(item),
                              }}
                            />
                          </li>
                        );
                      })}
                    </ul>
                    {showMore && (
                      <p className="text-xs text-muted-foreground mt-2">
                        +{whatsNew.length - maxBullets}
                      </p>
                    )}
                  </motion.div>
                );
              })}
            </div>

            <AnimatePresence>
              {selected && (
                <motion.div
                  className="fixed inset-0 h-full z-50 flex items-center justify-center backdrop-blur bg-black/50"
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  onClick={() => setSelected(null)}
                >
                  <motion.div
                    layoutId={`release-${selected.id}`}
                    onClick={(e) => e.stopPropagation()}
                    className="w-full max-w-lg rounded-xl bg-background p-6"
                    transition={{
                      type: "spring",
                      stiffness: 200,
                      damping: 22,
                    }}
                  >
                    <div className="flex items-center justify-between gap-3 mb-4">
                      <h3 className="text-lg font-semibold tracking-tight">
                        {selected.version}
                      </h3>
                      <div className="flex items-center justify-between text-xs font-medium text-muted-foreground/70">
                        <span>
                          Released{" "}
                          <span className="text-muted-foreground">
                            {formatDate(selected.releaseDate)}
                          </span>
                        </span>
                      </div>
                    </div>

                    {selected.whatsNew.length === 0 ? (
                      <p className="text-sm text-center text-muted-foreground">
                        This release has no changelog
                      </p>
                    ) : (
                      <div className="max-h-[24rem] overflow-y-auto">
                        <ul className="list-disc list-inside space-y-2 text-sm">
                          {selected.whatsNew.map((item, i) => {
                            const meta = changeMeta(item);
                            return (
                              <li key={i} className="flex gap-3 items-start">
                                <CircleIcon
                                  className={`mt-[7px] shrink-0 size-2 ${meta.className}`}
                                  weight="bold"
                                />
                                <span
                                  className="leading-relaxed font-medium"
                                  dangerouslySetInnerHTML={{
                                    __html: renderMarkdown(item),
                                  }}
                                />
                              </li>
                            );
                          })}
                        </ul>
                      </div>
                    )}
                  </motion.div>
                </motion.div>
              )}
            </AnimatePresence>
          </>
        )}
      </div>

      <Pagination className="mt-2 mb-1 justify-center flex">
        <PaginationContent>
          <PaginationItem>
            <PaginationPrevious
              onClick={(e) => {
                e.preventDefault();
                setPage(Math.max(page - 1, 0));
              }}
            />
          </PaginationItem>

          {Array.from({ length: totalPages }).map((_, i) => (
            <PaginationItem key={i}>
              <PaginationLink
                isActive={i === page}
                onClick={(e) => {
                  e.preventDefault();
                  setPage(i);
                }}
              >
                {i + 1}
              </PaginationLink>
            </PaginationItem>
          ))}

          <PaginationItem>
            <PaginationNext
              onClick={(e) => {
                e.preventDefault();
                setPage(Math.min(page + 1, totalPages - 1));
              }}
            />
          </PaginationItem>
        </PaginationContent>
      </Pagination>
    </div>
  );
}
