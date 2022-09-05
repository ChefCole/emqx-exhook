use std::convert::TryInto;

use exhook::hook_provider_client::HookProviderClient;
use exhook::{ClientConnectRequest,ConnInfo};
use tonic::IntoRequest;



pub mod exhook {
    tonic::include_proto!("emqx.exhook.v2");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HookProviderClient::connect("http://192.168.1.6:3000").await?;

    let conninfo = ConnInfo{
        node:"".to_string(),
        clientid:"".to_string(),
        username:"".to_string(),
        peerhost:"".to_string(),
        sockport:1,
        proto_name:"".to_string(),
        proto_ver:"".to_string(),
        keepalive:1
    };

    let request = tonic::Request::<ClientConnectRequest>::new(ClientConnectRequest {
        conninfo:Some(conninfo),
        props:vec![],
        meta:None
    });

    let response = client.on_client_connect(request).await;

    println!("RESPONSE={:?}", response);

    Ok(())
}