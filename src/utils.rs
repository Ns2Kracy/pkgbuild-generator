use crate::consts::VERSION;

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

pub async fn create_dirs() -> Result<(), Box<dyn std::error::Error>> {
    tokio::fs::create_dir_all("./build").await?;
    tokio::fs::create_dir_all("./build/casaos").await?;
    tokio::fs::create_dir_all("./build/casaos-app-management").await?;
    tokio::fs::create_dir_all("./build/casaos-local-storage").await?;
    tokio::fs::create_dir_all("./build/casaos-user-service").await?;
    tokio::fs::create_dir_all("./build/casaos-message-bus").await?;
    tokio::fs::create_dir_all("./build/casaos-gateway").await?;
    tokio::fs::create_dir_all("./build/casaos-cli").await?;
    tokio::fs::create_dir_all("./build/casaos-ui").await?;
    Ok(())
}

pub async fn output_package() -> Vec<tokio::fs::File> {
    let files = vec![
        "./build/casaos",
        "./build/casaos-app-management",
        "./build/casaos-local-storage",
        "./build/casaos-user-service",
        "./build/casaos-message-bus",
        "./build/casaos-gateway",
        "./build/casaos-cli",
        "./build/casaos-ui",
    ];

    let mut outputs = Vec::new();
    for file in files {
        tokio::fs::create_dir_all(file).await.unwrap();
        let output = tokio::fs::File::create(format!("{}/PKGBUILD", file))
            .await
            .unwrap();
        outputs.push(output);
    }
    outputs
}

pub async fn output_install() -> Vec<tokio::fs::File> {
    let files = vec![
        "./build/casaos/casaos.install",
        "./build/casaos-app-management/casaos-app-management.install",
        "./build/casaos-local-storage/casaos-local-storage.install",
        "./build/casaos-user-service/casaos-user-service.install",
        "./build/casaos-message-bus/casaos-message-bus.install",
        "./build/casaos-gateway/casaos-gateway.install",
        "./build/casaos-ui/casaos-ui.install",
    ];

    let mut outputs = Vec::new();
    for file in files {
        tokio::fs::create_dir_all(file).await.unwrap();
        let output = tokio::fs::File::create(format!("{}", file)).await.unwrap();
        outputs.push(output);
    }
    outputs
}

#[cfg(test)]
mod test {
    use crate::generators::{get_checksums, PackageType};

    #[tokio::test]
    async fn test_get_checksums() {
        let checksums = get_checksums(PackageType::CasaOS).await;
        println!("{:?}", checksums);
        assert_eq!(checksums.len(), 3);
    }
}
