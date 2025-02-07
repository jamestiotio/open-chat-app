use crate::{mutate_state, read_state, RuntimeState};
use candid::Principal;
use canister_tracing_macros::trace;
use event_sink_client::Event;
use ic_cdk_macros::update;
use online_users_canister::mark_as_online::{Response::*, *};
use types::{CanisterId, UserId};

#[update]
#[trace]
async fn mark_as_online(_args: Args) -> Response {
    let user_id = match read_state(try_get_user_id_locally) {
        Ok(u) => u,
        Err((p, user_index_canister_id)) => {
            let c2c_args = user_index_canister::c2c_lookup_user::Args { user_id_or_principal: p };
            match user_index_canister_c2c_client::c2c_lookup_user(user_index_canister_id, &c2c_args).await {
                Ok(user_index_canister::c2c_lookup_user::Response::Success(res)) => {
                    mutate_state(|state| state.data.principal_to_user_id_map.add(p, res.user_id));
                    res.user_id
                }
                Ok(_) => return UserNotFound,
                Err(error) => return InternalError(format!("{error:?}")),
            }
        }
    };

    mutate_state(|state| mark_as_online_impl(user_id, state))
}

fn try_get_user_id_locally(state: &RuntimeState) -> Result<UserId, (Principal, CanisterId)> {
    let caller = state.env.caller();
    state
        .data
        .principal_to_user_id_map
        .get(&caller)
        .ok_or((caller, state.data.user_index_canister_id))
}

fn mark_as_online_impl(user_id: UserId, state: &mut RuntimeState) -> Response {
    let now = state.env.now();
    state.data.last_online_dates.mark_online(user_id, now);
    state.data.mark_as_online_count += 1;
    state.data.event_sink_client.push(Event {
        name: "user_online".to_string(),
        timestamp: now,
        user: Some(user_id.to_string()),
        source: Some(state.env.canister_id().to_string()),
        payload: Vec::new(),
    });
    Success
}
