use crate::{mutate_state, Data, RuntimeState, WASM_VERSION};
use std::time::Duration;
use tracing::trace;
use types::{BuildVersion, Timestamped};
use utils::canister::get_random_seed;
use utils::env::canister::CanisterEnv;
use utils::env::Environment;

mod heartbeat;
mod init;
mod post_upgrade;
mod pre_upgrade;

fn init_env() -> Box<CanisterEnv> {
    ic_cdk_timers::set_timer(Duration::ZERO, reseed_rng);
    Box::default()
}

fn init_state(env: Box<dyn Environment>, data: Data, wasm_version: BuildVersion) {
    let now = env.now();
    let state = RuntimeState::new(env, data);

    crate::init_state(state);
    WASM_VERSION.with(|v| *v.borrow_mut() = Timestamped::new(wasm_version, now));
}

fn reseed_rng() {
    ic_cdk::spawn(reseed_rng_inner());

    async fn reseed_rng_inner() {
        let seed = get_random_seed().await;
        mutate_state(|state| state.env = Box::new(CanisterEnv::new(seed)));
        trace!("Successfully reseeded rng");
    }
}
