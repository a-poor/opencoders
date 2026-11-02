use opencoders::models::ServerInfo;
use serde_json::json;

#[test]
fn server_process_id_respects_the_documented_integer_range() {
    for pid in [json!(-1), json!(1.5), json!(9_007_199_254_740_992_u64)] {
        let value = json!({"version": "2.test", "pid": pid, "urls": [], "paths": {"tmp": "/tmp"}});
        assert!(serde_json::from_value::<ServerInfo>(value).is_err());
    }
    for pid in [0, 9_007_199_254_740_991_u64] {
        let value = json!({"version": "2.test", "pid": pid, "urls": [], "paths": {"tmp": "/tmp"}});
        assert_eq!(
            serde_json::from_value::<ServerInfo>(value).unwrap().pid,
            pid
        );
    }
}

#[test]
fn nullable_optional_server_fields_decode() {
    for capabilities in [json!(null), json!({}), json!({"persistentPty": null})] {
        let value = json!({"version": "2.test", "pid": 0, "urls": [], "hostname": null,
                           "paths": {"tmp": "/tmp"}, "capabilities": capabilities});
        let info: ServerInfo = serde_json::from_value(value).unwrap();
        assert!(info.hostname.is_none());
        assert!(
            info.capabilities
                .and_then(|value| value.persistent_pty)
                .is_none()
        );
    }
}
