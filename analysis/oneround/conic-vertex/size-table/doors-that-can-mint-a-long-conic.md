# Production doors that can mint a conic with a semi-axis over 1 km

A survey by reading, 2026-11-21, of `crates/*/src` on
`oneround/anchored-conic` (test code left out). It was read by a
delegated reader and its lines were not each re-checked; two are marked
unsure. "Held point" is a point of the arc the door has in hand where it
builds the conic.

No constructor bounds a size: `Curve3::ellipse` / `ellipse_anchored`
check both semi-axes bounded and the minor positive; `Circle::new` /
`anchored` check finite and not negative.

| door | can a semi-axis pass 1 km, and what sets it | held point at the mint |
|---|---|---|
| `plane_cone_section`, tilted ellipse (`geom-brep/src/intersect.rs`) | yes: `major = |δ|·sin α·cos α / K`, `K → 0` toward the parabola | `vertex`, the near vertex (finite at `K = 0`); stored as the anchor |
| `plane_cone_section`, axis-normal circle | yes: `|apex_gap / c|·tan α`, a far apex or a near-flat cone | none: centre and size |
| `plane_cylinder_section_read`, rim circle | only if the cylinder's radius is | none: centre |
| `plane_cylinder_section_read`, tilted ellipse | yes: `r/|c|`, `c = a·n`, a plane nearly along the axis; the centre is far too | none: centre (`q` is the plane's origin, not on the arc) |
| `bisector_ellipses`, equal cylinders crossing (`pair_section/cylinders.rs`) | yes: `r/sin(φ/2)` as the axes near parallel | centre `foot` only; the nodes (minor vertices) are read after the mint |
| `plane_sphere_section`, `sphere_sphere_section` | no more than the sphere's radius | none: `foot` |
| coaxial cone×sphere, cone×torus, cone×cylinder circles | bounded by the other operand's radius | none: centre on the axis |
| `plane_torus_read` meridian, concentric, tangent circles | at most `R + r` | none |
| Villarceau circles | not minted: a generic tilt is refused | |
| torus parallels, cylinder×sphere station circles | bounded by the operand | none: centre |
| revolve latitudes, `AxisFrame::latitude` (`sweep/src/revolve/`) | yes: the profile vertex's distance from the axis | the caller holds the vertex |
| a sketch arc placed by sweep / extrude (`sweep/src/swept.rs`) | yes: a bulge arc's `L(1 + b²)/(4|b|)`, unbounded as `b → 0`; `profile/src/validate.rs` refuses at the scene's resolution, not by size (unsure whether anything else caps it) | `q_from`, the start vertex |
| blend arms' torus trims, sheet trim, chamfer frustum (`sweep/src/blend/arms.rs`) | bounded by the input rim and the blend radius | `rim_pt`, `con_pt`, `q`, the feet |
| `junction::scaled` (`blend/junction.rs`) | the caller's radius (unsure of its bound) | none: centre |
| oblique end-face ellipse, `cap_transverse` (`blend/battery.rs`) | yes: `radius/|n·τ|` | `foot_a`, `foot_b` (in `open/end_face.rs`) |
| blend surgery's section circle | the plan's radius | the feet `p0`, `p1` |
| STEP `CIRCLE` / `ELLIPSE` (`step-import/src/entities.rs`) | yes: the file's own sizes, checked finite and positive | none: the placement frame |
| `recognize_curve` circle | the circumradius of the file's curve | the samples `s0`, `s1`, `s2` |
| `step-import/src/build.rs` generators, `normalize.rs` torus seam | bounded by a sphere or torus | `p`; `p_start`, `p_end` |
| `offset_axial::mint_carrier`, latitude rule | `frame.radial(p_start).norm()` | `p_start` |
| `offset_axial`, sphere seam and torus seam meridian | radius plus the offset | centre only (`p_start` in scope, unused) |
| `replace_face::transport_curve`, cylinder / cone / torus arms | the old carrier plus the offset | `mid`, the curve's mid-parameter point |
| `replace_face::homothety` | `radius·k` | none: the mapped centre |
