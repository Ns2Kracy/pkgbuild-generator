use super::Pkgbuild;
use crate::utils::*;

pub fn casaos_ui_package() -> Result<(), Box<dyn std::error::Error>> {
    Pkgbuild::new()
        .name("casaos-ui".to_owned())
        .pkgver(VERSION.to_owned())
        .pkgrel("1".to_owned())
        .pkgdesc("The front-end of CasaOS,build with VueJS.".to_owned())
        .arch("any".to_string())
        .url(PackageType::CasaOSUI.url().to_owned())
        .license("unknown".to_owned())
        .source(UI_SOURCE.to_owned())
        .sha256sums("SKIP".to_owned())
        .package(
            r#"

"#
            .trim()
            .to_owned(),
        )
        .output_package()?;

    Ok(())
}
