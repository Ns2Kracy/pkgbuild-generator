/// CasaOS Version
pub const VERSION: &str = "0.4.1";

/// Aur Maintainer
pub const MAINTAINER: &str = "Maintainer: Ns2Kracy <2220496937@qq.com>";

/// CasaOS UI Source
pub const UI_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-all-casaos-v${pkgver}.tar.gz";

/// x86_64 Arch Source
pub const X86_64_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-amd64-${pkgname}-v${pkgver}.tar.gz";

/// aarch64 Arch Source
pub const AARCH64_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-arm64-${pkgname}-v${pkgver}.tar.gz";

/// armv7h Arch Source
pub const ARMV7H_SOURCE: &str =
    "${url}/releases/download/v${pkgver}/linux-armv7-${pkgname}-v${pkgver}.tar.gz";

/// Install File
pub const INSTALL: &str = "${pkgname}.install";

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

    pub fn create_file(&self) -> std::fs::File {
        let dir_name = format!("./build/{}", self.to_string());
        std::fs::create_dir_all(dir_name).unwrap();
        let file_name = format!("./build/{}/PKGBUILD", self.to_string());
        std::fs::File::create(file_name).unwrap()
    }
}

pub async fn get_checksums(package: PackageType) -> Vec<String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/releases/download/v{}/checksums.txt",
        package.url(),
        VERSION
    );
    let resp = client.get(&url).send().await.unwrap();
    let checksums = resp.text().await.unwrap();

    let x86_64_filter = format!("linux-amd64-{}-v{}.tar.gz", package.to_string(), VERSION);
    let aarch64_filter = format!("linux-arm64-{}-v{}.tar.gz", package.to_string(), VERSION);
    let armv7h_filter = format!("linux-arm-7-{}-v{}.tar.gz", package.to_string(), VERSION);

    let x86_64_checksum = checksums
        .split_whitespace()
        .filter(|x| x.contains(&x86_64_filter))
        .collect::<Vec<&str>>()[0];
    let aarch64_checksum = checksums
        .split_whitespace()
        .filter(|x| x.contains(&aarch64_filter))
        .collect::<Vec<&str>>()[0];
    let armv7h_checksum = checksums
        .split_whitespace()
        .filter(|x| x.contains(&armv7h_filter))
        .collect::<Vec<&str>>()[0];

    vec![
        x86_64_checksum.to_string(),
        aarch64_checksum.to_string(),
        armv7h_checksum.to_string(),
    ]
}

// pub async fn create_dirs() -> Result<(), Box<dyn std::error::Error>> {
//     tokio::fs::create_dir_all("./build").await?;
//     tokio::fs::create_dir_all("./build/casaos").await?;
//     tokio::fs::create_dir_all("./build/casaos-app-management").await?;
//     tokio::fs::create_dir_all("./build/casaos-local-storage").await?;
//     tokio::fs::create_dir_all("./build/casaos-user-service").await?;
//     tokio::fs::create_dir_all("./build/casaos-message-bus").await?;
//     tokio::fs::create_dir_all("./build/casaos-gateway").await?;
//     tokio::fs::create_dir_all("./build/casaos-cli").await?;
//     tokio::fs::create_dir_all("./build/casaos-ui").await?;
//     Ok(())
// }

// #[cfg(test)]
// mod test {
//     use crate::generators::{get_checksums, PackageType};

//     #[tokio::test]
//     async fn test_get_checksums() {
//         let checksums = get_checksums(PackageType::CasaOS).await;
//         println!("{:?}", checksums);
//         assert_eq!(checksums.len(), 3);
//     }
// }
