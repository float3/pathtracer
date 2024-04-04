#[tokio::main]
async fn main() {
    #[cfg(feature = "oidn")]
    {
        oidn::setup_oidn_environment().await;
    }
}

#[cfg(feature = "oidn")]
mod oidn {
    use std::{
        env,
        path::{Path, PathBuf},
    };

    pub fn get_oidn_dir() -> String {
        env::var("OIDN_DIR").expect("OIDN_DIR environment variable not set. Please set this to the OIDN install directory root.")
    }

    pub fn get_oidn_version() -> String {
        env::var("OIDN_VER").expect("OIDN_VER environment variable not set. Please set this to the OIDN version you want to install.")
    }

    pub fn get_current_oidn_version() -> String {
        let oidn_dir = get_oidn_dir();
        let version_file = Path::new(&oidn_dir)
            .parent()
            .expect("OIDN directory has no parent")
            .join("version");

        let mut current_version = String::new();
        if version_file.exists() {
            current_version =
                std::fs::read_to_string(&version_file).expect("Failed to read version file");
        }
        current_version
    }

    pub(crate) async fn setup_oidn_environment() {
        let oidn_dir = get_oidn_dir();
        let oidn_dir = Path::new(&oidn_dir);

        if oidn_dir.exists() {
            std::fs::remove_dir_all(oidn_dir).expect("Failed to delete OIDN directory");
        }

        download_oidn(oidn_dir)
            .await
            .expect("Failed to download OIDN binaries");
        extract_oidn(oidn_dir)
            .await
            .expect("Failed to extract OIDN binaries");
        copy_oidn_binaries(&oidn_dir.join("bin")).await;
    }

    pub(crate) async fn extract_oidn(oidn_dir: &Path) -> std::io::Result<()> {
        let target = env::var("TARGET").expect("TARGET environment variable not set.");
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

    pub(crate) fn construct_archive_path(oidn_dir: &Path, target: &str) -> PathBuf {
        let oidn_dir_parent = oidn_dir
            .parent()
            .expect("OIDN directory has no parent")
            .to_path_buf();

        let version = get_oidn_version();

        let version = version.trim_start_matches('v');
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

    pub(crate) async fn extract_zip(archive_path: &Path, oidn_dir: &Path) -> std::io::Result<()> {
        let archive_path = archive_path.to_owned();
        let oidn_dir = oidn_dir.to_owned();

        tokio::task::spawn_blocking(move || {
            let file = std::fs::File::open(archive_path)?;
            let mut archive = zip::ZipArchive::new(file)?;
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let name = PathBuf::from(
                    file.mangled_name()
                        .to_str()
                        .expect("Failed to convert mangled name to string")
                        .split('\\')
                        .skip(1)
                        .collect::<Vec<&str>>()
                        .join("\\"),
                );

                let outpath = Path::new(&oidn_dir).join(name);

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
        archive_path: &Path,
        oidn_dir: &Path,
    ) -> std::io::Result<()> {
        let archive_path = archive_path.to_owned();
        let oidn_dir = oidn_dir.to_owned();
        tokio::task::spawn_blocking(move || {
            let tar_gz = std::fs::File::open(&archive_path)?;
            let tar = flate2::read::GzDecoder::new(tar_gz);
            let mut archive = tar::Archive::new(tar);
            let oidn_path = Path::new(&oidn_dir);
            let parent_dir = oidn_path.parent().expect("Directory has no parent");
            archive.unpack(parent_dir)?;

            let binding = archive_path.display().to_string();
            let extracted_dir = binding.split(".tar.gz").collect::<Vec<&str>>()[0];
            let extracted_dir = Path::new(&extracted_dir);
            std::fs::rename(extracted_dir, oidn_path)?;

            Ok(())
        })
        .await?
    }

    pub(crate) async fn copy_oidn_binaries(oidn_bin_path: &Path) {
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

    pub(crate) fn get_output_path() -> PathBuf {
        let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
        let build_type = env::var("PROFILE").unwrap();
        let path = Path::new(&manifest_dir_string)
            .join("target")
            .join(build_type);
        path
    }

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

    pub(crate) async fn download_oidn(oidn_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let version_file = oidn_dir
            .parent()
            .expect("OIDN directory has no parent")
            .join("version");

        let current_version = get_current_oidn_version();
        let version = get_oidn_version();
        const BASE_URL: &str = "https://api.github.com/repos/OpenImageDenoise/oidn/releases";
        let latest_release_url = format!("{}/latest", BASE_URL);
        let current_release_url = format!("{}/tags/{}", BASE_URL, version);
        let oidn_dir_parent = oidn_dir.parent().ok_or("OIDN directory has no parent")?;

        println!("Current version: {}", current_release_url);

        let client = reqwest::Client::new();
        let response = fetch_release(&client, &current_release_url).await;
        let latest_response = fetch_release(&client, &latest_release_url).await;

        if let Err(e) = response {
            println!("Failed to fetch current release: {}", e);
            return Ok(());
        }
        let response = response.unwrap();
        if let Err(e) = latest_response {
            println!("Failed to fetch latest release: {}", e);
            return Ok(());
        }
        let latest_response = latest_response.unwrap();

        if current_version.trim() != latest_response.tag_name {
            println!("Newer version available: {}", latest_response.tag_name);
        }

        if current_version != version {
            println!("Version mismatch. Updating...");

            cleanup_directory(oidn_dir_parent)?;

            for asset in response.assets.iter().filter(|a| {
                a.name.contains("linux") || a.name.contains("macos") || a.name.contains("windows")
            }) {
                download_and_save_asset(&client, asset, oidn_dir_parent).await?;
            }

            std::fs::write(&version_file, response.tag_name.as_bytes())
                .expect("Failed to write new version");
        } else {
            println!("Version matches. No update needed.");
        }
        Ok(())
    }

    async fn fetch_release(
        client: &reqwest::Client,
        url: &str,
    ) -> Result<Release, Box<dyn std::error::Error>> {
        Ok(client
            .get(url)
            .header("User-Agent", "reqwest")
            .send()
            .await?
            .json::<Release>()
            .await?)
    }

    fn cleanup_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_file()
                && (path.extension() == Some("zip".as_ref())
                    || path.extension() == Some("tar.gz".as_ref()))
            {
                std::fs::remove_file(path)?;
            }
        }
        Ok(())
    }

    async fn download_and_save_asset(
        client: &reqwest::Client,
        asset: &Asset,
        dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = client
            .get(&asset.browser_download_url)
            .header("User-Agent", "reqwest")
            .send()
            .await?;
        let mut file = std::fs::File::create(dir.join(&asset.name))?;
        let bytes = response.bytes().await?;
        std::io::copy(&mut bytes.as_ref(), &mut file)?;
        Ok(())
    }
}
