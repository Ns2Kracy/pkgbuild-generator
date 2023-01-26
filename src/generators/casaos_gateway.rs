pub use crate::{consts::*, utils::*};
use tokio::io::AsyncWriteExt;

pub async fn generate_casaos_gateway_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let checksums = get_checksums(PackageType::CasaOSCLI).await;
    let content = format!(
        "# {}
pkgname=casaos-gateway
pkgver={}
pkgrel=1
pkgdesc='A dynamic API gateway service that can be used to expose APIs from different other HTTP based services.'
arch=('x86_64' 'aarch64' 'armv7h')
url={}
license=('APACHE')
backup=('etc/casaos/gateway.ini')
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
sha256sums_x86_64=({})
sha256sums_aarch64=({})
sha256sums_armv7h=({})
",
        MAINTAINER,
        VERSION,
        PackageType::CasaOSCLI.url(),
        X86_64_SOURCE, AARCH64_SOURCE, ARMV7H_SOURCE,
        checksums[0], checksums[1], checksums[2]
    );

    let package_content = r#"
package() {
	_sysdir="${srcdir}/build/sysroot"
	_name="${pkgname#*-}"
	install -Dm755 "${_sysdir}/usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
	install -Dm644 "${_sysdir}/etc/casaos/${_name}.ini.sample" "${pkgdir}/etc/casaos/${_name}.ini"
	install -Dm644 "${_sysdir}/usr/lib/systemd/system/${pkgname}.service" "${pkgdir}/usr/lib/systemd/system/${pkgname}.service"
}
"#;

    let content = format!("{}{}", content, package_content);

    output.write(&content.as_bytes()).await?;
    Ok(())
}

pub async fn generate_casaos_gateway_install(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let pre_install = r#"
pre_install() {
}
    "#;

    let post_install = r#"
post_install() {
}    
    "#;

    let pre_upgrade = r#"
pre_upgrade() {
}
    "#;

    let post_update = r#"
post_upgrade() {
}
    "#;

    let pre_remove = r#"
pre_remove() {
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
