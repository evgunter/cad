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
passes. How the rigid-motion change reaches this witness's operands
(placement of the spheres, or the frames their edges are certified in)
is unmeasured; the escalating check is PCERT's `pcurve_envelope`
(`geom_brep::pcurve_cache`).

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all \
      -E 'test(the_plain_witness_builds_its_seam_crossing_at_the_interval_scalar)'

## Done when

The witness's Interval union builds at 1e-12, its envelope decided
rather than the band loosened, and the pin comes out.
