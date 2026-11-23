Part (c) of 4, piece 1 of 2: the PR body, top half. Piece 2 continues it directly; the closing ===== line is at the end of piece 2.

=====
## Measurement

On `origin/main` at `a56edec9d6`, `tessellate::lane_of` routed a curved face by three kind tests: does it hold a ring; does its outer loop carry an ellipse, spline or spiric; on a sphere, does it store a projected image. Everything else went to the curved (rectangle) lane, and that lane then asked the two doors (`require_iso_rectangle`, `require_one_chart_branch`) and refused any face they did not pass. A face bounded by rims and rulings that is no rectangle (a pipe wall notched through its seam) therefore reached no lane.

The doors' refusal sites, by what they decide (each with a body or row that reaches it):

| class | site (variant / `what`) | reached by |
|---|---|---|
| no rectangle | carrier kind: ellipse on a cylinder or cone; non-circle on a sphere or torus; spline or spiric (`Unimplemented`) — `IsoChart::kind_refusal`, read first | tilted cuts (sweep), the vessel's spiric torus walls, spline-trimmed walls |
| no rectangle | `props_rim_axis_parallel`, `props_rim_center_on_axis`, `props_meridian_great` (on the sphere), `props_meridian_axial`, a circle too small to read on a cylinder | tilted sphere sections, oblique lens, wobble rows |
| no rectangle | `props_rim_level` | notched pipe walls, keyway, notched sphere faces |
| no rectangle | `props_rim_side` | apex bow tie |
| no rectangle | torus face without a meridian | rim-only torus faces |
| no rectangle | `NotOneChartBranch` | pole-crossing half cap, hemisphere pair |
| cannot say | `Escalated` (every decide) | a wall put 3 zero-bands off its surface (`route_rows`) |
| contradicts itself | `OffSurface` — incl. the cylinder's "not a rim" circle, the sphere's `props_meridian_great` and the torus's `props_rim_fit` where the circle misses the surface (decided on the refusing path), and the torus's `props_meridian_fit` and `props_meridian_plane` | `geom-brep` `incidence_at_the_shape_door`, the washer wall on a cone (`route_rows`) |
| contradicts itself | `SpanUncertified` (new): a stored span outside `0 < Δt ≤ τ`, split pieces that do not assemble one | the split child moved a period (`mesh10r1/r2`, `iso_rectangle_door`), hand-built span rows |
| contradicts itself | `Defect` (new): rims with two level representations, a torus piece the classification left uncircled, a torus axis or meridian radial of zero or non-finite length, a degenerate meridian orientation, a loop of meridian pieces with no rim | `torus_meridian_radial`, `mesh10r1/r2`; the rest unreached (each a kernel defect) |
| not a verdict on a face | a curved door asked of a plane | no caller in `mesh` |

`NotIsoRectangle` mixed the first and third classes; it is now only the decided "outside the inventory" verdict (standing row 2). The torus's `props_rim_fit` cannot tell a coaxial circle off the torus from a Villarceau circle on it in one decision, so the refusing path decides the rim incidence and answers `OffSurface` only for a coaxial circle; a circle on a cylinder whose axis class reads zero is off it only where its radius misses the cylinder's (on a cylinder narrower than the band the class reads zero for a true rim).

The spec has three things wrong: `takes_sphere_arm` does not exist on this tree; `face_target` was no kind test (it read the lane); and the doors read no ring, so a ringed face passes them on its outer loop and the ring stays the route's first question.

## The route

`tessellate::route_of` answers by the first of three facts:

1. A face holding a ring → the trimmed lane.
2. A face whose outer loop stores an image of a form no iso line takes — projected, focal section, spiric, fitted, general (`Pcurve::is_non_iso_form`) → the trimmed lane, no door asked. The form is what the mint stored when its class reading showed no iso class. A harmonic image does not answer here: it is the form of rims and meridians and also of a section conic on its cylinder, and its coefficients are not a stored class (a rim on an obliquely placed body carries sine terms of 1e-17).
3. Otherwise the two doors (`curved::iso_rectangle_shape`):
   - **Both pass** → the curved lane, carrying a `curved::Rectangle` that only the doors mint and `tessellate_curved` takes, so no route reaches that lane without asking them.
   - **A door decides no rectangle** → the trimmed lane where `topo::Body::face_lift` reads the domain.
     - If no image is stored, the lift's own `DomainUnread` gap is the refusal.
     - If the loops have no reading in one sheet (an arc through a pole, a loop that winds the chart), the doors' verdict, with its recourse, is the refusal: `UnsupportedCurvedShape`.
   - **A door escalates, or the stored data contradicts itself** → `UnsupportedCurvedShape` with that cause.

The route is read once per face per run (`Routes::of`), before the chord pass. The chord pass, `face_target`, `arm_steps` and the lanes take the value. `Routes::lane` answers `Routed::To(lane)` or `Routed::Refused` explicitly. The kind tests are gone (`has_trim_carrier`, `has_tilted_circle`, `has_ring`, `curved::require_iso_rectangle_face`). The door reads every carrier's kind first, from one table the parses' own non-iso arms read (`IsoChart`), so an ellipse is decided before a rim in band.

**K stream.** The route records the doors' decisions apart from the domain read's. The fold splices the doors' at the face's turn where the lane runs, and the domain read's on every run. A memo hit therefore records what it recorded before: the chord pass's, and a trimmed face's domain read. `tessellate_with`'s doc states it.
