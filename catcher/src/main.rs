use color_eyre::config::Theme;
use console_subscriber::ConsoleLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::config::HookBuilder::new()
        .display_location_section(true)
        .display_env_section(true)
        .capture_span_trace_by_default(true)
        .theme(Theme::dark())
        .install()?;

    let console_layer = ConsoleLayer::builder()
        .server_addr(([0, 0, 0, 0], 6660))
        .spawn();
    tracing_subscriber::registry()
        .with(console_layer)
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_file(true)
                .with_ansi(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .pretty(),
        )
        .init();

    basanos_catcher::execute().await
}
