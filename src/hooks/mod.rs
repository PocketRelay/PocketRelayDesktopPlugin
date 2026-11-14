use log::debug;

use crate::hooks::auth_hook::hook_online_component;

pub mod auth_hook;
pub mod connect;
pub mod host_lookup;
pub mod pattern;
pub mod process_event;

/// Applies all hooks
#[allow(clippy::missing_safety_doc)]
pub unsafe fn apply_hooks() {
    debug!("apply host lookup");
    host_lookup::hook_host_lookup();
    debug!("apply connect");
    connect::hook();
    debug!("apply process event hook");
    process_event::hook_process_event();
    hook_online_component();
    debug!("all hooks applied")
}
