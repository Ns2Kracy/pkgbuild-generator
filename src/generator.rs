use lazy_static::lazy_static;
use tokio::io::AsyncWriteExt;

lazy_static! {
    pub static ref VERSION: String = "0.4.1".to_string();
    pub static ref MAINTAINER: Vec<&'static str> = vec!["Maintainer: Ns2Kracy <2220496937@qq.com>"];
    pub static ref ARCH: Vec<&'static str> = vec!["amd64", "aarch64", "armv7h"];
    pub static ref URL: Vec<&'static str> = vec![
        "https://github.com/IceWhaleTech/CasaOS/releases/download",
        "https://github.com/IceWhaleTech/CasaOS-AppManagement/releases/download",
        "https://github.com/IceWhaleTech/CasaOS-LocalStorage/releases/download",
        "https://github.com/IceWhaleTech/CasaOS-UserService/releases/download",
        "https://github.com/IceWhaleTech/CasaOS-MessageBus/releases/download",
        "https://github.com/IceWhaleTech/CasaOS-Gateway/releases/download",
        "https://github.com/IceWhaleTech/CasaOS-CLI/releases/download",
        "https://github.com/IceWhaleTech/CasaOS-UI/releases/download",
    ];
    pub static ref PACKAGE_NAME: Vec<&'static str> = vec![
        "casaos",
        "casaos-app-management",
        "casaos-local-storage",
        "casaos-user-service",
        "casaos-message-bus",
        "casaos-gateway",
        "casaos-cli",
        "casaos-ui",
    ];
}

pub async fn generate_casaos_install(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let mut content = String::new();
    content.push_str("#!/bin/bash");
    output.write_all(content.as_bytes()).await?;

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
    Ok(())
}

pub async fn generate_casaos_package(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let source_x86_64 = generate_x86_64_sources()[0].clone();
    let source_aarch64 = generate_aarch64_sources()[0].clone();
    let source_armv7h = generate_armv7h_sources()[0].clone();
    // 获取async函数的返回值
    let checksums_x86_64 = get_x86_64_checksums().await;
    let checksums_aarch64 = get_aarch64_checksums().await;
    let checksums_armv7h = get_armv7h_checksums().await;
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
    ",
        VERSION.clone(),
        URL[0],
        "{pkgname}.install"
    );

    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        source_x86_64, source_aarch64, source_armv7h
    );

    let chechsums_content = format!(
        "
sha256sums_x86_64=({})
sha256sums_aarch64=({})
sha256sums_armv7h=({})
    ",
        checksums_x86_64[0].clone(),
        checksums_aarch64[0].clone(),
        checksums_armv7h[0].clone()
    );

    let package_content = r#"
_sysdir="${srcdir}/build/sysroot"
_name="${pkgname#*-}"
install -Dm755 "${_sysdir}/usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
install -Dm755 "${_sysdir}/usr/bin/${pkgname}-migration-tool" "${pkgdir}/usr/bin/${pkgname}-migration-tool"
install -Dm644 "${_sysdir}/etc/casaos/${_name}.conf.sample" "${pkgdir}/etc/casaos/${_name}.conf"
install -Dm644 "${_sysdir}/usr/lib/systemd/system/${pkgname}.service" "${pkgdir}/usr/lib/systemd/system/${pkgname}.service"
    "#;

    output.write_all(content.as_bytes()).await?;
    output.write_all(source_content.as_bytes()).await?;
    output.write_all(chechsums_content.as_bytes()).await?;
    output.write_all(package_content.as_bytes()).await?;
    Ok(())
}

pub async fn generate_casaos_app_management_package(
    mut output: tokio::fs::File,
) -> anyhow::Result<()> {
    let source_x86_64 = generate_x86_64_sources()[1].clone();
    let source_aarch64 = generate_aarch64_sources()[1].clone();
    let source_armv7h = generate_armv7h_sources()[1].clone();
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        source_x86_64, source_aarch64, source_armv7h
    );
    Ok(())
}
pub async fn generate_casaos_local_storage_package(
    mut output: tokio::fs::File,
) -> anyhow::Result<()> {
    let source_x86_64 = generate_x86_64_sources()[2].clone();
    let source_aarch64 = generate_aarch64_sources()[2].clone();
    let source_armv7h = generate_armv7h_sources()[2].clone();
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        source_x86_64, source_aarch64, source_armv7h
    );
    Ok(())
}
pub async fn generate_casaos_user_service_package(
    mut output: tokio::fs::File,
) -> anyhow::Result<()> {
    let source_x86_64 = generate_x86_64_sources()[3].clone();
    let source_aarch64 = generate_aarch64_sources()[3].clone();
    let source_armv7h = generate_armv7h_sources()[3].clone();
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        source_x86_64, source_aarch64, source_armv7h
    );
    Ok(())
}
pub async fn generate_casaos_message_bus_package(
    mut output: tokio::fs::File,
) -> anyhow::Result<()> {
    let source_x86_64 = generate_x86_64_sources()[4].clone();
    let source_aarch64 = generate_aarch64_sources()[4].clone();
    let source_armv7h = generate_armv7h_sources()[4].clone();
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        source_x86_64, source_aarch64, source_armv7h
    );
    Ok(())
}
pub async fn generate_casaos_gateway_package(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let source_x86_64 = generate_x86_64_sources()[5].clone();
    let source_aarch64 = generate_aarch64_sources()[5].clone();
    let source_armv7h = generate_armv7h_sources()[5].clone();
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        source_x86_64, source_aarch64, source_armv7h
    );
    Ok(())
}
pub async fn generate_casaos_cli_package(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let source_x86_64 = generate_x86_64_sources()[6].clone();
    let source_aarch64 = generate_aarch64_sources()[6].clone();
    let source_armv7h = generate_armv7h_sources()[6].clone();
    let source_content = format!(
        "
source_x86_64=({})
source_aarch64=({})
source_armv7h=({})
    ",
        source_x86_64, source_aarch64, source_armv7h
    );
    Ok(())
}
pub async fn generate_casaos_ui_package(mut output: tokio::fs::File) -> anyhow::Result<()> {
    let source = format!(
        "{}/{}/linux-all-casaos-{}.tar.gz",
        URL[7].clone(),
        VERSION.clone(),
        VERSION.clone(),
    );

    let content = format!(
        "
pkgname=casaos-ui
pkgver={}
pkgdesc=\"The front-end of CasaOS,build with VueJS.\"
arch=('any')
url={}
license=('unknown')
source=({})
sha256sums=('SKIP')
    ",
        VERSION.clone(),
        URL[7].clone(),
        source
    );
    let package = r#"
_sysdir="${srcdir}/build/sysroot"
mkdir -p "${pkgdir}/var/lib/casaos"
mv "${_sysdir}/var/lib/casaos/www" "${pkgdir}/var/lib/casaos/"
    "#;
    output.write(&content.as_bytes()).await?;
    output.write(&package.as_bytes()).await?;
    Ok(())
}

pub fn generate_x86_64_sources() -> Vec<String> {
    let mut sources = Vec::new();
    for i in 0..URL.len() - 1 {
        let source_x86_64 = format!(
            "{}/v{}/linux-{}-{}-v{}.tar.gz",
            URL[i].clone(),
            VERSION.clone(),
            ARCH[4].clone(),
            PACKAGE_NAME[i].clone(),
            VERSION.clone(),
        );
        sources.push(source_x86_64);
    }
    sources
}

pub fn generate_aarch64_sources() -> Vec<String> {
    let mut sources = Vec::new();
    for i in 0..URL.len() - 1 {
        let source_aarch64 = format!(
            "{}/v{}/linux-{}-{}-v{}.tar.gz",
            URL[i].clone(),
            VERSION.clone(),
            ARCH[1].clone(),
            PACKAGE_NAME[i].clone(),
            VERSION.clone(),
        );
        sources.push(source_aarch64);
    }
    sources
}

pub fn generate_armv7h_sources() -> Vec<String> {
    let mut sources = Vec::new();
    for i in 0..URL.len() - 1 {
        let source_armv7h = format!(
            "{}/v{}/linux-{}-{}-v{}.tar.gz",
            URL[i].clone(),
            VERSION.clone(),
            ARCH[2].clone(),
            PACKAGE_NAME[i].clone(),
            VERSION.clone(),
        );
        sources.push(source_armv7h);
    }
    sources
}

pub async fn get_x86_64_checksums() -> Vec<String> {
    let mut checksums = Vec::new();

    let client = reqwest::Client::new();
    for i in 0..URL.len() - 1 {
        let checksums_url = format!("{}/{}/checksums.txt", URL[i].clone(), VERSION.clone(),);
        let res = client.get(&checksums_url).send().await.unwrap();
        let checksums_text = res.text().await.unwrap();

        let checksums_filter = format!(
            "linux-{}-{}-{}.tar.gz",
            ARCH[0].clone(),
            PACKAGE_NAME[i].clone(),
            VERSION.clone()
        );
        let checksums_text = checksums_text
            .split_whitespace()
            .filter(|&x| x.contains(&checksums_filter))
            .collect::<Vec<&str>>();
        let checksums_text = checksums_text[0].to_string();
        checksums.push(checksums_text);
    }
    checksums
}

pub async fn get_aarch64_checksums() -> Vec<String> {
    let mut checksums = Vec::new();

    let client = reqwest::Client::new();
    for i in 0..URL.len() - 1 {
        let checksums_url = format!("{}/{}/checksums.txt", URL[i].clone(), VERSION.clone(),);
        let res = client.get(&checksums_url).send().await.unwrap();
        let checksums_text = res.text().await.unwrap();

        let checksums_filter = format!(
            "linux-{}-{}-{}.tar.gz",
            ARCH[1].clone(),
            PACKAGE_NAME[i].clone(),
            VERSION.clone()
        );
        let checksums_text = checksums_text
            .split_whitespace()
            .filter(|&x| x.contains(&checksums_filter))
            .collect::<Vec<&str>>();
        let checksums_text = checksums_text[0].to_string();
        checksums.push(checksums_text);
    }
    checksums
}

pub async fn get_armv7h_checksums() -> Vec<String> {
    let mut checksums = Vec::new();

    let client = reqwest::Client::new();
    for i in 0..URL.len() - 1 {
        let checksums_url = format!("{}/{}/checksums.txt", URL[i].clone(), VERSION.clone(),);
        let res = client.get(&checksums_url).send().await.unwrap();
        let checksums_text = res.text().await.unwrap();

        let checksums_filter = format!(
            "linux-{}-{}-{}.tar.gz",
            ARCH[2].clone(),
            PACKAGE_NAME[i].clone(),
            VERSION.clone()
        );
        let checksums_text = checksums_text
            .split_whitespace()
            .filter(|&x| x.contains(&checksums_filter))
            .collect::<Vec<&str>>();
        let checksums_text = checksums_text[0].to_string();
        checksums.push(checksums_text);
    }
    checksums
}

#[cfg(test)]
mod test {
    use crate::generator::{
        generate_casaos_app_management_package, generate_casaos_cli_package,
        generate_casaos_gateway_package, generate_casaos_install,
        generate_casaos_local_storage_package, generate_casaos_message_bus_package,
        generate_casaos_package, generate_casaos_ui_package, generate_casaos_user_service_package,
        get_x86_64_checksums,
    };

    #[tokio::test]
    async fn test_generate_casaos_package() {
        let output = tokio::fs::File::create("./casaos/PKGBUILD").await.unwrap();
        generate_casaos_package(output).await.unwrap();
        let install = tokio::fs::File::create("./casaos/casaos.install")
            .await
            .unwrap();
        generate_casaos_install(install).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_app_management_package() {
        let output = tokio::fs::File::create("./casaos-app-management/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_app_management_package(output)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_local_storage_package() {
        let output = tokio::fs::File::create("./casaos-local-storage/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_local_storage_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_user_service_package() {
        let output = tokio::fs::File::create("./casaos-user-service/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_user_service_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_message_bus_package() {
        let output = tokio::fs::File::create("./casaos-message-bus/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_message_bus_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_gateway_package() {
        let output = tokio::fs::File::create("./casaos-gateway/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_gateway_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_cli_package() {
        let output = tokio::fs::File::create("./casaos-cli/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_cli_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_generate_casaos_ui_package() {
        let output = tokio::fs::File::create("./casaos-ui/PKGBUILD")
            .await
            .unwrap();
        generate_casaos_ui_package(output).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_x86_64_checksums() {
        let checksums = get_x86_64_checksums().await;
        println!("{:?}", checksums);
        assert_eq!(checksums.len(), 8);
    }
}
