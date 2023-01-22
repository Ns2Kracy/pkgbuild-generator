pub mod generator;

use crate::generator::{
    generate_casaos_app_management_package, generate_casaos_cli_package,
    generate_casaos_gateway_package, generate_casaos_install,
    generate_casaos_local_storage_package, generate_casaos_message_bus_package,
    generate_casaos_package, generate_casaos_ui_package, generate_casaos_user_service_package,
};
use tokio::fs::File;

#[tokio::main]
async fn main() {
    let casaos = File::create("./casaos/PKGBUILD").await.unwrap();
    let casaos_install = File::create("./casaos/casaos.install").await.unwrap();
    let casaos_app_management = File::create("./casaos-app-management/PKGBUILD")
        .await
        .unwrap();
    let casaos_local_storage = File::create("./casaos-local-storage/PKGBUILD")
        .await
        .unwrap();
    let casaos_user_service = File::create("./casaos-user-service/PKGBUILD")
        .await
        .unwrap();
    let casaos_message_bus = File::create("./casaos-message-bus/PKGBUILD").await.unwrap();
    let casaos_gateway = File::create("./casaos-gateway/PKGBUILD").await.unwrap();
    let casaos_cli = File::create("./casaos-cli/PKGBUILD").await.unwrap();
    let casaos_ui = File::create("./casaos-ui/PKGBUILD").await.unwrap();
    generate_casaos_package(casaos).await.unwrap();
    generate_casaos_install(casaos_install).await.unwrap();
    generate_casaos_app_management_package(casaos_app_management)
        .await
        .unwrap();
    generate_casaos_local_storage_package(casaos_local_storage)
        .await
        .unwrap();
    generate_casaos_user_service_package(casaos_user_service)
        .await
        .unwrap();
    generate_casaos_message_bus_package(casaos_message_bus)
        .await
        .unwrap();
    generate_casaos_gateway_package(casaos_gateway)
        .await
        .unwrap();
    generate_casaos_cli_package(casaos_cli).await.unwrap();
    generate_casaos_ui_package(casaos_ui).await.unwrap();
}
