pub use crate::{consts::*, utils::*};
use tokio::io::AsyncWriteExt;

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
source_x86_64=('{}')
source_aarch64=('{}')
source_armv7h=('{}')
sha256sums_x86_64=({})
sha256sums_aarch64=({})
sha256sums_armv7h=({})

    ",
        VERSION.clone(),
        PackageType::CasaOS.url(),
        "{pkgname}.install",
        X86_64_SOURCE,
        AARCH64_SOURCE,
        ARMV7H_SOURCE,
        checksums[0],
        checksums[1],
        checksums[2]
    );
    let package_content = PACKAGE;

    let pkgbuild = format!("{}{}", content, package_content);

    output.write(pkgbuild.as_bytes()).await?;

    Ok(())
}

pub async fn generate_casaos_install(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let pre_install = r#"
pre_install() {
}
    "#;

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

    let pre_upgrade = r#"
pre_upgrade() {
}
    "#;

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

    let post_remove = r#"
post_remove() {
}
    "#;
    let install = format!(
        "{}{}{}{}{}{}",
        pre_install, post_install, pre_upgrade, post_update, pre_remove, post_remove
    );

    output.write(install.as_bytes()).await?;
    Ok(())
}
