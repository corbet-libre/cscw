# cscw

**Deposit admission gate for cvld.**

`cscw` will implement the escrow rule behind a deposit gate: when a deposit is held, released or forfeited. It rests on us and the member alone, not on a third party.

Status: name reserved, no implementation or qualified provider adapter yet.
Empty production coverage is a blocking failure. See
[the current contract](docs/CONTRACT.md).

One gate invocation performs one provider check; `cfbk` owns ordering and
fallback, with requirements supplied by `crbk`.

## Boundaries

What it does:

- Decides hold, release and forfeit of a deposit and returns an opaque gate proof.

What it never does:

- Never moves money itself: payments go through `cpmt` (fiat) or `crpt` (crypto).
- No payment details in output, logs, errors or fixtures.

## License

Copyright 2026 Julian Y. Richard Corbet. Licensed under
[LGPL-3.0-only](LICENSES/LGPL-3.0-only.txt)
[WITH LGPL-3.0-linking-exception](LICENSES/LGPL-3.0-linking-exception.txt).
See [LICENSE.md](LICENSE.md).

## Continuous verification

Dependency updates follow main and are tested against one CI-resolved lockfile.
Line and branch coverage target 100%; failures remain blocking. See
[the coverage contract](docs/COVERAGE.md) for measurement and exclusions.
