use super::Pkgbuild;
use crate::utils::*;

pub fn casaos_local_storage_package() -> Result<(), Box<dyn std::error::Error>> {
    Pkgbuild::new()
        .name("casaos-local-storage".to_owned())
        .pkgver(VERSION.to_owned())
        .pkgrel("1".to_owned())
        .pkgdesc("Local Storage service provides local storage and disk management functionalities to CasaOS.".to_owned())
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

"#.trim().to_owned())
        .output_package()?;

    Ok(())
}
