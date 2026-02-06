#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let exe_folder = std::env::current_exe().unwrap();
    let export_path = exe_folder.parent().unwrap().join("export");

    fs::create_dir(export_path.clone()).ok();

    let (backend_receiver, backend_handle, mut frontend_receiver, frontend_handle) =
        bridge::handle::create_pair();

    backend::start(
        export_path,
        frontend_handle,
        backend_handle.clone(),
        backend_receiver,
    );
    frontend::start("Bingo Gen", backend_handle, frontend_receiver);
}
