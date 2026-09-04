# TRIM-3 — the chart-boundary description that tightens M10-5's clearance windows

Item `work/trim/clearance-window-tightening-needs-chart-boundary.md`
(M10-5's declared deviation D3). Branch `trim/3-chart-boundary`.
Difficulty pre-logged **M** (two PRs: PR-1 M-low, PR-2 L; argued under
"PR shape"); task class **NUMERIC** (interval geometry, funnel rows,
ε-scale cells). Survey run 2026-09-04 against main `ec4c58e4b`, every
cite re-derived by symbol on that head.

A *pcurve* is an edge's image in a face's chart `(u, v)`; the *carrier
window* is the chart rectangle the E7 engine subdivides
(`crates/editor-core/src/clearance.rs`, `window_of`, `:2060`). Today a
window is a certified superset of the face and nothing more: an
L-shaped cap's window is its bounding square, a cylindrical face's is
the whole turn. This unit gives the engine a certified **outer bound of
the trimmed region in the window's own chart** and lets it discard
cells that bound certifies the face never reaches.

## What the survey refuted (binding premise corrections)

1. **"Both [shapes] pinned in `m10_5_r2_probes_interval.rs`" — half
   true.** The L-cap row is there
   (`a_violation_witness_can_land_where_the_face_is_not`, `:641`, bound
   0.3, true distance 0.45). No row named "coplanar" exists in any M10
   suite. The coplanar-pair instance is R1's
   `e2e_channel_slider_over_an_epsilon_box`
   (`m10_5_r1_probes_interval.rs:833`): the U-channel's caps and the
   slider's caps, on **two bodies**, not "two coplanar faces of one
   body". A second L instance is R1's
   `an_l_shaped_face_is_violated_where_it_has_no_material` (`:293`,
   bound 0.52, truth 0.539). All three flip.
2. **The window's chart is not the pcurve layer's chart for planes.**
   `window_of` RE-CHARTS every plane (`in_plane_axis`/`chart_frame`,
   `:2086-2097`) because the stored `u_ref` is sign-hulled at the
   interval scalar for vertical walls. A description in the stored
   chart would be useless on those faces and unusable on any; the
   description is therefore computed **in a chart the consumer names**.
   Cylinder/cone/sphere/torus keep their stored chart (`:2101`).
3. **Cell dropping alone does not flip the L-cap row.** The exhibit
   arm (`Sweep::run`, `:2543-2559`) probes an indeterminate ROOT pair
   at depth 0 through `verify_witness`'s 9-station lattice (`:2843`),
   whose stations `(0.5,0.5)`, `(1,0.5)`, `(0.5,1)`, `(1,1)` all lie in
   the L's notch. The phantom witness is minted before any cell is
   split. The consumer edit has a third site: station admission.
4. **Extruded bodies carry no stored pcurves.** `sweep::extrude` is
   `T: Decide` and never calls `mint_pcurves` (only revolve, loft,
   blend surgery, boolean ops, transform and STEP import do). Every
   fixture in the M10-5 suites is an extrude. The description must
   DERIVE its images, which the loop walk already does: `walk_loop`
   (`crates/topo/src/pcurves.rs:1401`) re-derives every half-edge
   through `chart_pcurve` / `nurbs_iso_derive` and never reads a cache.
5. **`chart_region.rs` cannot supply it.** Its `extract_face_uv`
   (`:2502`) is private, reads STORED caches on curved charts
   (`MissingCache` on every extruded cylinder wall), and refuses any
   non-straight image (`NonPlanarTrim` on the bumped block's arc). Its
   parity walk is `crate::ray_parity` — `pub(crate)` in `topo`
   (`lib.rs:182`). So the outside test lives in `topo`, reusing that
   walk; `chart_region.rs` is not touched.
6. **`Fitted`/`General` pcurves never reach the consumer today.**
   `PcurveCache::certify_fitted` has no shipped caller; `General` is
   minted only by `nurbs_iso_derive` on NURBS charts, which `window_of`
   refuses (`:2099`). The envelope arm below is specified and
   unit-pinned in `topo`, and has no e2e row.
7. **A box is not enough — measured on the fixtures.** The L's box is
   `[0,1]²`, the U-channel cap's is `[0,3]×[0,2]`: each IS today's
   window. Only the cylinder rows (bumped block, quarter annulus) move
   on a box alone.
8. **Tightening does not make `m = M`.** `measure::WINDOW_TIGHTENING`
   (`measure.rs:817`) says this item "retires" the
   `Certified::LowerBoundOnly` refusal. It does not: cells straddling
   the boundary or within `K·ε` of it are kept, so `window_hi` stays a
   window quantity. The refusal stays; the const's doc and its
   `work/m10/` path (the item lives in `work/trim/`) are corrected as a
   rider, as are the two `work/m10/` pointers at `clearance.rs:91,40`.
9. **SHELL-3 moves the same functions.** Ruled B on #1737
   (`work/shell/SHELL-3.md`): `window_of`, `Sweep`, `split`,
   `verify_witness`, `min_separation` move into `topo` behind
   `interval`, spec drafted, not dispatched. PR-2 below is written by
   FUNCTION and applies wherever those functions sit at dispatch.

## 1. The description — `topo::chart_boundary`

```rust
pub fn chart_boundary<T: PcurveFittedLane + SpanLocate + Bounds>(
    body: &Body<T>, face: FaceKey, chart: &Surface<T>, band: Band,
) -> Result<ChartBound<T>, PcurveMintError>
```

`chart` is the chart to describe in. Contract: its locus is the face's
carrier (the consumer passes the stored surface, or for a plane its
re-chart, whose origin and normal are the stored ones). The function
walks every loop (outer, then rings, D9 order) with `walk_loop`, which
takes the surface as an argument and needs no change beyond
`pub(crate)` visibility on it, `Walked` and `is_plus`.

**Value.** `ChartBound { loops: Vec<ChartLoop>, hull: ChartWindow<T>,
period: Option<T> }`; `ChartLoop { edges: Vec<ChartEdge<T>>, ring:
bool }`; an edge in loop direction (entry → exit, `is_plus`) is one of

- `Segment { a, b }` — the image is a straight chart segment BY
  STRUCTURE: `IsoLine`, `IsoArc` (its UV image is the segment
  `p0 → p0 + pd`), or a `Harmonic` whose four trig channels are
  exact-structural zeros (`chart_region::exact_zero`'s C6 idiom, read
  here the same way). Line carriers arrive with `a = b = 0` exactly
  (`carrier_harmonic`, `pcurve_cache.rs:2109`); a rim on an
  axis-aligned cylinder does too.
- `Envelope { a, b, image: Point2<T> }` — anything else. `image` is
  the pcurve evaluated at the WHOLE span as one enclosure,
  `pcurve.eval(t0.enclosure_hull(t1))`, which at the interval scalar
  is a certified box around the image (interval `sin_cos` over a range
  is exact-in-family); at a point scalar `enclosure_hull` is poison and
  the box is poison, which the test below never certifies against. A
  `Fitted`/`General` image takes `chart_box` (the control-hull box)
  instead, widened per axis by the stored certificate's `envelope`
  (`PcurveCertificate::envelope`, metres) divided by that axis's arm.
  A derived `General` with no certificate refuses
  (`PcurveMintError::Certify{ UnsupportedCarrier }`).

`hull` is the min/max box of every vertex and every envelope box.
`period` is `Some(τ)` for `Cylinder | Cone | Sphere | Torus`, the
knot-domain length for a spline chart that `chart_u_period` calls
closed, `None` for a plane — decided by surface KIND here; note
`chart_u_period` (`pcurves.rs:1042`) answers `τ` for a plane and must
not be asked.

**Certified statement.** For a loop `L` let `P_L` be its closed chord
polygon (each edge's `a → b`) and `E_L` the union of its envelope
boxes. The face's trimmed region, in this chart and on the walk's
branch, is contained in `(closure(int P_outer) ∪ E_outer)` minus
`⋃_rings (int P_ring \ E_ring)`, up to the ε-accuracy of the images
(below). The argument: the true boundary differs from `P_L` only inside
`E_L` — a lune between an arc and its chord is bounded by a closed
curve lying in the arc's convex box, hence lies in that box.

**Branches (the seam subtlety).** The walk pins each loop's branch from
its first half-edge's principal azimuth and certifies closure. The
description keeps that branch: an extruded arc's face is `[0, θ]`, a
negative-angle revolve's band is `[θ, 0]` (`revolve/mod.rs:14-25`) —
the azimuth is a real number, never folded into `[0, τ)`. Two rules:

- a loop whose walk closes one whole period off (the `± τ` arm of
  `loop_closes`, `:1174`) wraps the chart and bounds no chart polygon:
  refuse `PcurveMintError::LoopWraps { face, loop }` (new variant).
  No head constructor makes one (extruded and revolved closed walls
  carry a wrap strut / seam chain), so this is a fence, not a path;
- a ring is emitted once per whole-period shift `k` at which its
  shifted hull meets the outer's hull (at most two copies). Every copy
  is a genuine lift of the ring, so parity over all copies is exact; a
  missed copy could only make the test certify LESS.

**Accuracy premise, stated once.** The plane arm of `chart_pcurve`
(`pcurve_cache.rs:4096`) is affine and exact in interval arithmetic.
The cylinder arm's axial channel is exact; its azimuth channel is the
closed form of a carrier that lies on the chart, and a stored cache's
`envelope` bounds the residual where one exists. Everything the test
certifies is decided through the funnel at `escalate = K·ε ≥ 10ε`
(`Interval::sign_within`, `interval.rs:600`), which is what absorbs
an ε-scale image error: nothing closer than `K·ε` to the described
boundary is ever dropped. No widening constant is minted.

## 2. The outside test — `ChartBound::metred(arms).certifies_outside`

The consumer meters the chart once per window: `arms = (a_u, a_v)`
metres per chart unit — plane `(1, 1)`, cylinder `(r, 1)`, both EXACT
(`chart_region.rs`'s `certified_arms` says why these two need no
bound). A `MetredBound` is the description with every point scaled by
the arms, so every margin below is metres and every row is a length.

`certifies_outside(rect) -> bool` for a metred rectangle `R` (its four
`f64` bounds are exact structure; a point is `R` with zero extent):

1. **No edge meets `R`.** For a `Segment` with interval endpoints
   `A, B` (the family of all realizations is `hull(A ∪ B)`), the
   separating-axis test with five candidate axes: `R`'s four sides
   (`decide("chart_bound_gap", Margin::of(gap))`, `Ok(Positive)`
   separates) and the segment's own normal `n = ⊥(B − A)`, taking
   `s_c = n · (c − A)` at each corner `c` through
   `Margin::over_lever(s_c, |n|)` — all four `Ok(Positive)` or all
   four `Ok(Negative)` separates. For an `Envelope`, the box test
   alone (the chord lies in the box). Any edge not separated ⇒ **not
   certified**.
2. **A point of `R` is outside.** With (1) holding, `R` meets no
   boundary, so `R`'s centre decides for all of `R`. Run
   `ray_parity::on_boundary` then `ray_parity::ray_verdict` with rows
   `ParityRows { chart_bound_segment, chart_bound_boundary,
   chart_bound_side, chart_bound_advance }` and a two-member fixed
   schedule `(+u, +v)`; the first definite verdict counts, no verdict
   ⇒ not certified. Outside ⇔ the outer polygon answers `Out`, or some
   ring copy answers `In`. A grazing ray at one depth is answered by
   the children's different centres.

**Direction of every rounding.** All five rows are `decide` rows: a
drop needs a definite sign at `≥ K·ε`; `Zero`, in-band and poison keep
the cell. Interval endpoints widen the fat segment and the boxes
(harder to separate); envelope boxes are outer enclosures (harder to
separate); a missed ring copy keeps cells. There is no path from an
imprecise input to a drop.

## 3. The consumer edit — three functions in `clearance.rs`

`Window` gains `bound: Option<MetredBound<Interval>>`. Everything else
in the file — `CLEARANCE_MARGIN`, `SELF_INTERSECTION_GAP`, `decide`
sites, `split`, `combine`, `facet_restrict`, `LeafFold`,
`clearance_over`, the wedge rule, the BVH admission — is untouched.

**(a) `window_of` (`:2060`).** After the arm match, for `Plane` and
`Cylinder` only: `chart_boundary(body, face, &charted, band)` where
`charted` is the re-chart (plane) or the stored surface (cylinder).
`Ok(b)` ⇒ the root is tightened and `bound = Some(b.metred(arms))`;
`Err(_)` ⇒ today's window, `bound = None`, and the report counts it
(below). Root rule: `v ← v ∩ hull.v` (both certified supersets in one
coordinate). `u`: plane ⇒ `u ∩ hull.u`; cylinder ⇒ `hull.u` verbatim
when its extent is `≤ τ` (one ulp of slack, `full_turn`'s idiom), else
`full_turn()` — NEVER `[0, τ] ∩ hull.u`, which is meaningless mod `τ`
and empties a `[θ, 0]` band. `refines` runs on the tightened root.
Cone, sphere and torus keep today's windows: their loops' region side
(a polar cap is one circle) needs an argument this unit does not make;
the implementer files `work/trim/clearance-window-cone-sphere-torus.md`
with the PR that lands (a) — a disclosed residue owes its file.

**(b) `Sweep::run` (`:2480`) and `min_separation` (`:1588`).** At the
head of each task, before any enclosure: if `x.bound` certifies
`pair.a` outside or `y.bound` certifies `pair.b` outside, the pair is a
leaf — `receipt.discharged += 1; receipt.outside += 1; continue` — with
no width recorded and no `decide` at the clearance site. Vacuous over
the face and counted where a leaf must be counted: `holds()` and the
forest identity are unchanged; `outside ≤ discharged` is a sub-count,
serialized as `outside=` and rendered. In `min_separation` the dropped
pair contributes to neither `hi` nor `floor`: `lo` stays a lower bound
on the faces' minimum (their points are in kept cells); `window_hi`
bounds the minimum over root-minus-dropped, a superset of the faces.

**(c) `verify_witness` (`:2784`).** A lattice station whose `(u, v)`
the window's `bound` certifies outside is not a candidate. A cell pair
with no admitted station returns `Err`, which the caller already turns
into `WitnessUnverified` — the honest arm for a proven-violated window
cell whose face material, if any, sits between stations. The f64
stations are tested against the interval description: a definite
verdict holds for every realization, the f64 rebuild's chart included.

**Semantics preserved.** `Holds` was "every point pair on the two
windows satisfies the bound"; it becomes "every pair on the two roots
minus cells the description certifies empty of face" — a superset of
the faces still, so the certificate about the faces is unchanged and
the looseness runs the same way. `Violated` still carries an f64
verified pair; its witness may still be a window point, but only in a
cell not certified empty. The strictly-positive question is the same
sweep at `c = 0` and changes with it.

**Report.** `ClearanceReport` gains `windows: (usize, usize)`
(tightened, loose) so a phantom `Violated` can be read against a
window the description refused. Serialized last; no golden pins the
text. **Open question Q1** — drop it if the orchestrator prefers a
smaller seam.

**Riders on the seam (small, same PR).** `GeometryWitness::a_uv` doc:
azimuth is on the walk's branch, not folded. `WINDOW_TIGHTENING`: path
`work/trim/…`, doc sentence corrected per refutation 8.
`clearance.rs:40,91` pointers to `work/curved/` and `work/trim/`.

## 4. Rows, each red-first, with the mutant it kills

`topo` unit rows (PR-1), on hand-built `ChartBound`s at `Interval`:

| row | asserts | kills |
| --- | --- | --- |
| T1 L-polygon: cell deep in the notch | certified outside | drops nothing (no-op test) |
| T2 cell inside the material | not certified | parity sense inverted |
| T3 cell overlapping an edge, centre outside | not certified | SAT skipped, parity only |
| T4 ε cells on one edge: gap `0.5ε`, `3ε`, `20ε` (K = 10) | kept, kept, dropped | raw compare / widening constant |
| T5 ring copy: outer `[-3, 3.2]`, ring `[3.1, 3.4]` at `k = 0, -1` | cell at `-2.95` dropped, at `3.15` dropped | single ring copy |
| T6 `General` pcurve on a NURBS chart | envelope box widened by `envelope / arm` | control hull used bare |
| T7 wrapping loop (hand-built one-rim face) | `LoopWraps` | polygon of an open lift |
| T8 grazing centre (edge at the cell's mid-`v`) | not certified; both children certified | single-ray delay hidden |

e2e rows (PR-2), `crates/editor-core/tests/trim_3_windows_interval.rs`
plus flips in place:

| row | fixture / bound | asserts | kills |
| --- | --- | --- | --- |
| E1 (flip R2 `:641`) | L + block in notch, cap vs block, 0.3 | `Holds`, `outside > 0` | root probe not filtered (refutation 3) |
| E2 (flip R1 `:293`) | L-plate + floating block, 0.52 | `Holds` at 0.52 and 0.45 | box-only description |
| E3 (flip R1 `:833`) | U-channel whole-body, 0.3 | `Holds` | coplanar pair unaddressed |
| E4 planted-tight | L + block at `x ∈ [0.45, 0.55]` in the notch, cap only, 0.3 | `Violated`, witness `a_uv.x ≤ 0.4 + K·ε` | a description too tight by any amount, an inverted parity, a dropped boundary cell |
| E5 (flip R1 `:392`) | bumped block, strict | `Holds`; `candidates` may be 0 | full-turn root kept |
| E6 (flip R1 `:509`, +assert) | quarter annulus about ẑ, phantom quadrant, 1.0 | `Holds` | full-turn root kept |
| E7 negative revolve | `ang(-π/2)`, block in the real quadrant at 0.12, 1.0 | `Violated`, `d ≈ 0.12` | root `= [0,τ] ∩ hull` (empties the band) |
| E8 refusal is identity | body with a NURBS-carrier edge on an admitted plane (plane×torus split) | verdict equals today's; `windows.1 ≥ 1` | refusal turned into `Unsupported` |
| E9 (re-express M10-6 R1 `:273`) | notch measure | `lo ≤ 0.269`, `hi ≥ 0.269 − K·ε`; `LowerBoundOnly` still refuses `AtMost` | `Certified` arms retired |

`self_intersection_over_a_sound_body_examines_nothing_r2_finding`
stays: `CellReceipt::default()` has `outside = 0`. R1's y-axis quarter
annulus (`:435`) keeps its print-only shape: its hulled `u_ref` gives
the walk wide azimuths, nothing certifies, and the root falls back to
`full_turn` — the `interval-orthonormal-basis-sign-hull` issue's
territory, not this unit's. `the_cost_curve_is_flat…` and the
`min_separation` width rows are re-measured, not re-asserted.

## 5. Fences

- **`editor-core/src/clearance.rs` is SHELL's ground** (its
  `program.md` paths) and M10's deliverable; `work/trim/program.md`
  edits it by announced seam. The seam is exactly §3: `Window::bound`,
  `window_of`, the task head of `Sweep::run` and `min_separation`,
  `verify_witness`'s station admission, `CellReceipt::outside`,
  `ClearanceReport::windows`, and the riders. Announced to SHELL and
  M10 on the away channel before PR-2 opens. If SHELL-3 has moved the
  functions, PR-2 targets `topo`'s copy and the announcement names
  the new file.
- **`chart_region.rs`, `ray_parity.rs`: read-only.** Track Q's
  `chart_region.rs` is not edited; `ray_parity` is called as
  `pub(crate)` with this unit's own rows.
- **`predicate-dimension-audit.md`** (Track Q, `trim` an owner) gains
  five rows, all `m`, under one F-row modelled on F17.
- **`pcurves.rs`** (TRIM's): `chart_boundary`, the `LoopWraps`
  variant, three visibilities. No change to the walk's decisions.
- **Out:** cone/sphere/torus tightening (residue file), NURBS windows
  (`Unsupported` stands), exact-region cells (`m = M`), signed
  penetration, the sign-hull issue.

## 6. STOP conditions (pre-registered)

1. The opening measurement (below) shows the R2 witness is minted in
   an arm other than the exhibit probe or the `Negative` arm — the
   third site is misidentified; re-survey before coding.
2. `walk_loop` at `Interval` over the suites' `ε/64` boxes refuses
   (`Escalated`) on any fixture face — the description would be the
   identity everywhere; report the row and stop.
3. E4 cannot be made to discriminate (the block's approach is
   witnessed from a wall cell, not a cap cell) — the planted-tight
   mutant has no gate; stop and re-cut the fixture with the
   orchestrator.
4. Any k-lint fires on a `pcurve_loop_*` or `pcurve_chart_*` row from
   the new caller: distribution evidence, K-REPORT runbook, never a
   geometry change.
5. SHELL-3 dispatches while PR-2 is open: pause PR-2, announce, land
   after the move.

## 7. PR shape, difficulty, task class

**PR-1 (M-low, `topo` only):** `chart_bound.rs` (type, metring,
`certifies_outside`, rows), `chart_boundary` in `pcurves.rs`, T1–T8,
audit rows, K roster. No behaviour change anywhere. **PR-2 (L,
the seam):** §3 (a)(b)(c), E1–E9, the riders, the residue file. PR-2
opens only after PR-1 merges. Numeric class because every new decision
is a metred margin through the funnel and the acceptance is ε-shaped;
M overall because of the three-site consumer edit whose third site
(refutation 3) is the one a box-only or cell-only reading misses, and
the periodic-root rule (E7) that a natural spelling gets wrong.

**Opening measurement (PR-2, before code):** run E1's fixture on head
with a print of `pair.depth` and which arm minted the witness; quote it
in the PR. **Lane obligations:** `docs/prompts/implementer-discipline.md`
binds; own `CARGO_TARGET_DIR` outside the worktree; hosted CI is the
verification of record; no `CI-Config` trailer; no `-A` adds; announce
the seam before opening PR-2; do not merge PR-2 without the SHELL/M10
acknowledgement on the channel.

## 8. Open questions for a ruling

- **Q1** `ClearanceReport::windows` — keep (recommended: a phantom
  `Violated` on a loose window is otherwise indistinguishable) or drop.
- **Q2** Module split — `ChartBound` + test in a new
  `topo/src/chart_bound.rs` (recommended; `pcurves.rs` is 2.2k lines
  and the test has no minting semantics) or all in `pcurves.rs`.
- **Q3** Whether refutation 8's finding — `WINDOW_TIGHTENING` names a
  recourse this item cannot deliver — wants an item for the
  exact-region subdivision, or the const's doc simply stops promising.
