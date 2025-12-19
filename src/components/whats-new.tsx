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

type Release = {
  id: string;
  version: string;
  releaseDate: string;
  isBeta: boolean;
  isLatest: boolean;
  changelog: string;
  whatsNew: string[];
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

export function WhatsNew() {
  const [releases, setReleases] = useState<Release[]>([]);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(0); //zero‑based
  const itemsPerPage = 6;

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
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {pageItems.map((release) => {
          const maxBullets = 2;
          const whatsNew = release.whatsNew ?? [];
          const showMore = release.whatsNew.length > maxBullets;
          return (
            <div
              key={release.id}
              className="p-4 rounded-2xl bg-muted/70"
            >
              <h3 className="text-lg font-semibold mb-2">{release.version}</h3>
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
            </div>
          );
        })}
      </div>

      <Pagination className="mt-6 justify-center flex">
        <PaginationContent>
          <PaginationItem>
            <PaginationPrevious
              href="#"
              onClick={(e) => {
                e.preventDefault();
                setPage(Math.max(page - 1, 0));
              }}
            />
          </PaginationItem>

          {Array.from({ length: totalPages }).map((_, i) => (
            <PaginationItem key={i}>
              <PaginationLink
                href="#"
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
              href="#"
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
