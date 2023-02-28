pub mod casaos;
pub mod casaos_app_management;
pub mod casaos_cli;
pub mod casaos_gateway;
pub mod casaos_local_storage;
pub mod casaos_message_bus;
pub mod casaos_ui;
pub mod casaos_user_service;

use std::io::Write;

use crate::utils::*;

#[derive(Debug)]
pub struct Pkgbuild {
    name: String,
    pkgdesc: String,
    pkgver: String,
    pkgrel: String,
    url: Option<String>,
    arch: String,
    license: Option<String>,
    depends: Option<Vec<String>>,
    makedepends: Option<Vec<String>>,
    optdepends: Option<Vec<String>>,
    provides: Option<Vec<String>>,
    conflicts: Option<Vec<String>>,
    replaces: Option<Vec<String>>,
    backup: Option<Vec<String>>,
    install: Option<String>,
    source: Option<String>,
    sha256sums: Option<String>,
    source_x86_64: Option<String>,
    source_aarch64: Option<String>,
    source_armv7h: Option<String>,
    sha256sums_x86_64: Option<String>,
    sha256sums_aarch64: Option<String>,
    sha256sums_armv7h: Option<String>,
    package: String,
}

impl Pkgbuild {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            pkgdesc: String::new(),
            pkgver: String::new(),
            pkgrel: String::new(),
            url: None,
            arch: String::new(),
            license: None,
            depends: None,
            makedepends: None,
            optdepends: None,
            provides: None,
            conflicts: None,
            replaces: None,
            backup: None,
            install: None,
            source: None,
            sha256sums: None,
            source_x86_64: None,
            source_aarch64: None,
            source_armv7h: None,
            sha256sums_x86_64: None,
            sha256sums_aarch64: None,
            sha256sums_armv7h: None,
            package: String::new(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn pkgver(mut self, pkgver: String) -> Self {
        self.pkgver = pkgver;
        self
    }

    pub fn pkgrel(mut self, pkgrel: String) -> Self {
        self.pkgrel = pkgrel;
        self
    }

    pub fn pkgdesc(mut self, pkgdesc: String) -> Self {
        self.pkgdesc = pkgdesc;
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn arch(mut self, arch: String) -> Self {
        self.arch = arch;
        self
    }

    pub fn license(mut self, license: String) -> Self {
        self.license = Some(license);
        self
    }

    pub fn depends(mut self, depends: Vec<String>) -> Self {
        self.depends = Some(depends);
        self
    }

    pub fn makedepends(mut self, makedepends: Vec<String>) -> Self {
        self.makedepends = Some(makedepends);
        self
    }

    pub fn optdepends(mut self, optdepends: Vec<String>) -> Self {
        self.optdepends = Some(optdepends);
        self
    }

    pub fn provides(mut self, provides: Vec<String>) -> Self {
        self.provides = Some(provides);
        self
    }

    pub fn conflicts(mut self, conflicts: Vec<String>) -> Self {
        self.conflicts = Some(conflicts);
        self
    }

    pub fn replaces(mut self, replaces: Vec<String>) -> Self {
        self.replaces = Some(replaces);
        self
    }

    pub fn backup(mut self, backup: Vec<String>) -> Self {
        self.backup = Some(backup);
        self
    }

    pub fn install(mut self, install: String) -> Self {
        self.install = Some(install);
        self
    }

    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    pub fn sha256sums(mut self, sha256sums: String) -> Self {
        self.sha256sums = Some(sha256sums);
        self
    }

    pub fn source_x86_64(mut self, source_x86_64: String) -> Self {
        self.source_x86_64 = Some(source_x86_64);
        self
    }

    pub fn source_aarch64(mut self, source_aarch64: String) -> Self {
        self.source_aarch64 = Some(source_aarch64);
        self
    }

    pub fn source_armv7h(mut self, source_armv7h: String) -> Self {
        self.source_armv7h = Some(source_armv7h);
        self
    }

    pub fn sha256sums_x86_64(mut self, sha256sums_x86_64: String) -> Self {
        self.sha256sums_x86_64 = Some(sha256sums_x86_64);
        self
    }

    pub fn sha256sums_aarch64(mut self, sha256sums_aarch64: String) -> Self {
        self.sha256sums_aarch64 = Some(sha256sums_aarch64);
        self
    }

    pub fn sha256sums_armv7h(mut self, sha256sums_armv7h: String) -> Self {
        self.sha256sums_armv7h = Some(sha256sums_armv7h);
        self
    }

    pub fn package(mut self, package: String) -> Self {
        self.package = package;
        self
    }

    pub fn output_package(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut pkgbuild = String::new();
        let build_dir = format!("build/{}", self.name);
        let pkgbuild_dir = format!("{}/PKGBUILD", build_dir);
        std::fs::create_dir_all(&build_dir)?;
        let mut writer = std::fs::File::create(&pkgbuild_dir)?;
        let content = format!(
            "# {}
pkgname={}
pkgver={}
pkgrel={}
pkgdesc={}
arch=({:?})
url={}
license=({:?})
source=({:?})
source_x86_64=({:?})
source_aarch64=({:?})
source_armv7h=({:?})
sha256sums=({})
sha256sums_x86_64=({})
sha256sums_aarch64=({})
sha256sums_armv7h=({})
{}
    ",
            MAINTAINER,
            self.name,
            self.pkgver,
            self.pkgrel,
            self.pkgdesc,
            self.arch,
            self.url.as_ref().unwrap(),
            self.license.as_ref().unwrap(),
            self.source.as_ref().unwrap(),
            self.source_x86_64.as_ref().unwrap(),
            self.source_aarch64.as_ref().unwrap(),
            self.source_armv7h.as_ref().unwrap(),
            self.sha256sums.as_ref().unwrap(),
            self.sha256sums_x86_64.as_ref().unwrap(),
            self.sha256sums_aarch64.as_ref().unwrap(),
            self.sha256sums_armv7h.as_ref().unwrap(),
            self.package,
        );
        pkgbuild.push_str(&content);

        writer.write_all(pkgbuild.as_bytes())?;

        Ok(())
    }
}
