---
id: extrude-step-8-doc-names-cusp-joints-which-nothing-reads
kind: issue
title: extrude's step 8 doc says the sweep trusts ValidatedLoop::cusp_joints, which no sweep code reads
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found by designer B on the cusp fork (#5354): "No sweep code reads
`cusp_joints` (only tests and the docs do); the cusp's legality is
derived from the body by the must-carry stations and tier 3's wedge-0
arm." Filed by `paths/joint-reads-its-turn`, which re-grepped it:
`ValidatedLoop::cusp_joints` appears in `crates/sweep/src` only in
`extrude.rs`'s module docs, step 8 ("Declared cusps. Every declared
cusp joint ([`profile::ValidatedLoop::cusp_joints`]) sweeps a strut at
material wedge 0").

Two things in that step read wrong:

- It is numbered as a step of the sweep, beside steps that do work,
  and names an accessor as its input; no code path takes the cusp set,
  so a reader looking for where extrude acts on it finds nothing.
- "Every declared cusp joint" is narrower than the set it links:
  `cusp_joints` holds every tangent joint that reverses its heading,
  decided ones (a raw-authored cusp, whose turn reads Zero) as well as
  the `.cusp()` door's.

The fix is the doc: say that a cusp joint, declared or decided, sweeps
a strut at material wedge 0 because the walls meet there, and that the
body, not the profile's record, is what makes it legal (the must-carry
stations and tier 3's wedge-0 arm). Cost: one paragraph.
