mod create;
mod delete;
mod get;
mod list;
mod modify;

use avina_wire::accounting::ServerState;

pub fn equal_server_states(
    server_state_1: &ServerState,
    server_state_2: &ServerState,
) -> bool {
    server_state_1.id == server_state_2.id
        && (server_state_1.begin - server_state_2.begin).num_milliseconds() < 1
        && server_state_1.instance_id == server_state_2.instance_id
        && server_state_1.instance_name == server_state_2.instance_name
        && server_state_1.flavor == server_state_2.flavor
        && server_state_1.flavor_name == server_state_2.flavor_name
        && server_state_1.status == server_state_2.status
        && server_state_1.user == server_state_2.user
        && server_state_1.username == server_state_2.username
}

pub fn assert_equal_server_states(
    server_state_1: &ServerState,
    server_state_2: &ServerState,
) {
    assert!(equal_server_states(server_state_1, server_state_2));
}

pub fn contains_server_state(
    server_states: &[ServerState],
    server_state: &ServerState,
) -> bool {
    server_states
        .iter()
        .any(|s| equal_server_states(s, server_state))
}

pub fn assert_contains_server_state(
    server_states: &[ServerState],
    server_state: &ServerState,
) {
    assert!(contains_server_state(server_states, server_state));
}
