---
id: a-sweep-on-a-boxed-turned-sketch-plane-refuses-its-ends-at-first-order-in-the-box
kind: issue
title: A sweep on a sketch plane turned by a boxed angle refuses its carriers' ends at first order in the box, before check 1
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Filed by ONEHOME (2026-11-21) from the measurement on
`work/onehome/check-1-reads-a-turning-frames-length-off-its-bracket.md`.
It stands behind that row: with check 1's frame half gone
(`carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes`),
this refusal still comes first.

**The case.** A disc of radius r = 5 mm and depth r, built at `Interval`
by `sweep::extrude` on `SketchPlane::from_frame(RigidMotion::rotation(y,
30° ± δ))`. The probe is `crates/sweep/tests/check1_turned_probe.rs` on the unmerged branch `onehome/check1-turned-probe` (`e487c39388`, on main `6abbae4ae5`), row `check1_turned_disc`.

**What it reads.** The extrude refuses in its own certification, before
any finish reads check 1:

| δ | refusal | margin |
|---|---|---|
| 1° | `carrier_endpoint_start` (`EndpointStart`, sample 0) | `[0, 3.37e-4]` |
| 0.1° | the same | `[0, 3.37e-5]` |
| 1e-2° | the same | `[0, 3.37e-6]` |
| 1e-3° | the same | `[0, 3.37e-7]` |
| 1e-4° | the same | `[0, 3.37e-8]` |
| 1e-5° | the same | `[0, 3.37e-9]` |
| 1e-6° | `pcurve_line_chart_incident` (an incidence) | `[0, 1.3e-9]` |
| ≤ 1e-7° | builds, and check 1 decides | |

The margin is first order in the box, about `3.9·r·δ`. It reads the same
on `onehome/carrier-frames` at `6762df403e` (PR 7131), where the plane's
frame is a boxed `RigidMotion`.

**Why it matters.** BOXFINISH measured that no document extrudes on a
turned plane (`eval::wire` builds on `SketchPlane::xy()` and moves the
body), so this door is reached by the API, not by a document. It is
P2: `delete-the-box-scalar-sweep-leaf-switch` is not blocked on it
(the check-1 row's 2026-11-14 measurement: the switch plays no part).
BOXFINISH's `boxfinish/sweep-frames` is the earlier reading of the same
door (`newell_plane_residual` at `Interval`).
