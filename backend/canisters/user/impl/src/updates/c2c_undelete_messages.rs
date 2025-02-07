use crate::{mutate_state, run_regular_jobs, RuntimeState};
use canister_api_macros::update_msgpack;
use canister_tracing_macros::trace;
use chat_events::DeleteUndeleteMessagesArgs;
use types::{EventIndex, UserId};
use user_canister::c2c_undelete_messages::{Response::*, *};

#[update_msgpack]
#[trace]
fn c2c_undelete_messages(args: Args) -> Response {
    run_regular_jobs();

    mutate_state(|state| c2c_undelete_messages_impl(args, state.env.caller().into(), state))
}

pub(crate) fn c2c_undelete_messages_impl(args: Args, caller_user_id: UserId, state: &mut RuntimeState) -> Response {
    if state.data.blocked_users.contains(&caller_user_id) {
        return UserBlocked;
    }

    if let Some(chat) = state.data.direct_chats.get_mut(&caller_user_id.into()) {
        chat.events.undelete_messages(DeleteUndeleteMessagesArgs {
            caller: caller_user_id,
            is_admin: false,
            min_visible_event_index: EventIndex::default(),
            thread_root_message_index: None,
            message_ids: args.message_ids,
            now: state.env.now(),
        });
        Success
    } else {
        ChatNotFound
    }
}
