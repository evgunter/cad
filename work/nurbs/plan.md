# NURBS — the plan

the spline doors: knot insertion, span metring, the loose net and what restrict composes

Opened 2026-09-20 by PROPS's priority-seam cut
(`work/README.md`, Track size). Picked up 2026-10-09.

## The slate

**32.5 budget points** of dispatchable work against a ceiling of 30;
the first wave takes 12 of them off, so the track is worked down rather
than split.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one` | E | a reversed domain refuses as a collapsed span |
| P0 | `refine-dir-hairline-knot-insertion` | H | the exact-equality insertion guard leaves knot pairs one ulp apart |
| P1 | `coefficient-vector-pairing-survivors` | H | the loose (knot vector, coefficient array) shape outside hull |
| P1 | `knot-mirror-symmetry-belongs-on-knotvector` | E | `mirror_symmetric` sits on `NurbsSurface` |
| P1 | `mapped-curve-restrict-composes-placements-per-split` | H | restrict pays the rotation's enclosure once per split |
| P3 | `span-locator-lands-a-nan-parameter-on-the-first-span` | H, design | the locator tie-breaks NaN onto the first span |
| P3 | `a-swaying-loft-corner-refuses-as-a-vanishing-span` | M | the span meter's chord bound collapses on a sound curve |
| P3 | `an-exact-pcurve-image-certifies-worse-than-an-interpolated-one` | M | an exact pcurve image certifies worse |
| P3 | `certified-blossom-primitive-in-geom-core-spline` | M | one certified blossom primitive |
| P4 | `degree-elevation-recomposition-error-grows-as-one-over-the-closest-knot-gap` | M | elevation's removal chains are conditioned by the knot gap |

Parked: `parametric-polygon-loop-certifies-nothing` waits on the D10
hold (`d10-one-way-to-say-intent-is-unbuilt`). It measures certification
through `Expr` document parameters and the symbolic tier, both of which
D10 rewrites. No other row stands on the hold's ground.

## Order

Wave 1, dispatched together:

- `refine-dir-hairline-knot-insertion`: flip the three `GridSkip::BitEqual`
  sites to `WithinUlps(SLIVER_CLEARANCE_ULPS)` and re-baseline. This is
  a live numeric failure.
- `nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one`
  and `knot-mirror-symmetry-belongs-on-knotvector`: one lane, two PRs.
  The mirror predicate moves now, without waiting for a second caller.
  It is a property of a `KnotVector` alone, and that placement is right
  whatever calls it next.
- `mapped-curve-restrict-composes-placements-per-split`: compose in the
  parameter and keep one placement, as the row states.
- `span-locator-lands-a-nan-parameter-on-the-first-span`: a designer
  pair weighs it before any lane builds.

Wave 2: `coefficient-vector-pairing-survivors`, then the P3 rows. The
loft row comes after the span-meter split, since it is the same arm.
`certified-blossom-primitive-in-geom-core-spline` waits until FLUX
answers `the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column`.
That row asks whether the certified Boehm step should be convex or lerp,
which is the form the primitive would fix.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`). The A/B
protocol PROPS inherited is suspended, so the triage question this plan
used to leave open no longer arises. Each unit's tier and reason are
recorded in the log at dispatch.
