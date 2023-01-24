use tokio::{io::AsyncWriteExt, process::Command};

const VERSION: &str = "0.4.1";
const MAINTAINER: &str = "Maintainer: Ns2Kracy <2220496937@qq.com>";
const X86_64_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-amd64-${pkgname}-v${pkgver}.tar.gz";
const AARCH64_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-arm64-${pkgname}-v${pkgver}.tar.gz";
const ARMV7H_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-armv7-${pkgname}-v${pkgver}.tar.gz";
const SOURCE: &str = "${url}/releases/download/v${pkgver}/linux-all-casaos-v${pkgver}.tar.gz";
const INSTALL: &str = "${pkgname}.install";

#[derive(Debug, Clone, Copy)]
pub enum PackageType {
    CasaOS,
    CasaOSAppManagement,
    CasaOSLocalStorage,
    CasaOSUserService,
    CasaOSMessageBus,
    CasaOSGateway,
    CasaOSCLI,
    CasaOSUI,
}

impl PackageType {
    pub fn url(&self) -> &str {
        match self {
            PackageType::CasaOS => "https://github.com/IceWhaleTech/CasaOS",
            PackageType::CasaOSAppManagement => {
                "https://github.com/IceWhaleTech/CasaOS-AppManagement"
            }
            PackageType::CasaOSLocalStorage => {
                "https://github.com/IceWhaleTech/CasaOS-LocalStorage"
            }
            PackageType::CasaOSUserService => "https://github.com/IceWhaleTech/CasaOS-UserService",
            PackageType::CasaOSMessageBus => "https://github.com/IceWhaleTech/CasaOS-MessageBus",
            PackageType::CasaOSGateway => "https://github.com/IceWhaleTech/CasaOS-Gateway",
            PackageType::CasaOSCLI => "https://github.com/IceWhaleTech/CasaOS-CLI",
            PackageType::CasaOSUI => "https://github.com/IceWhaleTech/CasaOS-UI",
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            PackageType::CasaOS => "casaos",
            PackageType::CasaOSAppManagement => "casaos-app-management",
            PackageType::CasaOSLocalStorage => "casaos-local-storage",
            PackageType::CasaOSUserService => "casaos-user-service",
            PackageType::CasaOSMessageBus => "casaos-message-bus",
            PackageType::CasaOSGateway => "casaos-gateway",
            PackageType::CasaOSCLI => "casaos-cli",
            PackageType::CasaOSUI => "casaos-ui",
        }
    }
}

pub struct CasaOSPackage;

impl CasaOSPackage {
    pub fn new() -> Self {
        Self
    }

    pub async fn add_package(&self, package_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let outputs = output_package().await;
        let casaos_install = tokio::fs::File::create("./build/casaos/casaos.install")
            .await
            .unwrap();

        match package_name {
            "casaos" => {
                let mut makepkg = Command::new("makepkg");
                create_package_dir(PackageType::CasaOS.to_string())
                    .await
                    .unwrap();
                generate_casaos_package(outputs[0].try_clone().await.unwrap())
                    .await
                    .unwrap();
                generate_casaos_install(casaos_install).await.unwrap();
                makepkg
                    .current_dir("./build/casaos")
                    .arg("-s")
                    .spawn()
                    .unwrap();
            }
            "casaos-app-management" => {
                let mut makepkg = Command::new("makepkg");
                create_package_dir(PackageType::CasaOSAppManagement.to_string())
                    .await
                    .unwrap();
                generate_casaos_app_management_package(outputs[1].try_clone().await.unwrap())
                    .await
                    .unwrap();
                makepkg
                    .current_dir("./build/casaos-app-management")
                    .arg("-s")
                    .spawn()
                    .unwrap();
            }
            "casaos-local-storage" => {
                let mut makepkg = Command::new("makepkg");
                create_package_dir(PackageType::CasaOSLocalStorage.to_string())
                    .await
                    .unwrap();
                generate_casaos_local_storage_package(outputs[2].try_clone().await.unwrap())
                    .await
                    .unwrap();
                makepkg
                    .current_dir("./build/casaos-local-storage")
                    .arg("-s")
                    .spawn()
                    .unwrap();
            }
            "casaos-user-service" => {
                let mut makepkg = Command::new("makepkg");
                create_package_dir(PackageType::CasaOSUserService.to_string())
                    .await
                    .unwrap();
                generate_casaos_user_service_package(outputs[3].try_clone().await.unwrap())
                    .await
                    .unwrap();
                makepkg
                    .current_dir("./build/casaos-user-service")
                    .arg("-s")
                    .spawn()
                    .unwrap();
            }
            "casaos-message-bus" => {
                let mut makepkg = Command::new("makepkg");
                create_package_dir(PackageType::CasaOSMessageBus.to_string())
                    .await
                    .unwrap();
                generate_casaos_message_bus_package(outputs[4].try_clone().await.unwrap())
                    .await
                    .unwrap();
                makepkg
                    .current_dir("./build/casaos-message-bus")
                    .arg("-s")
                    .spawn()
                    .unwrap();
            }
            "casaos-gateway" => {
                let mut makepkg = Command::new("makepkg");
                create_package_dir(PackageType::CasaOSGateway.to_string())
                    .await
                    .unwrap();
                generate_casaos_gateway_package(outputs[5].try_clone().await.unwrap())
                    .await
                    .unwrap();
                makepkg
                    .current_dir("./build/casaos-gateway")
                    .arg("-s")
                    .spawn()
                    .unwrap();
            }
            "casaos-cli" => {
                let mut makepkg = Command::new("makepkg");
                create_package_dir(PackageType::CasaOSCLI.to_string())
                    .await
                    .unwrap();
                generate_casaos_cli_package(outputs[6].try_clone().await.unwrap())
                    .await
                    .unwrap();
                makepkg
                    .current_dir("./build/casaos-cli")
                    .arg("-s")
                    .spawn()
                    .unwrap();
            }
            "casaos-ui" => {
                let mut makepkg = Command::new("makepkg");
                create_package_dir(PackageType::CasaOSUI.to_string())
                    .await
                    .unwrap();
                generate_casaos_ui_package(outputs[7].try_clone().await.unwrap())
                    .await
                    .unwrap();
                makepkg
                    .current_dir("./build/casaos-ui")
                    .arg("-s")
                    .spawn()
                    .unwrap();
            }
            _ => {}
        }
        Ok(())
    }
}

/// 生成casaos的install文件
/// 用于执行一些PKGBUILD不能执行的脚本
pub async fn generate_casaos_install(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}

/// Generate the PKGBUILD file for casaos.
/// Src: https://github.com/IceWhaleTech/CasaOS
/// Output: /casaos/PKGBUILD
pub async fn generate_casaos_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let checksums = get_checksums(PackageType::CasaOS).await;
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
        PackageType::CasaOS.url(),
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
sha256sums_x86_64=({:?})
sha256sums_aarch64=({:?})
sha256sums_armv7h=({:?})
    ",
        checksums[0], checksums[1], checksums[2]
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
    Ok(())
}

/// Generate the PKGBUILD file for casaos-app-management.
/// Src: https://github.com/IceWhaleTech/CasaOS-AppManagement
/// Output: /casaos-app-management/PKGBUILD
pub async fn generate_casaos_app_management_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    Ok(())
}

/// Generate the PKGBUILD file for casaos-local-storage.
/// Src: https://github.com/IceWhaleTech/CasaOS-LocalStorage
/// Output: /casaos-local-storage/PKGBUILD
pub async fn generate_casaos_local_storage_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    Ok(())
}

/// Generate the PKGBUILD file for casaos-user-service.
/// Src: https://github.com/IceWhaleTech/CasaOS-UserService
/// Output: /casaos-user-service/PKGBUILD
pub async fn generate_casaos_user_service_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    Ok(())
}

/// Generate the PKGBUILD file for casaos-message-bus.
/// Src: https://github.com/IceWhaleTech/CasaOS-MessageBus
/// Output: /casaos-message-bus/PKGBUILD
pub async fn generate_casaos_message_bus_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    Ok(())
}

/// Generate the PKGBUILD file for casaos-gateway.
/// Src: https://github.com/IceWhaleTech/CasaOS-Gateway
/// Output: /casaos-gateway/PKGBUILD
pub async fn generate_casaos_gateway_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );
    output.write_all(source_content.as_bytes()).await?;
    Ok(())
}

/// Generate the PKGBUILD file for casaos-cli.
/// Src: https://github.com/IceWhaleTech/CasaOS-CLI
/// Output: /casaos-cli/PKGBUILD
pub async fn generate_casaos_cli_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let checksums = get_checksums(PackageType::CasaOSCLI).await;
    let content = format!(
        "# {}
pkgname=casaos-cli
pkgver={}
pkgrel=1
pkgdesc='A command-line tool to interact with CasaOS for testing and diagnosing purpose'
arch=('x86_64' 'aarch64' 'armv7h')
url={}
license=('APACHE')
",
        MAINTAINER,
        VERSION,
        PackageType::CasaOSCLI.url()
    );

    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE
    );

    let checksum_content = format!(
        "
sha256sums_x86_64=({})
sha256sums_aarch64=({})
sha256sums_armv7h=({})
    ",
        checksums[0], checksums[1], checksums[2]
    );

    let package_content = r#"
package() {
    _sysdir="${srcdir}/build/sysroot"
    install -Dm755 "${_sysdir}/usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
    install -Dm644 "${_sysdir}/etc/bash_completion.d/${pkgname}-completion" "${pkgdir}/etc/bash_completion.d/${pkgname}-completion"
}    
"#;

    output.write_all(content.as_bytes()).await?;
    output.write_all(source_content.as_bytes()).await?;
    output.write_all(checksum_content.as_bytes()).await?;
    output.write_all(package_content.as_bytes()).await?;
    Ok(())
}

/// Generate the PKGBUILD file for casaos-ui.
/// Src: https://github.com/IceWhaleTech/CasaOS-UI
/// Output: /casaos-ui/PKGBUILD
pub async fn generate_casaos_ui_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
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
        VERSION,
        PackageType::CasaOSUI.url(),
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
    Ok(())
}

/// Generate the install file for casaos-ui.
/// mainly used for removing the /var/lib/casaos directory.
pub async fn generate_casaos_ui_install(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let post_remove = r#"#!/bin/bash
post_remove() {
    rm -rf /var/lib/casaos
}
    "#;
    output.write(&post_remove.as_bytes()).await?;
    Ok(())
}

pub async fn get_checksums(package: PackageType) -> Vec<String> {
    vec![
        get_x86_64_checksums(package).await,
        get_aarch64_checksums(package).await,
        get_armv7h_checksums(package).await,
    ]
}

pub async fn get_x86_64_checksums(package: PackageType) -> String {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/releases/download/v{}/checksums.txt",
        package.url(),
        VERSION
    );
    let resp = client.get(&url).send().await.unwrap();
    let checksums = resp.text().await.unwrap();
    let filter = format!(
        "linux-amd64-casaos-{}-v{}.tar.gz",
        package.to_string(),
        VERSION
    );

    let checksums = checksums
        .split_whitespace()
        .filter(|s| s.contains(&filter))
        .map(|s| s.to_string())
        .collect::<String>();
    checksums
}

pub async fn get_aarch64_checksums(package: PackageType) -> String {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/releases/download/v{}/checksums.txt",
        package.url(),
        VERSION
    );
    let resp = client.get(&url).send().await.unwrap();
    let checksums = resp.text().await.unwrap();
    let filter = format!(
        "linux-arm64-casaos-{}-v{}.tar.gz",
        package.to_string(),
        VERSION
    );

    let checksums = checksums
        .split_whitespace()
        .filter(|s| s.contains(&filter))
        .map(|s| s.to_string())
        .collect::<String>();
    checksums
}

pub async fn get_armv7h_checksums(package: PackageType) -> String {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/releases/download/v{}/checksums.txt",
        package.url(),
        VERSION
    );
    let resp = client.get(&url).send().await.unwrap();
    let checksums = resp.text().await.unwrap();
    let filter = format!(
        "linux-arm-7-casaos-{}-v{}.tar.gz",
        package.to_string(),
        VERSION
    );
    let checksums = checksums
        .split_whitespace()
        .filter(|s| s.contains(&filter))
        .map(|s| s.to_string())
        .collect::<String>();
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
        generate_casaos_user_service_package, get_checksums, CasaOSPackage, PackageType,
    };

    #[tokio::test]
    async fn test_generate_casaos_package() {
        create_package_dir("./build/casaos").await.unwrap();
        let output = tokio::fs::File::create("./build/casaos/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_install() {
        create_package_dir("./build/casaos").await.unwrap();
        let output = tokio::fs::File::create("./build/casaos/casaos.install")
            .await
            .unwrap();
        generate_casaos_install(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_app_management_package() {
        create_package_dir("./build/casaos-app-management")
            .await
            .unwrap();
        let output = tokio::fs::File::create("./build/casaos-app-management/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_app_management_package(output)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_local_storage_package() {
        create_package_dir("./build/casaos-local-storage")
            .await
            .unwrap();
        let output = tokio::fs::File::create("./build/casaos-local-storage/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_local_storage_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_user_service_package() {
        create_package_dir("./build/casaos-user-service")
            .await
            .unwrap();
        let output = tokio::fs::File::create("./build/casaos-user-service/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_user_service_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_message_bus_package() {
        create_package_dir("./build/casaos-message-bus")
            .await
            .unwrap();
        let output = tokio::fs::File::create("./build/casaos-message-bus/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_message_bus_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_gateway_package() {
        create_package_dir("./build/casaos-gateway").await.unwrap();
        let output = tokio::fs::File::create("./build/casaos-gateway/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_gateway_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_cli_package() {
        create_package_dir("./build/casaos-cli").await.unwrap();
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
    async fn test_get_checksums() {
        let casaos_checksums = get_checksums(PackageType::CasaOS).await;
        let casaos_app_management_checksums = get_checksums(PackageType::CasaOSAppManagement).await;
        let casaos_local_storage_checksums = get_checksums(PackageType::CasaOSLocalStorage).await;
        let casaos_user_service_checksums = get_checksums(PackageType::CasaOSUserService).await;
        let casaos_message_bus_checksums = get_checksums(PackageType::CasaOSMessageBus).await;
        let casaos_gateway_checksums = get_checksums(PackageType::CasaOSGateway).await;
        let casaos_cli_checksums = get_checksums(PackageType::CasaOSCLI).await;

        println!("casaos_checksums: {:?}\n", casaos_checksums);
        println!(
            "casaos_app_management_checksums: {:?}\n",
            casaos_app_management_checksums
        );
        println!(
            "casaos_local_storage_checksums: {:?}\n",
            casaos_local_storage_checksums
        );
        println!(
            "casaos_user_service_checksums: {:?}\n",
            casaos_user_service_checksums
        );
        println!(
            "casaos_message_bus_checksums: {:?}\n",
            casaos_message_bus_checksums
        );
        println!("casaos_gateway_checksums: {:?}\n", casaos_gateway_checksums);
        println!("casaos_cli_checksums: {:?}\n", casaos_cli_checksums);
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
