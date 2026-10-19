---
id: steep-frustum-operand-refuses-its-own-gate-at-1e-12-since-5038
kind: issue
title: reach_cone_split's steep frustum (α 1.3, S 100) is refused at its own at-rest gate at ε 1e-12 since PR 5038; props_meridian_on_surface escalates on its wall
status: open
opened: 2026-10-19
priority: P2
cost: M
refs: [props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold]
---

Found by CONTACTHOLD's nightly-red lane.

**The red.** `sweep::all reach_cone_split::steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt` (slow set) fails at ε 1e-12 in the 2026-10-19 nightly (run 39205007024, `07a57312bf`). It is green at the default ε and at 1e-6:

    the frustum is not a finished body: [VolumeUncomputable { solid: SolidKey(1v1),
    source: Face { face: FaceKey(3v1), source: Escalated { cause: Indeterminate {
    margin: MarginDiag(Enclosure { lo: -2.255973186038318e-13, hi: 1.167204555044455e-12, .. }),
    band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("props_meridian_on_surface") }, check: Exact } } }]

The panic comes from `topo::test_support::finished`, on the **operand** before the split runs. It is the first row, `(α 1.3, S 100, f 0.5)`: that row prints a line when it completes at 1e-12, below its `measured_from`, and nothing printed. That row pins `builds_from = 0.0`: the revolved frustum is expected to be a finished body at every ε, and only its halves to need ε ≥ 1e-9. The enclosure straddles zero and is about 1.4e-12 wide. That is inside the band at 1e-12. The body's base radius is `0.5·S + S·tan α` ≈ 410.

**Bisected** over first-parent merges from `6faa629db4` to `07a57312bf`, building `sweep`'s `all` binary at each step. It passes at `4e0cdbd563` (the first parent of the merge) and fails at **`8ef77cd2f5`**, the merge of PR 5038 (`flux/line-iso-incidence`). PR 5038 makes the cylinder and cone line arms decide `props_meridian_on_surface` at an upper bound on the segment's distance from the surface: the ends' residuals plus `radial_sag`. At this extent, that bound no longer settles inside 1e-12 on the frustum's wall.

**Owed.** Decide whether the bound is loose at this extent (then tighten it, as `props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold` asks of the ellipse bound), or whether a scale-100 frustum honestly cannot be measured at 1e-12 (then the row's `builds_from` for `(1.3, 100)` moves to 1e-9, with the reason in the row).

Reproduce: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --profile ci --ignore-default-filter -E 'test(=reach_cone_split::steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt)'`.
