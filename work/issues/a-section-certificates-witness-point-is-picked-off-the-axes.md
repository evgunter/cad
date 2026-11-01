---
id: a-section-certificates-witness-point-is-picked-off-the-axes
kind: issue
title: The section certificate's witness on a sphere section circle is picked off the axes, so which point decides can follow the computing frame
status: open
opened: 2026-10-21
---


`topo/src/boolean/section_cert.rs` picks the one closed-form point of a
sphere section circle it certifies (`sphere_plane` and `sphere_sphere`,
each `lone(.. + b1 * rad)` with `b1` from `Vec3::orthonormal_basis`) and
the scratch chart of `cone_cylinder`'s ruling reader (`d.orthonormal_basis()`)
off the coordinate axes. No output keeps either, so the
`minted-reference-from-inputs` gate admits the file as a home. But W3
(a witness `Out`) and the no-event decision read that one point: where
the circle lies partly inside a face, which point W3 tries can decide
between a certificate and an `R-undec` refusal, and so can follow the
computing frame. D10 `[placement.frame]` asks every verdict outside the
sliver band to be the same in any frame.

Found by the `a-minted-reference-direction-follows-the-computing-axes`
sweep. Not examined further: whether any configuration reaches W3 with
a circle that is partly inside, and whether the point should come from
the operands (the sphere's own `u_ref`, as `geom_brep::plane_sphere_section`
derives its circle's) instead.
