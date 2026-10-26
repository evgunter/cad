---
id: a-plane-cone-apex-tangent-reads-a-residue-generator-off-the-tangency
kind: issue
title: a plane through a cone's apex, tangent within the band from inside, returns one of the residue's two generators as its tangent line
status: closed
branch: germ/pn-apex-tangent-generator
pr: 5152
opened: 2026-10-06
closed: 2026-10-14
priority: P3
cost: E
---


## What

`geom_brep::intersect::plane_cone_section`'s apex lane decides
`pn_apex_section` (the conic-type discriminant, levered at the extent)
and, on `Zero`, returns `ApexTangentLine(gen_at(phi + delta))` with
`delta = acos(clamp(−cos α·c / (sin α·s)))`. When the plane dips into
the cone by a sub-ε angle the ratio sits just under 1, and `delta` is
not 0 but √(2(1 − ratio)): the line returned is one of the two
generators the residue solves for, which lie in the plane, not the
generator the plane is tangent along. When the plane tilts out by the
same angle the clamp makes `delta` exactly 0 and the line is the
tangent generator.

Measured on `cleave/inband-graze` with an ignored probe beside
`intersect_table.rs::plane_cone_apex_trio` (cone α = π/6 at apex
(0, 0, 1), plane through the apex with normal (cos(α + τ), 0,
−sin(α + τ)), extent 1, ε = 1e-9):

| τ | returned | angle from the tangent generator |
|---|---|---|
| −5e-10 | `ApexTangentLine` | 2.4e-5 |
| −1e-10 | `ApexTangentLine` | 1.1e-5 |
| 0 | `ApexTangentLine` | 0 |
| +1e-10, +5e-10 | `ApexTangentLine` | 0 |

At a lever of 1 the inside poses' line is ~2e-5 m off the cone's
tangency at unit distance from the apex, four orders past ε. The
consumer is `chord_join`'s `SectionCase::Tangent`.

## Where to look

The same shape was fixed in the split's conic root lane
(`splitting/classify.rs`, `conic_plane_meet`, branch
`cleave/inband-graze`): a graze's one root is the extremum, so on the
`Zero` arm the ratio is `−sign` of the numerator, not the residue's
quotient. Here that is `gen_at(phi)` or `gen_at(phi + π)` by the sign
of `c`. No end-to-end pose through a split or a Boolean has been
measured.

## Found by

`cleave/inband-graze`'s sweep for a pair of in-band roots read as two
crossings.

The same solve exists in two other places. The proposal for one home is
`work/honeone/the-conic-plane-root-solve-has-three-homes.md`.

## Fixed

`plane_cone_section`'s `Zero` arm (`crates/geom-brep/src/intersect.rs`,
the apex lane after `pn_apex_tangent_reach`) now returns the extremum
generator, `gen_at(φ + π/2·sign(c) + π/2)`: `φ` where `c = axis·normal < 0`,
`φ + π` where `c > 0`, by `Real::copysign`, a value-level selection, so
no new trilean. `c` is definite on that arm: a Zero `D` puts `|c|` at
`tan α·s`, and `pn_aperture_sin` has decided `sin α` at the same
lever. The line pair keeps the clamped `acos`.

The table above is reproduced and pinned in
`geom-brep/tests/pn_apex_tangent_generator.rs`
(`a_grazing_plane_serves_the_generator_it_grazes`): τ ∈ {−0.5, −0.1, 0,
0.1, 0.5}·ε, upright, turned and far (apex at (400, −650, 300)) frames
with the seam turned off the tangency, both signs of `c`, at `f64` and
`Interval`. Before the fix it measured 2.40e-5 and 1.07e-5 rad at ε =
1e-9 (as above), 7.6e-4 and 3.4e-4 at 1e-6, and 7.6e-7 and 3.4e-7 at
1e-12, in every frame and for either sign of `c`. After the fix every row
is within 1e-12. `pn_apex_reach.rs::apex_outcomes_agree_with_the_oracle`
was blind to this, because its tangent-line check asked only for a
generator in the plane, which a residue generator is. It now also checks
the line against the grazed generator, in closed form (`Pose::grazed`),
to `1e-12 + 64u/s`.

## End to end: not reachable today

The line's direction is consumed in one place, the split's tangent chord
mint (`topo/src/chord_join.rs`, `SectionCase::Tangent` in the planar-face
chord lane, `split_tangent_chord_forward`), and only where the section
has two vertices on the grazed ruling. The other consumers read only the
classification. The Boolean join refuses a tangent germ pair typed
(`chord_join.rs`, "tangent plane×wall germ pair in the boolean join").
`boolean/join.rs` (`pk_germ_frame`) and `offset_derive.rs` refuse or
route on the variant. `splitting/join.rs` pairs nothing on a tangent
section. Two families of split poses were probed (not committed) on this
branch, against main and against the fix:

- **A real apex**: a revolved cylinder with a conical pit (α = π/6, apex
  vertex), split by the item's plane family through the apex, at four
  spins. Every pose refuses `Reduce(SliverSector)` on `sector_straight`
  at the apex vertex, in the reduction, before any chord is minted. That
  is `work/cleave/a-convex-graze-of-a-cone-refuses-at-some-azimuths.md`.
- **A virtual apex**: a frustum (1 → ½) and a flared frustum (½ → 1),
  split by the same family through the virtual apex at five azimuths off
  the seam, τ ∈ {−30, −10, −3, −0.5, −0.1, 0, 0.1, 0.5}·ε, both normals.
  All 160 outcomes are identical on main and on the fix. At τ ∈ {−0.1,
  0, 0.1}·ε the body lands whole on one side at its closed-form volume.
  At ±0.5ε and −3ε it refuses `CrossingEscalated(BellyGraze)` on a rim the
  plane grazes in band. At −10ε and −30ε, past the band, it cuts a tier-3
  sliver of about 4e-12 and 2e-11. Off the seam the grazed ruling carries
  no section vertex, so no tangent chord is minted.

A pose that reaches the chord mint needs section vertices on the grazed
ruling: either the real apex, which refuses first, or a seam vertex,
whose frustum grazes refuse in the reduction (the cleave item's frustum
paragraph). Neither is reachable until those land.

