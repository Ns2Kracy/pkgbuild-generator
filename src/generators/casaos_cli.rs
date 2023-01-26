pub use crate::{consts::*, utils::*};
use tokio::io::AsyncWriteExt;

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
        X86_64_SOURCE,
        AARCH64_SOURCE,
        ARMV7H_SOURCE,
        checksums[0],
        checksums[1],
        checksums[2]
    );

    let package_content = r#"
package() {
    _sysdir="${srcdir}/build/sysroot"
    install -Dm755 "${_sysdir}/usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
    install -Dm644 "${_sysdir}/etc/bash_completion.d/${pkgname}-completion" "${pkgdir}/etc/bash_completion.d/${pkgname}-completion"
}    
"#;

    let content = format!("{}{}", content, package_content);

    output.write(&content.as_bytes()).await?;
    Ok(())
}
