# Deposit boundary

This reservation concerns deposit eligibility under configured hold, release
and forfeit rules. A gate invocation performs one check through its provider
port. It must not become a payment engine or independently invent escrow
policy. It moves no money; payment integration belongs to the payment owners.
No deposit/provider adapter or accepted state-transition contract is implemented.

`cfbk` owns provider ordering/fallback and `crbk` owns requirements. Each gate
invocation checks one provider result.

No local security primitive, confidence ranking or accepting placeholder may
be introduced. Registry publication, paid operations and live provider calls
are outside this configuration task. Empty production coverage must fail;
real owner/provider contracts and adversarial tests are required before any
implementation can claim verification.
