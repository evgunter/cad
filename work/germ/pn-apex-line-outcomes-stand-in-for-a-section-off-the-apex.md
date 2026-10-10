---
id: pn-apex-line-outcomes-stand-in-for-a-section-off-the-apex
kind: issue
title: plane×cone serves the apex line pair or tangent generator on a needle cone whose real section starts an extent-scale distance from the apex
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [pn-apex-point-snap-discards-an-extent-scale-section, a-plane-cone-apex-tangent-reads-a-residue-generator-off-the-tangency]
---


## What

Found by the sweep on `pn-apex-point-snap-discards-an-extent-scale-section`.
`plane_cone_section`'s apex lane (`crates/geom-brep/src/intersect.rs`)
serves `ApexLinePair` and `ApexTangentLine` on `decide_across` over
`D·extent` with the apex gap `δ` as the swing. Neither bounds where the
real plane's section lies. On every generator the plane stands
`|δ|/|g·n|` from the apex, so the section's nearest point is
`|δ|/(sin α·s + cos α·|c|)` from it. On a needle cone that is a quotient
by `≈ sin α`, which `pn_aperture_sin` only holds at `10·ε/extent`.

Measured with the plane `0.9·ε` off the apex at unit extent and
`sin α = 12·ε`, at ε 1e-6, 1e-9 and 1e-12 alike:

- **Plane along the axis.** `D·extent = 12·ε`, served `ApexLinePair`.
  In closed form the section is `{x = δ, y² + δ² = z²·tan²α}`. It has no
  point below `z = δ/tan α = 0.075 m`, yet both served lines run through
  the apex. That is a 0.075 m Hausdorff miss.
- **Tangent pose.** `D = 0`, served `ApexTangentLine`. The nearest
  section point is 0.0375 m from the apex.

At `α = π/6` the nearest point is about `ε` from the apex, but the
asymptotes lie `|δ|·sin α/√(D·(cos α·|c| + sin α·s))` off the served
generators. At `D·extent = 11·ε` that is about `0.15·√ε` m. It is the
same √ε near-tangency amplification as `plane_cylinder_section`'s
`ParallelLines` beside a gap just past the band. Whether a served line
within ε of both surfaces owes more than that is a question for this
row too.

Consumers: `topo::offset_derive` mints the pair as curves.
`topo::chord_join` reads it as `SectionCase::Straight`, and the tangent
generator as a chord carrier its split lane certifies.
`topo::boolean::join`'s frame reads the pair as `Ok(None)`.

## The shape of a fix

Bound the section's near vertex `|δ|/(sin α·s + cos α·|c|)` in the band
before serving either line outcome, as `pn_apex_point_reach` bounds the
ellipse's far point, and route past it to the off-apex lane. There the
hyperbola refuses typed, naming its conic (R1), and a near-parabola
escalates. Decide, with a derivation, whether the √ε asymptote offset
also has to be inside the band.
