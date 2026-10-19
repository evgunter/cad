---
id: decided-tangent-point-is-the-radical-foot
kind: issue
title: decided-tangent contact points taken at the radical-line foot sit off both circles by gap*(r1+r2)/d, not the gap
status: open
opened: 2026-10-07
priority: P3
cost: E
---


Found by the fillet-radius P0 (`profile-fillet-radius-off-at-eps-1e-6`),
which fixed the one instance that stores geometry: the fillet centre on a
decided offset tangency (`crates/profile/src/sugar.rs`,
`ArcCarrier::offset_circles`).

## The shape

Two circles (or spheres) decided tangent inside the band, `|gap| < ε`, and
the one contact point taken at the radical-line foot `c₁ + û·a`,
`a = (d² + r₁² − r₂²)/2d`. When the circles miss tangency by `gap`, that
foot sits `gap·r₂/d` off circle 1 and `gap·r₁/d` off circle 2: together
`gap·(r₁ + r₂)/d`. On internal tangency `d ≈ |r₁ − r₂|`, so near-equal
radii amplify the gap by far more than its own size: by the formula 16x
on the `1/0.9/0.01` lens of the fillet row
`near_half_turn_and_extreme_sweep_fillets_meet_the_oracle_at_both_scalars`,
and up to 4.1x measured over the fillet fuzz's natural draws at ε = 1e-4,
both before the fix. The point midway between the circles'
nearest points on the link carries `gap/2` to each. Where the circles are
separated that is the least any point can carry; where they overlap by
`gap` they cross at two exact points, but those sit about `√(gap·r)` off
the link and are ill-conditioned in the gap, so a decided tangency's one
point is still taken on the link.

## Hits outside the fillet

Grepped for `powi(2) - … .powi(2)) / (` and the `d * d + r1 * r1 - r2 * r2`
spelling across `crates/`; the src hits that take the foot as a decided
tangent's one point, each its owner's to weigh:

- `crates/profile/src/seg.rs`, the arc/arc contact classifier's internal
  `carrier_circles_internal` Zero arm (PATHS). Filed on its own as
  `an-internal-tangency-reads-its-spans-at-the-radical-foot-off-both-arcs`
  (P2): near-concentric carriers put the foot centimetres off both
  arcs, and the chordal `arc_span` reading of that point refuses
  `TangentialContact` between arcs that are 1–3 cm apart, so the
  radial place does decide a verdict there. Its `a` now comes from the
  clearances (`geom_core::Circles2::foot`), which takes out rounding of about
  `u·r²/d` but keeps the point at the foot, so the amplification here
  is unchanged. The external arm takes
  `c₁ + û·r₁` and is already gap-optimal. **This point is read as a
  location**: `crates/profile/src/validate.rs` `judge_pair` discounts a
  `Touch` only where `seg::coincident("contact_at_shared_vertex",
  contact.point, v)` is Zero. Measured with a probe (two arcs meeting at
  a declared joint `v`, their carriers through `v` and decided
  internally tangent at a margin of 0.5ε, at ε = 1e-9 and 1e-6, carrier
  radii 1 against 0.99, 0.999 and 0.5): where the arcs meet end to start
  the contact falls outside one arc's span and the loop validates; where
  they form a cusp at `v` and the carriers' second crossing falls on
  both arcs, the contact lies on both and the loop refuses
  `TangentialContact`. That refusal is the contact's ANGLE, not its
  radial place: two carriers through `v` decided tangent at margin `m`
  really cross again about `r·√(2m/Δr)` from `v`, the foot sits midway,
  and its radial error `m·r/Δr` is smaller than that by `√(m/2Δr)`. So
  the radial amplification this issue is about changes this consumer's
  verdict only where `Δr` is within a few ε of the gap, and a
  gap-optimal point would not have validated the probe's cusp.
- `crates/geom-brep/src/intersect.rs`, `sphere_sphere_section`'s
  `TangentPoint` on both Zero arms (GERM/REACH/TANG). Its one in-tree
  consumer, `crates/topo/src/boolean/join.rs` `pair_section_frame`,
  matches `TangentPoint(_)` and discards the point (a typed desync), so
  nothing reads its place today; only `geom-brep`'s own
  `intersect_table` test reads it.
- `crates/topo/src/boolean/carrier_cross.rs`, the circle crossing's
  `apart`/`nested` Zero return of `foot`: retired with
  `carrier_cross::meetings` (stage (c) of
  `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)).
- `crates/topo/src/validate.rs`, the circle-circle edge-pair candidates'
  tangent arm (RESTFRONT). Its `a` now comes from the clearances
  (`geom_core::Circles::radical`), as seg.rs's does; the point is
  still the foot.
- `crates/topo/src/boolean/join.rs` `parallel_radical_plane`, the same
  formula for two parallel cylinders' radical plane. One reviewer
  judged it crossing-only, because the frame dispatch refuses tangent
  walls before it is asked; not re-verified here.

- `crates/topo/src/boolean/section_cert.rs` `sphere_sphere`, the
  `tan.touch` closure on a `REACH` or `NEST` escalation: its touch is
  `c₁ + k·(dd² + r₁² − r₂²)/2dd` (CLEAVE/HONE/ORBIT). Found by FUSE's
  sweep for the curve-pair reader (stage (b) of
  `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)). The reader itself places a
  touch at the residual's extreme, on one circle exactly
  (`geom_brep::curve_pair::pose`, `harmonic_roots`), which
  `carrier_cross` now reads.

All of them put the point on the link line, so its ANGLE about either
centre is the link's; a reader that only span-checks that angle sees
nothing. What moves is its radial place, which matters wherever the
point is used as a location. The location reader found, above, is
dominated by the angle; the span reading of the same point is not, and
has its own item. That keeps this row at P3 for the other hits.

The same formula also rounds badly at radii orders of magnitude apart,
which is a separate defect from the foot's place: fixed in `seg.rs`'s
secant and internal-tangent arms by
`arc-arc-secant-arm-misses-crossings-at-unequal-radii`, and filed for
the other sites as `circle-pair-edge-candidates-cancel-at-unequal-radii`
(RESTFRONT), `circle-crossing-closed-forms-cancel-at-unequal-radii`
(HONE) and `sphere-sphere-section-radius-cancels-at-unequal-radii`
(ROOTS).

## The arc/arc pair pass's instance, closed (PATHS, 2026-10-15)

`an-internal-tangency-reads-its-spans-at-the-radical-foot-off-both-arcs`
closes the `seg.rs` hit: a tangency's candidate in either arm of
`arc_arc` is the midpoint of the circles' nearest points on the line of
centres, `|g|/2` off each, which both spans and the shared-vertex
discount (`validate.rs` `judge_pair`) read. Where the internal
clearance is within ε but the radical foot stands deeper than the band
inside both circles (`carrier_circles_chord`), the pair is two
crossings at the chord's ends, as for a proper secant: one tangency
there stood for crossings far apart, and a cusp at one of them
discounted the other at its joint.

Its sweep grepped `\.foot()`, `\.radical()`, `radical_foot(`,
`chord_ends(` and the `(d² + r₁² − r₂²)/2d` spellings
(`powi(2) + … .powi(2) - … .powi(2)) / (`, `d * d + r1 * r1 - r2 * r2`)
under `crates/`, src only, and read every hit for whether a span or a
trim is decided at the point:

- `crates/topo/src/validate.rs` `segments_meet` circle × circle
  (`placed.foot()`, `chord_ends`): **reads both trims at the foot**
  (`window` → `containment::arc_trim`, chordal). Same defect, a false
  `Meet`; filed RESTFRONT
  `check-9-reads-an-internal-tangencys-trims-at-the-radical-foot` (P3),
  with the margin measured and the silent-acceptance direction argued
  closed by arms 1–3.
- `crates/topo/src/boolean/carrier_cross.rs` `parallel_circles`: the
  foot is a candidate that `boundary_crossing` projects onto the swept
  carrier by parameter (`param_near`) before reading the span
  (`strictly_inside`), so only its angle is read. Not this defect.
- `crates/topo/src/boolean/section_cert.rs` `sphere_sphere`: the
  `tan.touch` place; no span or trim read. Stays a hit of this row.
  Its secant arm and `torus_sphere`'s `along` place crossings, not a
  decided tangency.
- `crates/geom-brep/src/intersect.rs` `sphere_sphere_section`: as
  above; no span read.
- `crates/topo/src/boolean/join.rs` `parallel_radical_plane`: a plane,
  no span. Its two other hits are test oracles.
- `crates/sweep/src/blend/arms.rs` (round × round rail point) places a
  crossing with its half-chord, not a decided tangency; not a hit.
- `crates/profile/src/sugar.rs` `offset_circles`: already midway.
- `crates/profile/src/seg.rs` `line_arc`'s tangent arm reads the arc's
  span at the line's foot, which is on the line and `|g| ≤ ε` off the
  circle: no amplification, left as is.

What the grep could not match: a foot spelled through another helper
or built by hand from `a` (`c₁ + û·a` with `a` named otherwise), and
trims read at a point the caller computed elsewhere. A second pass
grepped the trim and span readers (`arc_trim(`, `arc_span(`,
`window(`, `strictly_inside(`) for their call sites and read what
point each is handed; none beyond the above is handed a tangency foot.

## The class's open sweep (from cleave's conditioning P1)

`work/cleave/a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles.md` asked for every closed-form arm to be swept for a generic formula evaluated at data a `Zero` reading has just called degenerate. Its `carrier_cross` arm is retired. Two arms of the shape remain, and this row holds them:
- `splitting::conic_plane_candidates` (`crates/topo/src/splitting/classify.rs`): its graze row's `Zero` places the double root by the generic root formula.
- `rim_wedge::crossings` (`crates/topo/src/boolean/rim_wedge.rs`): its line and circle closed forms are lenient in the band, and place a touch the same way.
Not measured on a body. Both were found by reading, in the review of PR 5154.
