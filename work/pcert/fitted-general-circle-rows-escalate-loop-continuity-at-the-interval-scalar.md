---
id: fitted-general-circle-rows-escalate-loop-continuity-at-the-interval-scalar
kind: issue
title: A sphere face's fitted general-circle rows escalate at the Interval scalar at eps 1e-12 (pcurve_loop_continuity, since its retirement pcurve_map_residual), so a tilted sphere pair builds in f64 and refuses there
status: open
opened: 2026-10-02
priority: P2
cost: M
---


## Measured

Found by the `reach/tilted-sphere-pair` lane. Two unit balls (both
`y`-poled) at `(2, 2, 0.5)` and `(3.4, 2, 0.5)`, built at the
`Interval` scalar, under ∪, ∩ and ∖: at ε 1e-9 and 1e-6 every op
builds, tier 3 clean, its volume bracket around the lens closed form;
at ε 1e-12 every op refuses at the pcurve mint:

```
Pcurves { source: Escalated { half_edge: HalfEdgeKey(18v1), cause:
  Indeterminate { margin: Enclosure { lo: -9.52e-11, hi: 9.52e-11 },
  band: (1e-12, 1e-11), predicate: Some("pcurve_loop_continuity") } } }
```

(∩: ±3.9e-11; ∖: ±3.4e-11). The f64 twins build at all three ε. The
faces carry arcs of the radical-plane circle, tilted against the
sphere's chart, whose rows the mint routes through the fitted lane
(PR 3733's general-circle route); the joint gap's enclosure at a
fitted row's end is ~10× the escalate band at 1e-12. Pinned per-ε by
`crates/sweep/tests/tilted_sphere_pair.rs`,
`a_tilted_sphere_pair_builds_at_the_interval_scalar`, which flips when
this lands.

## What a fix owes

A joint-gap enclosure on a fitted general-circle row that is tight
enough to decide at the 1e-12 band at the interval scalar (the fitted
image evaluated at its end parameter through the carrier's exact end
point, say, rather than through the fit), or the reason it cannot be,
stated at `pcurve_loop_continuity`.

## Moved: the joint margin retired, the row's own residual is next (2026-10-03)

The loop walk no longer decides a joint's chart gap on an analytic
chart (`pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`:
a joint states its deck element, and its 3-D coincidence follows from
the rows' envelopes and the endpoint pinning). The tilted pair still
refuses at 1e-12 at the `Interval` scalar, one check earlier on the
same row: the fitted row's own certificate schedule,

```
Pcurves { source: Certify { half_edge: HalfEdgeKey(18v1), error:
  Escalated { check: MapResidual, sample: 0, cause: Indeterminate {
  margin: Enclosure { lo: 0.0, hi: 2.89e-11 }, band: (1e-12, 1e-11),
  predicate: Some("pcurve_map_residual") } } } }
```

(Union; the test pins the predicate for all three ops). So what a fix
owes is now the fitted image's residual at its end sample: the image
evaluated at an end parameter is the fit's, not the carrier's exact end
point, and its enclosure is about 3× the escalate band at 1e-12.
`crates/sweep/tests/tilted_sphere_pair.rs` pins the new predicate.
