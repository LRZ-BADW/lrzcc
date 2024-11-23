mod create;
mod delete;
mod get;

use lrzcc_wire::accounting::ServerState;

pub fn assert_equal_server_states(
    server_state_1: &ServerState,
    server_state_2: &ServerState,
) {
    assert_eq!(server_state_1.id, server_state_2.id);
    assert!(
        (server_state_1.begin - server_state_2.begin).num_milliseconds() < 1
    );
    assert_eq!(server_state_1.instance_id, server_state_2.instance_id);
    assert_eq!(server_state_1.instance_name, server_state_2.instance_name);
    assert_eq!(server_state_1.flavor, server_state_2.flavor);
    assert_eq!(server_state_1.flavor_name, server_state_2.flavor_name);
    assert_eq!(server_state_1.status, server_state_2.status);
    assert_eq!(server_state_1.user, server_state_2.user);
    assert_eq!(server_state_1.username, server_state_2.username);
}
