mod config;
mod error;

use config::Config;
use error::AppResult;
use heimdall_cli::{configure_logger, spawn_command};
use std::{collections::HashMap, fs::File};
use tracing::{info, trace};

use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, hotkey::HotKey};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

#[derive(Debug)]
enum AppEvent {
    HotKey(GlobalHotKeyEvent),
}

struct App {
    /// We will need this when we implement reload
    hotkeys_manager: GlobalHotKeyManager,
    key_command_map: HashMap<u32, String>,
}

impl ApplicationHandler<AppEvent> for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
        unimplemented!();
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::HotKey(event) => {
                println!("{event:?}");
                trace!("Received hotkey event: {:?}", event);
                if global_hotkey::HotKeyState::Released == event.state {
                    info!("key: {:?} released", &self.key_command_map.get(&event.id));
                    spawn_command(self.key_command_map.get(&event.id).unwrap()).unwrap();
                }
            }
        }
    }
}

fn main() -> AppResult<()> {
    configure_logger();

    info!("Starting Heimdall");
    let file = File::create("/tmp/heim.lock")?;
    file.lock()?;
    info!("Lock file acquired");

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

    let mut app = App {
        hotkeys_manager,
        key_command_map,
    };

    let event_loop = EventLoop::<AppEvent>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let proxy = event_loop.create_proxy();

    GlobalHotKeyEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(AppEvent::HotKey(event));
    }));

    event_loop.run_app(&mut app).unwrap();

    file.unlock()?;
    // Remove file
    Ok(std::fs::remove_file("/tmp/heim.lock")?)
}
