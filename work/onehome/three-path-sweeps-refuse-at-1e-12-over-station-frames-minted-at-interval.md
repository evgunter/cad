---
id: three-path-sweeps-refuse-at-1e-12-over-station-frames-minted-at-interval
kind: issue
title: three path-sweep injectivity rows refuse at ε = 1e-12 once each station's frame is minted at Interval (about 1.4e-13 wide)
status: open
opened: 2026-11-14
priority: P1
---



Filed by the skin-turn lane (`onehome/skin-turn`), which mints each station's frame alone at the lane scalar (`crates/sweep/src/skin.rs`, `PathCarry::minted`; DESIGN-FORK-LOG row 266).

**What moved.** On main a station's frame entered `Interval` as twelve point enclosures (`RigidMotion::computed_at`, now gone). Minted at `Interval` it is about 1.4e-13 wide: flat along the path, at most 4e-13 on the 48-station ribbon and 6.5e-13 on the 33-station helix. At the default ε the five rows of `crates/sweep/tests/self_intersection_proves_injectivity.rs` that refused under a carry chained at `T` all pass. At `CAD_TOLERANCE_EPS=1e-12` three of them refuse inside `sweep_body_construction::<Interval>`, where main passes all three:

| Row (`self_intersection_proves_injectivity.rs`) | Refusal at 1e-12 |
|---|---|
| `a_closed_spine_reads_violated_top_against_bottom` | `pcurve_loop_continuity`, enclosure ±4.7e-11 |
| `a_long_helical_sweep_certifies_within_the_default_budget` | `pcurve_loop_continuity`, enclosure ±2.1e-12 |
| `the_s_duct_walls_hold_against_themselves` | `pcurve_map_residual`, hi 1.0e-12 |

**Why they refuse** (measured on `onehome/sweep-walls-at-t`, probes not committed). Two widths compound.

1. **The station frame's floor.** The frame is about 1.4e-13 wide, and almost all of it is the derivative's enclosure:
   - `NurbsCurve3::ders1` at `Interval` of a point parameter gives 3–17e-13 (relative 4–9e-14, a few hundred ulps). It comes from the interval de Boor recursion, not from hulling across knots: only the end stations sit on a knot.
   - Normalizing (`UnitVec3::new`) adds almost nothing.
   - `from_aim_and_reference` adds up to 2–3×, depending on the reference.

   Evaluating the derivative through the hodograph at `Interval` is just as sound and agrees with `f64`: control net `p·(Pᵢ₊₁ − Pᵢ)/Δu` from the data, then a degree-(p−1) Cox–de Boor. It gives:
   - a derivative about 3e-14 wide, and a tangent about 6e-15;
   - a frame about 1e-14, 14× narrower.

   With it, `a_long_helical_sweep_certifies_within_the_default_budget` passes at 1e-12, and the other two still refuse.
2. **The joint's lever.** The refusal is `topo`'s loop joint on a spline chart (`pcurves.rs` `spline_gap_closes`). On the closed spine it is half-edge `18v1`, at the wall chart's corner (1, 1):
   - the entry is the half-edge's stored pcurve evaluated at its start parameter;
   - the predecessor's exit is the exact point (1, 1);
   - the gap is metered by the chart's sup stretch, u ×0.26 and v ×11.

   The pcurve is a **general fitted NURBS (degree 5), not an iso line**. Its start parameter is an enclosure `[0.5 ± 2.7e-14]`, 5.4e-14 wide, and evaluating the pcurve there gives an entry `(1 ± 9.2e-11, 1 ± 9.2e-11)`, 1.83e-10 wide: about 3400× the parameter's width. Narrowing the frames 14× moves the entry by only 4% (1.83e-10 to 1.76e-10, margin ±4.7e-11 to ±4.5e-11). So the width is the interval evaluation of the fitted pcurve over a wide parameter, far wider than the geometry needs (the pcurve's own speed is order one). It is not a width the frame owes. The S duct behaves alike: with hodograph frames it moves to the same check on half-edge `20v1`, ±1.9e-12.

**Ruled out:** building the walls at `T`. With the enclosure arm stepping aside wherever the lane's placements are not points:
- the closed spine and the helix refuse with margins identical to 11 digits;
- the S duct and the ribbon refuse earlier, at `CarrierKnotNotAPoint`, the carrier's non-point-knot frontier.

**Open** (a design fork):
- The joint's chart gap reads a fitted pcurve evaluated over an interval parameter. Does it owe that width, or should the stored pcurve on a wall boundary be the exact iso image (the loft's module docs claim every wall boundary stores one), or the evaluation be centred?
- Should the station mint adopt the hodograph evaluation of the tangent? It is implementation, and it is not enough alone.

**The closed INLANE row's premise no longer holds.** `work/inlane/a-path-sweeps-station-turn-at-interval-widens-its-walls-past-the-band-at-the-tightest-eps.md` was closed by PR 6138 on the premise that every station's rotation enters the lane scalar as a point (`RigidMotion::computed_at`). That door is gone, and the entries are no longer points. Its reproduction (the wide-start-frame row at `CAD_TOLERANCE_EPS=5e-13`) still passes, by the PR 6949 reviewer's run, so the row stays closed. These three rows are where the premise's loss shows.

**The refusal does not name the station frame.** Each refuses as the wall certificate's own text (`Pcurve(Escalated … pcurve_loop_continuity)`, `Euler(Certification … pcurve_map_residual)`), not as a typed refusal naming the station whose frame is too wide for the band. That is the same ask as the variable-path row's (`work/onehome/a-path-held-at-a-lane-scalar-owes-a-roll-that-follows-it.md`, "Must hold"): a typed refusal naming the station frame.

**Pinned.** The three rows are pinned red at 1e-12 (`test_utils::pinned_red::row`) on these refusals, citing this row; the pins lift with its fix.

**Built: the hodograph tangent** (`onehome/sweep-walls-at-t`, 6655bf301). Every station reads its tangent through the path's hodograph, at `f64` and at every lane. The frames are minted at most 3.3e-14 wide, pinned by `crates/sweep/tests/a_station_frame_is_minted_to_its_floor.rs`. `a_long_helical_sweep_certifies_within_the_default_budget` passes at 1e-12, and its pin is lifted.

**Traced: why `18v1` and `20v1` are General.** Each is a cap rim, a line on the wall's boundary row whose exact image is that row. The mint's iso selection (`geom-brep` `edge_nurbs.rs` `probe_pick`, from `topo` `pcurves.rs`'s boundary-iso schedule) rejects the right candidate by a hair:
- On the closed spine its end miss encloses `[0, 1.40e-12]`; on the S duct `[0, 1.46e-12]`.
- The rim's end parameter `t₁` is an enclosure, 5.2e-14 wide: the line's length between two vertices minted at `Interval`.
- The candidate's `u = p₀ + (Δu/span)·t₁` is exactly 1, but interval dependency gives `1 ± 1.04e-13`, and the wall evaluated over that `u` is 2.6e-12 wide.

So the General pcurve is no legitimate lane; the docs' claim stands. But the fix is not local. With the selection sampled at point parameters (a choice, which the certification then checks), the iso line is picked on both rows, and its own certificate escalates on the same dependency: `MapResidual`, sample 6, hi 1.035e-12 on the closed spine and 1.095e-12 on the S duct. The source is the rim edge's parameter range being an interval at `T`. Closing these two rows needs one of two things, either of which touches `topo` and `geom-brep` semantics:
- the edge's parameter range held as points (a line carrier parameterized by a chosen span);
- an iso-line evaluation and certificate that do not re-read the span.

The General pcurve's naive interval evaluation is filed separately: `work/pcert/a-general-pcurve-evaluated-over-a-wide-parameter-widens-thousands-fold.md`.
