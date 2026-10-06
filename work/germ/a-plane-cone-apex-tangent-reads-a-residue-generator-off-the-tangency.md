---
id: a-plane-cone-apex-tangent-reads-a-residue-generator-off-the-tangency
kind: issue
title: a plane through a cone's apex, tangent within the band from inside, returns one of the residue's two generators as its tangent line
status: open
opened: 2026-10-06
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
