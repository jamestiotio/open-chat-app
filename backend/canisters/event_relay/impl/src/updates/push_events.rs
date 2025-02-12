use crate::guards::caller_can_push_events;
use crate::{mutate_state, RuntimeState};
use candid::Principal;
use canister_tracing_macros::trace;
use event_relay_canister::push_events::*;
use event_sink_canister::Event;
use ic_cdk_macros::update;
use sha256::sha256_string;

#[update(guard = "caller_can_push_events")]
#[trace]
fn push_events(args: Args) {
    mutate_state(|state| push_events_impl(args, state))
}

fn push_events_impl(args: Args, state: &mut RuntimeState) {
    let now = state.env.now();

    let salt = state.data.salt.get();

    state.data.events_sink_client.push_many(
        args.events
            .into_iter()
            .filter(|e| state.data.event_deduper.try_push(e.idempotency_key, now))
            .map(|e| {
                let user = e.user.map(|u| obfuscate_user(u, salt));
                Event {
                    name: e.name,
                    timestamp: e.timestamp,
                    user,
                    source: e.source,
                    payload: e.payload,
                }
            }),
    );
}

pub fn obfuscate_user(user: String, salt: [u8; 32]) -> String {
    // We only want to obfuscate userId principals, so if the string is not a principal we return it as is
    if Principal::from_text(&user).is_err() {
        return user;
    }

    // Generates a 32 character string from the input value + the salt
    let mut bytes = Vec::new();
    bytes.extend_from_slice(user.as_bytes());
    bytes.extend_from_slice(&salt);
    sha256_string(&bytes).split_off(32)
}
