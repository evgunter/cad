---
id: a-crossing-parameters-check-bounds-across-the-face-and-its-readers-decide-along-the-edge
kind: issue
title: A stored crossing parameter is admitted by a check that bounds distance across the face, and its readers decide on position along the edge
status: open
opened: 2026-11-24
priority: P1
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


Found answering Ev's question on `[ev]` PR 7641 ("does any decision depend on what the proposal happened to be?"), 2026-11-24. Three designers traced the readers of an admitted value independently and each reached this. **Argued from the code; nothing was run.** Line numbers are `origin/main` of that day.

## What

A crossing parameter with no closed form is found at `f64` and admitted because its point reads on the other face within the band. That check bounds distance across the face. Two parameters it would both admit can sit band ÷ sin θ apart along the edge, θ the crossing angle. Readers of the stored parameter then decide on position along the edge, at the band:

- `crates/geom-brep/src/curve_face/composite.rs` `settle` (about 742–789; rows `spline_composite_root_on`, `_at_end`, `_roots_apart`): at a crease, at an end (the root dropped as the end vertex's), roots apart (else a tangency). No transversality floor in the file.
- `crates/geom-brep/src/curve_face/spline_face.rs` `settle` (`at_end` about 1158, `roots_apart` about 1172). The designers differ here: one reads the proposal as the middle of a proven cell whose width `settle` charges (so no exposure), one reads `at_end` on `C(tm)` at the band (exposed). Unsettled.
- `crates/topo/src/boolean/reduce.rs` about 4057 (`curved_boundary_containment` at `m.point`; another designer cites `contfp_decided` about 1308–1330): the pierce `In`, `OnEdge` or `OnVertex` on the other face. Two definite verdicts, two topologies, neither a refusal: the clearest case.
- `crates/topo/src/split.rs` `split_edge_minting` (about 290) through `Span::place_param_read` (`crates/geom-brep/src/curve_pair/span.rs:734`): inside the span, or at an end (`SplitParamNotInterior`). Split or no split.
- `split.rs` about 316 (`tangent_piece` over each child's own stations, which are fractions of `[t0, tm]`): possibly; not traced.
- `crates/topo/src/sector_face.rs` about 495 and `pcurves/tier3.rs` about 429 read a tangent or normal at the admitted place; the certificate bounds position, not tangent. Not traced.

What hides it today is the search: bisection on a strict sign to adjacent floats lands at the crossing. The body is right because the proposer is good, which no check says. At `Interval` the search reads enclosure midpoints, so its proposal can differ from the `f64` run's, and a leaf and its witness could decide differently in these zones (a guess; `drive`'s comparison not read).

## What it asks

1. A constructed case first: an `f64` boolean with a spline edge crossing a face at a few degrees, its true crossing about two bands from the edge's end and, separately, from a trim edge; move the admitted parameter inside the admissible stretch and compare `settle`, `split_at` and the containment verdict. Check first whether `boolean::reduce` classes the end vertex On before the pierce reaches `split_at`. If nothing flips, say what prevents it and close the sites it clears.
2. For each site that flips: decide against the model's description (is the end vertex within the band of the face; is the trim edge within the band of the carrier along the admissible stretch) or carry the stretch as an allowance, and refuse as unresolved where neither can be done.
3. The other stored solved values, untraced: `replace_face::carrier_foot`, `SplineWall::paths`, the rim arms' `computed_foot`.

The rule this is measured against is the paragraph `[ev]` PR 7641 adds to DL5.2 (nothing is decided from an admitted proposal beyond what its check bounded; the door returns the value with that bound). It waits on Ev there; step 1 does not. The solved-value door unit (`a-solved-value-the-body-stores-is-admitted-by-its-check`) was asked to keep its return open to carrying the bound and to record per site what the check bounds.

The designers' reports: `~/.local/share/cad-work/scalar-forks/design-fitted-{A,B,C}-round2.md` (local), their `For Ev` halves verbatim in PR 7641's body.
