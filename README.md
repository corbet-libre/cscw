# cscw

Reusable deposit operations through `cpmt` and `crpt` payment adapters. The caller
authorizes the financial instruction; this library returns what the adapter
actually established. It never chooses when a deposit is required, released or
forfeited, and it holds no funds or wallet keys.

`execute` consumes the shared payment cost permit and delegates exactly once to
`Payments::dispatch_with_output`. `Deposit::fact` projects fully established
financial terms into held, collected, released or returned. Pending, uncertain,
failed and unqualified results remain distinct. Generic x402 settlement is not
silently interpreted as a deposit hold: the selected scheme must establish the
specific financial effect through a suitable adapter.

`Deposit::evidence` produces only an authenticated, context-bound production fact.
Partial amounts, unknown terms, sandbox results and dummy simulations cannot
produce deposit proof. Caller rules decide what that fact permits. `payment` and
`into_payment` expose the full general result, including explicitly requested
private provider details, without introducing an alternative payment engine.

Configure real adapters, credentials, endpoints and modes in `cpmt` or `crpt`.
No provider is enabled by including this crate. `Payments::preview` takes a
trusted bounded service-fee tariff with provenance/expiry; the shared `cost`
planner checks the whole-cascade budget and binds a permit to the actual terms.
Principal and service fees are separate. Reads use fresh observation instructions
and their own permits. Unknown/ambiguous results retain reserved exposure and
never trigger an automatic new charge.

All data in this library stays in RAM. There is no SQL, database connection,
file cache or persistent SDK session. The host implements the payment `Journal`
contract, atomically reserving operation identity and fee budget across processes
and restarts. It receives only the minimal operational record, not raw requests
or complete provider results. [The capability catalogue](docs/CATALOGUE.md) separates implemented composition
from candidate deposit workflows. [The contract](docs/CONTRACT.md) describes the
boundaries and [cpmt credential documentation](https://git.corbet.ch/corbet-libre/cpmt/src/branch/main/docs/CREDENTIALS.md)
covers account provisioning and replacement separately from stable identity keys.

`simulation::simulate` exercises shared cost/fallback planning with explicit
dummy outcomes. `simulate_fact` exercises this crate's actual deposit projection
and returns a distinct non-proof type. Neither accepts credentials or a transport.
Sandbox is separate and still requires a cost authorization; it is not assumed
free. `run` and `cost` re-export the maintained shared orchestration rather than
creating a second fallback implementation.

Tests use the real maintained Stripe SDK against inert loopback responses for
hold, capture and refund, plus mismatched financial amounts, context binding,
replays, private results and sandbox refusal. No live provider account, chain
transaction, sandbox ceremony or paid call is run. Fixture coverage does not
qualify any production provider account. Owned Rust forbids unsafe code;
dependencies use compatible ranges, and the lockfile records the tested graph.

Licensed LGPL-3.0-only WITH LGPL-3.0-linking-exception. No registry publication is
part of this implementation.

The shared Payments instance requires an injected live `cost::Clock`. Dispatch
checks fresh deadlines before transport. Unexpected authenticated fees remain
visible through `Deposit::payment().cost.violation`. A failed final caller-state
write is visible as `commit_error` and yields Unknown, preserving explicit private
results for reconciliation without proof or automatic fallback.
