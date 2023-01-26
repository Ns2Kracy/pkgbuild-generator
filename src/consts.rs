/// CasaOS Version
pub const VERSION: &str = "0.4.1";

/// Aur Maintainer
pub const MAINTAINER: &str = "Maintainer: Ns2Kracy <2220496937@qq.com>";

/// x86_64 Arch Source
pub const X86_64_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-amd64-${pkgname}-v${pkgver}.tar.gz";

/// aarch64 Arch Source
pub const AARCH64_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-arm64-${pkgname}-v${pkgver}.tar.gz";

/// armv7h Arch Source
pub const ARMV7H_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-armv7-${pkgname}-v${pkgver}.tar.gz";

/// casaos-ui Arch Source
pub const SOURCE: &str = "${url}/releases/download/v${pkgver}/linux-all-casaos-v${pkgver}.tar.gz";

/// install name
pub const INSTALL: &str = "${pkgname}.install";

pub const BUILD_PATH: &str = "build";

pub const PACKAGE: &str = r#"
package() {
    _sysdir="${srcdir}/build/sysroot"
    _name="${pkgname#*-}"
    install -Dm755 "${_sysdir}/usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
    install -Dm644 "${_sysdir}/etc/casaos/${_name}.conf.sample" "${pkgdir}/etc/casaos/${_name}.conf"
    install -Dm644 "${_sysdir}/usr/lib/systemd/system/${pkgname}.service" "${pkgdir}/usr/lib/systemd/system/${pkgname}.service"
}  
"#;
