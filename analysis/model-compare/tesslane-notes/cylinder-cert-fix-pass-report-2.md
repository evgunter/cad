FIX PASS REPORT 2/2 — PR 7044, head e43ce957e9bc0d50f7dc77f4343373a8a4b89c49. The whole PR body as it should read, between the two lines of =====.

=====
# The cylinder and sphere certificates read their true distance on every triangle

Closes `work/tesslane/the-cylinder-certificate-reads-a-ruling-sided-triangle-as-zero.md`.

## The measurement

A probe in both curved lanes (not in this diff) took, for every cylinder and sphere triangle certified, the distance as main computes it in all six corner orders, the distance as this PR computes it, both verdicts, and the triangle's deviation sampled on a 561-point grid against the surface's closed form. It ran with main's certificate deciding, over the `mesh` suite whole, the 583 rows of `sweep` whose files call `tessellate`, every test of `tools/tess-meter`, and the tour's `ci` rows.

**In these runs no mesh `tessellate` returns holds a triangle over δ that the certificate read as 0, or read short.** That is a measurement of this corpus, not a property of the lanes. The largest true deviation among the triangles main misread is 0.99980 of its δ (a tour row), and one row built to land a triangle on δ at a coordinate of 900 sits 1.2·10⁻¹⁰ of δ past it, inside the verdict's allowance there under either reading.

| run | triangles read NaN, so 0 | curved lane / trimmed lane | read finite and short | tests reaching it |
|---|---|---|---|---|
| `mesh` | 100 677 | 6 291 / 94 386 | 0 | 59 |
| `sweep` meshing rows | 587 390 | 1 633 / 585 757 | 127 (14 / 113) | 92 |
| `tools/tess-meter` | 9 | 9 / 0 | 0 | n/a |
| tour, `ci` rows | 1 471 | 221 / 1 250 | 0 | 11 |

- **Every one is a cylinder triangle.** No sphere triangle read a distance that was not finite, or one that differs from this PR's by more than the verdict's allowance, in any of its six orders: no body reaches the sphere's clamp. It is reached by a corner that is not finite, and by two corners at one point in some orders, which the curved lane drops before certifying (a pole's collapsed sliver).
- **Corner order.** Each of those triangles reads NaN in exactly two of its six corner orders and its true distance in the other four.
- **No verdict differs** between main's reading and this PR's on any triangle of any of those runs.

Three things the row's finding had differently:

1. **The sibling is not a safeguard.** Both halves of a lattice cell have a side on a ruling, so both have a shadow of no area; which half read NaN was its corner order. What kept these triangles within δ differs by lane. The rectangle lane sizes its step from a worst-triangle theorem (`[mesh.sizing]`), so its cell-halves land at δ or under by construction. The chart lane's cylinder arm has no such theorem: it sizes at `UNPROVED_HEADROOM` and names the certificate as its guarantee, so there the triangles main read as 0 were passed on no guarantee at all. Review measured them on the `mesh` corpus at 0.4 to 0.5 of δ in bulk, with a tail to 0.652 of δ (a ruling side spanning 1.14 lattice steps). The certificate now decides every one of them.
2. **The misreading is not only a NaN.** On a cylinder whose axis is off the coordinate axes the two shadows of a ruling's ends differ by rounding, the shadow is a sliver with a rounding-sized area, and the interior branch divided by that: 127 emitted triangles read a finite distance too long, a certificate up to 11 % under the true deviation (`sweep`'s `bisector_nodes` and `a_closed_conic_in_the_partners_face` rows). Answering "the nearest side" only where the area is exactly zero would have left these.
3. **The trimmed cylinder arm**, which the row had not measured, holds most of the misread triangles and none over δ.

## The change

`crates/mesh/src/cert.rs` only.

**`dist_point_triangle`.** The nearest point of a triangle is the foot of the point on its plane where that is inside, and on a side otherwise. With `s` the distance to the nearest side and `ρ` the inradius, the distance is in `[s − ρ, s]` either way. It answers:

- `s`, to its own rounding, where the foot is outside and for a triangle of no area;
- the foot's distance, to its own rounding, where the foot is inside a triangle whose two shorter sides are not near parallel, a needle included. Corners on a sphere about the point, or a cylinder about the line, are always one of these two: the foot is then the circumcentre, inside only a triangle with no angle past a right one;
- a value within `ρ` of the distance, in `[s − ρ, s]`, where the foot is inside a sliver (one corner close to the opposite side), whose plane rounding has tilted. Review measured that gap at 5·10⁵ allowances on a sliver off the surface; no triangle with its corners on the surface is one.

How it is computed:

- the corners are put in one order first, so the answer is the same to the bit in all six orders, and the normal is the cross product of the first two sides of that order. The order is what keeps a needle's digits in that product: where those two are a needle's long sides, the far corner sorts between the close two, the needle lies across the first coordinate to within its width, and the one component that cancels tilts the plane about the needle's own length. Read in the order the corners came in, the same needle is 8·10⁻¹⁰ off (a row holds it);
- the corners are read at the scale of the furthest one from the point, a power of two, so no size of triangle overflows or underflows on the way; a feature smaller than 10⁻⁷⁷ of that scale can underflow, and is within the bracket by its own size. `dist_line_triangle` reads its direction at its own scale the same way before it takes the shadow: handed as stored, a direction of length 10¹⁵⁵ answered 2.449 for a true 0.5385. It states no unit direction;
- a coordinate that is not finite, or two points whose difference is not, answers NaN. So does a zero direction.

What each degenerate projection answers:

| handed | answers |
|---|---|
| a side on a ruling (the shadow is the column's chord) | `r` less the chord's distance: the sagitta, as its cell's other half |
| three corners on one ruling (the shadow is a point) | 0 |
| a side on a ruling, third corner across the axis (a diameter) | `r` |
| a corner on the axis | `r` |
| a triangle seen edge-on (three shadow points in line) | `r` less the nearest shadow point's distance |
| sphere: two corners at one point | the chord's sagitta |
| sphere: three corners at one point | 0 |
| sphere: a triangle through the centre, or a corner at it | `r` |
| any corner, anchor, axis or radius not finite; a zero axis; two points whose difference overflows | NaN, which the verdict reads as exceeded |

**One rule for an input that is not finite, in one place.** All five certificates (`cert_cylinder`, `cert_sphere`, `cert_cone`, `cert_torus`, `cert_spindle`) ask `all_finite` of everything they are handed and answer NaN. `cert_torus` tested for NaN alone and let an infinity through as `+∞`, and `cert_spindle` did not read an apex corner's azimuth at all; both now answer NaN. `+∞` keeps its other meaning, finite corners that are no triangle of the surface. `cert_cylinder` and `cert_sphere` are then `r − dist` kept at least 0 by `cert::worse`, which keeps a NaN.

**`cone_chord_sag`** answers NaN where its slant product overflows. Its doc said so; under a small `sin α·cos α` the numerator stayed finite over an infinite denominator and it answered 0 (slants near 1.4·10¹⁵⁴ at `α = 0.05`). Found by the sweep below.

## What moves

**No face's verdict and no mesh byte.** The D9 goldens (`d9_mesh_goldens`), the meter's bytes (`tools/tess-meter`, `r2_wedge_bytes` included), `docs/tess-budget-data` and the tour's pins are unchanged, and every row of the runs above passes with the new certificate deciding; review found all 11 443 meshes of its corpus byte-identical. The probe says why: every triangle main read as 0 or short is within its δ at its true value, so nothing refines or refuses that did not.

**A reported bound moves in its last digits.** The distance is computed differently, so a bound differs from main's by rounding. Review found four `CertificateExceeded` refusals whose reported `bound` moves in the last one or two digits (0.11990393700810942 becomes …953). No test or golden reads those digits.

What does change is that the certificate is now a check on those triangles: 689 547 emitted triangles in these runs were passed without one.

## Rows

In `cert::tests`. None reads ε: each expects the same at `CAD_TOLERANCE_EPS` 1e-6, 1e-12 and the default, and passes at all three.

- `a_shadow_of_no_area_certifies_at_its_true_deviation_in_every_corner_order` (was `the_cylinder_and_sphere_certificates_read_a_nan_as_zero`): the table above, each row to 10⁻¹⁵ in all six orders. It does not assert the orders agree to the bit: none of its fixtures moves a bit by its order, so that pin is the fuzz's.
- `a_needles_plane_keeps_its_digits_in_every_corner_order`: a needle 10⁻⁸ as wide as long at no special pose, half a length from a point, to one allowance in all six orders.
- `the_two_distances_are_the_distance_on_triangles_built_to_be_degenerate`: a seeded fuzz with the nearest point built first, so the distance is known on both sides: needles, slivers, corners exactly in line, two and three corners at one point, in any pose up to 10⁴ lengths out. It pins eight allowances where the nearest point is on a side, at a corner or inside a triangle that is not thin; the inradius, the distance's own stated width and nothing tighter, where it is inside a sliver; all six orders to the bit; and the size: the same triangle 2⁴⁰⁰ and 2⁹⁰⁰ times larger and smaller answers the same to the bit, scaled, and the line's direction 2⁻⁹⁰⁰ to 2⁹⁰⁰ times as long changes nothing.
- `the_cylinder_and_sphere_certificates_are_the_deviation_on_triangles_built_to_be_degenerate`: a seeded fuzz of both certificates against the deviation found on the triangle and, for the cylinder, its closed form, neither under nor over, within the one rounding allowance the verdict gives a bound at that pose.
- `the_certificates_poison_on_an_input_that_is_not_finite` (was `the_cone_torus_and_spindle_…`): one assertion, NaN, for all five certificates: every corner coordinate, an apex corner's unread azimuth, every radius, anchor and axis, a zero axis, two points whose difference overflows, and the cone's overflowed slant product.

Mutants, each run against `cert::` and `sizing::`:

| mutant | reds |
|---|---|
| the old clamp, `(r − dist).max(0.0)` | the poison row |
| a certificate's entry check removed | the poison row |
| `cert_torus` guarding NaN alone | the poison row |
| one fixed side where the triangle has no area | the table row, both fuzz rows, `sizing::tests::cylinder_rim_triangle_lands_at_delta` |
| the foot's distance read outside the bracket | the distance fuzz |
| the corners not put in one order | the needle row, the distance fuzz |
| the normal from the last two sides of the order | the needle row, the distance fuzz |
| the corners not read at their own scale | the distance fuzz |
| the direction not read at its own scale | the distance fuzz |
| the cone's overflow reading 0 | the poison row |

The normal from the two shorter sides, which this PR first carried, is gone: with the corners in one order the first two sides read a needle as well (worst 3·10⁻¹⁵ of a unit length over 20 000 needles down to a width of 10⁻⁹), so no row could tell the two apart and the select carried nothing.

## The class (implementer-discipline §5)

**Shape:** in `crates/mesh/src` production code, a `.max(`, `.min(`, `f64::max`, `f64::min`, `.clamp(` or a max/min fold, and an accept/refuse comparison, where a NaN operand reads as a number; and a division by something zero on a degenerate input (an area, a cross product's norm, a squared length, a `1/(a+b+c)`).

Fixed here:

- `cert::cert_cylinder`, `cert::cert_sphere`: the clamp.
- `cert::dist_point_triangle`: the division by the area, and products past the range at either end.
- `cert::dist_line_triangle`: `|d|²` past the range.
- `cert::cert_torus`, `cert::cert_spindle`: an infinity read as `+∞`, an apex azimuth not read.
- `cert::cone_chord_sag`: an overflow reading 0.

Not this unit, filed:

- `nurbs_cert::NurbsFaceBound::cert`, `NurbsCellGrid::cert` (`min3`/`max3` drop a NaN chart corner, and the crease test can then miss a crease): already `work/chord/nurbs-cell-bounds-cert-drops-a-nan-vertex-and-lands-a-nan-end-on-the-first-cell.md`; evidence added there. Latent: the trimmed lane's corners are CDT vertices and the CDT refuses a NaN coordinate.
- `sizing::sphere_face_widest_cos`, `sizing::cone_face_rho_max` (window folds drop a NaN ahead of their own guard), `sizing::cap_angular` and `sagitta_step` (a NaN takes the cap), `curved::require_swept_rectangle` (a refusal's payload overwrites a NaN): `work/tesslane/sizing-folds-drop-a-nan-before-their-own-guard.md`. None decides a verdict.

Read and sound as they stand:

- `cert::worse`; `abs_radial_max`, `abs_sin_max` (reached only behind `cert_torus`'s entry check); `position_scale`, `uv_scale`, `cone_scale` (the allowance's scale: a dropped NaN makes the verdict stricter); `cert_cone`'s `weights > 0`.
- `cert::verdict`, `tessellate`'s δ check, `sizing::ceil_count`, `chords::tighten`, `validate::check_volume`, the trimmed lane's winding read and refinement gate: a NaN takes the refusing branch in each.
- `nurbs_cert`'s cell sups (cells are refused unless finite where the grid is built), `split_steps`, `lattice_metric`; `curved`'s and `trimmed`'s chart boxes (a NaN entry refuses at the box test or at the CDT); `chords::edge_target`; the allowance payloads.
- Divisions: `dist_origin_segment`, `cert_cone`, `uv_scale` (guarded); `chart_table::lattice_rows`, `chords::tighten`, `sizing::curvature_step`, `ellipse_step`, `torus_grid_steps` (the quotient goes to `ceil_count` or a certificate, which refuses); `chart_table::metric`, `trimmed::inside_point`, `planar::chart_frame` (shape only; the CDT refuses a NaN point).

**What the pattern could not match:** a max written as an `if`; a sort or `partition_point` with a comparator; componentwise `Point3::min/max`; a max or a division inside a callee in another crate; an overflow that reads a finite wrong value with no NaN in it.

**Second pass at that gap:** `if`-selects of two floats (`require_swept_rectangle`, filed above; the trimmed meter's, sticky; `planar::chart_frame`'s farthest point and `walk::unwrap_tie`, placement only); no `partial_cmp`, `max_by` or `min_by` in production, and every `total_cmp` orders and selects no bound; the callees in a deciding position (`geom::Line::offset` divides by `|dir|²` unguarded: its NaN now refuses, and `dist_line_triangle` hands it a direction at unit scale; `geom::periodic_window_may_hold` answers true on a NaN; `geom_brep::implicit_outward_normal`'s NaN reads as a fold; `geom_brep::harmonic_range` drops a NaN endpoint, in the filed row). The overflow gap is where `cone_chord_sag` was found, and where review found `dist_line_triangle`'s direction and the corners' underflow. Not opened in depth: `Pcurve::chart_box`, `spiric_curvature_sup`, `nonrational_second_derivative_sup`, `accel_upper_bound`, `patch_cells_across_creases`, whose results `mesh` checks finite or certified where it reads them.

## Also filed

Evidence on `work/chord/S236.md` (the cylinder certificate had a defect behind its missing falsifier; a standing sample of the mesher's own triangles is still that row) and on `work/chord/dist-line-triangle-takes-an-unchecked-unit-direction.md` (the function scales its direction first and states no unit direction, and a row runs it far off unit).

## What was run

Hosted CI's change filter skips the `tools` and `demos` jobs on this diff, so the meter's bytes and the tour's pins rest on local runs: the tools roots (`k-lint`, `tess-lint`, `tess-meter`: fmt, clippy, tests) and the tour's `ci` suite here, and the meter again in review. Also run here: `cargo nextest run -p mesh` whole, `cargo clippy -p mesh --all-targets --features budget` and `cargo test -p mesh --features budget`, `sweep`'s 583 meshing rows, the viewer's clippy and tests (three need a GPU adapter and did not run: `gpu::tests::every_pass_builds_on_a_real_device`, `gpu::tests::the_culled_passes_draw_every_face_that_faces_the_eye`, `drive::driven::a_picture_shows_the_window_with_the_part_drawn`), the gates, the doc gate and fmt.
=====
