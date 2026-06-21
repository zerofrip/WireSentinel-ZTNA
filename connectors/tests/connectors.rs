use connectors::{ApplicationConnector, ConnectorHealthMonitor};
use uuid::Uuid;

#[test]
fn connector_registers_outbound() {
    let app = ApplicationConnector::new();
    let reg = app.register("edge-1", "https://edge.internal:8443", vec![Uuid::new_v4()]);
    assert_eq!(reg.name, "edge-1");
    assert!(app.get(reg.connector_id).is_some());
}

#[test]
fn health_monitor_reports_healthy() {
    let id = Uuid::new_v4();
    let health = ConnectorHealthMonitor::check(id, "https://edge.internal");
    assert!(health.healthy);
}
