---
id: a-crossing-parameters-check-bounds-across-the-face-and-its-readers-decide-along-the-edge
kind: unit
title: A stored crossing parameter is admitted by a check that bounds distance across the face, and its readers decide on position along the edge
status: dispatched
opened: 2026-11-24
priority: P0
cost: H
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

## Measured, 2026-11-24 (a reviewer of PR 7651, on head `544ecd51de`; DEMONSTRATED on a verbatim copy of `composite::settle` linked against the real crate)

- **`settle` itself does answer three ways on admissible candidates.** Line `z = 1e-3·(t − t*)`, `t* = 1 − 2e-7`; every candidate below has `|z| ≤ zero` and is admitted: `t* − 9e-7 … t* + 1e-7` is reported; `1 − 5e-9` is `Escalated` (`spline_composite_root_at_end`); `1 − 5e-10` and `1` are dropped as the end's.
- **Production does not reach it on that fixture.** `crossings` on the same net returns `Ok([])` (the walk reads the end within the band first), and with the end past the band (`t* = 1 − 2e-5`) returns `t*` to the last bit. By inspection: `bisect_row` (`composite.rs:696-720`) bisects on the certified form's strict sign and asks the residual only where that enclosure straddles zero, so the candidate sits within the enclosure's width of the root, not anywhere in the admissible stretch.
- So at this site the exposure is the door's contract, not today's output: the check admits more than the search proposes, and nothing states the search's reach. Step 1's constructed case now needs a net on which the certified form's enclosure is wide at the crossing (a high-degree or badly conditioned span). `spline_face::settle`: a second independent reading that its cell is charged; unsure where a narrowing stops early (the at-end reading is taken on the box's middle before the box is compared to the ends).
- Untouched by this measurement: `reduce.rs`'s containment, `split_edge_minting`, and the other stored solved values.
- A second reviewer of PR 7651 (by arithmetic, its probe not run): the same three answers at `composite::settle` on `z = 0.01(t − c)`, `c = 1 − 3e-8`. On `spline_face::settle` it reads the candidates' points as within 2·zero of each other, so no 1/sin θ stretch, but a drop-versus-refuse flip at an end is still possible. So `spline_face` is narrower than `composite`, not clear.

## Measured, 2026-11-24 (the measurement lane; rows in `crossing_parameter_rows.rs` beside the two `geom-brep` arms and in `crates/topo/tests/a_crossings_containment_turns_on_its_admitted_parameter.rs`, band `(1e-9, 1e-8)`; DEMONSTRATED unless marked)

**The reader's answer does turn on the admitted parameter, at every reader tried.** Hand-moved inside the admissible stretch of a crossing at a sine of `10⁻²`, the true crossing two escalation lengths from the edge's end or the trim edge:

- `reduce.rs`'s containment (`contfp_decided` on the plane arm's root): `In` at the true crossing, `OnEdge` over the trim edge, escalated five zero bands short of it, `Out` three escalation lengths past it. Three definite verdicts and a refusal (`containment_answers_four_ways_on_parameters_the_on_reading_admits`, read through the public `contfp`; run as an example binary with the suite's body, the aggregated `topo::all` binary not built locally).
- `composite::settle` and `Span::place_param_read`: reported and `Inside`; dropped and `End(Stop)`; escalated at both. They read the same chord at the same band, so a parameter `settle` reports is one `split_edge_minting` reads inside: `SplitParamNotInterior` is not reachable from a parameter this arm reported on the edge it was found on (`geom-brep`, `settle_and_the_splits_interiority_answer_alike_on_admitted_parameters`). `crossings` on that carrier returns nothing: the walk reads the span's end within the band of the surface first, so the end vertex is classed before any pierce reaches `split_at`.
- `spline_face::settle`: reported at the box's own middle, dropped, escalated (`settle_answers_three_ways_on_parameters_its_on_reading_admits`). The at-end reading is taken on the middle before the box is compared to the ends: of two boxes that hold the true meeting and reach past the end, the one whose middle is at the meeting refuses `Meter` and the one whose middle is half a zero band short of the end is dropped (`a_box_past_the_end_is_dropped_on_its_middle_before_it_is_compared`).

**What the production path proposes, and what bounds it.**

- `composite::crossings` at `f64`: `bisect_row` returns the first midpoint at which the certified form reads no strict sign and the `f64` residual reads ON. Its reach is the stretch where that enclosure holds zero; nothing compares the stretch to the band. On a plane near its frame origin it is rounding (`10⁻⁶` zero bands along the edge). It grows with the form's conditioning, not the degree: between the proposal and the `f64` residual's own root, in zero bands along the edge at sines `10⁻²` and `10⁻³` on a cubic: torus 10 m by 1 cm, 0.2 and 24; torus 100 m by 0.1 m, 2.4 and 244; cylinder of radius 1 km, 0.1 and 1; plane with its frame origin 10 km away, 0.1 and 5 (`on_a_thin_torus_the_proposal_sits_bands_along_the_edge_from_the_residuals_root` pins the 244). A torus of 100 m by 1 cm at `10⁻²` refuses a clean crossing: a midpoint in the stretch reads in the gap and the bisection escalates on its iterate.
- `spline_face::isolate` at `f64`: the proposal is the middle of the Krawczyk-narrowed box, which holds the true meeting; the box is `1.4·10⁻¹⁵` m of the line wide on a metre patch and `1.5·10⁻¹²` m on a kilometre patch at a sine of `10⁻²`. Nothing compares its width to the band, but it is the residual's rounding over the sine, so this arm is not exposed at `f64` short of a patch about `10⁶` m across (`at_f64_the_walks_box_is_far_narrower_than_the_band`).
- The sweep itself was not run: `topo`'s unit-test binary did not build inside a ninety-minute hold of the build slot. On the containment row's own arc and plane `crossings` returns `t = ¼` to `2·10⁻¹⁶` (a scratch example, not committed), and BY-INSPECTION the plane arm hands that parameter to `contfp_decided` unchanged (`curve.carrier().eval(t)`), as `spline_wall_event` hands it to `curved_boundary_containment`. So on that fixture the sweep reads `In`, and a sweep that lands on another verdict needs the thin torus as a trimmed face, which was not built: the two measured halves are the proposer's reach and the reader's answers.

**At `Interval`.**

- `composite::crossings`: the leaf's proposal differs from its witness's by up to the box's width over the sine. With the net's heights widened by `±0.4` zero at a sine of `10⁻³`, the witness returns the crossing and the leaf a parameter 190 zero bands along the edge, both admitted (`a_leafs_proposal_sits_bands_along_the_edge_from_its_witnesss`). At `±0.9` zero the same leaf refuses.
- `spline_face`: the leaf's box is the members' roots (20 zero bands wide at `±0.1` zero, sine `10⁻²`) and its middle is the witness's proposal to `10⁻³` zero, the box being symmetric about the witness; `settle` reports it (`at_interval_the_leafs_box_is_bands_wide_and_its_middle_is_its_witnesss`).

**Not reached**: `replace_face::carrier_foot`, `mint::computed_foot`, `tangent_piece`'s child stations, `sector_face.rs`, `tier3.rs`; the containment reading at `Interval`; `drive`'s comparison of a leaf with its witness.

**Found beside it**: `composite::binomial` overflows `usize` on a degree-9 carrier against a torus (`work/splice/the-composite-arms-binomial-overflows-on-a-high-degree-carrier.md`).

## Ruled and raised, 2026-11-25

Ev on `[ev]` PR 7641 (merged): "ok, this can go ahead but we need to keep the fix to not be dependent on the proposal at P0 since it's a live inconsistent answer". DL5.2 now carries the paragraph this is measured against: nothing is decided from an admitted proposal beyond what its check bounded; the door returns the admitted value with that bound; a check that bounds less than its readers decide on is widened, or the reader re-posed. This row is a unit at P0 (cost raised to H: the door's return, the composite proposer's reach, the containment and split readers, and the leaf-against-witness agreement).

Dispatched 2026-11-25 on branch `scalar/no-decision-turns-on-which-crossing-parameter-was-drawn`. Letter **H**; tier **DUAL**: readers that decide a boolean's topology are re-posed, and the door's return is the shape every later proposal door copies.
