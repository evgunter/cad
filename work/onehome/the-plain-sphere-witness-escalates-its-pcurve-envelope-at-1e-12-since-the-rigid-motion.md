---
id: the-plain-sphere-witness-escalates-its-pcurve-envelope-at-1e-12-since-the-rigid-motion
kind: issue
title: Main red at ε 1e-12: spheres_crossing_off_every_edge's plain witness escalates a pcurve envelope at Interval since PR 6717
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Filed by GREEN (`green/eps-1e-12`) from nightly 43371638371 (main
`e36675c1`, 2026-11-13); reproduces on main `2c843a2e7` (2026-11-14).

## What fails

`crates/sweep/tests/spheres_crossing_off_every_edge.rs`
`the_plain_witness_builds_its_seam_crossing_at_the_interval_scalar`, at
`CAD_TOLERANCE_EPS=1e-12` only (green at the default ε and 1e-6):

    Interval a ∪ b: wanted a body, got Err(Pcurves { source: Certify { half_edge: HalfEdgeKey(3v1),
      error: Escalated { check: EnvelopeTerm(Radius), sample: 0, cause: Indeterminate {
      margin: Enclosure { lo: 0.0, hi: 3.5038530533241485e-12 }, band: { zero: 1e-12, escalate: 1e-11 },
      predicate: "pcurve_envelope" } } } })

A refusal at `Interval`, not a wrong body. The row is pinned at 1e-12 on
that stop and goes red the day the union builds.

## The merge

A first-parent bisect of main at 1e-12 over `50cfcf499..da77d8754`
names `08f0298b1`, the merge of PR 6717 (`onehome/rigid-motion`, which
rewrote `geom_core::linalg::rigid_motion`); its first parent `61844c4a7`
passes. The witness's balls are placed by `topo::transform_rigid`
(the test's `placed`: a turn, then a translation), the door that PR
rewrites. Which of the two moves the envelope into band is not
measured. The escalating check is PCERT's `pcurve_envelope`
(`geom_brep::pcurve_cache`).

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all \
      -E 'test(the_plain_witness_builds_its_seam_crossing_at_the_interval_scalar)'

## Done when

The witness's Interval union builds at 1e-12, its envelope decided
rather than the band loosened, and the pin comes out.

- 2026-11-17 — **The escalating check moved at PR 6988** (`onehome/least-turn-mint`, merge `034c11e24`; first parent `bf210d351` still stops on `EnvelopeTerm(Radius)`). The boolean's sphere re-cut (`topo::boolean::ops::apply_recuts`) now turns by `RigidMotion::least_turn` (Rodrigues where `a·b ≥ 0`), not `turned_between`'s quaternion, so the Interval re-charted sphere's coefficients and their widths moved. The union still refuses at `HalfEdgeKey(3v1)`, sample 0, `pcurve_envelope`, in the same band (zero 1e-12, escalate 1e-11), but on the aggregate `PcurveCheck::Envelope`, margin `hi` 3.50e-12 → 1.13e-12. Same finding; the pin's text follows it (GREEN's 11-17 PR). PR 6988 ran sweep and topo whole only at the default ε, so it did not see this. Deterministic: two runs at `034c11e24` give byte-identical messages, equal to main's. Reproduce: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run --profile ci --ignore-default-filter -p sweep -E 'test(the_plain_witness_builds_its_seam_crossing_at_the_interval_scalar)'`. "Done when" stands. (GREEN)

- 2026-11-22 — **A second body of the same shape, from EMBED's `embed/collinear-cover` lane.** `crates/sweep/tests/self_intersection_proves_injectivity.rs` builds a three-arc disc with a filleted rim (`filleted_arc_disc`: `prism_at` of `arc_polygon(3, 0.5)`, its top rim filleted at `0.1`) at both scalars, at ε = 1e-12 as at the others. Two refusals at `Interval` and 1e-12, neither at `f64` or at the default ε or 1e-6:
  - On main `b99f71c3d` the fillet itself refuses: `Certify { site: "pcurve re-mint after surgery", half_edge: 20v1, Escalated { check: EnvelopeTerm(Radius), margin: [0, 3.0204e-12], band: { zero: 1e-12 }, predicate: "pcurve_envelope" } }`, this row's check and term.
  - On main `0c1eb97ad` (a day earlier) the fillet built, and so did the body moved a hundred units by a translation alone; the body turned by `topo::transform_rigid` one radian about the axis `(0.48, −0.6, 0.64)` through `(0.3, −0.2, 0.1)` did not finish: `Pcurve { Certify { half_edge: 19v1 and 20v1, Escalated { check: Envelope, margin: [0, 1.2105e-12], predicate: "pcurve_envelope" } } }`.

  Which merge between the two moved the fillet into the band is not bisected. The rows ask the body at `f64` alone wherever its `Interval` twin does not build and print `SKIPPED`; they ask both scalars the day it does.

**Since PR 7131 (GREEN U12, 2026-11-23).** The pinned stop moved, so the pin is red in nightly 44989234943. The row's `pcurve_envelope` escalation now comes from `check: EnvelopeTerm(Radius)` (hi 3.277e-12), where the pin names `check: Envelope`. Cause: bracket. F1X alone builds the union, and A or F1F alone leaves it red. The refusal is counted in ONEHOME's `pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted` (PR 7402's branch until it merges), with probe runs 45083192761 (green at `e4d679d37e`), 45083200534 (red at `9bc9d6517b`) and 45083184695 (mutants). GREEN pins nothing until Ev rules on that count.

**Re-pinned as 7131's cost (GREEN U13, 2026-11-23, PR 7558).** Ev ruled on PR 7505 that PR 7131's frame cost stands. On main `e38e85bebe` and after, the plain witness no longer reaches this row's pcurve envelope at 1e-12: a ∪ b and a ∖ b build, and b ∖ a stops on its seam jet (`SeamJet`, `tangent_second_order`, 1.380e-12). Under mutant F1X **every op builds**, so the only stop left is the frame cost. The row's pin now names that stop and its cause, and F1X turns it red. Whether this row still has a defect of its own to close is for ONEHOME.

**This row has no 1e-12 witness (GREEN U13, 2026-11-23, PR 7558 review).** The plain witness is now pinned as 7131's cost (`PLAIN_FRAME_COST`, b ∖ a's seam jet, (1.3e-12, 1.45e-12]) and no longer as this row's red. No pose was found that reaches this row's pcurve envelope: under mutant F1X every op builds. So either the defect is gone on main, and the row can close, or its fix must bring a witness pose that 7131's cost does not stop and that still reaches the envelope.
