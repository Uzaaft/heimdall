// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod config;

use config::Config;
use fs4::fs_std::FileExt;
use heimdall_cli::{configure_logger, spawn_command};
use std::{collections::HashMap, fs::File};
use tracing::{error, info, trace};

use anyhow::{bail, Result};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<()> {
    configure_logger();

    info!("Starting Heimdall");
    info!("Aquiring lock file");
    let file = File::create("/tmp/heim.lock")?;
    if file.try_lock_exclusive().is_err() {
        error!("Couldn't aquire lock-file. Aborting..");
        bail!("Couldn't aquire lock-file. Aborting..");
    }
    info!("Lock file aquired");

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let hotkeys_manager = GlobalHotKeyManager::new()?;

    let key_command_map: HashMap<u32, String> = Config::read_config()?
        .bindings
        .iter()
        .map(|hotkey| {
            let key: HotKey = hotkey.to_string().parse().unwrap();
            info!(
                "Registering hotkey: {:?} with command {:?}",
                key, hotkey.command
            );
            hotkeys_manager.register(key).unwrap();
            (key.id(), hotkey.command.to_string())
        })
        .collect();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop.run(move |_event, _| {
        if let Ok(event) = global_hotkey_channel.try_recv() {
            trace!("Received hotkey event: {:?}", event);
            if global_hotkey::HotKeyState::Released == event.state {
                info!("key: {:?} released", key_command_map.get(&event.id));
                spawn_command(key_command_map.get(&event.id).unwrap());
            }
        }
    })?;

    file.unlock()?;
    // Remove file
    Ok(std::fs::remove_file("/tmp/heim.lock")?)
}
