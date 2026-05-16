import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Logo, Logomark, LogomarkCustom } from "@/components/logo";
import { Skeleton } from "../ui/skeleton";
import { ArrowUpRightIcon, WarningIcon } from "@phosphor-icons/react";
import { Button } from "../ui/button";

interface StatCardProps {
  title: string;
  icon?: React.ReactNode;
  stats: { label: string; value: string | number }[];
  warning?: boolean;
  loading?: boolean;
}

export function StatCard({
  title,
  icon,
  stats,
  warning,
  loading,
}: StatCardProps) {
  return (
    <div className="p-1.5 rounded-3xl bg-muted/70">
      <div className="flex justify-between items-center gap-3 px-2 pt-1 mb-2">
        <div className="flex items-center gap-1.5">
          {icon && <div className="text-muted-foreground/50">{icon}</div>}
          <h2 className="text-xs font-bold tracking-widest uppercase text-muted-foreground/50">
            {title}
          </h2>
        </div>
        {warning && (
          <WarningIcon weight="fill" className="size-4 fill-amber-400" />
        )}
      </div>

      <div className="rounded-2xl bg-background p-1">
        <div className="grid grid-cols-2 gap-4 p-1.5">
          {stats.map((stat) => (
            <div key={stat.label} className="flex flex-col items-start">
              {loading ? (
                <Skeleton className="bg-muted/70 h-5.5 w-12 mb-1.5" />
              ) : (
                <span className="text-lg font-bold text-foreground">
                  {stat.value}
                </span>
              )}
              <span className="text-xs text-muted-foreground">
                {stat.label}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

type UserStatsResponse = {
  success: boolean;
  stats?: {
    wins: number;
    losses: number;
    winLossRatio: number;
    winRate: number;
  };
};

type GlobalStatsResponse = {
  data: {
    globalStats: {
      totalWins: number;
      totalLosses: number;
      totalGames: number;
      totalPlayers: number;
      winLossRatio: number;
    };
  };
};

export function Stats() {
  const [userStats, setUserStats] = useState<UserStatsResponse["stats"] | null>(
    null,
  );
  const [globalStats, setGlobalStats] = useState<
    GlobalStatsResponse["data"]["globalStats"] | null
  >(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    setLoading(true);
    setError(false);

    Promise.all([
      invoke<UserStatsResponse>("get_user_stats"),
      invoke<GlobalStatsResponse>("get_global_stats"),
    ])
      .then(([userResp, globalResp]) => {
        if (userResp.success && userResp.stats) {
          setUserStats(userResp.stats);
        } else {
          setUserStats(null);
        }
        setGlobalStats(globalResp.data.globalStats);
      })
      .catch((err) => {
        setError(true); //unused
      })
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="space-y-4">
      <h2 className="text-base font-medium">Statistics</h2>

      <StatCard
        title="User"
        icon={
          <LogomarkCustom className="h-3 w-auto text-muted-foreground/50" />
        }
        stats={[
          {
            label: "Games Played",
            value:
              (
                (userStats?.wins ?? 0) + (userStats?.losses ?? 0)
              ).toLocaleString() ?? "-",
          },
          { label: "Wins", value: userStats?.wins.toLocaleString() ?? "-" },
          { label: "Losses", value: userStats?.losses.toLocaleString() ?? "-" },
          { label: "WLR", value: userStats?.winLossRatio.toFixed(2) ?? "-" },
        ]}
        warning={error}
        loading={loading}
      />

      <StatCard
        title="Global"
        icon={
          <LogomarkCustom className="h-3 w-auto text-muted-foreground/50" />
        }
        stats={[
          {
            label: "Total Games",
            value: globalStats?.totalGames.toLocaleString() ?? "-",
          },
          {
            label: "Total Wins",
            value: globalStats?.totalWins.toLocaleString() ?? "-",
          },
          {
            label: "Total Losses",
            value: globalStats?.totalLosses.toLocaleString() ?? "-",
          },
          { label: "WLR", value: globalStats?.winLossRatio.toFixed(2) ?? "-" },
        ]}
        loading={loading}
      />

      <div className="w-full flex justify-center">
        <a
          href="https://dash.duelsplus.com/?tab=statistics"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Button variant="input" size="sm">
            View detailed stats
            <ArrowUpRightIcon />
          </Button>
        </a>
      </div>
    </div>
  );
}
