---
id: axis-projections-spelled-v-minus-a-a-dot-v-outside-the-section-certificate
kind: issue
title: Projections square to an axis spelled v − a(a·v) outside the section certificate keep an axial rounding component; unmeasured
status: open
opened: 2026-10-09
---


## What

PR 4352's dual review measured the hazard in the section certificate's
cone arms: a part of `v` square to the unit axis `a`, spelled
`v − a·(a·v)`, keeps a component along `a` of about `ε|v|`. Divided by a
small norm (a point near the axis), that tilts the derived direction off
the plane square to `a` by `ε|v|/|v⊥|`. There it misread near-axis
sphere poses in every unsafe direction and put witnesses up to 1.1 m
off a carrier at 3 m–2 km scales. The fix there is `section_cert`'s
`square_to` (`a × (v × a)`).

The same spelling stands in production code outside the certificate.
None of these is measured; whether each divides by a small norm, or
only takes a norm (where the error is an absolute `ε|v|`, harmless), is
the question for each site:

- `contain.rs` `curved_face_placement`'s cylinder/cone radial (`:624`)
  and the cone arm's radial (`:955`) — an azimuth read off it near the axis;
- `solid_contain.rs` radials (`:2766`, `:2809`, `:3786`) and `dp`
  (`:4118`);
- `join.rs` `:1123` (`radial`);
- `recl.rs` `:840` (`off`);
- `carrier_eq.rs` `:595` (the returned perpendicular);
- norms only: `edge_join.rs` `:255`, `rim_wedge.rs` `:621`, `:843`.

Test-only hits (`join.rs` `:3678` on, `boxes.rs` `:6352`,
`reduce.rs` `:5814` on, `ellipse_torus.rs`, `conic_oracle.rs`) are not
listed. The grep matched the literal shape `x − a·x.dot(a)` and
`x − a·a.dot(x)` in `crates/topo/src/boolean/`; it cannot see the
projection spelled through a helper, with the terms reordered, or in
other crates.
