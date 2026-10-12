---
id: no-door-rejects-geometry-past-eps-f64-resolution
kind: issue
title: D4 ¶4's size range is unenforced: past 2^53·ε a certification residual reads rounding, and the f64 lane's Zero is luck
status: open
opened: 2026-10-12
---


(Filed by TANG's `tang/far-origin-rod-mapped-source` lane, from the
root cause of
`work/tang/a-far-origin-rod-cut-escalates-its-mapped-source-at-eps-1e-12.md`.)

## What

D4 ¶4 promises "a documented model size range; geometry outside the
range is rejected at construction". No door implements it, and no
range is named next to ε. f64 spaces coordinates at `ulp(x) ≈
|x|·2⁻⁵²`, so ε names a point only within `|x| ≲ 2⁵³·ε`: about 9 km at
ε = 1e-12, 9000 km at 1e-9. Past that range, every point-to-point
certification residual (`check_residual` in `geom-brep`'s
`certify.rs`, `pcurve_map_residual` in `pcurve_cache.rs`) reads the
rounding of its two evaluations, not a miss. The f64 lane then
certifies Zero wherever the two roundings happen to agree, and it
escalates wherever they don't. The Interval lane straddles at every
sample. Nothing tells the user that the body sits outside the range
where ε means anything.

A stored PARAMETER carries the same floor at its lever. A split
extrude strut stores its sub-range as `u ∈ [0, 1]` along the whole
extrusion vector (`geom-brep` `mapped.rs`, `SubRange`), so it resolves
a cut only to `|vec|·2⁻⁵³` along the strut. TANG's far-origin rod
measured this: a rod extruded over z ∈ ±1e5 and cut at z = 0.5 stores
`span = fl(0.5000025)`. In exact rationals its description misses the
carrier by 1.228e-12 at sample 3 and by 3.28e-12 at s = 1. Its f64
evaluation adds one ulp at |z| ≈ 6.2e4, which is 7.28e-12. Both
readings sit in the 1e-12 band, so `carrier_matches_mapped_source`
escalates. That verdict is correct; the row now skips that pose where
`far·f64::EPSILON` exceeds the zero band.

## Repair shape (a design question)

Name the range once beside `Tol` (geom-core, `tolerance.rs`) as a
function of ε, and decide where it is enforced: at construction, as
D4 ¶4 says, or as a typed refusal naming the range wherever a residual's
own f64 floor exceeds the band. A parameter-levered floor, like the
strut's `|vec|·2⁻⁵³`, is a second question: either the range bounds
`|vec|` too, or a split description stores its cut against the nearer
end. Weigh this with designers first; it touches D4 ¶4's wording.
`work/encl/no-tighten-offer-quotes-a-value-below-the-runs-resolution.md`
needs the same floor from the other side, a resolution below which no
ε decides anything, and the two should share one definition.
