---
id: the-integral-composite-reads-its-midpoint-rule-at-a-rounded-midpoint
kind: issue
title: quad: the integral composite lane evaluates its midpoint rule at the rounded f64 midpoint, so the first-order term f_d·δ·h sits outside its remainder
status: closed
opened: 2026-10-22
priority: P1
cost: E
closed: 2026-10-22
---


## Found (QUAD, `quad/rational-quotient-statement`, 2026-10-22)

The midpoint rule's remainder `hull(f_dd)·h³/24` holds about the
EXACT midpoint of a cell. Evaluated at a rounded one, `m + δ`, the
rule carries a first-order term `f_d·δ·h` that no remainder covers.

**Reproduced on the rational lane.** Once its `f_dd` hulls came from
the quotient statement, `crates/geom-brep/tests/cert5_r1_patch_probes.rs`
`knots_hugging_the_trim_edges_stay_sound` (knots at `1e-9` and
`1 − 1e-9`, uniform weights 2) certified a flux of width `2.1e-13`
that excluded the oracle by `8.9e-8`: on the cell `[1 − 1e-9, 1]` the
integrand's slope is near `1e18`, and the f64 midpoint is off the true
one by up to half an ulp of 1. That branch reads the midpoint as an
enclosure now (`props::quad::cell_midpoint`, an `AtSpan` read at
`(lo + hi)/2` clamped to the cell).

**Open on the integral composite lane, inferred from the code.**
`nurbs_patch_face_rounds`' composite arm (`props/quad.rs`, the
`Collapse::At(c_ulo.midpoint(c_uhi))` / `Collapse::At(c_vlo.midpoint(c_vhi))`
pair beside `g_f_uu * remainder_cell_u`) cuts the same knot-aligned
cells and reads the same rounded midpoint. Its `f_dd` hulls are wide
today, which may mask the term, as it masked it on the rational lane
until the hulls tightened. Not reproduced there.

The area rules are not this defect: their Lipschitz pad bounds the
cell mean about any point within `h/4 + δ²/h ≤ h/2` of every cell
point, and the pad already pays `h/2`. The trimmed lane's Newton–Cotes
nodes are interval reads already.

Sweep: `midpoint(` over `crates/geom-brep/src/props/` and
`crates/topo/src/props/`; a midpoint spelled `0.5 * (lo + hi)` or read
through a cell struct's carried `umid` would not match it, and
`AreaBox::umid` is the one such carrier (area rule, covered above).

## Closed (`quad/integral-composite-midpoint`, 2026-10-22)

Reproduced, though not by the knot-hugging twin. That patch (degree 5
in u, knots at `1e-9` and `1 − 1e-9`, unit weights) refuses typed: its
global `f_uu` hull is non-finite, which masks the term entirely. The
witness that exposed it is a degree-5 patch linear in both parameters,
with its parameter interval at `[1e8, 1e8 + 0.3]`. There
`f_uu = f_vv = 0`, so the remainders vanish, and the f64 midpoint is up
to half an ulp of 1e8 off the exact one. Its certified flux was
`[0.8749999992239, 0.8749999992239]`, width `2.3e-14`, and excluded the
exact `7/8` by `7.8e-10` with the far interval in u or in v.

The composite arm now reads both midpoints through `cell_midpoint` as
`Collapse::AtSpan`. On that witness the flux is about `2.8e-8` wide and
contains `7/8`. Pinned as
`cert5_r1_patch_probes::integral_composite_midpoints_far_from_the_origin_stay_sound`.

The sweep turned up
`the-trimmed-area-rule-squares-its-chord-trapezoid-in-rounded-f64`.
