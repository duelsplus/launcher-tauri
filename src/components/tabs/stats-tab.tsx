import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Logo, Logomark, LogomarkCustom } from "@/components/logo";
import { Skeleton } from "../ui/skeleton";

interface StatCardProps {
  title: string;
  icon?: React.ReactNode;
  stats: { label: string; value: string | number }[];
  loading?: boolean;
}

export function StatCard({ title, icon, stats, loading }: StatCardProps) {
  return (
    <div className="p-4 rounded-2xl bg-muted">
      <div className="flex items-center gap-1.5 mb-3">
        {icon && <div className="text-muted-foreground/50">{icon}</div>}
        <h3 className="text-xs text-muted-foreground/50 tracking-widest font-semibold uppercase">
          {title}
        </h3>
      </div>

      <div className="grid grid-cols-2 gap-4">
        {stats.map((stat) => (
          <div key={stat.label} className="flex flex-col items-start">
            {loading ? (
              <Skeleton className="bg-background h-5.5 w-12 mb-1.5" />
            ) : (
              <span className="text-lg font-bold text-foreground">
                {stat.value}
              </span>
            )}
            <span className="text-xs text-muted-foreground">{stat.label}</span>
          </div>
        ))}
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
  const [error, setError] = useState(false)

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
    </div>
  );
}
