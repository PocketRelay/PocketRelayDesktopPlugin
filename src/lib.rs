#![warn(unused_crate_dependencies)]

use anyhow::Context;
use log::error;
use std::{net::SocketAddrV4, path::Path, str::FromStr, time::Duration};
use windows_sys::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use crate::rpc::{initialize_rpc, initialize_services};

pub mod game;
pub mod hooks;
pub mod rpc;
pub mod threads;

/// Constant storing the application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handles the plugin being attached to the game
fn attach() {
    // Suspend all game threads so the user has a chance to connect to a server
    threads::suspend_all_threads();

    // Debug allocates a console window to display output
    // #[cfg(debug_assertions)]
    {
        unsafe { windows_sys::Win32::System::Console::AllocConsole() };
    }

    // Initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log_panics::init();

    // Apply hooks
    std::thread::spawn(|| {
        unsafe { hooks::apply_hooks() };
    });

    std::thread::spawn(|| {
        if let Err(error) = connect_rpc() {
            log::error!("error occurred when connecting rpc: {error}")
        }

        threads::resume_all_threads();
    });

    // TODO: Splash while initializing
}

fn connect_rpc() -> anyhow::Result<()> {
    // Create tokio async runtime
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to build async runtime")?;

    // Enter the tokio runtime
    let _enter = runtime.enter();

    let rpc_path = Path::new("./pocket-relay-rpc");

    // Desktop app has not initialized the PRC server configuration
    if !rpc_path.exists() {
        log::debug!("rpc configuration not present, assuming non existent");
        return Ok(());
    }

    let configuration =
        std::fs::read_to_string(rpc_path).context("failed to read rpc configuration")?;

    let address =
        SocketAddrV4::from_str(&configuration).context("failed to parse rpc service address")?;

    let rpc_future = tokio::time::timeout(Duration::from_secs(1), initialize_rpc(address));
    let rpc = match runtime.block_on(rpc_future) {
        Ok(value) => value,

        // Timeout connecting to the RPC means that it probably wasn't running since a
        // localhost connection should be instant
        _ => {
            log::debug!("timeout exceeded waiting for desktop, assuming playing without");
            return Ok(());
        }
    };
    let rpc = rpc?;
    runtime.block_on(initialize_services(&rpc))?;

    Ok(())
}

/// Handles the plugin being detached from the game, this handles
/// cleaning up any extra allocated resources
fn detach() {
    // Debug console must be freed on detach
    #[cfg(debug_assertions)]
    {
        unsafe {
            windows_sys::Win32::System::Console::FreeConsole();
        }
    }
}

/// Windows DLL entrypoint for the plugin
#[no_mangle]
#[allow(non_snake_case)]
extern "stdcall" fn DllMain(_hmodule: isize, reason: u32, _: *mut ()) -> bool {
    match reason {
        // Handle attaching
        DLL_PROCESS_ATTACH => attach(),
        // Handle detaching
        DLL_PROCESS_DETACH => detach(),
        _ => {}
    }

    true
}
