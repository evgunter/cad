---
id: debug-only-bit-witness-callers-are-on-no-row
kind: issue
title: Two cfg(debug_assertions) callers of the bit-channel witnesses are in files with no row
status: open
opened: 2026-09-06
---



## Finding

Found by the `gates/debug-only-topo-class` lane's class sweep over
`crates/topo/src` (the sweep
`debug-only-assert-euler-postcondition-is-on-no-row` asked for: every
`#[cfg(debug_assertions)]` statement whose text names no spelling on any
row of `scripts/gates/bit-identity-debug-only.sh`).

`crates/topo/src/source.rs` defines three debug-only witnesses on the
bit channel — `plane_bits_witness` (`:172`), `vec3_bits_witness`
(`:195`) and `bits_witness` (`:211`), each a `#[cfg(debug_assertions)]`
`pub(crate) fn`. That lane put all three on the `source.rs` row, so the
DEFINITIONS are pinned. Their callers are not, because a subject is a
file and neither caller's file has a row:

- `crates/topo/src/boolean/plane_eq.rs:173-175` — a statement-position
  `#[cfg(debug_assertions)]` over `if let Some(agree) =
  crate::source::plane_bits_witness(…)`.
- `crates/topo/src/merge_faces.rs:1006-1020` — the same shape over an
  `if let (Surface::Plane {…}, Surface::Plane {…}) = …` that calls both
  `plane_bits_witness` and `vec3_bits_witness`.

Both are live hazards on the terms the rows exist for: this workspace's
`[profile.release]` keeps debug assertions on, so dropping either
attribute compiles and passes every test here, and the first build that
refuses it is a consumer's after publish — the witnesses do not exist
without `debug_assertions`.

Neither was reachable by the sweep that filed
`debug-only-helpers-outside-the-subject-list`: that one swept for
`#[cfg(debug_assertions)]` ITEM HEADS, and these two attributes stand in
STATEMENT position over an `if let`, which is the shape the reader could
not place at all until PR 2059.

## The candidate fix

A row per caller file — `crates/topo/src/boolean/plane_eq.rs` and
`crates/topo/src/merge_faces.rs` — with the witness spellings its file
names, pins re-taken. This is the gate's KNOWN GAP 7 (a subject is a
file) worked one population at a time, not a change to the reader.

## What it costs

Two more subjects on the largest subject list in the directory, and the
self-test is quadratic in subject count: 13 subjects take ~82 s on the
lane's box, so 15 take roughly 110 s. If that is judged too much, the
per-case loop is where to look first — each case plants every subject
and runs the gate once over all of them, so a case could run once over
all subjects rather than once per subject per case.

## What the sweep could not match

The sweep read `crates/topo/src` only, so a `cfg(debug_assertions)`
caller of these witnesses in another crate is not covered by it; a grep
for the three spellings across `crates/` finds none today. It also read
attribute text, so a witness reached through a re-export under another
name is invisible to it.
