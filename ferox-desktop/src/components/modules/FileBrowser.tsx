import { useState, useCallback } from 'react';
import {
  Folder,
  File,
  Download,
  Upload,
  Trash2,
  RefreshCw,
  Home,
  ArrowUp,
} from 'lucide-react';
import { clsx } from 'clsx';
import { Spinner } from '../Loading';

interface FileEntry {
  name: string;
  path: string;
  type: 'file' | 'directory';
  size: number;
  modified: string;
  permissions: string;
}

interface FileBrowserProps {
  sessionId: string;
}

// Mock data for demonstration
const mockFiles: FileEntry[] = [
  { name: 'Documents', path: '/home/user/Documents', type: 'directory', size: 0, modified: '2024-01-15', permissions: 'drwxr-xr-x' },
  { name: 'Downloads', path: '/home/user/Downloads', type: 'directory', size: 0, modified: '2024-01-14', permissions: 'drwxr-xr-x' },
  { name: '.bashrc', path: '/home/user/.bashrc', type: 'file', size: 3526, modified: '2024-01-10', permissions: '-rw-r--r--' },
  { name: '.ssh', path: '/home/user/.ssh', type: 'directory', size: 0, modified: '2024-01-08', permissions: 'drwx------' },
  { name: 'notes.txt', path: '/home/user/notes.txt', type: 'file', size: 1024, modified: '2024-01-12', permissions: '-rw-r--r--' },
];

function formatSize(bytes: number): string {
  if (bytes === 0) return '-';
  const units = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}

export function FileBrowser({ sessionId }: FileBrowserProps) {
  const [currentPath, setCurrentPath] = useState('/home/user');
  const [files, setFiles] = useState<FileEntry[]>(mockFiles);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleNavigate = useCallback((path: string) => {
    setLoading(true);
    setCurrentPath(path);
    // Simulate API call
    setTimeout(() => {
      setFiles(mockFiles); // Would fetch from backend
      setLoading(false);
    }, 500);
  }, []);

  const handleGoUp = useCallback(() => {
    const parentPath = currentPath.split('/').slice(0, -1).join('/') || '/';
    handleNavigate(parentPath);
  }, [currentPath, handleNavigate]);

  const handleRefresh = useCallback(() => {
    handleNavigate(currentPath);
  }, [currentPath, handleNavigate]);

  const handleDownload = useCallback(() => {
    if (selectedFile) {
      console.log('Download:', selectedFile);
      // Would trigger file download via Tauri
    }
  }, [selectedFile]);

  const handleUpload = useCallback(() => {
    console.log('Upload to:', currentPath);
    // Would open file picker via Tauri
  }, [currentPath]);

  const handleDelete = useCallback(() => {
    if (selectedFile) {
      console.log('Delete:', selectedFile);
      // Would confirm and delete via Tauri
    }
  }, [selectedFile]);

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Toolbar */}
      <div className="flex items-center gap-2 p-2 bg-dark-800 border-b border-dark-600">
        <button
          onClick={() => handleNavigate('/home/user')}
          className="p-1.5 hover:bg-dark-600 rounded transition-colors"
          title="Home"
        >
          <Home size={16} />
        </button>
        <button
          onClick={handleGoUp}
          className="p-1.5 hover:bg-dark-600 rounded transition-colors"
          title="Go up"
        >
          <ArrowUp size={16} />
        </button>
        <button
          onClick={handleRefresh}
          className="p-1.5 hover:bg-dark-600 rounded transition-colors"
          title="Refresh"
        >
          <RefreshCw size={16} />
        </button>

        <div className="flex-1 px-2">
          <input
            type="text"
            value={currentPath}
            onChange={(e) => setCurrentPath(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleNavigate(currentPath)}
            className="w-full px-2 py-1 text-sm bg-dark-700 border border-dark-600 rounded text-text-primary"
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
                <th className="text-left p-2 font-medium w-28">Modified</th>
                <th className="text-left p-2 font-medium w-24">Permissions</th>
              </tr>
            </thead>
            <tbody>
              {files.map((file) => (
                <tr
                  key={file.path}
                  className={clsx(
                    'cursor-pointer hover:bg-dark-700 transition-colors',
                    selectedFile === file.path && 'bg-dark-600'
                  )}
                  onClick={() => setSelectedFile(file.path)}
                  onDoubleClick={() => file.type === 'directory' && handleNavigate(file.path)}
                >
                  <td className="p-2">
                    <div className="flex items-center gap-2">
                      {file.type === 'directory' ? (
                        <Folder size={16} className="text-warning" />
                      ) : (
                        <File size={16} className="text-text-muted" />
                      )}
                      <span className="text-text-primary">{file.name}</span>
                    </div>
                  </td>
                  <td className="p-2 text-right text-text-muted">{formatSize(file.size)}</td>
                  <td className="p-2 text-text-muted">{file.modified}</td>
                  <td className="p-2 text-text-muted font-mono text-xs">{file.permissions}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {/* Status bar */}
      <div className="px-3 py-1.5 bg-dark-800 border-t border-dark-600 text-xs text-text-muted">
        {files.length} items | Session: {sessionId.slice(0, 8)}...
      </div>
    </div>
  );
}
