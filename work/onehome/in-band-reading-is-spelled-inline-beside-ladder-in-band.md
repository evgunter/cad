---
id: in-band-reading-is-spelled-inline-beside-ladder-in-band
kind: issue
title: Whether an undecided reading lies inside the band is spelled inline at chord_join and bound_side beside geom_core::ladder::in_band
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Filed by GERM's `section-class-lane-choices-escalate-in-band` lane (PR
7171, whose review found the class). That PR reads
`geom_core::ladder::in_band` at `geom_brep::intersect::closed_form_shown`.

## What

`geom_core::ladder::in_band` is the one home of "is this `Indeterminate`
inside the band": a point margin, or an enclosure wholly inside one
sliver (`terminal_sliver`); not a straddle and not a poisoned margin.
Two sites re-spell it:

- `crates/topo/src/chord_join.rs`, the guard
  `cause.margin.kind() == geom_core::MarginKind::Value || cause.terminal_sliver`
  (near line 2323).
- `crates/topo/src/bound_side.rs` `in_one_gap`, a three-arm match on
  `MarginKind` identical to `ladder::in_band`.

Checked and not a copy: `crates/topo/src/merge_faces.rs`
`is_the_zero_story` (a test helper that asks for a point margin only).

## Fix

Read `geom_core::ladder::in_band` at both sites, and delete `in_one_gap`.
Mechanical: the predicates agree on every `MarginKind`.
