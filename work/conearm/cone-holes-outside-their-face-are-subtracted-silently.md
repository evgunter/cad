---
id: cone-holes-outside-their-face-are-subtracted-silently
kind: issue
title: A cone face's hole outside the loops that bound it is subtracted silently, not refused
status: open
opened: 2026-10-19
priority: P2
cost: M
refs: [4822, a-cone-face-whose-rings-leave-or-overlap-its-region-measures-silently, validate-tier3-curved-boundary-containment]
---

Found in review of PR 5333 (`flux/cone-closure-and-per-term-whole`;
reviewer probes on `review/5333-01A5XHA3`,
`crates/geom-brep/tests/review_5333_probes.rs`).

`require_cone_holes` (`crates/geom-brep/src/props/curved.rs:2374`)
decides each hole wound against the loops that bound the face
(`require_holes_against`, `curved.rs:1480`), and the holes together
inside them only in sum: the whole's side (`props_cone_area_side`) must
be theirs, or `HolesExceedFace`. A hole that is wound correctly but
lies wholly or partly outside the face, and is smaller than it, passes
both, and its area is subtracted from a region it is not in. The
probes, on the cone of half-angle ½ at the origin, all measuring at
`f64` and at the interval scalar, at the flux lane and the material sign
alike:

- **P09**: the band `[1, 2]` with a window `[3, 3.3] × [1, 2]` wound
  against it measures `4.0654`, the band (`4.5185`) less the window;
- **P47**: the cap under the rim at slant 2 with a window `[3, 3.5] ×
  [1, 2]` outside it measures `5.2456`, the cap (`6.0246`) less the
  window;
- **P49**: the window `[1, 2] × [1, 2]` with a window `[1.2, 1.4] × [3,
  3.5]` outside it measures `0.6568`, the window (`0.7191`) less the
  other.

The class is not the cone's alone and predates PR 5333: the cylinder
(`cylinder_holes`, `curved.rs:1425`) and the cap and window cases above
were read this way before it; PR 5333 extended it to bands, which used
to refuse. Each lane's doc says so ("nor is a hole decided to lie inside
the loops that bound the face, beyond the sum of the holes").

The repair is a containment decide for each hole against the loops that
bound the face, on the chart, refusing a hole outside them by name. At
the time this was filed tier 3 did not catch it either: no tier-3 check
read a curved face's holes against its region.

**Tier 3 now does** (RESTCURVE's region arm, PR 6163,
`crates/topo/src/one_region.rs`): it refuses a curved face's hole
outside its face (`CycleOffFaceSide`). P09, P47 and P49 are its rows
(`crates/sweep/tests/one_region_rows.rs`). So a valid body never carries
this shape; what remains here is the lane measuring an unvalidated body
silently, and its guards retire onto the arm
(`work/conearm/props-cone-region-guards-retire-onto-the-region-arm.md`).

## Owner (Ev's decision on PR 4822)

Ev decided this class on PR 4822, the design fork
`a-cone-face-whose-rings-leave-or-overlap-its-region-measures-silently`:
tier 3's check 9 owns "a face's loops bound one region" on every surface
kind, through a chart-polygon arm, and the closed-form lanes read their
loops as such a boundary rather than half-deciding it. So the fix here
is not a lane decide: it is RESTFRONT's
`validate-tier3-curved-boundary-containment` arm, whose rows should
cover these three probes (bands, caps and windows: P09, P47, P49). Once
that arm lands, the cone's own half-decisions of region (`HolesExceedFace`
and the hole winding) are FLUX's to retire, per that decision.

