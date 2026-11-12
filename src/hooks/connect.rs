use log::debug;
use minhook::MinHook;
use std::mem::size_of;
use std::os::raw::c_void;
use std::sync::LazyLock;
use windows_sys::Win32::{
    Networking::WinSock::{AF_INET, SOCKADDR, SOCKADDR_IN, SOCKET},
    System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
};

use crate::rpc::redirector_service_addr;

type ConnectFn = unsafe extern "system" fn(SOCKET, *const SOCKADDR, i32) -> i32;

static ORIGINAL_CONNECT: LazyLock<(ConnectFn, ConnectFn)> = LazyLock::new(|| unsafe {
    let ws2_32 = GetModuleHandleA(b"ws2_32.dll\0".as_ptr());
    if ws2_32.is_null() {
        panic!("Failed to obtain handle for ws2_32.dll");
    }

    let target_addr = GetProcAddress(ws2_32, b"connect\0".as_ptr());
    let Some(target_fn) = target_addr else {
        panic!("Failed to locate address of connect() inside ws2_32.dll");
    };
    let target_fn: ConnectFn = std::mem::transmute(target_fn);

    let ori: ConnectFn = std::mem::transmute(target_fn);
    let original =
        MinHook::create_hook(target_fn as _, fake_connect as _).expect("failed to create hook");
    let original: ConnectFn = std::mem::transmute(original);
    (original, target_fn)
});

pub const REDIRECTOR_PORT: u16 = 42127;

#[no_mangle]
pub unsafe extern "system" fn fake_connect(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32 {
    let (connect, _) = *ORIGINAL_CONNECT;

    if !name.is_null() && namelen as usize >= size_of::<SOCKADDR_IN>() {
        let name = name as *mut SOCKADDR_IN;
        let sockaddr_in = name.as_mut_unchecked();

        if sockaddr_in.sin_family == AF_INET {
            let bytes = sockaddr_in.sin_addr.S_un.S_un_b;
            let port = u16::from_be(sockaddr_in.sin_port);

            debug!(
                "connect attempt to {}.{}.{}.{}:{}",
                bytes.s_b1, bytes.s_b2, bytes.s_b3, bytes.s_b4, port
            );

            // If a redirector service is available redirect the default port to the service
            if bytes.s_b1 == 127
                && bytes.s_b2 == 0
                && bytes.s_b3 == 0
                && bytes.s_b4 == 1
                && port == REDIRECTOR_PORT
            {
                if let Some(service_addr) = redirector_service_addr() {
                    let port: u16 = service_addr.port();
                    sockaddr_in.sin_port = port.to_be();
                }
            }
        }
    }

    connect(s, name, namelen)
}

/// Hooks the connect() function from ws2_32.dll to override the port that
/// clients connect to for the redirector
pub unsafe fn hook() {
    let (_, target) = *ORIGINAL_CONNECT;
    MinHook::enable_hook(target as *mut c_void).expect("failed to enable connect hook");
}
