---
id: census-snapshot-arm1-discard-has-no-justification
kind: issue
title: census.rs snapshot's arm-1 LoopBoundary discard is the one of three with no written justification
status: closed
opened: 2026-09-06
refs: [2044]
priority: P3
cost: E
closed: 2026-10-26
---


## Finding

Filed by the GATES orchestrator from the `gates/loop-boundary-discards`
lane (PR 2044), which registered every `LoopBoundary` discard under
`crates/*/src` (80 sites; 2 audited). `crates/topo/src/census.rs:611`
(`snapshot`) is the third arm-1 discard #737 points at and the only one
of `census.rs`'s three with no justification at all — the other two, in
`sweep_cross_solid_backstop`, name the arm that asks the same question
about the same pair and are registered `audited`. This one is registered
`unaudited`; the audit (name the deferring arm, or make the discard a
typed refusal) is TOPO's, and closing it moves the register entry's
disposition in `scripts/gates/loop-boundary-discards.sh` in the same
PR. `census.rs` is CURVED's by glob and TOPO's by the #737 lineage;
whichever takes it announces the other.

## Refs at GATES' sweep (2026-09-08)

GATES closed and its item files left the tracker (`docs/DOC-LEDGER.md`, sweep 7); `S49` is now cited by its closing PR 2044.

## Closed (2026-10-26, tracker sweep)

Fixed on main. `census.rs` now walks a face's loops through the one typed walk `face_cycles` (`crates/topo/src/census.rs:1830`), which hands an unwalkable loop back as `Err(loop)` and decides nothing; `snapshot` (`:2198`) answers it at its own site with a written reason ("A loop this cannot walk contributes no boundary vertex, and the face is no polygon the touch analysis reads", `:2233`). The register `scripts/gates/loop-boundary-discards.sh:169-173` no longer lists an unaudited `census.rs|snapshot` discard: its one `census.rs` entry is `face_cycles`, registered `audited` with that reason. (The per-hop skips inside those walks are the separate, still-live `census-face-walks-skip-torn-hops-silently`.)
