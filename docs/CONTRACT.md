# Deposit domain contract

The caller decides deposit policy. `cscw` executes only the supplied, explicitly
cost-authorized operation through `cpmt`'s maintained adapters or compatible
`crpt` adapters. There is no custody, accounting ledger, release schedule,
forfeiture decision, provider default or independent payment transport.

A positive deposit fact requires the payment adapter to establish the exact
instruction's financial terms. A held amount is distinct from collected money,
and a refund is distinct from releasing an uncollected hold. A generic scheme's
settlement, verified authorization or active subscription alone is unqualified.
Proofs retain context/expiry and refuse sandbox. Caller-owned atomic state owns
replay and consumption; all domain buffers remain transient RAM-only data.

The shared payment journal is a caller port, not a database implementation. Its
minimum operational record contains continuation/receipt bindings and reserved
fees, never raw KYC, complete responses or member actions. A host must atomically
reserve operation identity and maximum service cost across concurrent callers
and restarts. Cost discovery/preview is inert; explicit bounded tariffs/quotes
must cover the provider, account, mode, operation and all terms. Unknown actual
fees remain unknown, and ambiguous payment effects keep maximum exposure.

Minimal disclosure is normal. Explicit callers can borrow or take complete
available private payment results; default errors and Debug output reveal no
private data. Provider payloads cannot construct deposit proofs. Upstream SDK
copies and host swap/dumps remain host concerns; application-level zeroization
is not a perfect memory-erasure guarantee.

The crate reuses shared fallback and simulation. Deterministic simulation has no
I/O/credentials and cannot construct production results. Sandbox does not mean
free and cannot grant production evidence. Tests use maintained upstream code
with inert fixtures; all live/paid/hardware cases are unexecuted. Safe Rust is
required for owned code, with no custom cryptographic primitive or SDK fork.
