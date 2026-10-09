---
id: a-loop-area-nurbs-segment-integrates-an-inverted-window-as-zero
kind: issue
title: props loop_area: a NURBS boundary piece with t0 > t1 integrates to zero per span (half_len.max(0)) where the signed integral is negative
status: open
opened: 2026-10-09
priority: P3
cost: E
---

Found by the NURBS span-locator designer pair (2026-10-09), off their
question and **not verified against the callers' contract**.

`crates/geom-brep/src/props/loop_area.rs`, `nurbs_vector_area`, reads
its span window as the min/max of two point locates and clamps each
span's half-length with `half_len.max(0)`. For an inverted pair
`(t0, t1)` with `t0 > t1`, each span therefore contributes zero, where
the signed boundary integral over a reversed piece is the negative of
the forward one. A NaN end is safe here, because `Real::max` and `min`
propagate it.

The first step is to establish whether any caller can hand this
function an inverted pair. If none can, the arm should be stated as
unreachable. If one can, the piece should either integrate signed or
refuse. NURBS's span-locator unit (`span-locator-lands-a-nan-parameter-on-the-first-span`)
gives window readers a `ParamRange`, but this reader is built from two
point locates and keeps its own inversion semantics, so that unit does
not settle it. (NURBS orchestrator)
