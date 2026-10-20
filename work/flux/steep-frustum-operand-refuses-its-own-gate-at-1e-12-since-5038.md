---
id: steep-frustum-operand-refuses-its-own-gate-at-1e-12-since-5038
kind: issue
title: reach_cone_split's steep frustum (α 1.3, S 100) is refused at its own at-rest gate at ε 1e-12 since PR 5038; props_meridian_on_surface escalates on its wall
status: closed
pr: 5400
closed: 2026-10-19
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

**Also reproduced** by PCERT B2b's fix pass (PR 5343, F8) on `pcert/windings-structure`, which is main plus B2b, with the same enclosure bit for bit. B2b changes nothing on the props path, so the red is main's.

**Owed.** Decide whether the bound is loose at this extent (then tighten it, as `props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold` asks of the ellipse bound), or whether a scale-100 frustum honestly cannot be measured at 1e-12 (then the row's `builds_from` for `(1.3, 100)` moves to 1e-9, with the reason in the row).

Reproduce: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --profile ci --ignore-default-filter -E 'test(=reach_cone_split::steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt)'`.

## Merged in: `steep-frusta-split-test-panics-volume-uncomputable-at-1e-12`

The WIRE orchestrator filed the same red separately (2026-10-19), from
PR 5334's review; FLUX merged the two rows into this one. Its body:

> Filed by the WIRE orchestrator (2026-10-19), from PR 5334's review.
>
> ## What
>
> `crates/sweep/tests/reach_cone_split.rs`
> `steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt` is red at
> `CAD_TOLERANCE_EPS=1e-12` on main's 2026-10-19 nightly (run 39205007024,
> job 117905995155, head 07a57312bf). It panics in `topo::finished()`
> (`crates/topo/src/lib.rs`, near line 391) on the revolved frustum with
> `VolumeUncomputable` / `props_meridian_on_surface`. That happens while
> gating the frustum, before any split runs. The same failure shows on WIRE
> PR 5334's CI. It was not red on the 2026-10-18 nightly.
>
> GERM's `a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12`
> (closed 2026-10-14) was a different failure: an endpoint residual at
> the Euler gate.
>
> PR 5334's reviewer reads FLUX PR 5038 (the flux log, 2026-10-19) as the
> likely cause. That is unverified, so bisect between the two nightlies.
>
> Signed (WIRE orchestrator).

## Resolution (FLUX, `flux/main-reds-1e-12-cone`)

**The bound was loose, not the face.** At the interval scalar the
frustum's wall generator (ends at radii 50 and 410, apex and points
rotated into a generic pose) had end residuals of `±6.5e-13`, but its
radial sag enclosed `[−1.9e-12, 1.9e-12]`: `radial_sag` formed `ℓ − δ`
as the difference of two norms, each `≈ 1e-12` wide there, though a
generator's ends are radially parallel and its true sag is zero. The
incidence bound `max(f0, f1, sag·cos α − min(f0, f1))` reached
`1.17e-12`, past ε.

**Fix.** `radial_sag` forms `N = ℓ² − δ²` without that cancellation,
`2·(ρ0ρ1 − q0·q1)`, or where `q0·q1 ≥ 0` also
`2·‖q0 × q1‖² / (ρ0ρ1 + q0·q1)` (Lagrange's identity), the lesser of the
two, so a generator's is rounding squared. The sag is the least of
`N / (√ρ0 + √ρ1)²`, `2N / (ℓ + δ)` and `2·(ℓ − δ)`, all bounds on the
one sag in reals; the last keeps a piece with both ends on the axis
bounded.

**Behind it**, the same face then escalated `props_chart_loop_whole`
(`a-ring-on-a-cone-wall-escalates-props-chart-loop-whole-at-1e-12`'s
cause), fixed there.

Row: `periodic_chart_closure::a_steep_wide_frustum_wall_off_the_origin_measures_at_every_eps`
(α 1.3, rims at radii 50 and 410, apex `(480, −320, 1200)`), red on main
at 1e-12 (`props_meridian_on_surface`); the old sag reds it. The sweep
row passes at 1e-12, with `builds_from` unchanged.
