/// Cross-platform single instance enforcement for applications.
///
/// Uses Unix domain sockets on macOS/Linux and named mutexes on Windows
/// to ensure only one instance of an application runs at a time.
use anyhow::Result;
use std::path::PathBuf;

/// Error returned when another instance of the application is already running.
#[derive(Debug)]
pub struct AlreadyRunning;

impl std::fmt::Display for AlreadyRunning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Another instance is already running")
    }
}

impl std::error::Error for AlreadyRunning {}

/// A guard that enforces single-instance behavior for an application.
///
/// When acquired successfully, this struct holds a platform-specific lock
/// that prevents other instances from starting. The lock is released when
/// this struct is dropped.
pub struct SingleInstance {
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
    _listener: std::os::unix::net::UnixListener,
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
    _socket_path: PathBuf,
    #[cfg(target_os = "windows")]
    _mutex: WindowsMutexHandle,
}

#[cfg(target_os = "windows")]
struct WindowsMutexHandle {
    handle: windows::Win32::Foundation::HANDLE,
}

#[cfg(target_os = "windows")]
impl Drop for WindowsMutexHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = windows::Win32::Foundation::CloseHandle(self.handle);
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
fn socket_path(app_id: &str) -> PathBuf {
    let dir = std::env::var("XDG_RUNTIME_DIR")
        .or_else(|_| std::env::var("TMPDIR"))
        .unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(dir).join(format!("{}.sock", app_id))
}

impl SingleInstance {
    /// Attempt to acquire the single-instance lock for the given application ID.
    ///
    /// Returns `Ok(SingleInstance)` if this is the first instance, or
    /// `Err(AlreadyRunning)` if another instance already holds the lock.
    pub fn acquire(app_id: &str) -> std::result::Result<Self, AlreadyRunning> {
        Self::platform_acquire(app_id)
    }

    /// Register a callback to be invoked when another instance attempts to start
    /// and sends an activation message.
    ///
    /// On Unix platforms, this spawns a background thread that listens for
    /// incoming connections on the Unix domain socket.
    pub fn on_activate(&self, callback: Box<dyn Fn() + Send + 'static>) {
        self.platform_on_activate(callback);
    }
}

/// Send an activation message to an already-running instance of the application.
///
/// This is typically called after `SingleInstance::acquire` returns `Err(AlreadyRunning)`
/// to signal the existing instance to come to the foreground.
pub fn send_activate_to_existing(app_id: &str) -> Result<()> {
    platform_send_activate(app_id)
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
impl SingleInstance {
    fn platform_acquire(app_id: &str) -> std::result::Result<Self, AlreadyRunning> {
        use std::os::unix::net::UnixListener;

        let path = socket_path(app_id);

        if std::os::unix::net::UnixStream::connect(&path).is_ok() {
            return Err(AlreadyRunning);
        }

        let _ = std::fs::remove_file(&path);
        let listener = UnixListener::bind(&path).map_err(|_| AlreadyRunning)?;
        listener.set_nonblocking(true).ok();

        Ok(Self {
            _listener: listener,
            _socket_path: path,
        })
    }

    fn platform_on_activate(&self, callback: Box<dyn Fn() + Send + 'static>) {
        use std::io::Read;
        use std::os::unix::net::UnixListener;

        let listener = unsafe {
            use std::os::unix::io::{AsRawFd, FromRawFd};
            let fd = self._listener.as_raw_fd();
            let dup_fd = libc::dup(fd);
            if dup_fd < 0 {
                return;
            }
            UnixListener::from_raw_fd(dup_fd)
        };
        listener.set_nonblocking(false).ok();

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let mut buf = [0u8; 64];
                        if let Ok(n) = stream.read(&mut buf) {
                            if n > 0 && &buf[..n.min(8)] == b"activate" {
                                callback();
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
impl Drop for SingleInstance {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self._socket_path);
    }
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
fn platform_send_activate(app_id: &str) -> Result<()> {
    use std::io::Write;
    use std::os::unix::net::UnixStream;

    let path = socket_path(app_id);
    let mut stream = UnixStream::connect(&path)?;
    stream.write_all(b"activate")?;
    Ok(())
}

#[cfg(target_os = "windows")]
impl SingleInstance {
    fn platform_acquire(app_id: &str) -> std::result::Result<Self, AlreadyRunning> {
        use windows::Win32::Foundation::ERROR_ALREADY_EXISTS;
        use windows::Win32::Foundation::GetLastError;
        use windows::Win32::System::Threading::CreateMutexW;
        use windows::core::HSTRING;

        let name = HSTRING::from(format!("Global\\{}", app_id));
        unsafe {
            let handle = CreateMutexW(None, true, &name).map_err(|_| AlreadyRunning)?;
            if GetLastError() == ERROR_ALREADY_EXISTS {
                let _ = windows::Win32::Foundation::CloseHandle(handle);
                return Err(AlreadyRunning);
            }
            Ok(Self {
                _mutex: WindowsMutexHandle { handle },
            })
        }
    }

    fn platform_on_activate(&self, _callback: Box<dyn Fn() + Send + 'static>) {}
}

#[cfg(target_os = "windows")]
fn platform_send_activate(_app_id: &str) -> Result<()> {
    Ok(())
}
