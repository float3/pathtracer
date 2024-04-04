#[tokio::main]
async fn main() {
    #[cfg(feature = "oidn")]
    {
        let oidn_dir = std::env::var("OIDN_DIR").expect("OIDN_DIR environment variable not set. Please set this to the OIDN install directory root.");
        oidn::setup_oidn_environment(&oidn_dir).await;
    }
}

#[cfg(feature = "oidn")]
mod oidn {
    pub(crate) async fn setup_oidn_environment(oidn_dir: &str) {
        if std::path::Path::new(oidn_dir).exists() {
            std::fs::remove_dir_all(oidn_dir).expect("Failed to delete OIDN directory");
        }
        download_oidn(oidn_dir).await;
        extract_oidn(oidn_dir)
            .await
            .expect("Failed to extract OIDN binaries");
        let oidn_bin_path = std::path::Path::new(oidn_dir).join("bin");
        copy_oidn_binaries(&oidn_bin_path).await;
    }

    pub(crate) async fn extract_oidn(oidn_dir: &str) -> std::io::Result<()> {
        let target = std::env::var("TARGET").expect("TARGET environment variable not set.");
        let archive_path = construct_archive_path(oidn_dir, &target);

        match archive_path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("zip") => {
                extract_zip(&archive_path, oidn_dir).await?;
            }
            Some("gz") => {
                extract_tar_gz(&archive_path, oidn_dir).await?;
            }
            Some(ext) => panic!("Unknown archive format: {}", ext),
            None => panic!("No extension found"),
        }
        Ok(())
    }

    pub(crate) fn construct_archive_path(oidn_dir: &str, target: &str) -> std::path::PathBuf {
        let oidn_dir_parent = std::path::Path::new(oidn_dir)
            .parent()
            .expect("OIDN directory has no parent")
            .to_path_buf();
        let mut version = String::new();
        std::io::Read::read_to_string(
            &mut std::fs::File::open(oidn_dir_parent.join("version"))
                .expect("Failed to open version file"),
            &mut version,
        )
        .expect("Failed to read version file");
        let version = version.trim_start_matches("v");
        let file_name = if target.contains("windows") {
            format!("oidn-{}.x64.windows.zip", version)
        } else if target.contains("linux") {
            format!("oidn-{}.x86_64.linux.tar.gz", version)
        } else if target.contains("darwin") && target.contains("aarch64") {
            format!("oidn-{}.arm64.macos.tar.gz", version)
        } else if target.contains("darwin") {
            format!("oidn-{}.x86_64.macos.tar.gz", version)
        } else {
            panic!("Unsupported target: {}", target);
        };

        oidn_dir_parent.join(file_name)
    }

    pub(crate) async fn extract_zip(
        archive_path: &std::path::Path,
        oidn_dir: &str,
    ) -> std::io::Result<()> {
        let archive_path = archive_path.to_owned();
        let oidn_dir = oidn_dir.to_owned();

        tokio::task::spawn_blocking(move || {
            let file = std::fs::File::open(archive_path)?;
            let mut archive = zip::ZipArchive::new(file)?;
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let name = std::path::PathBuf::from(
                    file.mangled_name()
                        .to_str()
                        .expect("Failed to convert mangled name to string")
                        .split('\\')
                        .skip(1)
                        .collect::<Vec<&str>>()
                        .join("\\"),
                );

                let outpath = std::path::Path::new(&oidn_dir).join(name);

                if file.name().ends_with('/') {
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            std::fs::create_dir_all(p)?;
                        }
                    }
                    let mut outfile = std::fs::File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
            }
            Ok(())
        })
        .await?
    }

    pub(crate) async fn extract_tar_gz(
        archive_path: &std::path::Path,
        oidn_dir: &str,
    ) -> std::io::Result<()> {
        let archive_path = archive_path.to_owned();
        let oidn_dir = oidn_dir.to_owned();
        tokio::task::spawn_blocking(move || {
            let tar_gz = std::fs::File::open(&archive_path)?;
            let tar = flate2::read::GzDecoder::new(tar_gz);
            let mut archive = tar::Archive::new(tar);
            let oidn_path = std::path::Path::new(&oidn_dir);
            let parent_dir = oidn_path.parent().expect("Directory has no parent");
            archive.unpack(parent_dir)?;

            let binding = archive_path.display().to_string();
            let extracted_dir = binding.split(".tar.gz").collect::<Vec<&str>>()[0];
            let extracted_dir = std::path::Path::new(&extracted_dir);
            std::fs::rename(extracted_dir, oidn_path)?;

            Ok(())
        })
        .await?
    }

    pub(crate) async fn copy_oidn_binaries(oidn_bin_path: &std::path::Path) {
        let mut entries = tokio::fs::read_dir(oidn_bin_path)
            .await
            .expect("Error finding OIDN binaries");
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let mut output_path = get_output_path();
            output_path.push(file_name);
            tokio::fs::copy(path, output_path)
                .await
                .expect("Failed to copy OIDN binary");
        }
    }

    pub(crate) fn get_output_path() -> std::path::PathBuf {
        let manifest_dir_string = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let build_type = std::env::var("PROFILE").unwrap();
        let path = std::path::Path::new(&manifest_dir_string)
            .join("target")
            .join(build_type);
        path
    }

    pub(crate) async fn download_oidn(oidn_dir: &str) {
        let version_file = std::path::Path::new(oidn_dir)
            .parent()
            .expect("OIDN directory has no parent")
            .join("version");
        const RELEASES_URL: &str =
            "https://api.github.com/repos/OpenImageDenoise/oidn/releases/tags/v2.1.0"; //until oidn-rs updates to 2.2.2

        #[derive(serde::Deserialize)]
        struct Release {
            tag_name: String,
            assets: Vec<Asset>,
        }

        #[derive(serde::Deserialize)]
        struct Asset {
            browser_download_url: String,
            name: String,
        }

        let oidn_dir = std::path::PathBuf::from(oidn_dir);
        let oidn_dir_parent: String = oidn_dir.parent().unwrap().display().to_string();

        let client = reqwest::Client::new();
        let response: Release = match client
            .get(RELEASES_URL)
            .header("User-Agent", "reqwest")
            .send()
            .await
            .expect("Failed to send request")
            .json::<Release>()
            .await
        {
            Ok(response) => response,
            Err(err) => {
                eprintln!("Failed to parse JSON: {}", err);
                eprintln!("Response: {:?}", err.status());
                return;
            }
        };

        let mut current_version = String::new();
        if version_file.exists() {
            current_version =
                std::fs::read_to_string(&version_file).expect("Failed to read version file");
        }

        if current_version.trim() != response.tag_name {
            println!("Version mismatch. Updating...");

            if let Ok(entries) = std::fs::read_dir(&oidn_dir_parent) {
                entries.filter_map(Result::ok).for_each(|entry| {
                    let path = entry.path();
                    if path.is_file()
                        && (path.extension().map_or(false, |ext| ext == "zip")
                            || path.extension().map_or(false, |ext| ext == "tar.gz"))
                    {
                        std::fs::remove_file(path).expect("Failed to delete file");
                    }
                });
            }

            for asset in response.assets.iter().filter(|a| {
                a.name.contains("linux") || a.name.contains("macos") || a.name.contains("windows")
            }) {
                let response = client
                    .get(&asset.browser_download_url)
                    .header("User-Agent", "reqwest")
                    .send()
                    .await
                    .expect("Failed to download asset");

                let mut file = std::fs::File::create(format!("{}/{}", oidn_dir_parent, asset.name))
                    .expect("Failed to create file");
                let bytes = response
                    .bytes()
                    .await
                    .expect("Failed to read response bytes");
                std::io::copy(&mut bytes.as_ref(), &mut file).expect("Failed to write to file");
            }

            std::fs::write(&version_file, response.tag_name.as_bytes())
                .expect("Failed to write new version");
        } else {
            println!("Version matches. No update needed.");
        }
    }
}
