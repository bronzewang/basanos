#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    basanos_catcher::parse().await
}
