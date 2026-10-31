---
id: slit-reads-a-faces-edges-through-its-own-walk
kind: issue
title: boolean/slit.rs keeps its own walk of a face's edges beside solid_contain::face_edges
status: open
opened: 2026-10-27
priority: P4
cost: E
---


Found by the review of PR 5853 (`atrest/curved-copies-filling-one-volume-interfere`).

## What

`crates/topo/src/boolean/slit.rs` `face_edges` reads a face's edges
through its own walk (`face_halves`, a `BTreeSet<EdgeKey>`). That is
the walk `crates/topo/src/boolean/solid_contain.rs` `face_edges` already
makes, typed: every loop, rings included, with each edge's half, ends
and the face across, and `CorruptFace` where a loop does not resolve.
The census reads that one (`census.rs` `shared_interior_witness`,
`axial_span`) and keeps no copy of its own. `slit.rs` does keep one, and
its copy answers a torn loop as no edges (`unwrap_or_default`), not as
the refusal.

## The fix

`slit.rs`'s two callers (`face_edges(body, face)` in the slit's
boundary reads) read `solid_contain::face_edges` and map to `.edge`,
deciding at each one what a torn loop means there. Its `face_halves`
stays only if `loop_leaving` still needs it.
