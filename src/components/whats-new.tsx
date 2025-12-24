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
  //link
  escaped = escaped.replace(
    /\[([^\]]+)\]\(([^)]+)\)/g,
    '<a href="$2" target="_blank" rel="noopener noreferrer">$1</a>',
  );

  return escaped;
}

const bannedReleases: Release[] = [
  {
    id: "six-seven",
    version: "v6.9.420",
    releaseDate: "",
    isBeta: false,
    isLatest: false,
    changelog: "",
    whatsNew: [
      "Everyone gets **fifteen** American dollars per win",
      "Losses are automatically converted to wins",
      "Zero ping mode has rolled out globally",
      "Such a shame you can't experience all those wonderful additions",
    ],
  },
];

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

  const shownReleases =
    user?.isBanned === true ? [...bannedReleases, ...releases] : releases;

  const totalPages = Math.ceil(shownReleases.length / itemsPerPage);
  const pageItems = shownReleases.slice(
    page * itemsPerPage,
    page * itemsPerPage + itemsPerPage,
  );

  useEffect(() => {
    invoke<Release[]>("fetch_releases")
      .then(setReleases)
      .finally(() => setLoading(false));
  }, []);

  if (loading) {
    return (
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {Array.from({ length: 6 }).map((_, i) => (
          <Skeleton key={i} className="h-44" />
        ))}
      </div>
    );
  }

  return (
    <>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        {pageItems.map((release) => {
          const maxBullets = 2;
          const whatsNew = release.whatsNew ?? [];
          const showMore = release.whatsNew.length > maxBullets;
          return (
            <motion.div
              key={release.id}
              layoutId={`release-${release.id}`}
              onClick={() => setSelected(release)}
              className="p-4 rounded-2xl bg-muted/70"
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
            >
              <div className="flex items-center gap-2 mb-2">
                <h3 className="text-lg font-semibold">{release.version}</h3>
                {release.isLatest && (
                  <div className="size-1.5 bg-primary rounded-full" />
                )}
              </div>
              <ul className="list-disc list-inside space-y-1 text-sm">
                {whatsNew.slice(0, maxBullets).map((item, idx) => (
                  <li
                    key={idx}
                    dangerouslySetInnerHTML={{ __html: renderMarkdown(item) }}
                  />
                ))}
              </ul>
              {showMore && (
                <p className="text-xs text-muted-foreground mt-2">
                  +{whatsNew.length - maxBullets} more
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
              className="w-full max-w-lg rounded-2xl bg-background p-6 shadow-xl"
            >
              <div className="flex items-center gap-3 mb-4">
                <h3 className="text-xl font-bold">{selected.version}</h3>
                {selected.isLatest && (
                  <div className="size-1.5 bg-primary rounded-full" />
                )}
              </div>

              <ul className="list-disc list-inside space-y-2 text-sm">
                {selected.whatsNew.map((item, i) => (
                  <li
                    key={i}
                    dangerouslySetInnerHTML={{ __html: renderMarkdown(item) }}
                  />
                ))}
              </ul>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      <Pagination className="mt-6 justify-center flex">
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
    </>
  );
}
