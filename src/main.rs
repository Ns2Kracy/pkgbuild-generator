pub mod generator;

use generator::CasaOSPackage;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    init_logger();
    let package_builder = CasaOSPackage::new();
    package_builder.add_package("casaos").await.unwrap();
    package_builder
        .add_package("casaos-app-management")
        .await
        .unwrap();
    package_builder
        .add_package("casaos-local-storage")
        .await
        .unwrap();
    package_builder
        .add_package("casaos-user-service")
        .await
        .unwrap();
    package_builder
        .add_package("casaos-message-bus")
        .await
        .unwrap();
    package_builder.add_package("casaos-gateway").await.unwrap();
    package_builder.add_package("casaos-cli").await.unwrap();
    package_builder.add_package("casaos-ui").await.unwrap();
}

fn init_logger() {
    let appender = tracing_appender::rolling::never("logs", "build.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
