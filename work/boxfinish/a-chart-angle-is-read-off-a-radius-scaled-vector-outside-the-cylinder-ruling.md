---
id: a-chart-angle-is-read-off-a-radius-scaled-vector-outside-the-cylinder-ruling
kind: issue
title: A chart angle is read off a radius-scaled vector at every arm but the cylinder ruling's, so a carrier built along the reference direction reads atan2(0, r) once its radius is stored signed
status: open
opened: 2026-11-15
---

Found by the audit of `a-radial-distance-and-a-rulings-azimuth-recover-the-radius-through-a-root`
(`boxfinish/radial-readers`). That unit re-posed the cylinder arm's
ruling phase as the angle of the ruling's radial offset over the
chart's radius (`pcurve_cache.rs` `ruling_direction`), so a ruling
built at `c + r·x` reads `x`'s angle and not `atan2(0, r)`, which the
symbolic tier folds only where its second argument is non-negative by
its syntax (`geom-core/src/sym/trig.rs`, the second fold). The other
readers below pass a vector at a radius's length.

**Measured:** on `boxfinish/doors-read-arcs` at `95b1e397` with the
circle verb storing `r` (`PROBE_R`) and the ruling re-posed, the
tour's hole's extrude passes every pcurve check, its two rims
included. So the cylinder rim's read is not a refusal on that tree:
the rim circle's radius is not spelled `r` there. Once an arc's
carrier stores the signed radius
(`a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided`)
that is expected to change, and is to be measured, not assumed.

**The sites** (a sweep by pattern, each classed by reading where its
vector comes from; none but the first paragraph's is measured):

- `geom-brep/src/pcurve_cache.rs`, the class arms' `Azimuth::of(v·cv,
  v·u_ref)`: the cylinder rim (`a_r`, the circle's radius long); the
  cone rim (`a_dir`) and the anchored cone ruling (`r_dir`, `ρ` long,
  with no stored radius to divide by); the cone's focal section
  (`start`, semi-major long); the sphere parallel (`a_r`); the sphere
  meridian's latitude (`(aa, reach)`, the sphere's radius long); the
  torus parallel's azimuth and its tube direction (`Azimuth::tube`,
  minor radius long); the torus meridian's azimuth (`side`, major
  radius long) and tube; `torus_oblique_circle`'s `t0`, whose frame
  legs `form.a`, `form.b` are radius long. Already unit: the sphere
  meridian's azimuth (`d_hat`), the Villarceau `u0`, the spiric wall.
- `geom-brep/src/edge_nurbs.rs`: `arc_rim_measured`, a corner control
  point less the circle's centre.
- `geom/src/curves/conic.rs`: `Circle::azimuth` and
  `Ellipse::azimuth` scale the frame's read BY the radius or the
  semi-axes (`Azimuth::scaled`); `geom/src/frame.rs` `Frame::azimuth`
  and `geom/src/chart_point.rs` `ChartChannels::read` read a point's
  offset at its length. Each serves a point on its own carrier
  (`Curve3::param_near` at an edge's own vertex: `topo`'s `shell.rs`,
  `replace_face.rs`, `boolean/reduce.rs`, `ring_path.rs`,
  `offset_axial.rs`, `chart.rs`'s `uv_of`) and an arbitrary query
  point alike.
- `topo/src/cover.rs`: `sphere_window`'s latitude `h.atan2(ρ)`;
  `topo/src/ring_path.rs`: `parallel`; `topo/src/boolean/rim_wedge.rs`:
  `locus_param`'s rim arm.
- `sweep/src/revolve/axis.rs`: `turn_past`, an arc's start less its
  centre.
- Turns between two radius-long rays (`atan2(cross, dot)`):
  `topo/src/offset_axial.rs` `param_on`, `reauthor`,
  `azimuth_turn_as`; `geom-brep/src/props/closure.rs` `turn_about`.

Not this class: a harmonic's crossing phase (`curve_face`'s
`harmonic_roots`, `plane_crossing`, `curve_pair/pose.rs`), whose
coefficients are radius-scaled but whose angle is a root and no built
point's; the projected lane's reads of a general carrier at any
parameter; `f64`-only readers (`pair_section/winds.rs`,
`census/self_touch_curved.rs`).

**What the sweep could not match:** an angle read through `sin_cos`
components with no `atan2` (`Azimuth::along`, half-plane decisions); a
closure or trait object that holds the read; crates other than `geom`,
`geom-brep`, `topo` and `sweep`. Whether the tier folds
`atan2(0, r·r)` (`Circle::azimuth`'s spelling) and `atan2(0, −√…)`
(the sphere meridian's far half) was not measured.

**The tests that say it is closed.** For each identity reader above,
a carrier built along the chart's reference direction at a parameter
radius `r`, stored signed, is placed at the literal angle at
`Sym<Interval>` over a box on `r`; a reader that serves a query point
alone is left.
