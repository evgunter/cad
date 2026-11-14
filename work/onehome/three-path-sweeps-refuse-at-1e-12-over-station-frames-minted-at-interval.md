---
id: three-path-sweeps-refuse-at-1e-12-over-station-frames-minted-at-interval
kind: issue
title: two path-sweep injectivity rows refuse at ε = 1e-12 over station frames minted at Interval (at most 3.3e-14 wide), on a cap rim's General pcurve
status: closed
opened: 2026-11-14
priority: P1
closed: 2026-11-14
pr: 6953
---



Filed by the skin-turn lane (`onehome/skin-turn`), which mints each station's frame alone at the lane scalar (`crates/sweep/src/skin.rs`, `PathCarry::minted`; DESIGN-FORK-LOG row 266).

**What moved.** On main before PR 6949 a station's frame entered `Interval` as twelve point enclosures (`RigidMotion::computed_at`, gone). Each station's frame is minted at `Interval`, its tangent read through the path's hodograph (`NurbsCurve3::derivative_at`, PR 6952): at most 3.3e-14 wide, flat along the path, pinned by `crates/sweep/tests/a_station_frame_is_minted_to_its_floor.rs`. Read through `ders1` instead, the same frames reach 2.6e-13 (the 370° ribbon) and 2.5e-13 (the two-turn helix). At the default ε the five rows of `crates/sweep/tests/self_intersection_proves_injectivity.rs` that refused under a carry chained at `T` all pass. At `CAD_TOLERANCE_EPS=1e-12` two of them refuse inside `sweep_body_construction::<Interval>`, where main before PR 6949 passes both:

| Row (`self_intersection_proves_injectivity.rs`) | Refusal at 1e-12 |
|---|---|
| `a_closed_spine_reads_violated_top_against_bottom` | `pcurve_loop_continuity` on half-edge `18v1`, ±4.5e-11 |
| `the_s_duct_walls_hold_against_themselves` | `pcurve_loop_continuity` on half-edge `20v1`, ±1.9e-12 |

`a_long_helical_sweep_certifies_within_the_default_budget`, which refused through `ders1`'s frames (`pcurve_loop_continuity`, ±2.1e-12), passes at 1e-12 on the hodograph's.

**Why they refuse.** The refusal is `topo`'s loop joint on a spline chart (`pcurves.rs` `spline_gap_closes`). On the closed spine it is half-edge `18v1`, at the wall chart's corner (1, 1):
- the entry is the half-edge's stored pcurve evaluated at its start parameter;
- the predecessor's exit is the exact point (1, 1);
- the gap is metered by the chart's sup stretch, u ×0.26 and v ×11.

The pcurve is a **general fitted NURBS (degree 5), not an iso line**. Its start parameter is an enclosure `[0.5 ± 2.7e-14]`, 5.4e-14 wide, and evaluating the pcurve there gives an entry `(1 ± 8.8e-11, 1 ± 8.8e-11)`, 1.76e-10 wide: over 3000× the parameter's width. Through `ders1`'s frames the entry is 1.83e-10 wide, 4% more, so the width is not one the frame owes: it is the interval evaluation of the fitted pcurve over a wide parameter, far wider than the geometry needs (the pcurve's own speed is order one). The S duct reads alike on half-edge `20v1`.

**Why `18v1` and `20v1` are General.** Each is a cap rim, a line on the wall's boundary row whose exact image is that row. The mint's iso selection (`geom-brep` `edge_nurbs.rs` `probe_pick`, from `topo` `pcurves.rs`'s boundary-iso schedule) rejects the right candidate by a hair:
- On the closed spine its end miss encloses `[0, 1.40e-12]`; on the S duct `[0, 1.46e-12]`.
- The rim's end parameter `t₁` is an enclosure, 5.2e-14 wide: the line's length between two vertices minted at `Interval`.
- The candidate's `u = p₀ + (Δu/span)·t₁` is exactly 1, but interval dependency gives `1 ± 1.04e-13`, and the wall evaluated over that `u` is 2.6e-12 wide.

So the General pcurve is no legitimate lane; the loft's module docs' claim that every wall boundary stores its iso image stands. The fix is not local: with the selection sampled at point parameters (a choice, which the certification then checks), the iso line is picked on both rows, and its own certificate escalates on the same dependency (`MapResidual`, sample 6, hi 1.035e-12 on the closed spine and 1.095e-12 on the S duct). The source is the rim edge's parameter range being an interval at `T`.

**Ruled out:** building the walls at `T`. With the enclosure arm stepping aside wherever the lane's placements are not points:
- the closed spine and the helix refuse with margins identical to 11 digits;
- the S duct and the ribbon refuse earlier, at `CarrierKnotNotAPoint`, the carrier's non-point-knot frontier.

**Open** (a design fork): closing the two rows needs one of two things, either of which touches `topo` and `geom-brep` semantics:
- the edge's parameter range held as points (a line carrier parameterized by a chosen span);
- an iso-line evaluation and certificate that do not re-read the span.

The General pcurve's naive interval evaluation is filed separately: `work/pcert/a-general-pcurve-evaluated-over-a-wide-parameter-widens-thousands-fold.md`.

**The closed INLANE row's premise no longer holds.** `work/inlane/a-path-sweeps-station-turn-at-interval-widens-its-walls-past-the-band-at-the-tightest-eps.md` was closed by PR 6138 on the premise that every station's rotation enters the lane scalar as a point (`RigidMotion::computed_at`). That door is gone, and the entries are not points. Its reproduction (the wide-start-frame row at `CAD_TOLERANCE_EPS=5e-13`) still passes, by the PR 6949 reviewer's run, so the row stays closed. These rows are where the premise's loss shows.

**The refusal does not name the station frame.** Each refuses as the wall certificate's own text (`Pcurve(Escalated … pcurve_loop_continuity)`), not as a typed refusal naming the station whose frame is too wide for the band. That is the same ask as the variable-path row's (`work/onehome/a-path-held-at-a-lane-scalar-owes-a-roll-that-follows-it.md`, "Must hold"): a typed refusal naming the station frame.

**Pinned.** The two rows are pinned red at 1e-12 (`test_utils::pinned_red::row`) on these refusals, citing this row; the pins lift with its fix.

## Closed

Closed by PR 6953 (`onehome/rim-span`), which settles fork 269 (record on `analysis/design-fork/edge-span`): a line rim's span is the sketch segment's own length (`crates/sweep/src/swept.rs`, `placed_segment_spec_about`'s `SegmentKind::Line` arm), not its placed vertices' distance, and `crates/geom-brep/README.md` "Parameterization conventions" says an edge's bounds are stated from the quantity that defines them, the vertices certifying them (geom-core W1; DESIGN `[intent.stored]`). Each cap rim's `t₁` is a point, so the probe picks its iso image and the joint reads the exact boundary row. Both pins are lifted; the closed spine and the S duct pass at 1e-12, as the long helix does since PR 6952. The whole `self_intersection_proves_injectivity` suite passes at the default ε and at 1e-6, and at 1e-12 apart from the cylinder tee, which is red on main and GREEN's (`work/green/nightly-reds-the-1e-12-rows.md`). Exact chart ends, an `IsoLine` storing its two ends, are BOXFINISH's `boxfinish/chart-ends` unit (`work/boxfinish/check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends.md`), not this row's.
