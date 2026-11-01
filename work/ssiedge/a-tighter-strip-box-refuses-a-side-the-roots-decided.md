---
id: a-tighter-strip-box-refuses-a-side-the-roots-decided
kind: issue
title: A tighter strip box refuses a side as tangent that a looser one left to the side's roots
status: open
opened: 2026-10-13
priority: P3
cost: M
design: true
---

## What

`crates/geom-brep/src/ssi/boundary.rs` `side_region` classifies a side
on the first rung whose strip has the slope across the side
one-signed, and refuses with `SsiError::BoundaryTangent` when that
slope's margin lies inside the band. A rung whose slope is NOT
one-signed is skipped, and when none is one-signed the side's own
roots decide it (`Ok(None)`).

So the outcome is not monotone in the derivative box's tightness: a
looser box that straddles zero falls through to the roots and answers,
while a tighter box that reads the slope one-signed but inside the band
refuses.

The NURBS quotient bound (`nurbs/quotient-bound`) tightened
`NurbsBoxes::deriv_box`. On `m5_pr7_ssi.rs`'s bent-path flat wall
(`a_flat_wall_whose_chart_path_bends_answers_as_the_plane_it_is`,
`α = 1e-6`, `μ = 0.2`):

- before, the `v = 0` strip's `n·S_v` box was `[−1.6e-8, 0]`, so it was
  not one-signed, the side went to its roots, and the march answered
  the line at `ε = 1e-9`;
- now it is `[−1.6e-8, −7.98e-9]`, the true range to rounding, and the
  slope margin `8e-9` lies inside `(1e-9, 1e-8)`.

The wall's `v` direction runs along `y`, in the plane, so the side is
near-tangent in this lane's sense and the refusal reads the geometry
correctly. The row now stands down at `ε = 1e-9` as it does at `1e-6`,
and answers the line at `1e-12`.

## Shape

The question is the policy, not the bound: should a one-signed
in-band slope refuse, when the side's roots could still decide it (as
they do for a straddling slope)? Either answer makes the outcome
monotone in the box's width:

- fall through to the roots on an in-band slope too;
- or refuse a straddling slope as well.

The second would refuse more of today's rows.
