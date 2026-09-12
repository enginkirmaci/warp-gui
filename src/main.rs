use ksni::blocking::TrayMethods;
use ksni::menu::StandardItem;
use ksni::{MenuItem, Tray};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

const CONNECTED_PNG: &[u8] = include_bytes!("../warp_connect.png");
const DISCONNECTED_PNG: &[u8] = include_bytes!("../warp_disconnected.png");

#[derive(Debug)]
struct WarpTray {
    connected: bool,
    icon_dir: PathBuf,
}

impl Tray for WarpTray {
    fn id(&self) -> String {
        "warp-gui".into()
    }

    fn icon_name(&self) -> String {
        let name = if self.connected {
            "warp_connect.png"
        } else {
            "warp_disconnected.png"
        };
        self.icon_dir.join(name).to_string_lossy().into_owned()
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let connected = self.connected;
        let mut items = vec![];

        if connected {
            items.push(
                StandardItem {
                    label: "Disconnect".into(),
                    activate: Box::new(|this: &mut Self| {
                        if run_warp_cli("disconnect") {
                            thread::sleep(Duration::from_secs(1));
                            this.connected = get_status();
                        }
                    }),
                    ..Default::default()
                }
                .into(),
            );
        } else {
            items.push(
                StandardItem {
                    label: "Connect".into(),
                    activate: Box::new(|this: &mut Self| {
                        if run_warp_cli("connect") {
                            thread::sleep(Duration::from_secs(2));
                            this.connected = get_status();
                        }
                    }),
                    ..Default::default()
                }
                .into(),
            );
        }

        items.push(MenuItem::Separator);
        items.push(
            StandardItem {
                label: "Quit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        );

        items
    }
}

fn get_status() -> bool {
    Command::new("warp-cli")
        .arg("status")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.to_lowercase().contains("status update: connected"))
        .unwrap_or(false)
}

fn run_warp_cli(arg: &str) -> bool {
    Command::new("warp-cli")
        .arg(arg)
        .status()
        .ok()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() {
    let icon_dir = std::env::temp_dir().join("warp-gui");
    fs::create_dir_all(&icon_dir).expect("failed to create icon directory");
    fs::write(icon_dir.join("warp_connect.png"), CONNECTED_PNG)
        .expect("failed to write connected icon");
    fs::write(icon_dir.join("warp_disconnected.png"), DISCONNECTED_PNG)
        .expect("failed to write disconnected icon");

    let connected = get_status();

    let _handle = loop {
        let tray = WarpTray {
            connected,
            icon_dir: icon_dir.clone(),
        };
        match tray.spawn() {
            Ok(handle) => break handle,
            Err(_) => {
                eprintln!("warp-gui: tray not ready, retrying in 2s...");
                thread::sleep(Duration::from_secs(2));
            }
        }
    };

    loop {
        thread::park();
    }
}