---
id: the-cone-has-no-implicit-enclosure-or-metres-conversion
kind: issue
title: The cone has no implicit enclosure and no metres conversion, so every rung-3 pair with a cone (cone × cone, cone × sphere, cone × torus, NURBS × cone) refuses
status: closed
opened: 2026-11-03
priority: P0
cost: H
refs: [cone-torus-rung-three-arm-waits-on-the-cone, a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm, cone-germ-pairs-against-a-curved-face-have-no-section-frame]
closed: 2026-11-22
---


Filed 2026-11-03 from the curved-boolean bookkeeping pass
(`work/splice/curved-demo-joins-build.md`). Several rows name this gap as
"the cone's own unit" or "SECTRUNG's cone unit", and no row owned it.

## What

The ℝ³ certificate's analytic halves have no cone arm:

- `crates/geom-brep/src/ssi/enclose.rs` `implicit_enclosure` (line 388)
  and `implicit_gradient_enclosure` (line 438) answer
  `Interval::refused()` for `Surface::Cone`;
- `crates/geom-brep/src/ssi/certify.rs`, module docs: the composite's
  units convert to metres exactly for the plane, sphere and cylinder, and
  per span for the torus (`ToMeters::Torus`); "the cone has no
  conversion and refuses".

So `crates/geom-brep/src/intersect.rs` `route` answers
`implemented: false` for `(Cone, Cone)`, `(Cone, Sphere)` and
`(Cone, Torus)`, each note naming the cone's metres conversion.

## What waits on it

- `cone-torus-rung-three-arm-waits-on-the-cone` (P0, lily wall 2, the
  flower welded onto the stem), parked on this row.
- `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm` (P0): Ev's
  ruling on PR 6121 builds "the cone after SECTRUNG's cone unit", and
  klein wall 3's partner is the flare cone.
- `cone-germ-pairs-against-a-curved-face-have-no-section-frame` (P1):
  its classification does not wait on this, and its march does.
- Cone × sphere in general pose. No row names it; the pair is in the
  same `route` arm.

## Open

The torus's conversion reads its residual through the quartic's own
factor (`F/(2r·(A + 2Rρ))`). The cone's implicit form carries `tan² α`,
and the rows above call its exactness the open question: whether the
half-angle enters as an exact rational, its tangent, or an enclosure,
and what that does to the "exactly" the other kinds' conversions claim.
That is why `design` is set. Weigh it with the designer pair before a
lane builds it.

## 2026-11-20 — a cone on a torus's meridian stops at the interior-loop guard (SECT)

Measured on `sect/closed-edge-on-torus` (main `d44f46d3`), at the
default ε, 1e-6 and 1e-12, every op in both orders: the torus `R = 2,
r = 1` about `y`, against the triangle `(1,0), (1.5,0.5), (0.5,0.5)`
revolved about the line through `(2, 0, 0)` along `z`. Its vertex
sweeps the torus's meridian circle in the `xy` plane, and its two cones
cross the tube there. The join builds. `ops.rs` `interior_loop_verdict`
then refuses `CurvedPairUnsupported { site: InteriorLoopGuard }` on the
torus × cone pair: a cone × torus pair in general pose, this row's.

## Weighed (2026-11-21, two designers)

The premise was wrong. The cone already has a metres conversion: the edge certificate's limb-2 reader (`pcurve_cache::limb2_offset_sup`, `projected::Meters::Cone`) converts the canonical composite `cos²α·ρ² − sin²α·z²` by a lever, as C2 describes. Only the search certificate's second limb-2 pipeline (`ssi::certify` `composite_form` → `geom_core` `ImplicitSurface` → `ToMeters`) never learned the cone. "Exact" is not owed: limb 2 owes a certified upper bound on distance, which is what the lever gives. The cone's limb-1 residual is in metres (`cone_elevation`), so `pair_section::traced_in_r3`'s cone exclusion rests on a false note.

Both designers agree on the final state:

1. **One limb-2 reader.** The search reads limbs 1–2 through the edge reader. That reader reports its uncleared parts for `ssi/refine.rs`. `ToMeters`, `composite_form`, `ImplicitSurface` and the torus's search-side conversion retire.
2. **The tube's cone arm.** `implicit_enclosure` / `implicit_gradient_enclosure` read limb 1's own function, the double cone's elevation `g = ρ cos α − |h| sin α`, whose gradient `ŵ cos α − sgn(h)·â sin α` is unit. It is enclosed by the mean-value form met with the direct form, and gated on `h` and `|w|` excluding zero, which is the analogue of the torus's `A > 0`. A box at the apex refuses, and C5's node clause owns the point. Exclusion reads the quadric's sign where the gate fails.
3. **What the cone stores: its slope `k = tan α`** (Ev, `[ev]` PR 7326, 2026-11-22). That is the unit `the-cone-stores-its-slope`. Items 1 and 2 did not wait on it. They cross `sin α`, `cos α` on the lane at `Interval`, as `curve_face/composite.rs` does.

## Closed (2026-11-22)

Both units of the weighed design landed:

- `the-tube-enclosure-has-a-cone-arm` (PR 7358);
- `the-search-certificate-reads-limb-two-through-the-edge-reader` (PR 7350).

The cone now has its implicit enclosure in the tube and its metres conversion at the search. What it stores (its slope) is `the-cone-stores-its-slope`. The rung-3 pair doors are `cone-torus-rung-three-arm-waits-on-the-cone` and `cone-sphere-and-cone-cone-in-general-pose-have-no-rung-three-door`, both now open.
