//! Caller-authorized deposit operations over the shared payment domain boundary.
//! No custody, deposit eligibility policy, database connection or hidden fallback.
#![forbid(unsafe_code)]

pub use cpmt::{access, cost, details, payments, run, simulation};
use payments::{
    Binding, Error, Evidence, Execution, Instruction, Journal, Payments, Result, State,
};

/// A deposit fact established by the configured payment adapter's financial checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fact {
    /// The requested amount is authorized but not collected.
    Held,
    /// The requested amount was collected; the caller decides its permitted use.
    Collected,
    /// The requested hold was released.
    Released,
    /// The requested refund completed.
    Returned,
    /// Awaiting the provider or member; another payment must not start implicitly.
    Pending,
    /// Provider effect is uncertain; reconcile the existing operation.
    Unknown,
    /// Provider confirmed failure.
    Refused,
    /// The adapter did not establish the requested deposit's financial fact.
    Unqualified,
}

fn project(state: State, satisfies_terms: bool) -> Fact {
    match state {
        State::Pending | State::RequiresAction => Fact::Pending,
        State::Unknown => Fact::Unknown,
        State::Failed => Fact::Refused,
        _ if !satisfies_terms => Fact::Unqualified,
        State::Authorized => Fact::Held,
        State::Collected => Fact::Collected,
        State::Released => Fact::Released,
        State::Refunded => Fact::Returned,
        _ => Fact::Unqualified,
    }
}

/// Authenticated deposit result. Full private details remain explicit and transient.
/// Arbitrary provider JSON or simulated data cannot construct this type.
pub struct Deposit {
    execution: Execution,
}
impl Deposit {
    /// The observed domain fact; no hold/release/forfeiture rule is inferred.
    pub fn fact(&self) -> Fact {
        project(
            self.execution.outcome.state(),
            self.execution.outcome.satisfies_terms(),
        )
    }
    /// Borrow complete payment results, including requested private upstream data.
    pub fn payment(&self) -> &Execution {
        &self.execution
    }
    /// Return the underlying result to a general-purpose payment consumer.
    pub fn into_payment(self) -> Execution {
        self.execution
    }
    /// Produce a minimal production proof only for an established financial fact.
    /// The host controls proof consumption and any rule authorizing the operation.
    pub fn evidence(&self, binding: &Binding, now: u64, valid_until: u64) -> Result<Evidence> {
        match self.fact() {
            Fact::Held | Fact::Collected | Fact::Released | Fact::Returned => self
                .execution
                .outcome
                .evidence(binding, self.execution.outcome.state(), now, valid_until),
            _ => Err(Error::Rejected),
        }
    }
}

/// Execute exactly one caller-authorized deposit operation. A hold, capture,
/// cancellation or refund is expressed by the selected adapter's instruction;
/// this library never decides when one is owed or allowed. Reads use a fresh
/// `Payments::observation` instruction and require their own cost authorization.
pub async fn execute<J: Journal>(
    payments: &Payments<J>,
    instruction: &Instruction,
    permit: cost::DispatchPermit<String>,
    now: u64,
    disclosure: details::Disclosure,
) -> Result<Deposit> {
    Ok(Deposit {
        execution: payments
            .dispatch_with_output(instruction, permit, now, disclosure)
            .await?,
    })
}

/// Explicit dummy deposit transformation for local simulations. It grants nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Simulated {
    /// Simulated normalized fact, never accepted by `Deposit::evidence`.
    pub fact: Fact,
}
/// Exercise the same deposit transformation with explicit dummy provider facts.
/// Use `simulation::simulate` for the shared cost/fallback scenario; no adapter,
/// credential or transport is accepted by either simulation function.
pub fn simulate_fact(state: State, satisfies_terms: bool) -> Simulated {
    Simulated {
        fact: project(state, satisfies_terms),
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "tests/deposit.rs"]
mod tests;
