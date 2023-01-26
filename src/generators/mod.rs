pub mod casaos;
pub mod casaos_app_management;
pub mod casaos_cli;
pub mod casaos_gateway;
pub mod casaos_local_storage;
pub mod casaos_message_bus;
pub mod casaos_ui;
pub mod casaos_user_service;

use self::{
    casaos::generate_casaos_install, casaos::generate_casaos_package,
    casaos_app_management::generate_casaos_app_management_install,
    casaos_app_management::generate_casaos_app_management_package,
    casaos_cli::generate_casaos_cli_package, casaos_gateway::generate_casaos_gateway_install,
    casaos_gateway::generate_casaos_gateway_package,
    casaos_local_storage::generate_casaos_local_storage_install,
    casaos_local_storage::generate_casaos_local_storage_package,
    casaos_message_bus::generate_casaos_message_bus_install,
    casaos_message_bus::generate_casaos_message_bus_package, casaos_ui::generate_casaos_ui_install,
    casaos_ui::generate_casaos_ui_package,
    casaos_user_service::generate_casaos_user_service_install,
    casaos_user_service::generate_casaos_user_service_package,
};
pub use crate::consts::*;
pub use crate::utils::*;

pub async fn generate_packages() -> Result<(), Box<dyn std::error::Error>> {
    create_dirs().await.unwrap();
    let package_outputs = output_package().await;
    let install_outputs = output_install().await;

    // casaos
    generate_casaos_package(package_outputs[0].try_clone().await.unwrap())
        .await
        .unwrap();
    generate_casaos_install(install_outputs[0].try_clone().await.unwrap())
        .await
        .unwrap();

    // casaos-app-management
    generate_casaos_app_management_package(package_outputs[1].try_clone().await.unwrap())
        .await
        .unwrap();
    generate_casaos_app_management_install(install_outputs[1].try_clone().await.unwrap())
        .await
        .unwrap();

    // casaos_local_storage
    generate_casaos_local_storage_package(package_outputs[2].try_clone().await.unwrap())
        .await
        .unwrap();
    generate_casaos_local_storage_install(install_outputs[2].try_clone().await.unwrap())
        .await
        .unwrap();

    // casaos_user_service
    generate_casaos_user_service_package(package_outputs[3].try_clone().await.unwrap())
        .await
        .unwrap();
    generate_casaos_user_service_install(install_outputs[3].try_clone().await.unwrap())
        .await
        .unwrap();

    // casaos_message_bus
    generate_casaos_message_bus_package(package_outputs[4].try_clone().await.unwrap())
        .await
        .unwrap();
    generate_casaos_message_bus_install(install_outputs[4].try_clone().await.unwrap())
        .await
        .unwrap();

    // casaos_gateway
    generate_casaos_gateway_package(package_outputs[5].try_clone().await.unwrap())
        .await
        .unwrap();
    generate_casaos_gateway_install(install_outputs[5].try_clone().await.unwrap())
        .await
        .unwrap();

    // casaos_cli
    generate_casaos_cli_package(package_outputs[6].try_clone().await.unwrap())
        .await
        .unwrap();

    // casaos_ui
    generate_casaos_ui_package(package_outputs[7].try_clone().await.unwrap())
        .await
        .unwrap();
    generate_casaos_ui_install(install_outputs[6].try_clone().await.unwrap())
        .await
        .unwrap();

    make_packages().await;
    Ok(())
}

async fn make_packages() {
    let packages = vec![
        "./build/casaos/PKGBUILD",
        "./build/casaos-app-management/PKGBUILD",
        "./build/casaos-local-storage/PKGBUILD",
        "./build/casaos-user-service/PKGBUILD",
        "./build/casaos-message-bus/PKGBUILD",
        "./build/casaos-gateway/PKGBUILD",
        "./build/casaos-cli/PKGBUILD",
        "./build/casaos-ui/PKGBUILD",
    ];
    for package in packages {
        tokio::process::Command::new("makepkg")
            .arg("-f")
            .arg("-c")
            .arg("-p")
            .arg("--printsrcinfo > .SRCINFO")
            .current_dir(package)
            .output()
            .await
            .expect("failed to execute process");
    }
}
