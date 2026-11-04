#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {
    /// Record a success or failure for a worker. Callable only by the
    /// escrow contract set at initialization. Updates the EWMA score.
    ///
    /// EWMA formula (basis-point arithmetic, no floats):
    ///   new_score = (alpha_bps * sample + (10_000 - alpha_bps) * old_score) / 10_000
    /// where sample = 10_000 for success, 0 for failure.
    ///
    /// Emits event `("reputation", "recorded")`.
    pub fn record(_env: Env, _worker: Address, _success: bool, _weight: u32) {
        // Phase 2
        todo!("Phase 2: implement record")
    }

    /// Read the current reputation score for a worker.
    /// Returns basis points 0..10_000. Defaults to 5_000 (neutral) for unknown workers.
    pub fn score(_env: Env, _worker: Address) -> u32 {
        // Phase 2
        todo!("Phase 2: implement score")
    }
}

#[cfg(test)]
mod test {
    // Phase 2: EWMA rounding, overflow, auth tests
}
