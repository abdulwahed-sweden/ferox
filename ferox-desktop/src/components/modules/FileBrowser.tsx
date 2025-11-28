/**
 * FileBrowser - Simulated File Browser
 * For demo/training purposes only - no real file system access
 */

import { useState, useCallback, useEffect } from "react";
import {
  Folder,
  File,
  Download,
  Upload,
  Trash2,
  RefreshCw,
  Home,
  ArrowUp,
  Eye,
  EyeOff,
  Lock,
} from "lucide-react";
import { clsx } from "clsx";
import toast from "react-hot-toast";
import { Spinner } from "../Loading";
import { simulateDirectoryListing } from "../../lib/tauri";
import type { SimulatedFileEntry, DirectoryListing } from "../../types";

interface FileBrowserProps {
  sessionId: string;
}

function formatSize(bytes: number): string {
  if (bytes === 0) return "-";
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}

export function FileBrowser({ sessionId }: FileBrowserProps) {
  const [currentPath, setCurrentPath] = useState("/home/user");
  const [listing, setListing] = useState<DirectoryListing | null>(null);
  const [files, setFiles] = useState<SimulatedFileEntry[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [showHidden, setShowHidden] = useState(false);

  const handleNavigate = useCallback(
    async (path: string) => {
      setLoading(true);
      setSelectedFile(null);
      try {
        const result = await simulateDirectoryListing(path, sessionId);
        setListing(result);
        setFiles(result.entries);
        setCurrentPath(result.path);
      } catch (error) {
        console.error("Failed to list directory:", error);
        toast.error("Failed to access directory");
      } finally {
        setLoading(false);
      }
    },
    [sessionId],
  );

  useEffect(() => {
    handleNavigate(currentPath);
  }, []);

  const handleGoUp = useCallback(() => {
    if (listing?.parent) {
      handleNavigate(listing.parent);
    }
  }, [listing, handleNavigate]);

  const handleRefresh = useCallback(() => {
    handleNavigate(currentPath);
  }, [currentPath, handleNavigate]);

  const handleDownload = useCallback(() => {
    if (selectedFile) {
      toast.success(`Simulated download: ${selectedFile}`);
    }
  }, [selectedFile]);

  const handleUpload = useCallback(() => {
    toast.success(`Simulated upload to: ${currentPath}`);
  }, [currentPath]);

  const handleDelete = useCallback(() => {
    if (selectedFile) {
      setFiles((prev) => prev.filter((f) => f.path !== selectedFile));
      setSelectedFile(null);
      toast.success("File deleted (simulated)");
    }
  }, [selectedFile]);

  const filteredFiles = showHidden ? files : files.filter((f) => !f.hidden);

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-3 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center gap-2">
          <Folder className="text-warning-text" size={18} />
          <h2 className="text-sm font-semibold text-text-primary">
            File Browser
          </h2>
          <span className="text-xs bg-warning-soft text-warning-text px-2 py-0.5 rounded">
            SIMULATION
          </span>
        </div>
      </div>

      {/* Toolbar */}
      <div className="flex items-center gap-2 p-2 bg-dark-800 border-b border-dark-600">
        <button
          onClick={() => handleNavigate("/home/user")}
          className="p-1.5 hover:bg-dark-600 rounded transition-colors"
          title="Home"
        >
          <Home size={16} />
        </button>
        <button
          onClick={handleGoUp}
          disabled={!listing?.parent}
          className="p-1.5 hover:bg-dark-600 rounded transition-colors disabled:opacity-50"
          title="Go up"
        >
          <ArrowUp size={16} />
        </button>
        <button
          onClick={handleRefresh}
          className="p-1.5 hover:bg-dark-600 rounded transition-colors"
          title="Refresh"
        >
          <RefreshCw size={16} className={loading ? "animate-spin" : ""} />
        </button>
        <button
          onClick={() => setShowHidden(!showHidden)}
          className={clsx(
            "p-1.5 rounded transition-colors",
            showHidden
              ? "bg-purple-soft text-purple-text"
              : "hover:bg-dark-600",
          )}
          title={showHidden ? "Hide hidden files" : "Show hidden files"}
        >
          {showHidden ? <Eye size={16} /> : <EyeOff size={16} />}
        </button>

        <div className="flex-1 px-2">
          <input
            type="text"
            value={currentPath}
            onChange={(e) => setCurrentPath(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleNavigate(currentPath)}
            className="w-full px-2 py-1 text-sm bg-dark-700 border border-dark-600 rounded text-text-primary font-mono"
          />
        </div>

        <div className="flex items-center gap-1 border-l border-dark-600 pl-2">
          <button
            onClick={handleDownload}
            disabled={!selectedFile}
            className="p-1.5 hover:bg-dark-600 rounded transition-colors disabled:opacity-50"
            title="Download"
          >
            <Download size={16} />
          </button>
          <button
            onClick={handleUpload}
            className="p-1.5 hover:bg-dark-600 rounded transition-colors"
            title="Upload"
          >
            <Upload size={16} />
          </button>
          <button
            onClick={handleDelete}
            disabled={!selectedFile}
            className="p-1.5 hover:bg-dark-600 rounded transition-colors text-danger disabled:opacity-50"
            title="Delete"
          >
            <Trash2 size={16} />
          </button>
        </div>
      </div>

      {/* File list */}
      <div className="flex-1 overflow-auto">
        {loading ? (
          <div className="flex items-center justify-center h-full">
            <Spinner size="lg" />
          </div>
        ) : (
          <table className="w-full text-sm">
            <thead className="sticky top-0 bg-dark-800 text-text-muted">
              <tr>
                <th className="text-left p-2 font-medium">Name</th>
                <th className="text-right p-2 font-medium w-24">Size</th>
                <th className="text-left p-2 font-medium w-40">Modified</th>
                <th className="text-left p-2 font-medium w-28">Permissions</th>
                <th className="text-left p-2 font-medium w-20">Owner</th>
              </tr>
            </thead>
            <tbody>
              {filteredFiles.map((file) => (
                <tr
                  key={file.path}
                  className={clsx(
                    "cursor-pointer hover:bg-dark-700 transition-colors",
                    selectedFile === file.path && "bg-dark-600",
                    file.hidden && "opacity-60",
                  )}
                  onClick={() => setSelectedFile(file.path)}
                  onDoubleClick={() =>
                    file.file_type === "directory" && handleNavigate(file.path)
                  }
                >
                  <td className="p-2">
                    <div className="flex items-center gap-2">
                      {file.file_type === "directory" ? (
                        <Folder size={16} className="text-warning-text" />
                      ) : (
                        <File size={16} className="text-text-muted" />
                      )}
                      <span
                        className={clsx(
                          "text-text-primary",
                          file.hidden && "italic",
                        )}
                      >
                        {file.name}
                      </span>
                      {file.permissions.includes("------") && (
                        <span title="Restricted">
                          <Lock size={12} className="text-danger-text" />
                        </span>
                      )}
                    </div>
                  </td>
                  <td className="p-2 text-right text-text-muted">
                    {formatSize(file.size)}
                  </td>
                  <td className="p-2 text-text-muted">
                    {new Date(file.modified).toLocaleDateString()}
                  </td>
                  <td className="p-2 text-text-muted font-mono text-xs">
                    {file.permissions}
                  </td>
                  <td className="p-2 text-text-muted">{file.owner}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {/* Status bar */}
      <div className="px-3 py-1.5 bg-dark-800 border-t border-dark-600 text-xs text-text-muted flex items-center justify-between">
        <span>
          {filteredFiles.length} items{" "}
          {listing && `| ${formatSize(listing.total_size)}`}
        </span>
        <span>Session: {sessionId.slice(0, 8)}...</span>
      </div>
    </div>
  );
}

export default FileBrowser;
