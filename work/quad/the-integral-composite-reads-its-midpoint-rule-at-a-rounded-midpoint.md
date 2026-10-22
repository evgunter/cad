---
id: the-integral-composite-reads-its-midpoint-rule-at-a-rounded-midpoint
kind: issue
title: quad: the integral composite lane evaluates its midpoint rule at the rounded f64 midpoint, so the first-order term f_d·δ·h sits outside its remainder
status: open
opened: 2026-10-22
priority: P1
cost: E
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
