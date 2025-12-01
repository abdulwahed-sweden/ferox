// ferox-desktop/src/components/ui/DataTable.tsx
// Reusable data table component with sorting and styling

import { ReactNode, useState, useMemo } from "react";
import { ChevronUp, ChevronDown, ChevronsUpDown } from "lucide-react";

// Column definition
export interface Column<T> {
  key: keyof T | string;
  header: string;
  width?: string;
  align?: "left" | "center" | "right";
  sortable?: boolean;
  render?: (value: unknown, row: T, index: number) => ReactNode;
}

// Table props
interface DataTableProps<T> {
  columns: Column<T>[];
  data: T[];
  keyField?: keyof T;
  className?: string;
  striped?: boolean;
  hoverable?: boolean;
  compact?: boolean;
  emptyMessage?: string;
  onRowClick?: (row: T, index: number) => void;
}

type SortDirection = "asc" | "desc" | null;

export function DataTable<T extends Record<string, unknown>>({
  columns,
  data,
  keyField,
  className = "",
  striped = true,
  hoverable = true,
  compact = false,
  emptyMessage = "No data available",
  onRowClick,
}: DataTableProps<T>) {
  const [sortKey, setSortKey] = useState<string | null>(null);
  const [sortDirection, setSortDirection] = useState<SortDirection>(null);

  // Handle sort click
  const handleSort = (key: string) => {
    if (sortKey === key) {
      // Cycle through: asc -> desc -> null
      if (sortDirection === "asc") {
        setSortDirection("desc");
      } else if (sortDirection === "desc") {
        setSortDirection(null);
        setSortKey(null);
      } else {
        setSortDirection("asc");
      }
    } else {
      setSortKey(key);
      setSortDirection("asc");
    }
  };

  // Sorted data
  const sortedData = useMemo(() => {
    if (!sortKey || !sortDirection) {
      return data;
    }

    return [...data].sort((a, b) => {
      const aVal = a[sortKey];
      const bVal = b[sortKey];

      if (aVal === bVal) return 0;
      if (aVal === null || aVal === undefined) return 1;
      if (bVal === null || bVal === undefined) return -1;

      const comparison = aVal < bVal ? -1 : 1;
      return sortDirection === "asc" ? comparison : -comparison;
    });
  }, [data, sortKey, sortDirection]);

  // Get cell value
  const getCellValue = (row: T, column: Column<T>, index: number): ReactNode => {
    const value = row[column.key as keyof T];
    if (column.render) {
      return column.render(value, row, index);
    }
    return value as ReactNode;
  };

  // Get row key
  const getRowKey = (row: T, index: number): string | number => {
    if (keyField && row[keyField] !== undefined) {
      return String(row[keyField]);
    }
    return index;
  };

  const tableClasses = [
    "data-table",
    striped && "data-table-striped",
    hoverable && "data-table-hoverable",
    compact && "data-table-compact",
    className,
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <div className="data-table-container">
      <table className={tableClasses}>
        <thead>
          <tr>
            {columns.map((column) => (
              <th
                key={String(column.key)}
                style={{ width: column.width }}
                className={`
                  ${column.align === "center" ? "text-center" : ""}
                  ${column.align === "right" ? "text-right" : ""}
                  ${column.sortable ? "cursor-pointer select-none" : ""}
                `}
                onClick={
                  column.sortable
                    ? () => handleSort(String(column.key))
                    : undefined
                }
              >
                <div className="flex items-center gap-1">
                  <span>{column.header}</span>
                  {column.sortable && (
                    <span className="sort-indicator">
                      {sortKey === column.key ? (
                        sortDirection === "asc" ? (
                          <ChevronUp size={14} />
                        ) : sortDirection === "desc" ? (
                          <ChevronDown size={14} />
                        ) : (
                          <ChevronsUpDown size={14} className="opacity-40" />
                        )
                      ) : (
                        <ChevronsUpDown size={14} className="opacity-40" />
                      )}
                    </span>
                  )}
                </div>
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {sortedData.length === 0 ? (
            <tr>
              <td colSpan={columns.length} className="data-table-empty">
                {emptyMessage}
              </td>
            </tr>
          ) : (
            sortedData.map((row, index) => (
              <tr
                key={getRowKey(row, index)}
                onClick={onRowClick ? () => onRowClick(row, index) : undefined}
                className={onRowClick ? "cursor-pointer" : ""}
              >
                {columns.map((column) => (
                  <td
                    key={String(column.key)}
                    className={`
                      ${column.align === "center" ? "text-center" : ""}
                      ${column.align === "right" ? "text-right" : ""}
                    `}
                  >
                    {getCellValue(row, column, index)}
                  </td>
                ))}
              </tr>
            ))
          )}
        </tbody>
      </table>
    </div>
  );
}

// Simple key-value table for displaying object properties
interface KeyValueTableProps {
  data: Record<string, ReactNode>;
  className?: string;
  labelWidth?: string;
}

export function KeyValueTable({
  data,
  className = "",
  labelWidth = "140px",
}: KeyValueTableProps) {
  const entries = Object.entries(data);

  if (entries.length === 0) {
    return null;
  }

  return (
    <div className={`key-value-table ${className}`}>
      {entries.map(([key, value]) => (
        <div key={key} className="key-value-row">
          <div className="key-value-label" style={{ width: labelWidth }}>
            {key}
          </div>
          <div className="key-value-value">{value}</div>
        </div>
      ))}
    </div>
  );
}

// Stats table for displaying statistics in a grid
interface Stat {
  label: string;
  value: ReactNode;
  subtext?: string;
  trend?: "up" | "down" | "neutral";
}

interface StatsTableProps {
  stats: Stat[];
  columns?: 2 | 3 | 4;
  className?: string;
}

export function StatsTable({
  stats,
  columns = 3,
  className = "",
}: StatsTableProps) {
  const gridCols = {
    2: "grid-cols-2",
    3: "grid-cols-3",
    4: "grid-cols-4",
  };

  return (
    <div className={`stats-grid ${gridCols[columns]} ${className}`}>
      {stats.map((stat, index) => (
        <div key={index} className="stats-item">
          <div className="stats-value">
            {stat.value}
            {stat.trend && (
              <span
                className={`stats-trend ${
                  stat.trend === "up"
                    ? "stats-trend-up"
                    : stat.trend === "down"
                      ? "stats-trend-down"
                      : ""
                }`}
              >
                {stat.trend === "up" ? "↑" : stat.trend === "down" ? "↓" : "–"}
              </span>
            )}
          </div>
          <div className="stats-label">{stat.label}</div>
          {stat.subtext && <div className="stats-subtext">{stat.subtext}</div>}
        </div>
      ))}
    </div>
  );
}
