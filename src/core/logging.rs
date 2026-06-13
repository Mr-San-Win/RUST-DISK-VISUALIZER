use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling;
use directories::ProjectDirs;

pub fn init_logging() {
    let proj = ProjectDirs::from("com", "SAN", "diskviz");
    if let Some(p) = proj {
        let dir = p.data_dir();
        std::fs::create_dir_all(dir).ok();
        let file_appender = rolling::daily(dir, "diskviz.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        fmt()
            .with_env_filter(filter)
            .with_writer(non_blocking)
            .init();
    } else {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        fmt().with_env_filter(filter).init();
    }
}
