---
id: a-plane-a-milliradian-off-a-cones-ruling-through-its-apex-reads-its-section-off-the-apex
kind: issue
title: a plane through a cone's apex a milliradian off tangency reads its section's apex pair in band at 1e-9 and off the apex at 1e-12
status: open
opened: 2026-10-28
priority: P2
cost: M
---


## What

A full cone (profile `(0, 0), (tan α, 0), (0, 1)` revolved whole about
`y`, α ∈ {30°, 45°, 60°}), split by a plane through its apex that holds
the ruling at azimuth θ ∈ {0, 0.3, 2} and is turned `t = 10⁻³` off
tangency about it, either normal. The plane is built through the apex
(`test_support::split_plane` at the ruling's foot), so its apex gap is
the rounding of the apex's offset alone. The split's walk reads the
apex ON the plane and its sectors crossed, and asks the section table
for the two section rulings at the apex
(`splitting::insert::apex_germ`, via `chord_join::wall_section` →
`geom_brep::plane_cone_section`'s apex lane). The table's pair reach
(`pn_apex_pair_reach`, `pair_bound` in `geom-brep/src/intersect.rs`)
does not serve the pair:

- ε 1e-9: `pn_apex_pair_reach` escalates, margin `1.1e-9` to `2.2e-9`
  in the band `(1e-9, 1e-8)` (split refuses
  `Reduce(SectionGermsEscalated { reading: Line, .. })`), all 18 poses;
- ε 1e-12: it reads Positive, the table serves the section off the
  apex, and `plane_wall_case` refuses
  `OpenConicChord { conic: Hyperbola }`, which the split carries as
  `Reduce(ApexSection { section: OpenConicChord { .. } })`, all 18 poses.

At ε 1e-6 the pair is served (θ ≠ 0 lands whole, the sliver ≈ t²/2
within the band). Every other tilt of the row
(0.05 … −1.2) is served at every ε and lands at its closed form.

Measured on `splitdoor/cone-apex` (the PR that reads a cone's apex as
its rulings): an instrumented `apex_germ` printed the table's error,
`OpenConicChord { face, conic: Hyperbola }`, at every refused pose at
1e-12. Pinned as typed refusals in
`crates/sweep/tests/a_plane_through_a_cones_apex.rs`
(`near_tangent`).

## Where to look

`pair_bound`'s bound over the reach against a plane whose gap is the
apex offset's rounding: at `t = 10⁻³` the discriminant `D` is about
`sin α·t`, so `|δ|/D` and the reach bound amplify the rounding by 10³.
Whether the split should instead hand the table the plane as through
the apex (its walk decided the apex ON) is the other question.
