---
id: the-conic-plane-root-solve-has-three-homes
kind: issue
title: the clamped-acos root solve of a conic against a plane is written three times, and each copy decides its graze arm on its own
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## What

Three functions solve where a conic, or a cone's generator family, meets a
plane with the same closed form: a sinusoid `D + R·cos(θ − φ)`, a phase
`φ = atan2`, and roots at `φ ± acos(clamp(−D/R))`. Each one decides on
its own what the tangent arm (`R − |D|` within the band) means:

- `topo::splitting::classify::conic_plane_meet`, the split's and the
  Boolean sweep's crossing lane. Its graze arm (`split_conic_belly_graze`
  `Zero`) took `acos` of the residue's ratio. Inside the conic, that
  placed the one root on a residue crossing √(2ε/R) rad off the
  extremum. Since `cleave/inband-graze` (PR 4179) it places the root at
  `φ` or `φ + π`, on the side `split_conic_graze_side` decides.
- `geom_brep::intersect::plane_cone_section`, apex lane. On
  `pn_apex_section` `Zero`, `ApexTangentLine(gen_at(phi + delta))` still
  takes `acos` of the residue's ratio. Measured: with the plane dipped
  5e-10 in, the line is 2.4e-5 rad off the tangent generator
  (`work/germ/a-plane-cone-apex-tangent-reads-a-residue-generator-off-the-tangency.md`).
- `topo::boolean::rim_wedge::crossings`, `Locus::Rim` arm. It keeps both
  `alpha ± delta` candidates whenever `|ratio| − 1` might not be
  positive, then filters them by `on_locus`. Near tangency both residue
  roots can survive as two cuts on the locus about √(2ε/R) apart. Not
  measured: no pose has been built.

## Proposed home

Put one sinusoid-root door in `geom_brep`, beside `Conic`. It takes `(D,
R, φ)` and the band, and returns `Miss`, `Graze(θ)` with θ = φ or φ + π
from the decided sign of D, `Secant(θ₁, θ₂)`, or the escalation. All three
sites would read its arms rather than solving the ratio themselves. The
crossing lane's arm is the reference, and its rows
(`classify::tests::belly_graze_trio`,
`an_interval_graze_root_is_as_tight_as_its_phase`) can move with it.

## Found by

CLEAVE's review of PR 4179 (MUST 1), from `cleave/inband-graze`'s sweep.
