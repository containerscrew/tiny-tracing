use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Mutex;

use crate::{errors::LoggerError, time::LocalTimer};
use tracing::Level;
use tracing_subscriber::{
    EnvFilter, Layer, Registry,
    filter::LevelFilter,
    fmt::{MakeWriter, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// Output format for log lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogFormat {
    /// Human-readable single-line text output.
    #[default]
    Text,
    /// One JSON object per log line.
    Json,
}

impl LogFormat {
    fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "json",
        }
    }
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for LogFormat {
    type Err = LoggerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            other => Err(LoggerError::InvalidFormat(other.to_string())),
        }
    }
}

/// Where log lines are written.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Output {
    /// Standard output only. This is the default.
    #[default]
    Stdout,
    /// The given file only. Created if missing, appended otherwise.
    File(PathBuf),
    /// Both standard output and the given file.
    ///
    /// ANSI colours are kept on stdout but stripped from the file, so the
    /// on-disk log stays free of escape codes.
    Both(PathBuf),
}

/// Builder for initializing the global tracing subscriber.
///
/// Uses a fluent builder pattern to configure log level, output format,
/// and optional environment-based filtering before calling [`init`](Logger::init).
///
/// # Examples
///
/// Minimal setup (text output at INFO level):
///
/// ```rust
/// use tiny_tracing::Logger;
///
/// Logger::new().init().unwrap();
/// tiny_tracing::info!("Ready");
/// ```
///
/// JSON output with environment filter:
///
/// ```rust
/// use tiny_tracing::{Logger, LogFormat, Level};
///
/// Logger::new()
///     .with_level(Level::DEBUG)
///     .with_format(LogFormat::Json)
///     .with_env_filter("info,my_crate=trace")
///     .init()
///     .unwrap();
/// ```
pub struct Logger {
    level: Level,
    format: LogFormat,
    env_filter: Option<String>,
    with_file: bool,
    with_target: bool,
    output: Output,
}

impl Logger {
    /// Creates a new [`Logger`] with default values.
    ///
    /// Defaults: [`Level::INFO`], [`LogFormat::Text`], no env filter,
    /// file location off, target on, output to stdout.
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: Level::INFO,
            format: LogFormat::Text,
            env_filter: None,
            with_file: false,
            with_target: true,
            output: Output::Stdout,
        }
    }

    /// Sets the global (default) log level.
    ///
    /// This is the base level applied to every target. Per-target directives
    /// passed to [`with_env_filter`](Self::with_env_filter) refine it on top —
    /// the level is never silently ignored. A global directive inside the env
    /// filter string (e.g. the `info` in `"info,my_crate=debug"`) does take
    /// precedence over this value.
    #[must_use]
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Sets the output format.
    #[must_use]
    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Adds environment-based, per-target filtering via [`EnvFilter`].
    ///
    /// The directives here are layered on top of the global
    /// [`with_level`](Logger::with_level) value. Supported syntax: `"info"`,
    /// `"info,my_crate=debug"`, etc.
    ///
    /// If not called, only the static [`with_level`](Logger::with_level) value applies.
    #[must_use]
    pub fn with_env_filter(mut self, filter: impl Into<String>) -> Self {
        self.env_filter = Some(filter.into());
        self
    }

    /// Controls whether the source file name appears in log lines.
    ///
    /// Disabled by default.
    #[must_use]
    pub fn with_file(mut self, enabled: bool) -> Self {
        self.with_file = enabled;
        self
    }

    /// Controls whether the module path (target) appears in log lines.
    ///
    /// Enabled by default.
    #[must_use]
    pub fn with_target(mut self, enabled: bool) -> Self {
        self.with_target = enabled;
        self
    }

    /// Sets where log lines are written: stdout, a file, or both.
    ///
    /// Defaults to [`Output::Stdout`]. When a file is involved it is opened in
    /// append mode (created if missing) and writes are synchronised, so the
    /// call stays panic-free — an unopenable path yields
    /// [`LoggerError::OpenLogFile`] from [`init`](Self::init).
    #[must_use]
    pub fn with_output(mut self, output: Output) -> Self {
        self.output = output;
        self
    }

    /// Returns the configured log level.
    #[must_use]
    pub fn level(&self) -> Level {
        self.level
    }

    /// Returns the configured output format.
    #[must_use]
    pub fn format(&self) -> LogFormat {
        self.format
    }

    /// Initializes the global tracing subscriber.
    ///
    /// Consumes the builder. Must be called only once per process;
    /// subsequent calls return [`LoggerError::TryInitError`].
    ///
    /// # Errors
    ///
    /// - [`LoggerError::InvalidEnvFilter`] if an env filter was set and
    ///   [`EnvFilter`] rejects it.
    /// - [`LoggerError::OpenLogFile`] if a file output was requested but the
    ///   path could not be opened for writing.
    /// - [`LoggerError::TryInitError`] if a global subscriber is already set.
    pub fn init(self) -> Result<(), LoggerError> {
        let filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::from_level(self.level).into())
            .parse(self.env_filter.as_deref().unwrap_or(""))
            .map_err(|e| LoggerError::InvalidEnvFilter(e.to_string()))?;

        let (to_stdout, file_path) = match &self.output {
            Output::Stdout => (true, None),
            Output::File(path) => (false, Some(path.clone())),
            Output::Both(path) => (true, Some(path.clone())),
        };

        let mut layers: Vec<Box<dyn Layer<Registry> + Send + Sync>> = Vec::new();

        if to_stdout {
            layers.push(self.fmt_layer(std::io::stdout, true));
        }

        if let Some(path) = file_path {
            let file = open_log_file(&path)?;
            layers.push(self.fmt_layer(Mutex::new(file), false));
        }

        tracing_subscriber::registry()
            .with(layers)
            .with(filter)
            .try_init()
            .map_err(|e| LoggerError::TryInitError(e.to_string()))
    }

    /// Builds a boxed `fmt` layer for the given writer, honouring the
    /// configured format and field flags. `ansi` toggles colour output —
    /// enabled for terminals, disabled for files.
    fn fmt_layer<W>(&self, writer: W, ansi: bool) -> Box<dyn Layer<Registry> + Send + Sync>
    where
        W: for<'writer> MakeWriter<'writer> + Send + Sync + 'static,
    {
        let layer = tracing_subscriber::fmt::layer()
            .with_writer(writer)
            .with_ansi(ansi)
            .with_thread_names(false)
            .with_span_events(FmtSpan::NONE)
            .with_file(self.with_file)
            .with_target(self.with_target)
            .with_timer(LocalTimer);

        match self.format {
            LogFormat::Json => layer.json().boxed(),
            LogFormat::Text => layer.boxed(),
        }
    }
}

/// Opens `path` in append mode (creating it if absent) for log output.
fn open_log_file(path: &Path) -> Result<File, LoggerError> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| LoggerError::OpenLogFile {
            path: path.display().to_string(),
            message: e.to_string(),
        })
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
