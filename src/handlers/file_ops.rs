use anyhow::{Context, Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// File operations handler for upload/download and file management
#[derive(Clone, Debug)]
pub struct FileOperationsHandler {
    working_dir: PathBuf,
}

impl FileOperationsHandler {
    /// Create a new file operations handler
    pub fn new() -> Self {
        Self {
            working_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Create handler with specific working directory
    pub fn with_working_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
        let working_dir = path.as_ref().to_path_buf();
        if !working_dir.exists() {
            return Err(anyhow!("Working directory does not exist"));
        }
        Ok(Self { working_dir })
    }

    /// Upload a file from local system to target
    pub async fn upload<P: AsRef<Path>>(
        &self,
        local_path: P,
        remote_path: P,
    ) -> Result<FileTransferResult> {
        let local_path = local_path.as_ref();
        let remote_path = self.resolve_path(remote_path.as_ref())?;

        // Read the local file
        let mut file = File::open(local_path)
            .await
            .context(format!("Failed to open local file: {:?}", local_path))?;

        let metadata = file.metadata().await?;
        let file_size = metadata.len();

        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .await
            .context("Failed to read file contents")?;

        // Create parent directories if they don't exist
        if let Some(parent) = remote_path.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create parent directories")?;
        }

        // Write to remote location
        let mut remote_file = File::create(&remote_path)
            .await
            .context(format!("Failed to create remote file: {:?}", remote_path))?;

        remote_file
            .write_all(&contents)
            .await
            .context("Failed to write file contents")?;

        Ok(FileTransferResult {
            local_path: local_path.to_string_lossy().to_string(),
            remote_path: remote_path.to_string_lossy().to_string(),
            bytes_transferred: file_size,
            success: true,
        })
    }

    /// Download a file from target to local system
    pub async fn download<P: AsRef<Path>>(
        &self,
        remote_path: P,
        local_path: P,
    ) -> Result<FileTransferResult> {
        let remote_path = self.resolve_path(remote_path.as_ref())?;
        let local_path = local_path.as_ref();

        // Read the remote file
        let mut file = File::open(&remote_path)
            .await
            .context(format!("Failed to open remote file: {:?}", remote_path))?;

        let metadata = file.metadata().await?;
        let file_size = metadata.len();

        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .await
            .context("Failed to read file contents")?;

        // Create parent directories if they don't exist
        if let Some(parent) = local_path.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create parent directories")?;
        }

        // Write to local location
        let mut local_file = File::create(local_path)
            .await
            .context(format!("Failed to create local file: {:?}", local_path))?;

        local_file
            .write_all(&contents)
            .await
            .context("Failed to write file contents")?;

        Ok(FileTransferResult {
            local_path: local_path.to_string_lossy().to_string(),
            remote_path: remote_path.to_string_lossy().to_string(),
            bytes_transferred: file_size,
            success: true,
        })
    }

    /// Encode file contents to base64 (useful for text-based exfiltration)
    pub async fn encode_file_base64<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let path = self.resolve_path(path.as_ref())?;

        let mut file = File::open(&path)
            .await
            .context(format!("Failed to open file: {:?}", path))?;

        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .await
            .context("Failed to read file contents")?;

        Ok(general_purpose::STANDARD.encode(&contents))
    }

    /// Decode base64 and write to file
    pub async fn decode_file_base64<P: AsRef<Path>>(
        &self,
        base64_data: &str,
        output_path: P,
    ) -> Result<()> {
        let output_path = self.resolve_path(output_path.as_ref())?;

        let decoded = general_purpose::STANDARD
            .decode(base64_data)
            .context("Failed to decode base64 data")?;

        // Create parent directories if they don't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create parent directories")?;
        }

        let mut file = File::create(&output_path)
            .await
            .context(format!("Failed to create file: {:?}", output_path))?;

        file.write_all(&decoded)
            .await
            .context("Failed to write decoded data")?;

        Ok(())
    }

    /// List files in a directory
    pub async fn list_directory<P: AsRef<Path>>(&self, path: P) -> Result<Vec<FileInfo>> {
        let path = self.resolve_path(path.as_ref())?;

        let mut entries = fs::read_dir(&path)
            .await
            .context(format!("Failed to read directory: {:?}", path))?;

        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            let file_type = if metadata.is_dir() {
                FileType::Directory
            } else if metadata.is_symlink() {
                FileType::Symlink
            } else {
                FileType::File
            };

            files.push(FileInfo {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_string_lossy().to_string(),
                size: metadata.len(),
                file_type,
                modified: metadata.modified().ok(),
            });
        }

        files.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(files)
    }

    /// Create a directory
    pub async fn create_directory<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = self.resolve_path(path.as_ref())?;
        fs::create_dir_all(&path)
            .await
            .context(format!("Failed to create directory: {:?}", path))?;
        Ok(())
    }

    /// Delete a file
    pub async fn delete_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = self.resolve_path(path.as_ref())?;
        fs::remove_file(&path)
            .await
            .context(format!("Failed to delete file: {:?}", path))?;
        Ok(())
    }

    /// Delete a directory recursively
    pub async fn delete_directory<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = self.resolve_path(path.as_ref())?;
        fs::remove_dir_all(&path)
            .await
            .context(format!("Failed to delete directory: {:?}", path))?;
        Ok(())
    }

    /// Copy a file
    pub async fn copy_file<P: AsRef<Path>>(&self, src: P, dst: P) -> Result<()> {
        let src = self.resolve_path(src.as_ref())?;
        let dst = self.resolve_path(dst.as_ref())?;

        // Create parent directories if they don't exist
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create parent directories")?;
        }

        fs::copy(&src, &dst)
            .await
            .context(format!("Failed to copy file from {:?} to {:?}", src, dst))?;
        Ok(())
    }

    /// Move/rename a file
    pub async fn move_file<P: AsRef<Path>>(&self, src: P, dst: P) -> Result<()> {
        let src = self.resolve_path(src.as_ref())?;
        let dst = self.resolve_path(dst.as_ref())?;

        // Create parent directories if they don't exist
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create parent directories")?;
        }

        fs::rename(&src, &dst)
            .await
            .context(format!("Failed to move file from {:?} to {:?}", src, dst))?;
        Ok(())
    }

    /// Read file contents as string
    pub async fn read_file_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let path = self.resolve_path(path.as_ref())?;
        fs::read_to_string(&path)
            .await
            .context(format!("Failed to read file: {:?}", path))
    }

    /// Write string to file
    pub async fn write_file_string<P: AsRef<Path>>(&self, path: P, contents: &str) -> Result<()> {
        let path = self.resolve_path(path.as_ref())?;

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create parent directories")?;
        }

        fs::write(&path, contents)
            .await
            .context(format!("Failed to write file: {:?}", path))
    }

    /// Check if path exists
    pub async fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = match self.resolve_path(path.as_ref()) {
            Ok(p) => p,
            Err(_) => return false,
        };
        path.exists()
    }

    /// Get file metadata
    pub async fn get_metadata<P: AsRef<Path>>(&self, path: P) -> Result<FileInfo> {
        let path = self.resolve_path(path.as_ref())?;
        let metadata = fs::metadata(&path)
            .await
            .context(format!("Failed to get metadata: {:?}", path))?;

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.is_symlink() {
            FileType::Symlink
        } else {
            FileType::File
        };

        Ok(FileInfo {
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: path.to_string_lossy().to_string(),
            size: metadata.len(),
            file_type,
            modified: metadata.modified().ok(),
        })
    }

    /// Change working directory
    pub fn set_working_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            return Err(anyhow!("Directory does not exist"));
        }
        if !path.is_dir() {
            return Err(anyhow!("Path is not a directory"));
        }
        self.working_dir = path;
        Ok(())
    }

    /// Get current working directory
    pub fn get_working_dir(&self) -> &Path {
        &self.working_dir
    }

    /// Resolve a path relative to working directory
    fn resolve_path(&self, path: &Path) -> Result<PathBuf> {
        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            Ok(self.working_dir.join(path))
        }
    }
}

impl Default for FileOperationsHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// File transfer result
#[derive(Debug, Clone)]
pub struct FileTransferResult {
    pub local_path: String,
    pub remote_path: String,
    pub bytes_transferred: u64,
    pub success: bool,
}

/// File information
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub file_type: FileType,
    pub modified: Option<std::time::SystemTime>,
}

/// File type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[tokio::test]
    async fn test_file_ops_handler() {
        let handler = FileOperationsHandler::new();
        assert!(handler.get_working_dir().exists());
    }

    #[tokio::test]
    async fn test_list_directory() {
        let handler = FileOperationsHandler::new();
        let result = handler.list_directory(".").await;
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(!files.is_empty());
    }

    #[tokio::test]
    async fn test_base64_encoding() {
        let handler = FileOperationsHandler::new();

        // Create a temporary file
        let temp_path = std::env::temp_dir().join("ferox_test_encode.txt");
        let mut file = std::fs::File::create(&temp_path).unwrap();
        file.write_all(b"Hello, Ferox!").unwrap();
        drop(file);

        let encoded = handler.encode_file_base64(&temp_path).await;
        assert!(encoded.is_ok());

        // Cleanup
        std::fs::remove_file(temp_path).ok();
    }
}
