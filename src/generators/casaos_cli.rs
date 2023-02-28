use super::Pkgbuild;
use crate::utils::*;

pub fn casaos_cli_package() -> Result<(), Box<dyn std::error::Error>> {
    Pkgbuild::new()
        .name("casaos-cli".to_owned())
        .pkgver(VERSION.to_owned())
        .pkgrel("1".to_owned())
        .pkgdesc("A command-line tool to interact with CasaOS for testing and diagnosing purpose".to_owned())
        .arch("x86_64 aarch64 armv7h".to_string())
        .url(PackageType::CasaOSCLI.url().to_owned())
        .license("APACHE".to_string())
        .source_x86_64(X86_64_SOURCE.to_owned())
        .source_aarch64(AARCH64_SOURCE.to_owned())
        .source_armv7h(ARMV7H_SOURCE.to_owned())
        .sha256sums_x86_64("".to_owned())
        .sha256sums_aarch64("".to_owned())
        .sha256sums_armv7h("".to_owned())
        .package(
r#"
package() {
    _sysdir="${srcdir}/build/sysroot"
    install -Dm755 "${_sysdir}/usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
    install -Dm644 "${_sysdir}/etc/bash_completion.d/${pkgname}-completion" "${pkgdir}/etc/bash_completion.d/${pkgname}-completion"
}
"#.trim().to_owned())
        .output_package()?;

    Ok(())
}
