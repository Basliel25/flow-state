///! complete API integration test
///! Test flowstate pipe from intern <--> resolve

use flow_state::{FlowState, FlowBuilder};

/// Create a key from the values
fn make_key(proto: &str, service: &str, conn_state: &str) -> FlowKey {
    FlowKey {
        proto: proto.to_string(),
        service: service.to_string(),
        conn_state: conn_state.to_string(),
    }
}

#[test]
fn intern_then_resolve_reutrns_same_key() {
    let mut state = FlowState::new();
    let key = make_key("tcp", "http", "SF");

    let id = state.intern(key.clone());
    let resolved = state.resolve(id).expect("id didnt resolve");

    assert_eq!(*resolved, key);
}

#[test]
fn intern_same_key_twice_returns_same_id() {
    let mut state = FlowState::new();
    let key = make_key("tcp", "http", "SF");

    let id_1 = state.intern(key.clone());
    let id_2 = state.intern(key);

    assert_eq!(id_1, id_2);
    assert_eq!(state.len(), 1);
}

#[test]
fn intern_on_distinct_key_distinct_id() {
    let mut state = FlowState::new();
    let key_1 = make_key("tcp", "http", "SF");
    let key_2 = make_key("udp", "https", "S0");
    let key_3 = make_key("udp", "ssl", "S0");

    let id_1 = state.intern(key_1.clone());
    let id_2 = state.intern(key_2.clone());
    let id_3 = state.intern(key_3.clone());

    assert_ne!(id_1, id_2);
    assert_ne!(id_3, id_2);
    assert_ne!(id_3, id_1);
    assert_eq!(state.len(), 3);
}

#[test]
fn ids_are_sequential() {
    // Since markov-rs depends on sequential states
    // ids should be dense and sequential
    let mut state = FlowState::new();
    let key_1 = make_key("tcp", "http", "SF");
    let key_2 = make_key("udp", "https", "S0");
    let key_3 = make_key("udp", "ssl", "S0");

    let id_1 = state.intern(key_1.clone());
    let id_2 = state.intern(key_2.clone());
    let id_3 = state.intern(key_3.clone());

    assert_eq!(id_1, 0);
    assert_eq!(id_2, 1);
    assert_eq!(id_3, 2);
}
