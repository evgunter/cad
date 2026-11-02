---
id: slit-reads-a-faces-edges-through-its-own-walk
kind: issue
title: boolean/slit.rs keeps its own walk of a face's edges beside solid_contain::face_edges
status: closed
opened: 2026-10-27
priority: P4
cost: E
branch: onehome/face-walks
closed: 2026-11-02
pr: 6362
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

## Closed (ONEHOME, 2026-11-02, PR 6362)

The walks have one home each, and the easy call is the right one:
- `Body::face_surface(face) -> Option<&Surface>`: `None` is a stale key only; a dangling surface link panics.
- `Body::proven_face_surface(face)`: a kernel-enumerated or recorded key; a miss panics through `proven`, naming the key.
- `Body::proven_face_edges` / `proven_face_edge_set`: the face → edges walk, moved out of `solid_contain`; it replaces slit's and census's own walks.

One rule: a key the kernel enumerated or recorded panics; only input stays typed (declarations, the public containment, region and readback doors). About 60 sites in topo are routed through these homes, and the private twins are deleted (`chart_region`, `splitting/rules.rs`, slit's `face_halves`, both `edge_set`s, `sectors::tangent_carrier`). FULL review FIX-THEN-MERGE: the first version minted 22 inline panic copies beside `proven`, and a test fixture shaped a production answer. Delta MERGE: no input-reachable panic. Follow-ups:
- `the-face-walk-units-follow-ups-from-its-review`;
- `a-faces-loops-are-walked-inline-beside-face-loops-linked` (P1);
- `face-to-surface-walks-outside-topo-predate-body-face-surface` (P2).
