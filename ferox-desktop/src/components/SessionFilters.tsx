import { useAppStore } from "../store";
import { Filter, X } from "lucide-react";
import { clsx } from "clsx";

const STATUS_OPTIONS = [
  {
    value: "active",
    label: "Active",
    color: "bg-ferox-green/20 text-ferox-green border-ferox-green/50",
  },
  {
    value: "sleeping",
    label: "Sleep",
    color: "bg-warning/20 text-warning border-warning/50",
  },
  {
    value: "dead",
    label: "Dead",
    color: "bg-danger/20 text-danger border-danger/50",
  },
] as const;

const OS_OPTIONS = [
  { value: "windows", label: "Windows" },
  { value: "linux", label: "Linux" },
  { value: "macos", label: "macOS" },
] as const;

export function SessionFilters() {
  const { sessionFilters, setStatusFilter, setOsFilter, clearFilters } =
    useAppStore();
  const hasFilters =
    sessionFilters.status.length > 0 || sessionFilters.os.length > 0;

  const toggleStatus = (status: "active" | "sleeping" | "dead") => {
    const current = sessionFilters.status;
    if (current.includes(status)) {
      setStatusFilter(current.filter((s) => s !== status));
    } else {
      setStatusFilter([...current, status]);
    }
  };

  const toggleOs = (os: string) => {
    const current = sessionFilters.os;
    if (current.includes(os)) {
      setOsFilter(current.filter((o) => o !== os));
    } else {
      setOsFilter([...current, os]);
    }
  };

  return (
    <div className="px-3 py-2 border-b border-dark-600">
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-1.5 text-xs text-text-muted">
          <Filter size={12} />
          <span>Filters</span>
        </div>
        {hasFilters && (
          <button
            onClick={clearFilters}
            className="flex items-center gap-1 text-xs text-text-muted hover:text-text-primary transition-colors"
          >
            <X size={12} />
            Clear
          </button>
        )}
      </div>

      {/* Status filters */}
      <div className="flex flex-wrap gap-1 mb-2">
        {STATUS_OPTIONS.map((opt) => {
          const isActive = sessionFilters.status.includes(opt.value);
          return (
            <button
              key={opt.value}
              onClick={() => toggleStatus(opt.value)}
              className={clsx(
                "px-2 py-0.5 text-xs rounded border transition-colors",
                isActive
                  ? opt.color
                  : "bg-dark-700 text-text-muted border-dark-600 hover:border-dark-500",
              )}
            >
              {opt.label}
            </button>
          );
        })}
      </div>

      {/* OS filters */}
      <div className="flex flex-wrap gap-1">
        {OS_OPTIONS.map((opt) => {
          const isActive = sessionFilters.os.includes(opt.value);
          return (
            <button
              key={opt.value}
              onClick={() => toggleOs(opt.value)}
              className={clsx(
                "px-2 py-0.5 text-xs rounded border transition-colors",
                isActive
                  ? "bg-info/20 text-info border-info/50"
                  : "bg-dark-700 text-text-muted border-dark-600 hover:border-dark-500",
              )}
            >
              {opt.label}
            </button>
          );
        })}
      </div>
    </div>
  );
}
