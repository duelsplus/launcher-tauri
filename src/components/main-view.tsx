export function MainView({ className }: { className?: string }) {
  return (
    <div
      className={`h-full overflow-auto ${className || ""}`}
    >
      <h1>test</h1>
    </div>
  );
}
