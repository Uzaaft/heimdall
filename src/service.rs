use std::env;

use auto_launch::AutoLaunchBuilder;
use tracing::info;

use anyhow::Result;
fn get_app_path() -> Result<String> {
    let app_name = match env::current_exe() {
        Ok(app_name) => app_name,
        Err(e) => return Err(e.into()),
    }
    .to_str()
    .unwrap()
    .to_string();
    Ok(app_name)
}

pub fn start_service() -> Result<()> {
    let app_path = get_app_path()?;
    info!("Starting service");
    let auto = AutoLaunchBuilder::new()
        .set_app_name("heimdall")
        .set_app_path(&app_path)
        .set_use_launch_agent(true)
        .build()
        .unwrap();
    match auto.enable().is_ok() {
        true => info!("Service started"),
        false => info!("Service already started"),
    }
    Ok(())
}

pub fn stop_service() -> Result<()> {
    let app_name = get_app_path()?;
    info!("Starting service");
    let auto = AutoLaunchBuilder::new()
        .set_app_name(&app_name)
        .set_app_path(&app_name)
        .set_use_launch_agent(true)
        .build()
        .unwrap();

    match auto.disable().is_ok() {
        true => info!("Service stopped"),
        false => info!("Service already stopped"),
    }
    Ok(())
}

pub fn restart_service() -> Result<()> {
    info!("Restarting service");
    stop_service()?;
    start_service()?;
    Ok(())
}
