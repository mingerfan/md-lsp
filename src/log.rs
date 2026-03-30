use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::Local;
use tracing::info;
use tracing_subscriber::fmt::format::Writer as FmtWriter;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;

const DEFAULT_MAX_LOG_FILE_BYTES: u64 = 10 * 1024 * 1024;
const DEFAULT_MAX_LOG_FILES: usize = 5;

#[derive(Clone, Copy, Debug, Default)]
struct LocalOffsetTimer;

impl FormatTime for LocalOffsetTimer {
    fn format_time(&self, w: &mut FmtWriter<'_>) -> std::fmt::Result {
        let now = Local::now();
        write!(w, "{}", now.format("%Y-%m-%dT%H:%M:%S%.6f UTC%:z"))
    }
}

struct RotatingFileWriter {
    path: PathBuf,
    file: Option<std::fs::File>,
    max_bytes: u64,
    max_files: usize,
}

impl RotatingFileWriter {
    fn new(path: PathBuf, max_bytes: u64, max_files: usize) -> io::Result<Self> {
        let file = Self::open_log_file(&path)?;
        Ok(Self {
            path,
            file: Some(file),
            max_bytes,
            max_files,
        })
    }

    fn open_log_file(path: &Path) -> io::Result<std::fs::File> {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
    }

    fn backup_path(path: &Path, index: usize) -> PathBuf {
        let mut backup = path.as_os_str().to_os_string();
        backup.push(format!(".{index}"));
        PathBuf::from(backup)
    }

    fn file_mut(&mut self) -> io::Result<&mut std::fs::File> {
        self.file
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "log file is not open"))
    }

    fn rotate_if_needed(&mut self, incoming_len: usize) -> io::Result<()> {
        if self.max_bytes == 0 {
            return Ok(());
        }

        let current_size = std::fs::metadata(&self.path).map(|meta| meta.len()).unwrap_or(0);
        if current_size.saturating_add(incoming_len as u64) <= self.max_bytes {
            return Ok(());
        }

        if let Some(file) = self.file.as_mut() {
            file.flush()?;
        }
        self.file.take();

        if self.max_files == 0 {
            match std::fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(err) if err.kind() == io::ErrorKind::NotFound => {}
                Err(err) => return Err(err),
            }
        } else {
            let oldest = Self::backup_path(&self.path, self.max_files);
            match std::fs::remove_file(oldest) {
                Ok(()) => {}
                Err(err) if err.kind() == io::ErrorKind::NotFound => {}
                Err(err) => return Err(err),
            }

            for i in (1..self.max_files).rev() {
                let src = Self::backup_path(&self.path, i);
                let dst = Self::backup_path(&self.path, i + 1);
                if src.exists() {
                    std::fs::rename(src, dst)?;
                }
            }

            if self.path.exists() {
                std::fs::rename(&self.path, Self::backup_path(&self.path, 1))?;
            }
        }

        self.file = Some(Self::open_log_file(&self.path)?);
        Ok(())
    }
}

impl Write for RotatingFileWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.rotate_if_needed(buf.len())?;
        self.file_mut()?.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file_mut()?.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.rotate_if_needed(buf.len())?;
        self.file_mut()?.write_all(buf)
    }
}

fn read_env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(default)
}

fn read_env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default)
}

pub fn init_logging() {
    // 优先读取环境变量 MD_LSP_LOG_PATH，未设置时回退到项目根目录的 target/md-lsp.log
    let log_file_path = std::env::var_os("MD_LSP_LOG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join("md-lsp.log")
        });
    let max_log_file_bytes = read_env_u64("MD_LSP_LOG_MAX_BYTES", DEFAULT_MAX_LOG_FILE_BYTES);
    let max_log_files = read_env_usize("MD_LSP_LOG_MAX_FILES", DEFAULT_MAX_LOG_FILES);

    if let Some(parent) = log_file_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let rotating_writer = RotatingFileWriter::new(
        log_file_path.clone(),
        max_log_file_bytes,
        max_log_files,
    )
    .unwrap();
    let make_writer = std::io::stderr.and(Mutex::new(rotating_writer));

    if tracing_subscriber::fmt()
        .with_writer(make_writer)
        .with_timer(LocalOffsetTimer)
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .try_init()
        .is_ok()
    {
        info!(
            log_file = %log_file_path.display(),
            max_bytes = max_log_file_bytes,
            max_files = max_log_files,
            "logging initialized"
        );
    }
}