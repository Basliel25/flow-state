///! FlowKey: the catagorical projection of a zeek conn.log
///! file.
///
///! Packages (proto, sevice, conn_state) as a state

use std::collections::HashMap;

// Potential borrow check problem with String
// But the states should outlive the conn.log line
pub struct FlowKey {
    pub proto: String,
    pub service: String,
    pub conn_state: String,
}





