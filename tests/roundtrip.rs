///! complete API integration test
///! Test flowstate pipe from intern <--> resolve

use flow_state::{FlowState, FlowKey};

/// Create a key from the values
fn make_key(proto: &str, service: &str, conn_state: &str) -> FlowKey {
    FlowKey {
        proto: proto.to_string(),
        service: service.to_string(),
        conn_state: conn_state.to_string(),
    }
}

#[test]
fn intern_then_resolve_reutrns_same_key () {
    let mut state = FlowState::new();
    let key = make_key("tcp", "http", "SF");

    let id = state.intern(key.clone());
    let resolved = state.resolve(id).expect("id didnt resolve");

    assert_eq!(*resolved, key);
}

#[test]
fn intern_same_key_twice_returns_same_id () {
    let mut state = FlowState::new();
    let key = make_key("tcp", "http", "SF");

    let id_1 = state.intern(key.clone());
    let id_2 = state.intern(key);

    assert_eq!(id_1, id_2);
    assert_eq!(state.len(), 1);
}
