// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod args;
mod config;
mod service;

use config::Config;
use fs2::FileExt;
use heimdall_cli::{configure_logger, spawn_command};
use std::{collections::HashMap, fs::File};
use tracing::{debug, info, trace};

use clap::Parser;

use anyhow::{anyhow, Result};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() -> Result<()> {
    configure_logger();
    let args = args::Args::parse();
    if args.start_service {
        return service::start_service().map_err(|e| anyhow!(e));
    } else if args.stop_service {
        return service::stop_service().map_err(|e| anyhow!(e));
    } else if args.restart_service {
        return service::restart_service().map_err(|e| anyhow!(e));
    }

    debug!("Starting Heimdall");

    let event_loop = EventLoopBuilder::new().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    let key_command_map: HashMap<u32, String> = Config::read_config()
        .unwrap()
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
    let file = File::open("/tmp/heim.lock")?;
    file.lock_exclusive()?;

    let _ = event_loop
        .run(move |_event, _| {
            if let Ok(event) = global_hotkey_channel.try_recv() {
                trace!("Received hotkey event: {:?}", event);
                match event.state {
                    global_hotkey::HotKeyState::Pressed => {
                        info!("key: {:?} pressed", key_command_map.get(&event.id));
                        spawn_command(key_command_map.get(&event.id).unwrap());
                    }
                    global_hotkey::HotKeyState::Released => {}
                }
            }
        })
        .map_err(|e| anyhow!(e));
    file.unlock().map_err(|e| anyhow!(e))
}
