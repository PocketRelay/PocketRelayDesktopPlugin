use std::net::SocketAddrV4;

use anyhow::Context;
use parking_lot::Mutex;
use pocket_relay_desktop_client_rpc::DesktopAppServiceClient;
use tarpc::{client, context, tokio_serde::formats::Json};

pub static REDIRECTOR_SERVICE_ADDR: Mutex<Option<SocketAddrV4>> = Mutex::new(None);

pub async fn initialize_rpc(rpc_addr: SocketAddrV4) -> anyhow::Result<DesktopAppServiceClient> {
    // Initialize the service
    let mut transport = tarpc::serde_transport::tcp::connect(rpc_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);
    let client = DesktopAppServiceClient::new(client::Config::default(), transport.await?).spawn();

    // Ping the server to test the connection
    let value = 2104;
    let pong = client
        .ping(context::current(), value)
        .await
        .context("failed to get ping response")?;
    if pong.value != value {
        anyhow::bail!("ping response value was incorrect");
    }

    Ok(client)
}

pub fn redirector_service_addr() -> Option<SocketAddrV4> {
    *REDIRECTOR_SERVICE_ADDR.lock()
}

pub async fn initialize_services(rpc: &DesktopAppServiceClient) -> anyhow::Result<()> {
    let services = rpc.services(context::current()).await?;

    *REDIRECTOR_SERVICE_ADDR
        .lock()
        .insert(services.redirector_address);

    Ok(())
}
