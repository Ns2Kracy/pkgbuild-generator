pub mod generator;

use generator::CasaOSPackage;

#[tokio::main]
async fn main() {
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