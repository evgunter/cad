---
id: contact-records-are-declared-under-boolean-while-the-finished-body-at-the-root-holds-them
kind: issue
title: ContactRecords and its vocabulary are declared under boolean/ while the finished body at the crate root holds them: [topo.homes] puts them at the root
status: open
opened: 2026-11-18
priority: P2
cost: M
---


Found by `embed/records` (2026-11-18), which made the finished body
hold its contact records.

## What

`ContactRecords`, `Contact`, `Cited`'s citations as records use them,
`CarryRefusal` and the carry (`carried_through`, `landings`) are
declared in `crates/topo/src/boolean/records.rs`. The finished type
(`AtRestBody`, `crates/topo/src/validate.rs`), its trait and the gate's
inputs (`Built`, `Assembled`, `Replay`, `crates/topo/src/finish.rs`)
are at the crate root and name them, as every door that carries
records does. `[topo.homes]` (`crates/topo/README.md`) puts a type the
root's own types hold at the root, not under an operation's directory.

## The pins this row carries

`scripts/gates/engine-leaves.counts`: `names-boolean` on `finish.rs`
(14), `validate.rs` (19 → 21) and `census/finding_solids.rs` (1) rose
with the change and cite this row.

## Owed

Move the records vocabulary to the crate root. It is not a file move:
`records.rs` reads the engine's own items (`OperandRecords` and the
per-kind contact structs, `Cell`, `reduce::index`, the census's record
confirm), so the cut between what a record is and how the Boolean
carries one in has to be drawn first. The pins fall to what the root
legitimately names once it is.
