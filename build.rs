fn main() {
    #[cfg(feature = "oidn")]
    {
        futures::executor::block_on(oidn::setup_oidn_environment());
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

    pub(crate) async fn setup_oidn_environment() {
        let oidn_dir = get_oidn_dir();
        let oidn_dir = Path::new(&oidn_dir);

        if oidn_dir.exists() {
            return;
        }

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
}
