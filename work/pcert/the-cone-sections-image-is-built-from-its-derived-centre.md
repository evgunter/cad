---
id: the-cone-sections-image-is-built-from-its-derived-centre
kind: issue
title: A cone section is born anchored at its near vertex, and its FocalSection image is still built from the derived centre and the semi-axis: the envelope reads their disagreement, 50 to 95 u of the section's size on a flat cone
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Filed by ONEROUND's `oneround/anchored-conic` (2026-11-20); here
because `crates/geom-brep/src/pcurve_cache.rs` and its cone-image row
are this program's ground.

`plane_cone_section` now mints its ellipse anchored at its near
vertex (`ConeTilt::near_vertex`), where it stands on the plane and
the cone to the rounding of its own coordinates. The ellipse's centre
is derived from that anchor, `anchor − u_ref·semi_u`. The
`FocalSection` image (`derive_class_zero`, and the certificate's
envelope) is still built from the centre and the semi-axis, each of
which is read through the small `K = c² − sin²α`.

Measured, `pcurve_cone_section::a_flat_cones_section_images_and_certifies_at_its_own_resolution`
(half-angle 1.55, height 0.1, the plane `f·(π/2 − α)` off square),
the envelope in `u` of the section's size:

| f | centre-born, the base | anchored | the anchor summed `centre + toward·a` |
|---|---|---|---|
| 0.5 | 36.1 | 50.2 | 36.1 |
| 0.8 | 31.4 | 81.3 | 86.8 |
| 0.9 | 30.4 | 94.6 | 30.4 |

The derived centre lies within 1e-15 of the old one (3.3e-16,
1.1e-15, 8.9e-16 m at semi-axes 3.3, 6.8, 12.9), so the growth is
not the centre moving: it is the image and the carrier now
disagreeing by the rounding of `centre − (anchor − u·a)`, where
before both read one stored centre. The row's ceiling went from 48 to
104 `u`, fitted, with its mutant (the anchor 30 `u` of the semi-axis
back along the major axis reads 42, 160, 82).

To try: build the image about the anchor, as the carrier is (the
vertex's azimuth and height on the cone are well-conditioned where
the centre's are not), so the envelope is again the two forms'
rounding at the arc's scale.
