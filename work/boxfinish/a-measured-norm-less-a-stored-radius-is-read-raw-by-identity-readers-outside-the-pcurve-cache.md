---
id: a-measured-norm-less-a-stored-radius-is-read-raw-by-identity-readers-outside-the-pcurve-cache
kind: issue
title: A measured norm less a stored radius is read raw by identity readers outside the pcurve cache's distance, so a point built on a cylinder or sphere misses it by √(r²) − r over a box
status: open
opened: 2026-11-15
---

Found by the audit of `a-radial-distance-and-a-rulings-azimuth-recover-the-radius-through-a-root`
(`boxfinish/radial-readers`). That unit re-posed `pcurve_cache.rs`'s
`chart_distance` (cylinder and sphere arms) through the rationalized
gap `|R² − w·w| / (R + ‖w‖)` (`radius_gap`, over `norm_gap`). The same
shape, a measured norm less a stored radius on a point BUILT on the
surface and read by a check that needs Zero, stands at the sites
below. Over a box on a radius `r` that a verb stores with no `abs`,
each reads `√(r²) − r`: zero at every `r > 0`, not the zero form.

**Measured** (the first site only): on `boxfinish/doors-read-arcs` at
`95b1e397` with the circle verb storing `r` (its probe `PROBE_R`) and
the unit's two re-posings applied, the tour's cut plate over its study
box passes the hole's extrude and refuses at its first `Subtract`:
`UnfinishedLeaf … VolumeUncomputable { Face 3v1, Escalated { check:
Exact, "props_meridian_on_surface", [0, 3.2e-4] } }`. Raising site:
`crates/geom-brep/src/props/curved.rs` `cylinder_boundary`, its line
arm, `(f0, f1) = (q0.norm() − radius, q1.norm() − radius)`. With that
one site spelled as the signed gap `(q·q − R²)/(‖q‖ + R)` (a local
probe) the premise passes and the same face refuses next at
`props_rim_fit [-6.0e-5, 6.0e-5]`. Which arm raised it was not
traced; on a cylinder face that predicate is the same function's
circle arm, `r_c − radius`, two stored radii and not this shape.

**The sites** (a sweep by pattern, each classed by reading its
function and usually one caller; only the first is measured):

- `geom-brep/src/props/curved.rs`: `cylinder_boundary`
  (`props_meridian_on_surface`, above); `sphere_loop`
  (`props_sphere_circle_fit`) and `sphere_boundary` (`props_rim_fit`),
  both `√(‖w‖² + r_c²) − R` for a circle seated on the sphere.
- `geom-brep/src/pcurve_cache.rs`: `torus_oblique_circle`,
  `w_r.norm() − r` (`pcurve_torus_villarceau_offset`).
- `topo/src/boolean/contain.rs`: `curved_interior_placement`'s cylinder
  arm (`radial.norm() − radius`) and `sphere_face_containment`
  (`(q − center).norm() − radius`), `bool_curved_contain_carrier`: a
  query point in general, but Zero is its on-carrier branch, which a
  point built on the wall must reach.
- `topo/src/boolean/solid_contain.rs`: `sphere_chart_trim`
  (`bool_sphere_iso_rim`).
- `topo/src/boolean/rim_wedge.rs`: `on_locus`, `rho − rim.radius`
  inside a root (`seam_cover_on_locus`).
- `topo/src/offset_axial.rs`: `surface_residual`'s cylinder and sphere
  arms (`offset_axial_edge_on_surface`) and `Profile::residual`
  (`offset_axial_concurrence`).
- `topo/src/validate.rs`: `locus_gap`, `rho − *radius` inside a root
  (`ring_outer_locus_gap`).
- `topo/src/link.rs`: `Round::offset`,
  `reach_from_axis(…) − self.radius`.
- `topo/src/boolean/torus_region.rs`: `on_edge_plane`'s circle arm,
  which picks a root's own circle by `‖w‖ − radius`.
- `geom/src/curves/on_carrier.rs`: `clearance`'s circle arm,
  `√(h² + (ρ − radius)²)`, read by `Foot::on_carrier`; whether a Zero
  is asked of it there was not traced.

Not this class (real margins, each a point that is off in general):
`sweep`'s `blend/reach.rs` region functions and `blend/surgery.rs`
distances, `topo`'s `carrier_touch`, `carrier_pair::torus_distance`,
`ring_path`, `injective`, `section_cert`, `geom`'s `cover.rs`,
`geom-brep`'s `locus::clearance` and `pair_section` levels.

**What the sweep could not match:** a radial distance returned by one
function and subtracted in another under a name that is no radius; the
same hazard as a ratio (`radius / ‖w‖`, `stretch`) or as a foot gap
`(q − carrier.eval(t)).norm()`; a cone's radius at a height
(`cone_offset`, which holds a raw `‖w‖` and no stored radius); crates
other than `geom`, `geom-brep`, `topo` and `sweep`.

**The tests that say it is closed.**
- Each site above reads a proven Zero at `Sym<Interval>` over a box on
  the radius for a point built at `centre + r·x`, or is shown to be a
  margin.
- The tour's cut plate, with the verbs storing `r`, no longer refuses
  on a `√(r²) − r` in any spelling.
- The source gate the parent row names
  (`a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided`,
  unit 3) holds the raw spelling out.

`work/honeone/point-to-carrier-distance-is-spelled-seven-times` is the
same readers seen as one reading spelled many times; one home for the
reading would close both.
