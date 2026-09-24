---
id: mint-face-surface-and-sense-reads-key-equality-where-same-chart-reads-provenance
kind: issue
title: mint_face_surface_and_sense decides 'the parent's own surface' by key equality while same_chart decides one chart by key or provenance, so a Shared second key the body records as one description carries the run's rows and resets sense
status: open
opened: 2026-09-24
priority: P1
cost: E
---


Found by the second reviewer of PR 2603
(`mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows`)
and confirmed by execution in that PR's fix pass.

**Two answers to "is the new face on the parent's surface".**
`Body::mint_face_surface_and_sense` (`crates/topo/src/euler.rs`)
inherits the parent's `sense` when `surface == inherit_surface` and
stamps `true` otherwise — its own doc says "Key equality, never a
numeric compare". `Body::same_chart` (`crates/topo/src/euler_ring.rs`),
which since PR 2603 decides whether `mef`'s moved run keeps its pcurve
rows, answers by key OR by provenance: two keys carrying one
`GeomSource` are one chart (N6). So a `FaceSurface::Shared(second)`
where `second` is a fresh key the body records as the parent's own
description is one chart to the rows and a foreign surface to the
sense bit.

**Measured** (on the sheet of
`crates/topo/tests/loop_reparenting_pcurve_rows.rs`, the lower panel
given `sense: false` through `Body::set_face_sense`, then split by the
suite's `split_low` chord):

| spec | new face's rows | new face's `sense` |
|---|---|---|
| `Shared(own key)` | `(2, 1)` — the run carried | `false` — inherited |
| `Shared(second key, one `GeomSource` on both)` | `(2, 1)` — the run carried | **`true` — reset** |

The rows say the fragment is a piece of the parent's region on the
parent's chart; the sense bit says it is a new surface facing the
mint's default way. The `sense` doc's own warning names the cost: a
fragment of a `sense: false` wall handed back facing the wrong way.
`mfkrh` takes the same helper's rule for a promoted ring and has the
same seam.

**What would close it.** Decide "the parent's own surface" the way the
rows decide it — `same_chart(surface, inherit_surface)` in
`mint_face_surface_and_sense` — or say in that helper's doc why the
sense bit is stricter than the chart (the merge door's rungs read the
sense bit too, `same_chart`'s doc says, but for a REGION question this
helper is not asking). One line either way; the row exists because
which line is a decision about the operator's contract, and the
caller that today attaches "the honest bit through `set_face_sense`"
after a `Shared` (the sweep constructors, per that doc) would stop
having to for this rung. Filed on TOPO's slate: `euler.rs` is this
program's.
