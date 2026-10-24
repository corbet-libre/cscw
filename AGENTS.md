# Agent instructions

- Read docs/CONTRACT.md. Execute caller-authorized deposit operations through the
  shared payment protocol; never invent hold/release/forfeiture policy or custody.
- All domain data stays in RAM. No database/SQL/file cache implementation or
  hidden SDK persistence; caller atomic state gets minimum operational records.
- Rich private results are explicit; default facts/errors/logs are minimal.
- Cost permits precede dispatch and read operations. Simulation and sandbox are
  separate from production evidence; never assume unknown charges are zero.
- Prefer maintained upstream safe Rust. Owned crates forbid unsafe code. Use
  compatible dependency ranges and first-party main branches, no exact pins.
- No live provider calls, spending, registrations, deployment or publication.
- Local checkouts edit/format; Crow compiles/tests committed candidates. Preserve
  coverage/quality gates and obtain independent review before completion claims.
- Comments/docs are English. LGPL-3.0-only WITH LGPL-3.0-linking-exception; no
  GPL/AGPL-only dependency implementation. Commit explicit paths, no AI credit.
