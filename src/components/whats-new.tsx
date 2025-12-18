import { useEffect, useState } from "react";
import { Skeleton } from "./ui/skeleton";

type Release = {
  id: string;
  version: string;
  releaseDate: string;
  isBeta: boolean;
  isLatest: boolean;
  changelog: string;
  whatsNew: string[];
};

export function WhatsNew() {
  const [releases, setReleases] = useState<Release[]>([]);
  const [loading, setLoading] = useState(true);

  async function fetchReleases() {
    try {
      //
    } catch (err) {
      //
    }
  }

  useEffect(() => {
    fetchReleases()
      //.then(setReleases)
      .finally(() => setLoading(false));
  }, []);

  if (loading) {
    return (
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {Array.from({ length: 6 }).map((_, i) => (
          <Skeleton key={i} className="h-48" />
        ))}
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
      {releases.map((release) => {
        const maxBullets = 3;
        const showMore = release.whatsNew.length > maxBullets;
        return (
          <div
            key={release.id}
            className="p-4 border rounded-lg shadow-sm bg-muted"
          >
            <h3 className="text-lg font-semibold mb-2">{release.version}</h3>
            <ul className="list-disc list-inside space-y-1 text-sm">
              {release.whatsNew.slice(0, maxBullets).map((item, idx) => (
                <li key={idx}>{item}</li>
              ))}
            </ul>
            {showMore && (
              <p className="text-xs text-muted mt-1">
                +{release.whatsNew.length - maxBullets} more
              </p>
            )}
          </div>
        );
      })}
    </div>
  );
}
