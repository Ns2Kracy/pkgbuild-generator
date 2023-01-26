pub use crate::{consts::*, utils::*};
use tokio::io::AsyncWriteExt;

pub async fn generate_casaos_ui_package(
    mut output: tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let info_content = format!(
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
    let package_content = r#"
package() {
    _sysdir="${srcdir}/build/sysroot"
    mkdir -p "${pkgdir}/var/lib/casaos"
    mv "${_sysdir}/var/lib/casaos/www" "${pkgdir}/var/lib/casaos/"
}
    "#;

    let content = format!("{}{}", info_content, package_content);

    output.write(&content.as_bytes()).await?;

    Ok(())
}

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