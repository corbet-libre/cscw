# Deposit capability catalogue

References checked 2026-10-10. A deposit is a caller-selected use of a payment
operation. This crate does not hold money or decide when to authorize, capture,
release or refund it. The list below does not enable an offer or select a vendor.

## Implemented composition

`cscw` delegates to the same configured `cpmt::payments::Payments` instance,
cost permit, trusted clock and atomic caller state as other payment operations.
It projects financially qualified Authorized, Collected, Released and Refunded
results into Held, Collected, Released and Returned. Provider payloads remain
private transient data, and a generic settlement is not silently a deposit hold.

| Payment adapter | Usable domain facts | Current limits |
|---|---|---|
| Stripe through cpmt | Hold, collection, release and completed refund when exact financial terms are authenticated | Real SDK loopback tests in this crate cover hold/capture/refund; cpmt covers its cancellation and observation integration. Partial amounts, sandbox or missing terms cannot produce a production deposit proof. |
| Square through cpmt | Qualified authorization, collection and completed refund | Cancellation currently yields Cancelled, which is unqualified for this crate's Released fact. An implemented API method does not promise every payment method supports a hold. |
| Mollie / GoCardless through cpmt | Qualified collection/refund, and provider-established authorization where actually supported | GoCardless collection is not an authorization hold. Asynchronous outcomes remain pending/unknown until established. Cancellation alone does not become released money. |
| BTCPay / CoinGate / generic x402 through crpt | General payment results remain available through `payment` / `into_payment` | Generic Settled is deliberately Unqualified as a deposit fact. A protocol-specific adapter must establish the exact requested held/collected/released/refunded effect before this projection applies. |

Exact implemented operations and credential requirements live in the
[cpmt provider contract](https://git.corbet.ch/corbet-foss/cpmt/src/branch/main/docs/PROVIDERS.md)
and [crypto catalogue](https://git.corbet.ch/corbet-foss/crpt/src/branch/main/docs/CATALOGUE.md).
These are reusable integrations, not live account certifications. No live or
provider-sandbox deposit was executed.

## Candidate extensions, not implemented here

| Candidate | Specific value and evidence requirement | Credentials and cost constraints |
|---|---|---|
| PayPal authorization/capture | A provider authorization can support a time-limited hold. Keep the order, authorization and capture identities distinct; reauthorization returns a new authorization ID. Completion/expiry must come from authenticated provider state. | Host-provided OAuth account capability, explicit environment and bounded fees. Reauthorization is a separate authorized operation, never automatic hold extension. [Provider contract](https://developer.paypal.com/v5/checkout/auth-capture/). |
| Adyen capture/cancellation | A future cpmt adapter can expose asynchronous capture/cancel/refund completion. A cancellation request acknowledgement does not itself release a deposit. Keep original and modification references. | Merchant account/API key, separately authenticated completion channel and explicit tariff. Provider-supported payment method capabilities must be qualified individually. [Cancellation lifecycle](https://docs.adyen.com/online-payments/cancel/). |
| Coinbase Payment Acceptance | Its authorize/capture operations are candidates for a provider-managed conditional payment flow. The returned operation ID needs authenticated completion and exact financial-effect checks. | Host-authorized merchant/JWT credentials; assess account availability, assets, fee receiver and fee basis points before adoption. No wallet creation, rewards or transfers. [Authorize](https://docs.cdp.coinbase.com/api-reference/payment-acceptance/payments/authorize-payment), [capture](https://docs.cdp.coinbase.com/api-reference/payment-acceptance/payments/capture-payment). |
| Lightning hold-invoice adapter | A dedicated provider/node integration may distinguish an accepted conditional payment from final settlement or cancellation. Ordinary paid invoices alone do not establish an indefinite refundable hold. | Inject a narrowly scoped node capability and explicit timeout/fee terms. Use maintained upstream RPC/types; no node custody, wallet key or local preimage policy. Needs its own lifecycle fixtures and receipt identity. [LND API](https://lightning.engineering/api-docs/api/lnd/lightning/lookup-invoice/). |
| BTCPay refund lifecycle | Existing invoice collection could be extended with the provider's refund/pull-payment workflow. Issuing a refund claim is not proof that funds reached the member. | Store-scoped credentials and an explicitly authorized refund workflow. Require completed provider payout evidence and exact amount/asset; never broaden into a member wallet or automatic payout system. [Greenfield contract](https://docs.btcpayserver.org/API/Greenfield/v1/swagger.json). |

Provider authorization periods, automatic capture/cancellation and method limits
must be exposed as facts. The caller's desired deposit duration does not extend a
provider hold. A refund is a new financial operation with its own identity,
reconciliation and fees; it is not a rollback of a prior charge.

## Qualification and cost

Implement new financial operations in cpmt/crpt adapters, keeping cscw's
projection small. Reuse maintained upstream SDK/protocol code and the shared
cost/fallback planner. Owned Rust forbids unsafe code; native/transitive safety
must be assessed separately. No copied cryptography or private SDK fork.

Each operation, including a read or attempted release, needs a quote/tariff with
source, unit, expiry and defensible maximum. No fees are invented by this list;
a zero principal change does not mean zero service cost. Failed/uncertain steps
can cost money. Unexpected charges remain available in the shared cost violation
report and block proof/fallback. Failed final caller-state commits yield Unknown
and explicit private reconciliation data. The host must reconcile before more
spending in that budget scope.

Use deterministic simulation to exercise planning and the real deposit projection
without I/O or credentials. Sandbox is separate and not presumed free. New
capabilities need maintained-upstream inert fixtures for exact wire behavior,
amounts, references, lost replies, replay, expiry, complete/minimal data, costs and
production proof isolation. A fixture passing does not authorize a paid call.
