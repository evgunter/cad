---
id: recourse-table-has-no-lever-only-ending
kind: issue
title: geom_brep::recourse has no lever-only ending or undecided-refusal shape, so each consumer spells both itself
status: open
opened: 2026-10-01
---


(BAND implementer, from the review of PR 3690.)

## What

`geom_brep::recourse` ends sized decisions (`SizedDecision`) and
size-less kernel decisions (`Unsized::{Defect, LastResort}`), but has no
ending for a decision whose refusal names its geometry lever alone — a
user-geometry residual (passes only at zero), or any decision no
smaller tolerance truthfully decides. Each consumer that has one writes
it by hand, byte-identically:

- `crates/sweep/src/blend/mod.rs`, `BlendDecision::recourse_with`:
  `format!("Recourse: {lever}; {UNREADABLE_MARGIN_NOTE}")` on a poisoned
  reading, else `format!("Recourse: {lever}")`.
- `crates/topo/src/boolean/refusal_routes.rs`, `Ending::Lever` in
  `render`: the same two spellings.
- `crates/geom-core/src/predicate.rs`, `MarginDiag::sized_recourse`'s
  `Reading::Invalid` arm: the same poisoned spelling.

The undecided refusal's shape is spelled per Display too:
`"{} is undecided: {}. {}"` / `"{} is undecided: {}. {ending}"` at
`topo/src/euler.rs`, `topo/src/merge_faces.rs`,
`topo/src/splitting/mod.rs`, `topo/src/boolean/refusal_routes.rs` and
`sweep/src/blend/mod.rs`.

A change to either (a reworded note, a different separator) has to be
found in each copy.

## Repair shape

A lever-only decision in the table (say `LeverOnly { lever }` with a
`recourse(arm, reading)` like its siblings), and one helper composing
`{subject} is undecided: {payload}. {ending}` from a subject, an
`Indeterminate` and an ending, which the five Displays call.
