use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use console::style;

const API_URL: &str = "https://api.github.com/repos/KirioXX/shelly/releases/latest";

#[derive(Debug, serde::Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Debug, serde::Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub async fn update() -> Result<(), Box<dyn Error>> {
    let current_version = env!("CARGO_PKG_VERSION");

    println!(
        "{} Checking for updates… (current: {})",
        style("🔄").cyan(),
        current_version
    );

    let release = fetch_latest_release().await?;
    let tag = release.tag_name.clone();
    let latest_version = tag.strip_prefix('v').unwrap_or(&tag);

    if latest_version == current_version {
        println!(
            "{} Already on the latest version ({}).",
            style("✓").green(),
            current_version
        );
        return Ok(());
    }

    println!(
        "{} New version available: {} (you have {})",
        style("⬆").yellow(),
        style(&tag).bold(),
        style(current_version).dim()
    );

    print!("  Proceed with update? [Y/n] ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    let input = buf.trim().to_lowercase();

    if !input.is_empty() && !input.starts_with('y') {
        println!("Update cancelled.");
        return Ok(());
    }

    let asset = find_asset(&release)?;
    let tmp_dir = make_temp_dir()?;
    let archive_path = download_asset(&asset.browser_download_url, &tmp_dir).await?;
    let extracted = extract_archive(&archive_path, &tmp_dir)?;

    // On macOS, check architecture of downloaded binary vs current process
    #[cfg(target_os = "macos")]
    {
        let file_output = std::process::Command::new("file")
            .arg(extracted.to_str().unwrap())
            .output()
            .ok();
        if let Some(out) = file_output {
            let info = String::from_utf8_lossy(&out.stdout);
            let local_arch = env::consts::ARCH;
            let binary_arch = if info.contains("arm64") || info.contains("aarch64") {
                "aarch64"
            } else if info.contains("x86_64") {
                "x86_64"
            } else {
                "unknown"
            };
            if local_arch != binary_arch && binary_arch != "unknown" {
                eprintln!(
                    "{} Architecture mismatch: running on {} but downloaded binary is {}",
                    style("⚠").yellow(),
                    local_arch,
                    binary_arch
                );
                eprintln!("  The update may fail. Consider building from source with 'cargo install'.");
            }
        }
    }

    replace_binary(&extracted).await?;

    // Clean up temp dir
    let _ = fs::remove_dir_all(&tmp_dir);

    println!("{} Updated to {}!", style("✓").green(), style(&tag).bold());

    Ok(())
}

async fn fetch_latest_release() -> Result<Release, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    let resp = client.get(API_URL).send().await?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API returned {}", resp.status()).into());
    }

    let release: Release = resp.json().await?;
    Ok(release)
}

fn find_asset(release: &Release) -> Result<&Asset, Box<dyn Error>> {
    let os = match env::consts::OS {
        "linux" => "ubuntu-latest",
        "macos" => "macos-latest",
        "windows" => "windows-latest",
        other => return Err(format!("Unsupported OS: {}", other).into()),
    };

    let ext = if os == "windows-latest" {
        "zip"
    } else {
        "tar.gz"
    };

    let pattern = format!("shelly-{}-{}", os, release.tag_name);

    let asset = release
        .assets
        .iter()
        .find(|a| a.name.starts_with(&pattern) && a.name.ends_with(ext));

    match asset {
        Some(a) => Ok(a),
        None => {
            let available: Vec<_> = release
                .assets
                .iter()
                .filter(|a| a.name.contains("shelly-"))
                .map(|a| a.name.clone())
                .collect();
            Err(format!(
                "No asset found for OS '{}' ({}).\nAvailable: {}",
                os,
                ext,
                available.join(", ")
            )
            .into())
        }
    }
}

async fn download_asset(url: &str, tmp_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    println!("{} Downloading…", style("⬇").cyan());
    let resp = client.get(url).send().await?;

    if !resp.status().is_success() {
        return Err(format!("Download failed: {}", resp.status()).into());
    }

    let bytes = resp.bytes().await?;
    let filename = url.rsplit('/').next().unwrap_or("download");
    let path = tmp_dir.join(filename);

    fs::write(&path, &bytes)?;
    println!("  Saved {} bytes", style(bytes.len()).dim());

    Ok(path)
}

fn extract_archive(archive: &Path, tmp_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    println!("{} Extracting…", style("📂").cyan());

    let dest = tmp_dir.join("extracted");
    fs::create_dir_all(&dest)?;

    let output = std::process::Command::new("tar")
        .args([
            "-xf",
            archive.to_str().unwrap(),
            "-C",
            dest.to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        // Fallback: try platform-specific extraction
        #[cfg(windows)]
        {
            let ps_output = std::process::Command::new("powershell")
                .args([
                    "-Command",
                    &format!(
                        "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                        archive.to_str().unwrap(),
                        dest.to_str().unwrap()
                    ),
                ])
                .output()?;
            if !ps_output.status.success() {
                return Err("Failed to extract archive".into());
            }
        }
        #[cfg(not(windows))]
        {
            return Err("Failed to extract archive".into());
        }
    }

    let binary_name = if env::consts::OS == "windows" {
        "shelly.exe"
    } else {
        "shelly"
    };

    let binary_path = dest.join(binary_name);
    if binary_path.exists() {
        return Ok(binary_path);
    }

    // Search one level deep
    for entry in fs::read_dir(&dest)? {
        let entry = entry?;
        if entry.path().join(binary_name).exists() {
            return Ok(entry.path().join(binary_name));
        }
    }

    Err(format!("Binary '{}' not found after extraction", binary_name).into())
}

async fn replace_binary(new_binary: &Path) -> Result<(), Box<dyn Error>> {
    let current = env::current_exe()?;
    let backup = current.with_extension("backup");

    println!("{} Replacing binary…", style("🔄").cyan());
    println!("  Current: {}", current.display());

    // Back up existing binary
    if current.exists() {
        if backup.exists() {
            let _ = fs::remove_file(&backup);
        }
        fs::copy(&current, &backup)?;
    }

    #[cfg(unix)]
    {
        fs::copy(new_binary, &current)?;
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&current, fs::Permissions::from_mode(0o755))?;

        // On macOS, strip the quarantine attribute so the downloaded binary can run
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("xattr")
                .args(["-d", "com.apple.quarantine", current.to_str().unwrap()])
                .output();
            // Also try to remove any other extended attrs that might block execution
            let _ = std::process::Command::new("xattr")
                .args(["-c", current.to_str().unwrap()])
                .output();
        }
    }

    #[cfg(windows)]
    {
        // On Windows we cannot overwrite a running executable.
        // Rename the running binary out of the way and place the new one.
        let tmp_old = current.with_extension("old.exe");
        fs::rename(&current, &tmp_old)?;
        fs::copy(new_binary, &current)?;
        let _ = fs::remove_file(&tmp_old);
    }

    // Verify — try --version first, then `version` subcommand as fallback
    println!("  Verifying…");
    let verify_output = std::process::Command::new(&current)
        .args(["--version"])
        .output();

    let (success, stdout, stderr, status) = match verify_output {
        Ok(out) => (
            out.status.success(),
            String::from_utf8_lossy(&out.stdout).trim().to_string(),
            String::from_utf8_lossy(&out.stderr).trim().to_string(),
            format!("{}", out.status),
        ),
        Err(e) => {
            eprintln!("  {} Failed to spawn verification: {}", style("✗").red(), e);
            (false, String::new(), e.to_string(), "spawn error".to_string())
        }
    };

    if !success {
        eprintln!(
            "  {} Verification failed (exit: {})",
            style("✗").red(),
            status
        );
        if !stdout.is_empty() {
            eprintln!("  stdout: {}", stdout);
        }
        if !stderr.is_empty() {
            eprintln!("  stderr: {}", stderr);
        }

        // Restore backup
        if backup.exists() {
            fs::copy(&backup, &current)?;
            let _ = fs::remove_file(&backup);
        }
        return Err("Updated binary failed verification. Restored previous version.".into());
    }

    println!("  {} Verified: {}", style("✓").green(), style(&stdout).dim());

    // Clean up backup on success
    let _ = fs::remove_file(&backup);

    Ok(())
}

fn make_temp_dir() -> Result<PathBuf, Box<dyn Error>> {
    let tmp = env::temp_dir().join(format!(
        "shelly-update-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs()
    ));
    fs::create_dir_all(&tmp)?;
    Ok(tmp)
}

/// Clean up leftover files from a previous Windows self-update.
pub fn cleanup_windows_update() {
    #[cfg(windows)]
    {
        if let Ok(exe) = env::current_exe() {
            let old = exe.with_extension("old.exe");
            let _ = fs::remove_file(&old);
        }
    }
}
