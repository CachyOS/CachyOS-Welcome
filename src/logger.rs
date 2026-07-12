use crate::utils;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

pub fn setup_logger() -> WorkerGuard {
    // set log level from RUST_LOG env var
    let env_filter = EnvFilter::try_from_default_env();

    // create subscriber env filter
    let subscriber_env_filter = env_filter.unwrap_or_else(|_| {
        EnvFilter::new(
            "debug,i18n_embed=warn,which=warn,zbus::proxy=warn,hickory_resolver=warn,\
             hickory_proto=warn,hickory_net=warn,rustls=warn,quinn=warn,quinn_proto=warn,\
             quinn_udp=warn,h2=warn",
        )
    });

    // create stdout layer
    let stdout_log = tracing_subscriber::fmt::layer().compact().with_writer(std::io::stdout);

    // create just a file appender, without rolling
    let file_appender = tracing_appender::rolling::never(
        utils::fix_path("~/.config/cachyos/cachyos-hello"),
        "cachyos-hello.log",
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_log =
        tracing_subscriber::fmt::layer().compact().with_ansi(false).with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(file_log)
        .with(stdout_log)
        .with(subscriber_env_filter)
        .init();

    _guard
}
