//! The auth hook overrides the games default authentication behavior to
//! force the automatic token based background authentication

use std::{os::raw::c_ulong, thread::sleep, time::Duration};

use anyhow::Context;

use crate::game::{
    core::{find_object_by_type, get_global_object, UClass, UObject},
    sfxonlinefoundation::USFXOnlineComponentOrigin,
};

pub unsafe fn hook_online_component() {
    loop {
        match try_hook_online_component() {
            Ok(_) => {
                log::debug!("successfully hooked online component");
                break;
            }
            Err(error) => {
                log::error!("failed to hook online component: {error}");
                sleep(Duration::from_secs(1));
                continue;
            }
        }
    }
}

unsafe fn try_hook_online_component() -> anyhow::Result<()> {
    let origin_component_static_class = get_global_object(2055)
        .context("unable to find USFXOnlineComponentOrigin static class")?
        .cast::<UClass>();
    log::debug!("found origin component: {origin_component_static_class:?}");

    let origin_component = find_object_by_type(origin_component_static_class)
        .context("unable to find USFXOnlineComponentOrigin instance")?
        .cast::<UObject>();

    let flags_ptr = origin_component.byte_offset(0x18C).cast::<c_ulong>();

    let m_disable_origin_mask = 0x00000001; // mDisableOrigin = true
    let m_is_origin_disabled_mask = 0x00000008; // mIsOriginDisabled = true
    let combined_mask = m_disable_origin_mask | m_is_origin_disabled_mask;
    *flags_ptr = *flags_ptr | combined_mask;

    Ok(())
}
