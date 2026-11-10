---
id: star-reads-whether-an-extremum-lies-inside-a-sector-twice
kind: issue
title: star::walk decides 'an extremum strictly inside a sector' twice: by azimuth at an apex, by cross products in a tangent plane
status: open
opened: 2026-11-10
priority: P2
cost: C
---


Found by PR 6725's FULL review (S1), in `topo::star` stage 2.

`Star::refine` cuts a sector at a datum's extremum strictly inside it, and two tests decide "strictly inside":
- **At a cone's apex:** `sector_shape::apex_extrema_inside` reads the extremum's azimuth from the sector's start, `sector_apex_window` on the chord between rulings, levered at the arm.
- **In a tangent plane** (a partner cone's nearest and farthest directions), `star/walk.rs` `extremum_inside` decides it inline: `bool_sector_within` on `(start × d)·n` and `(d × end)·n`, levered at the arm. Its tilt test is likewise inline (`bool_sector_partner_tilt`), beside the apex arm's `sector_apex_tilt`.

They are one question, "does the datum's side turn inside this sector", posed in two geometries, with two predicate families and two lever conventions.

Owed: one "inside the sector" reading per sector kind on the star's `Sector`, or say why the two must stay apart. Their K names are pooled in k-lint's distributions, so a rename is a k-lint re-baseline.
