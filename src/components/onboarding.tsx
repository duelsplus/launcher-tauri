import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { cn } from "@/lib/utils";
import { SpinnerIcon } from "@phosphor-icons/react";
import { Titlebar } from "./titlebar";
import { useTheme } from "@/components/theme-provider";

type OnboardingProps = {
  open: boolean;
  onFinish: () => void;
};

type VerifyTokenResponse = {
  success: boolean;
  code?: number;
};

type Step = "welcome" | "token" | "import" | "theme" | "done";

function OnboardingPanel({ children }: { children: React.ReactNode }) {
  return <div className="w-full px-6 py-8">{children}</div>;
}

function Panel({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex flex-col items-center justify-center text-center max-w-xl mx-auto space-y-4">
      <div>
        <h2 className="text-3xl font-light">{title}</h2>
        {description && (
          <p className="text-sm text-muted-foreground mt-2">{description}</p>
        )}
      </div>
      {children}
    </div>
  );
}

function TitleOnly({ title, onNext }: { title: string; onNext?: () => void }) {
  return (
    <div className="flex items-center justify-center" onClick={onNext}>
      <h1 className="text-4xl font-light text-center">{title}</h1>
    </div>
  );
}

export function Onboarding({ open, onFinish }: OnboardingProps) {
  const [step, setStep] = useState<Step>("welcome");
  const [hasValidToken, setHasValidToken] = useState<boolean | null>(null);
  const [token, setToken] = useState("");
  const [loading, setLoading] = useState(true);
  const [actionLoading, setActionLoading] = useState(false);
  const [actionError, setActionError] = useState(false);
  const { theme: currentTheme, setTheme: setAppTheme } = useTheme();
  const [theme, setTheme] = useState<"system" | "dark" | "light">(currentTheme);

  useEffect(() => {
    invoke<boolean>("token_exists")
      .then(setHasValidToken)
      .finally(() => setLoading(false));
  }, []);

  // autoclose after final step
  useEffect(() => {
    if (step === "welcome") {
      const t = setTimeout(onWelcome, 3000);
      return () => clearTimeout(t);
    }
    if (step === "done") {
      const t = setTimeout(onFinish, 3000);
      return () => clearTimeout(t);
    }
  }, [step, onFinish]);

  async function onWelcome() {
    if (hasValidToken && !loading) {
      setStep("theme");
    } else {
      setStep("token");
    }
  }

  async function handleVerify() {
    if (!token) return;

    setActionError(false);
    setActionLoading(true);
    try {
      const response = await invoke<VerifyTokenResponse>("verify_token", {
        token,
      });
      if (response?.success) {
        setHasValidToken(true);
        setStep("theme"); //switch to import once implemented
      } else {
        throw new Error();
      }
    } catch (err) {
      setActionError(true);
    } finally {
      setActionLoading(false);
    }
  }

  if (!open) return null;
  if (loading)
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-background select-none">
        <SpinnerIcon className="size-6 animate-spin text-muted-foreground" />
      </div>
    );

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-background select-none">
      <Titlebar />
      <OnboardingPanel>
        {step === "welcome" && (
          <TitleOnly
            title="Welcome to the new Duels+ Launcher"
            onNext={() => onWelcome()}
          />
        )}

        {step === "token" && (
          <Panel
            title="Verification Token"
            description="You need a verification token tied to your account. Retrieve it from our Discord server."
          >
            <div className="relative w-full max-w-md">
              <Input
                placeholder="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
                value={token}
                disabled={actionLoading}
                aria-invalid={actionError}
                onChange={(e) => {
                  setToken(e.target.value);
                  setActionError(false);
                }}
                onKeyDown={(e) => {
                  if (e.key === "Enter" && token && !actionLoading) {
                    handleVerify();
                  }
                }}
                className="pr-24 font-mono"
              />
              <Button
                className="absolute right-1 top-1/2 -translate-y-1/2 rounded-lg"
                size="sm"
                variant={actionLoading ? "ghost" : "outline"}
                disabled={actionError || actionLoading}
                onClick={handleVerify}
              >
                {actionLoading ? (
                  <SpinnerIcon className="animate-spin" />
                ) : (
                  "Continue"
                )}
              </Button>
            </div>
          </Panel>
        )}

        {/* {step === "import" && (
          <Panel
            title="Import Settings"
            description="We detected settings from the legacy Duels+ launcher."
          >
            <div className="flex gap-2 mt-4">
              <Button variant="input" onClick={() => setStep("theme")}>
                Import from Legacy Launcher
              </Button>
              <Button variant="outline" onClick={() => setStep("theme")}>
                Skip
              </Button>
            </div>

            ! todo: hook detection + import
          </Panel>
        )} */}

        {step === "theme" && (
          <Panel title="Choose Your Theme">
            <div className="flex flex-col gap-0 mt-4 w-full max-w-sm mx-auto">
              {(["dark", "light"] as const).map((t, idx, arr) => (
                <button
                  key={t}
                  onClick={() => {
                    setTheme(t);
                    setAppTheme(t);
                  }}
                  className={cn(
                    "w-full relative border px-4 py-3 h-14 text-sm text-left",
                    theme === t
                      ? "border-primary bg-primary/10"
                      : "border-border dark:border-input",
                    idx === 0
                      ? "rounded-t-xl rounded-b-none!" // first button
                      : idx === arr.length - 1
                        ? "rounded-b-xl rounded-t-none!" // last button
                        : "rounded-none", // middle button
                  )}
                >
                  <span className="absolute bottom-2 left-2">
                    {t[0].toUpperCase() + t.slice(1)}
                  </span>
                </button>
              ))}
            </div>

            <Button
              className="mt-4"
              variant="input"
              onClick={() => setStep("done")}
            >
              Continue
            </Button>
          </Panel>
        )}

        {step === "done" && <TitleOnly title="You’re all set" />}
      </OnboardingPanel>
    </div>
  );
}
