---
id: star-reads-whether-an-extremum-lies-inside-a-sector-twice
kind: issue
title: star::walk decides 'an extremum strictly inside a sector' twice: by azimuth at an apex, by cross products in a tangent plane
status: closed
opened: 2026-11-10
priority: P2
cost: C
closed: 2026-11-16
pr: 7015
branch: onehome/star-extremum
---


Found by PR 6725's FULL review (S1), in `topo::star` stage 2.

`Star::refine` cuts a sector at a datum's extremum strictly inside it, and two tests decide "strictly inside":
- **At a cone's apex:** `sector_shape::apex_extrema_inside` reads the extremum's azimuth from the sector's start, `sector_apex_window` on the chord between rulings, levered at the arm.
- **In a tangent plane** (a partner cone's nearest and farthest directions), `star/walk.rs` `extremum_inside` decides it inline: `bool_sector_within` on `(start × d)·n` and `(d × end)·n`, levered at the arm. Its tilt test is likewise inline (`bool_sector_partner_tilt`), beside the apex arm's `sector_apex_tilt`.

They are one question, "does the datum's side turn inside this sector", posed in two geometries, with two predicate families and two lever conventions.

Owed: one "inside the sector" reading per sector kind on the star's `Sector`, or say why the two must stay apart. Their K names are pooled in k-lint's distributions, so a rename is a k-lint re-baseline.

## Closed (2026-11-16): two readings, one per sector kind, homed together

They stay two readings, because the sectors are two kinds, and they now sit
side by side in `topo::sector_shape`, behind the one dispatch
(`Star::extremum_inside`, keyed on the sector's `StarFace`).

- **At a cone's apex** (`apex_extrema_inside`) the sector is the cone's fan of
  rulings between its bounds, which span no plane. So no `(start × d)·n` with
  one `n` reads it. "Inside" is the ruling's azimuth from the start, read as
  the chord between rulings (`sector_apex_window`).
- **In a tangent plane** (`tangent_extrema_inside`, moved out of the walk) the
  bounds are two directions in one plane. "Inside" is the sign of
  `(start × d)·n` and of `(d × end)·n` (`bool_sector_within`).

Each is a displacement at the sector's arm, levered at the same length: the
apex arm passes the sector's arm length through `metering_arm`, and the tangent
arm passes the sector's `Lever` of that length. So the lever conventions
differ in provenance, not in value. The move keeps every predicate name, lever
and error mapping, so no verdict and no k-lint name moves.
