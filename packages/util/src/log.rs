use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn set_up_logging(filename: &str) -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("log", filename);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Layer::new().with_writer(std::io::stdout).pretty())
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking)
                .with_file(true)
                .with_line_number(true),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");

    guard
}
