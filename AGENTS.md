# Agent instructions

Write all code comments and documentation in English.

## Product boundary

- Deposit gate for cvld: the escrow rule for holding, releasing and forfeiting a deposit.
- Decides hold, release and forfeit of a deposit and returns an opaque gate proof.
- Never moves money itself: payments go through `cpmt` (fiat) or `crpt` (crypto).
- No payment details in output, logs, errors or fixtures.
- This crate is LGPL-3.0-only WITH LGPL-3.0-linking-exception. Do not add implementation code available only under the full GPL or AGPL.
- License is borderline (some own logic): LGPL while it stays simple; raise with Julian if it grows substantive logic.

## Quality boundary

- `cargo fmt --check`, `cargo clippy --all-targets` (no warnings),
  `cargo test` — all green before every commit. No local workstation
  builds; use GHA, Crow as fallback.
