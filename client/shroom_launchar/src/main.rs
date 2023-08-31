#![allow(dead_code)]
use std::{fs::File, path::Path, process::Command, time::Duration};

use config::Config;
use indicatif::ProgressBar;
use replacer::{Patch, Replacer};
use serde::{Serialize, Deserialize};
use shrooming::files::DownloadProgressWatcher;
use tuf_update::{ProgressWatcher, UpdateProgress, Updater, UpdaterBuilder};

mod replacer;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShroomConfig {
    pub tuf_url: String,
    pub server_addr: String,
    pub server_port: u16,
    pub client_exe: String
}

/*
    Plan:
        * Provide an update api for the proxy dll
        * Write updating in a generic way maybe use bidiff crate for now only the launcher + proxy dll are required to be updated
        * Enable the launcher to unpack(for now) wz files into img files with a folder structure, later use tar or another proven archive format
        * Etablish a grpc communication to the server
        * Allow dev auto login bringing the client straight into the game
        * transmit crash/exception info


*/

fn patch_import(file: impl AsRef<Path>) -> anyhow::Result<()> {
    let patch = vec![Patch::Replace {
        needle: b"dinput8",
        replace: b"dinpuz8",
    }];
    let new_file = file.as_ref().to_path_buf().with_extension(".exe.patched");
    let r = File::open(file)?;
    let w = File::create(new_file)?;

    let mut patcher = Replacer::new(r, w, patch);
    patcher.run::<4096>()?;

    Ok(())
}

#[derive(Debug)]
pub struct DownloadProgressWatcherBar(ProgressBar);

impl Default for DownloadProgressWatcherBar {
    fn default() -> Self {
        let pb = ProgressBar::new(100);
        /*pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));*/
        Self(pb)
    }
}

impl DownloadProgressWatcher for DownloadProgressWatcherBar {
    fn update(&self, rx: u64, total: u64) {
        let perc = (rx * 100) / total;
        self.0.set_position(perc);
    }
}

impl ProgressWatcher for DownloadProgressWatcherBar {
    fn update_progress(&self, progress: UpdateProgress) {
        match progress {
            UpdateProgress::UpdateFileProgress(rx, total) => {
                self.update(rx, total);
            }
            UpdateProgress::StartFileDownload(file) => {
                self.update(0, 100);
                self.0
                    .set_message(format!("Downloading {}", file.resolved()));
            }
            UpdateProgress::FinishFileDownload => {
                self.update(100, 100);
                self.0.set_message("Finished Downloading");
            }
            UpdateProgress::FinishUpdate => {
                self.update(100, 100);
                self.0.set_message("Finished Updating");
            }
        }
    }
}

fn launch_shroom(cmd: &str, addr: &str, port: u16, token: Option<&str>) -> anyhow::Result<()> {
    let mut cmd = Command::new(cmd);
    let cmd_ = cmd.arg(addr).arg(port.to_string());

    if let Some(token) = token {
        cmd_.env("SHROOM_TOKEN", token);
    }

    cmd_.spawn()?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cfg: ShroomConfig = Config::builder()
        // Add in `./shroom_config.toml`
        .add_source(config::File::with_name("shroom_config"))
        .build()
        .unwrap()
        .try_deserialize()?;

    println!("Shroom Launchar v1.0");

    let repo = Updater::load_basic_http_repo(&cfg.tuf_url, "tuf")?;
    let exe = std::env::current_exe().expect("current_exe");
    let exe_target = exe.file_name().expect("file_name").to_str().unwrap();
    let cur = std::env::current_dir().expect("current_dir");
    println!("Exe target: {:?}", exe_target);
    let mut updater = UpdaterBuilder::default()
        .repo(repo)
        .dist_dir(cur)
        .manifest_file("tuf/manifest.json")
        .safe_delete_exe_target(exe_target)
        .build()?;

    //TODO handle progress bar
    let update_result = updater.update()?;
    println!("Update result: {:?}", update_result);

    launch_shroom("shroom.exe", &cfg.server_addr, cfg.server_port, Some("admin:test1234"))?;
    std::thread::sleep(Duration::from_secs(2));

    Ok(())
}
