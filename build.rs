fn main() {
    #[cfg(feature = "oidn")]
    oidn::setup_oidn_environment();
}

#[cfg(feature = "oidn")]
mod oidn {
    use std::{
        env, fs, io,
        path::{Path, PathBuf},
    };

    fn manifest_dir() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
    }

    pub fn get_oidn_dir() -> PathBuf {
        manifest_dir().join(
            env::var("OIDN_DIR")
                .expect("OIDN_DIR env var not set (set it to path like ./oidn/oidn)"),
        )
    }

    pub fn get_oidn_version() -> String {
        env::var("OIDN_VER").expect("OIDN_VER env var not set (ex: v2.3.3)")
    }

    pub fn setup_oidn_environment() {
        let oidn_dir = get_oidn_dir();

        // extraction ALWAYS re-runs; good for dev, deterministic
        extract_oidn(&oidn_dir).expect("Failed to extract OIDN binaries");
        copy_oidn_binaries(&oidn_dir.join("bin"));
    }

    pub fn extract_oidn(oidn_dir: &Path) -> io::Result<()> {
        let target = env::var("TARGET").expect("TARGET environment variable not set");

        let archive_path = construct_archive_path(&target);
        if !archive_path.exists() {
            panic!(
                "OIDN archive not found: {}\nExpected archive inside: {}",
                archive_path.display(),
                manifest_dir().display()
            );
        }

        let ext = archive_path.extension().and_then(|x| x.to_str());

        match ext {
            Some("zip") => extract_zip(&archive_path, oidn_dir),
            Some("gz") => extract_tar_gz(&archive_path, oidn_dir),
            _ => panic!("Unknown OIDN archive format: {:?}", ext),
        }
    }

    pub fn construct_archive_path(target: &str) -> PathBuf {
        let ver = get_oidn_version().trim_start_matches('v').to_string();

        let file = if target.contains("windows") {
            format!("oidn-{ver}.x64.windows.zip")
        } else if target.contains("linux") {
            format!("oidn-{ver}.x86_64.linux.tar.gz")
        } else if target.contains("darwin") && target.contains("aarch64") {
            format!("oidn-{ver}.arm64.macos.tar.gz")
        } else if target.contains("darwin") {
            format!("oidn-{ver}.x86_64.macos.tar.gz")
        } else {
            panic!("Unsupported target triple for OIDN: {target}");
        };

        manifest_dir().join("oidn").join(file)
    }

    pub fn extract_zip(archive_path: &Path, oidn_dir: &Path) -> io::Result<()> {
        let file = fs::File::open(archive_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        if oidn_dir.exists() {
            fs::remove_dir_all(oidn_dir)?;
        }
        fs::create_dir_all(oidn_dir)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.mangled_name();
            let name = name.to_string_lossy();

            let relative = name.split('\\').skip(1).collect::<Vec<_>>().join("/");

            let outpath = oidn_dir.join(relative);

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut out = fs::File::create(&outpath)?;
                io::copy(&mut file, &mut out)?;
            }
        }

        Ok(())
    }

    pub fn extract_tar_gz(archive_path: &Path, oidn_dir: &Path) -> io::Result<()> {
        let tar_gz = fs::File::open(archive_path)?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);

        let parent_dir = oidn_dir.parent().unwrap();

        if oidn_dir.exists() {
            fs::remove_dir_all(oidn_dir)?;
        }

        archive.unpack(parent_dir)?;

        let extracted_dir = fs::read_dir(parent_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| p.is_dir() && p.file_name().unwrap().to_string_lossy().contains("oidn"))
            .expect("Could not locate extracted OIDN directory");

        fs::rename(extracted_dir, oidn_dir)?;

        Ok(())
    }

    pub fn copy_oidn_binaries(bin_dir: &Path) {
        if !bin_dir.exists() {
            panic!(
                "OIDN bin directory missing after extraction: {}",
                bin_dir.display()
            );
        }

        let entries = fs::read_dir(bin_dir).expect("Error reading OIDN bin directory");

        for entry in entries {
            let entry = entry.expect("Bad fs entry");
            let path = entry.path();
            let file = path.file_name().unwrap();

            let mut out = get_output_path();
            out.push(file);

            fs::create_dir_all(out.parent().unwrap()).unwrap();

            fs::copy(&path, &out).unwrap_or_else(|_| {
                panic!("Failed to copy OIDN binary {}", file.to_string_lossy())
            });
        }
    }

    pub fn get_output_path() -> PathBuf {
        let manifest = manifest_dir();
        let profile = env::var("PROFILE").unwrap();
        manifest.join("target").join(profile)
    }
}
