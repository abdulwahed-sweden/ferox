//! Integration test wiring http_beacon and relay_manager in-process (scaffold)
//! Currently limited because http_beacon uses its own in-memory server model.
//! We simulate minimal interaction to ensure both modules compile and basic
//! behavior works without network calls.

use ferox::modules::c2::http_beacon::{BeaconClient, BeaconConfig, InMemoryBeaconServer};

#[tokio::test]
async fn beacon_integration_minimal() {
    let cfg = BeaconConfig { poll_interval_ms: 50, ..Default::default() };
    let client = BeaconClient::new(cfg.clone(), "int_token").unwrap();
    let (server, cmd_tx, mut res_rx) = InMemoryBeaconServer::new(BeaconClient::new(cfg, "int_token").unwrap());
    cmd_tx.send("date".into()).await.unwrap();
    client.tick(&server).await.unwrap();
    let out = res_rx.try_recv().unwrap();
    assert_eq!(out, "DATE");
}
