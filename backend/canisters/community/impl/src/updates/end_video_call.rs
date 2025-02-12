use crate::activity_notifications::handle_activity_notification;
use crate::guards::caller_is_video_call_operator;
use crate::{mutate_state, run_regular_jobs, RuntimeState};
use canister_tracing_macros::trace;
use chat_events::EndVideoCallResult;
use community_canister::end_video_call::{Response::*, *};
use ic_cdk_macros::update;

#[update(guard = "caller_is_video_call_operator")]
#[trace]
fn end_video_call(args: Args) -> Response {
    run_regular_jobs();

    mutate_state(|state| end_video_call_impl(args, state))
}

fn end_video_call_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(channel) = state.data.channels.get_mut(&args.channel_id) {
        match channel.chat.events.end_video_call(args.message_index, state.env.now()) {
            EndVideoCallResult::Success => {
                handle_activity_notification(state);
                Success
            }
            EndVideoCallResult::MessageNotFound => MessageNotFound,
            EndVideoCallResult::AlreadyEnded => AlreadyEnded,
        }
    } else {
        MessageNotFound
    }
}
