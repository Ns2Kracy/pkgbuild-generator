use lazy_static::lazy_static;
use tokio::io::AsyncWriteExt;
use tracing::{error, info};

const VERSION: &str = "0.4.1";
const MAINTAINER: &str = "Maintainer: Ns2Kracy <2220496937@qq.com>";
const X86_64_SOURCE: &str =
    "{url}/releases/download/v{pkgver}/linux-amd64-{pkgname}-v{pkgver}.tar.gz";
const AARCH64_SOURCE: &str =
    "{url}/releases/download/v{pkgver}/linux-arm64-{pkgname}-v{pkgver}.tar.gz";
const ARMV7H_SOURCE: &str =
    "{url}/releases/download/v{pkgver}/linux-armv7-{pkgname}-v{pkgver}.tar.gz";
const SOURCE: &str = "${url}/releases/download/v${pkgver}/linux-all-casaos-v${pkgver}.tar.gz";
const INSTALL: &str = "${pkgname}.install";

lazy_static! {
    pub static ref ARCH: Vec<&'static str> = vec!["amd64", "aarch64", "armv7h"];
    pub static ref URL: Vec<&'static str> = vec![
        "https://github.com/IceWhaleTech/CasaOS",
        "https://github.com/IceWhaleTech/CasaOS-AppManagement",
        "https://github.com/IceWhaleTech/CasaOS-LocalStorage",
        "https://github.com/IceWhaleTech/CasaOS-UserService",
        "https://github.com/IceWhaleTech/CasaOS-MessageBus",
        "https://github.com/IceWhaleTech/CasaOS-Gateway",
        "https://github.com/IceWhaleTech/CasaOS-CLI",
        "https://github.com/IceWhaleTech/CasaOS-UI",
    ];
    pub static ref PACKAGE_NAME: Vec<&'static str> = vec![
        "casaos",
        "casaos-app-management",
        "casaos-local-storage",
        "casaos-user-service",
        "casaos-message-bus",
        "casaos-gateway",
        "casaos-cli",
        "casaos-ui",
    ];
}

pub struct CasaOSPackage;

impl CasaOSPackage {
    pub fn new() -> Self {
        Self
    }

    pub async fn add_package(&self, package_name: &str) -> anyhow::Result<()> {
        let outputs = output_package().await;
        let casaos_install = tokio::fs::File::create("./build/casaos/casaos.install")
            .await
            .unwrap();

        match package_name {
            "casaos" => {
                info!("Generating casaos package dir");
                create_package_dir(PACKAGE_NAME[0]).await.unwrap();
                info!("Generating casaos package");
                generate_casaos_package(outputs[0].try_clone().await.unwrap())
                    .await
                    .unwrap();
                info!("Generating casaos.install");
                generate_casaos_install(casaos_install).await.unwrap();
            }
            "casaos-app-management" => {
                info!("Generating casaos-app-management package dir");
                create_package_dir(PACKAGE_NAME[1]).await.unwrap();
                info!("Generating casaos-app-management package");
                generate_casaos_app_management_package(outputs[1].try_clone().await.unwrap())
                    .await
                    .unwrap();
            }
            "casaos-local-storage" => {
                info!("Generating casaos-local-storage package dir");
                create_package_dir(PACKAGE_NAME[2]).await.unwrap();
                info!("Generating casaos-local-storage package");
                generate_casaos_local_storage_package(outputs[2].try_clone().await.unwrap())
                    .await
                    .unwrap();
            }
            "casaos-user-service" => {
                info!("Generating casaos-user-service package dir");
                create_package_dir(PACKAGE_NAME[3]).await.unwrap();
                info!("Generating casaos-user-service package");
                generate_casaos_user_service_package(outputs[3].try_clone().await.unwrap())
                    .await
                    .unwrap();
            }
            "casaos-message-bus" => {
                info!("Generating casaos-message-bus package dir");
                create_package_dir(PACKAGE_NAME[4]).await.unwrap();
                info!("Generating casaos-message-bus package");
                generate_casaos_message_bus_package(outputs[4].try_clone().await.unwrap())
                    .await
                    .unwrap();
            }
            "casaos-gateway" => {
                info!("Generating casaos-gateway package dir");
                create_package_dir(PACKAGE_NAME[5]).await.unwrap();
                info!("Generating casaos-gateway package");
                generate_casaos_gateway_package(outputs[5].try_clone().await.unwrap())
                    .await
                    .unwrap();
            }
            "casaos-cli" => {
                info!("Generating casaos-cli package dir");
                create_package_dir(PACKAGE_NAME[6]).await.unwrap();
                info!("Generating casaos-cli package");
                generate_casaos_cli_package(outputs[6].try_clone().await.unwrap())
                    .await
                    .unwrap();
            }
            "casaos-ui" => {
                info!("Generating casaos-ui package dir");
                create_package_dir(PACKAGE_NAME[7]).await.unwrap();
                info!("Generating casaos-ui package");
                generate_casaos_ui_package(outputs[7].try_clone().await.unwrap())
                    .await
                    .unwrap();
            }
            _ => {}
        }
        Ok(())
    }
}

/// 生成casaos的install文件
/// 用于执行一些PKGBUILD不能执行的脚本
pub async fn generate_casaos_install(mut output: tokio::fs::File) -> anyhow::Result<()> {
    // pre_install
    // 安装前执行的脚本
    let pre_install = r#"
pre_install() {
}
    "#;

    // post_install
    // 安装后执行的脚本
    let post_install = r#"
post_install() {
    systemctl enable --now casaos.service
    systemctl enable --now casaos-app-management.service
    systemctl enable --now casaos-gateway.service
    systemctl enable --now casaos-local-storage.service
    systemctl enable --now casaos-message-bus.service
    systemctl enable --now casaos-user-service.service
}    
    "#;

    // pre_upgrade
    // 更新前执行的脚本
    let pre_upgrade = r#"
pre_upgrade() {
}
    "#;

    // post_upgrade
    // 更新后执行的脚本
    let post_update = r#"
post_upgrade() {
    systemctl restart casaos.service
    systemctl restart casaos-app-management.service
    systemctl restart casaos-gateway.service
    systemctl restart casaos-local-storage.service
    systemctl restart casaos-message-bus.service
    systemctl restart casaos-user-service.service    
}
    "#;

    // pre_remove
    // 卸载前执行的脚本
    let pre_remove = r#"
pre_remove() {
    systemctl disable --now casaos.service
    systemctl disable --now casaos-app-management.service
    systemctl disable --now casaos-gateway.service
    systemctl disable --now casaos-local-storage.service
    systemctl disable --now casaos-message-bus.service
    systemctl disable --now casaos-user-service.service
}
    "#;

    // post_remove
    // 卸载后执行的脚本
    let post_remove = r#"
post_remove() {
}
    "#;
    output.write_all(pre_install.as_bytes()).await?;
    output.write_all(post_install.as_bytes()).await?;
    output.write_all(pre_upgrade.as_bytes()).await?;
    output.write_all(post_update.as_bytes()).await?;
    output.write_all(pre_remove.as_bytes()).await?;
    output.write_all(post_remove.as_bytes()).await?;
    info!("Generated casaos install file");
    Ok(())
}

/// Generate the PKGBUILD file for casaos.
/// Src: https://github.com/IceWhaleTech/CasaOS
/// Output: /casaos/PKGBUILD
pub async fn generate_casaos_package(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let checksums_x86_64 = get_x86_64_checksums().await;
    let checksums_aarch64 = get_aarch64_checksums().await;
    let checksums_armv7h = get_armv7h_checksums().await;
    let content = format!(
        "
pkgname=casaos
pkgver={}
pkgrel=1
pkgdesc=\"CasaOS is a lightweight operating system for IoT devices.\"
arch=('x86_64' 'aarch64' 'armv7h')
url={}
license=('APACHE')
install={}
backup=('etc/casaos/casaos.conf')
    ",
        VERSION.clone(),
        URL[0],
        "{pkgname}.install"
    );

    let source_content = format!(
        "
source_x86_64=('{:?}')
source_aarch64=('{:?}')
source_armv7h=('{:?}')
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );

    let chechsums_content = format!(
        "
sha256sums_x86_64=({})
sha256sums_aarch64=({})
sha256sums_armv7h=({})
    ",
        checksums_x86_64[0].clone(),
        checksums_aarch64[0].clone(),
        checksums_armv7h[0].clone()
    );

    let package_content = r#"
package() {
    _sysdir="${srcdir}/build/sysroot"
    _name="${pkgname#*-}"
    install -Dm755 "${_sysdir}/usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
    install -Dm755 "${_sysdir}/usr/bin/${pkgname}-migration-tool" "${pkgdir}/usr/bin/${pkgname}-migration-tool"
    install -Dm644 "${_sysdir}/etc/casaos/${_name}.conf.sample" "${pkgdir}/etc/casaos/${_name}.conf"
    install -Dm644 "${_sysdir}/usr/lib/systemd/system/${pkgname}.service" "${pkgdir}/usr/lib/systemd/system/${pkgname}.service"
}
    "#;

    output.write_all(content.as_bytes()).await?;
    output.write_all(source_content.as_bytes()).await?;
    output.write_all(chechsums_content.as_bytes()).await?;
    output.write_all(package_content.as_bytes()).await?;
    match output.sync_all().await {
        Ok(_) => {
            info!("Generated casaos local storage file");
            Ok(())
        }
        Err(e) => {
            error!("Failed to generate casaos local storage file");
            Err(anyhow::Error::new(e))
        }
    }
}

/// Generate the PKGBUILD file for casaos-app-management.
/// Src: https://github.com/IceWhaleTech/CasaOS-AppManagement
/// Output: /casaos-app-management/PKGBUILD
pub async fn generate_casaos_app_management_package(
    mut output: tokio::fs::File,
) -> anyhow::Result<()> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    match output.sync_all().await {
        Ok(_) => {
            info!("Generated casaos local storage file");
            Ok(())
        }
        Err(e) => {
            error!("Failed to generate casaos local storage file");
            Err(anyhow::Error::new(e))
        }
    }
}

/// Generate the PKGBUILD file for casaos-local-storage.
/// Src: https://github.com/IceWhaleTech/CasaOS-LocalStorage
/// Output: /casaos-local-storage/PKGBUILD
pub async fn generate_casaos_local_storage_package(
    mut output: tokio::fs::File,
) -> anyhow::Result<()> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;

    match output.sync_all().await {
        Ok(_) => {
            info!("Generated casaos local storage file");
            Ok(())
        }
        Err(e) => {
            error!("Failed to generate casaos local storage file");
            Err(anyhow::Error::new(e))
        }
    }
}

/// Generate the PKGBUILD file for casaos-user-service.
/// Src: https://github.com/IceWhaleTech/CasaOS-UserService
/// Output: /casaos-user-service/PKGBUILD
pub async fn generate_casaos_user_service_package(
    mut output: tokio::fs::File,
) -> anyhow::Result<()> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    match output.sync_all().await {
        Ok(_) => {
            info!("Generated casaos local storage file");
            Ok(())
        }
        Err(e) => {
            error!("Failed to generate casaos local storage file");
            Err(anyhow::Error::new(e))
        }
    }
}

/// Generate the PKGBUILD file for casaos-message-bus.
/// Src: https://github.com/IceWhaleTech/CasaOS-MessageBus
/// Output: /casaos-message-bus/PKGBUILD
pub async fn generate_casaos_message_bus_package(
    mut output: tokio::fs::File,
) -> anyhow::Result<()> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    match output.sync_all().await {
        Ok(_) => {
            info!("Generated casaos local storage file");
            Ok(())
        }
        Err(e) => {
            error!("Failed to generate casaos local storage file");
            Err(anyhow::Error::new(e))
        }
    }
}

/// Generate the PKGBUILD file for casaos-gateway.
/// Src: https://github.com/IceWhaleTech/CasaOS-Gateway
/// Output: /casaos-gateway/PKGBUILD
pub async fn generate_casaos_gateway_package(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    match output.sync_all().await {
        Ok(_) => {
            info!("Generated casaos local storage file");
            Ok(())
        }
        Err(e) => {
            error!("Failed to generate casaos local storage file");
            Err(anyhow::Error::new(e))
        }
    }
}

/// Generate the PKGBUILD file for casaos-cli.
/// Src: https://github.com/IceWhaleTech/CasaOS-CLI
/// Output: /casaos-cli/PKGBUILD
pub async fn generate_casaos_cli_package(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    match output.sync_all().await {
        Ok(_) => {
            info!("Generated casaos local storage file");
            Ok(())
        }
        Err(e) => {
            error!("Failed to generate casaos local storage file");
            Err(anyhow::Error::new(e))
        }
    }
}

/// Generate the PKGBUILD file for casaos-ui.
/// Src: https://github.com/IceWhaleTech/CasaOS-UI
/// Output: /casaos-ui/PKGBUILD
pub async fn generate_casaos_ui_package(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let content = format!(
        "# {}
pkgname=casaos-ui
pkgver={}
pkgrel=1
pkgdesc=\"The front-end of CasaOS,build with VueJS.\"
arch=('any')
url=\"{}\"
license=('unknown')
install=\"{}\"
source=({})
sha256sums=(SKIP)
    ",
        MAINTAINER,
        VERSION.clone(),
        URL[7].clone(),
        INSTALL,
        SOURCE
    );
    let package = r#"
package() {
    _sysdir="${srcdir}/build/sysroot"
    mkdir -p "${pkgdir}/var/lib/casaos"
    mv "${_sysdir}/var/lib/casaos/www" "${pkgdir}/var/lib/casaos/"
}
    "#;
    output.write(&content.as_bytes()).await?;
    output.write(&package.as_bytes()).await?;
    match output.sync_all().await {
        Ok(_) => {
            info!("Generated casaos local storage file");
            Ok(())
        }
        Err(e) => {
            error!("Failed to generate casaos local storage file");
            Err(anyhow::Error::new(e))
        }
    }
}

/// Generate the install file for casaos-ui.
/// mainly used for removing the /var/lib/casaos directory.
pub async fn generate_casaos_ui_install(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let post_remove = r#"#!/bin/bash
post_remove() {
    rm -rf /var/lib/casaos
}
    "#;
    output.write(&post_remove.as_bytes()).await?;
    match output.sync_all().await {
        Ok(_) => {
            info!("Generated casaos local storage file");
            Ok(())
        }
        Err(e) => {
            error!("Failed to generate casaos local storage file");
            Err(anyhow::Error::new(e))
        }
    }
}

/// Get x86_64 checksums from GitHub release page.
pub async fn get_x86_64_checksums() -> Vec<String> {
    let mut checksums = Vec::new();

    let client = reqwest::Client::new();
    for i in 0..URL.len() - 1 {
        let checksums_url = format!("{}/{}/checksums.txt", URL[i].clone(), VERSION.clone(),);
        let res = client.get(&checksums_url).send().await.unwrap();
        let checksums_text = res.text().await.unwrap();

        let checksums_filter = format!(
            "linux-{}-{}-{}.tar.gz",
            ARCH[0].clone(),
            PACKAGE_NAME[i].clone(),
            VERSION.clone()
        );
        let checksums_text = checksums_text
            .split_whitespace()
            .filter(|&x| x.contains(&checksums_filter))
            .collect::<Vec<&str>>();
        let checksums_text = checksums_text[0].to_string();
        checksums.push(checksums_text);
    }
    checksums
}

/// Get aarch64 checksums from GitHub release page.
pub async fn get_aarch64_checksums() -> Vec<String> {
    let mut checksums = Vec::new();

    let client = reqwest::Client::new();
    for i in 0..URL.len() - 1 {
        let checksums_url = format!("{}/{}/checksums.txt", URL[i].clone(), VERSION.clone(),);
        let res = client.get(&checksums_url).send().await.unwrap();
        let checksums_text = res.text().await.unwrap();

        let checksums_filter = format!(
            "linux-{}-{}-{}.tar.gz",
            ARCH[1].clone(),
            PACKAGE_NAME[i].clone(),
            VERSION.clone()
        );
        let checksums_text = checksums_text
            .split_whitespace()
            .filter(|&x| x.contains(&checksums_filter))
            .collect::<Vec<&str>>();
        let checksums_text = checksums_text[0].to_string();
        checksums.push(checksums_text);
    }
    checksums
}

/// Get armv7h checksums from the GitHub release page.
pub async fn get_armv7h_checksums() -> Vec<String> {
    let mut checksums = Vec::new();

    let client = reqwest::Client::new();
    for i in 0..URL.len() - 1 {
        let checksums_url = format!("{}/{}/checksums.txt", URL[i].clone(), VERSION.clone(),);
        let res = client.get(&checksums_url).send().await.unwrap();
        let checksums_text = res.text().await.unwrap();

        let checksums_filter = format!(
            "linux-{}-{}-{}.tar.gz",
            ARCH[2].clone(),
            PACKAGE_NAME[i].clone(),
            VERSION.clone()
        );
        let checksums_text = checksums_text
            .split_whitespace()
            .filter(|&x| x.contains(&checksums_filter))
            .collect::<Vec<&str>>();
        let checksums_text = checksums_text[0].to_string();
        checksums.push(checksums_text);
    }
    checksums
}

pub async fn create_package_dir(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    tokio::fs::create_dir_all(dir).await?;
    Ok(())
}

pub async fn output_package() -> Vec<tokio::fs::File> {
    let files = vec![
        "./build/casaos/PKGBUILD",
        "./build/casaos-app-management/PKGBUILD",
        "./build/casaos-local-storage/PKGBUILD",
        "./build/casaos-user-service/PKGBUILD",
        "./build/casaos-message-bus/PKGBUILD",
        "./build/casaos-gateway/PKGBUILD",
        "./build/casaos-cli/PKGBUILD",
        "./build/casaos-ui/PKGBUILD",
    ];

    let mut outputs = Vec::new();
    for file in files {
        let output = tokio::fs::File::create(file).await.unwrap();
        outputs.push(output);
    }
    outputs
}

#[cfg(test)]
mod test {
    use crate::generator::{
        create_package_dir, generate_casaos_app_management_package, generate_casaos_cli_package,
        generate_casaos_gateway_package, generate_casaos_install,
        generate_casaos_local_storage_package, generate_casaos_message_bus_package,
        generate_casaos_package, generate_casaos_ui_install, generate_casaos_ui_package,
        generate_casaos_user_service_package, get_aarch64_checksums, get_armv7h_checksums,
        get_x86_64_checksums, CasaOSPackage,
    };

    #[tokio::test]
    async fn test_generate_casaos_package() {
        let output = tokio::fs::File::create("./build/casaos/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_install() {
        let output = tokio::fs::File::create("./build/casaos/casaos.install")
            .await
            .unwrap();
        generate_casaos_install(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_app_management_package() {
        let output = tokio::fs::File::create("./build/casaos-app-management/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_app_management_package(output)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_local_storage_package() {
        let output = tokio::fs::File::create("./build/casaos-local-storage/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_local_storage_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_user_service_package() {
        let output = tokio::fs::File::create("./build/casaos-user-service/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_user_service_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_message_bus_package() {
        let output = tokio::fs::File::create("./build/casaos-message-bus/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_message_bus_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_gateway_package() {
        let output = tokio::fs::File::create("./build/casaos-gateway/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_gateway_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_cli_package() {
        let output = tokio::fs::File::create("./build/casaos-cli/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_cli_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_ui_package() {
        create_package_dir("./build/casaos-ui").await.unwrap();
        let output = tokio::fs::File::create("./build/casaos-ui/PKGBUILD")
            .await
            .unwrap();
        let install = tokio::fs::File::create("./build/casaos-ui/casaos-ui.install")
            .await
            .unwrap();
        generate_casaos_ui_install(install).await.unwrap();
        generate_casaos_ui_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_x86_64_checksums() {
        let checksums = get_x86_64_checksums().await;
        println!("{:?}", checksums);
        assert_eq!(checksums.len(), 8);
    }

    #[tokio::test]
    async fn test_get_aarch64_checksums() {
        let checksums = get_aarch64_checksums().await;
        println!("{:?}", checksums);
        assert_eq!(checksums.len(), 8);
    }

    #[tokio::test]
    async fn test_get_armv7_checksums() {
        let checksums = get_armv7h_checksums().await;
        println!("{:?}", checksums);
        assert_eq!(checksums.len(), 8);
    }

    #[tokio::test]
    async fn test_generator() {
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
}
