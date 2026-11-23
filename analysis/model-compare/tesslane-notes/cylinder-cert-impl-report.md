REPORT 1/2 — tesslane/cylinder-cert (the cylinder certificate unit). Head 5e933bfff167b56606cbbde5397e2c1256345dfa. Part 2/2 is the PR body.

## 1. The measurement

**No mesh `tessellate` returns today is over δ because of this.** No verdict differs between main's reading and the new one on any triangle measured, and no committed byte moves.

A probe in both lanes (never committed) took, per cylinder/sphere triangle certified: main's distance in all six corner orders, the new distance, both verdicts, and the deviation sampled on a 561-point grid against the closed form. Run with main's certificate deciding.

| run | triangles read NaN → 0 | curved / trimmed | read finite and short | tests reaching it |
|---|---|---|---|---|
| `mesh` whole | 100 677 | 6 291 / 94 386 | 0 | 59 |
| `sweep`, the 583 rows whose files call `tessellate` | 587 390 | 1 633 / 585 757 | 127 (14 / 113) | 92 |
| `tools/tess-meter`, all tests | 9 | 9 / 0 | 0 | n/a |
| tour, `ci` rows (97) | 1 471 | 221 / 1 250 | 0 | 11 |

- All cylinder. No sphere triangle reads a non-finite distance or one off by more than the verdict's allowance, in any order: no body reaches the sphere's clamp.
- Each such triangle reads NaN in exactly 2 of its 6 corner orders.
- Largest true deviation among misread triangles: 0.99980 δ (tour). One `mesh` row built to land on δ at coordinate 900 sits 1.2e-10 of δ past it, inside the allowance under either reading.

What the spec had wrong:
1. **The sibling is no safeguard.** Both halves of a lattice cell have a side on a ruling; which half read NaN was its corner order alone. Faces are within δ because the lanes size their steps so.
2. **Not only NaN.** On a cylinder off the coordinate axes the shadow is a rounding-sized sliver and the interior branch divides by that: 127 emitted triangles read a finite distance too long, a certificate up to 11 % under the true deviation (`sweep`'s `bisector_nodes` and `a_closed_conic_in_the_partners_face`). "Nearest side where the area is exactly zero" would have left these, so the cure is wider than the spec's sentence.
3. The trimmed cylinder arm (unmeasured in the row) holds most of the misread triangles and none over δ.
4. Brief said the budget job runs `-p mesh --lib` in release; `ci.yml`'s `mesh-budget` job is `cargo clippy -p mesh --all-targets --features budget` then `cargo test -p mesh --features budget` (dev profile). I ran those two steps as written.

## 2. What changed

`crates/mesh/src/cert.rs` is the only source file.
- `dist_point_triangle`: rewritten. Nearest side `s` always; the foot's distance only where the foot is inside, read inside the bracket `[s − inradius, s]`; normal from the two shorter sides; corners sorted first (bit-identical in all six orders); NaN for a non-finite coordinate or corners past the range.
- `dist_line_triangle`: same shadow, doc no longer claims a unit direction.
- `cert_cylinder`, `cert_sphere`: `r − dist`, ≥ 0, NaN unless both finite.
- `cone_chord_sag`: NaN where the slant product overflows (answered 0 under a small sin·cos; found by the sweep, reproduced as a row first).
- Rows: the pin replaced by a degenerate-projection table in every corner order; two seeded fuzz rows; the poison row extended and renamed.

Degenerate projections: side on a ruling → the chord's sagitta (same as its cell's other half, to the bit); three corners on a ruling → 0; ruling + corner across the axis → r; corner on the axis → r; edge-on triangle → r less the nearest shadow point; sphere two corners at a point → chord's sagitta; three at a point → 0; through the centre or a corner at it → r; anything non-finite, a zero axis, out of range → NaN.

Commits (mine, oldest first): cae930634b wip distance + poisoning; 178e99042f rows; c1dbdf424c six-order bit equality in the fuzz; a3bb1cf992 cone overflow; 0102179aa8 filings; 9a96539202 tolerances via `rounding_allowance` (the `bare-rounding-allowlist` gate); c00831c2f9 draw counts; 1f288c0507 doc of what the foot's distance rounds at; plus two merges of origin/main (the cone unit had already landed there, so I merged main, not the cone branch).

Filed: `work/tesslane/sizing-folds-drop-a-nan-before-their-own-guard.md` (new). Evidence appended to `work/chord/nurbs-cell-bounds-cert-drops-a-nan-vertex-and-lands-a-nan-end-on-the-first-cell.md` (the sweep's NURBS hit was already filed there; I created no duplicate), `work/chord/S236.md`, `work/chord/dist-line-triangle-takes-an-unchecked-unit-direction.md`. The unit's item file and `log.md` untouched by me.

## 4. Run locally

All on the change; the last merge of main touched no mesh/tools/demos source (one sweep test file), after which I re-ran fmt, gates, doc-gate, mesh clippy and `-p mesh --lib`.
- `bash scripts/run-gates.sh`: rc 0 (first run red on `bare-rounding-allowlist`, 6 new `f64::EPSILON` in my rows; cured by stating tolerances as `rounding_allowance`, no allowlist edit).
- `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`: rc 0. `cargo fmt --all --check`: clean. `work.py lint`: ok.
- clippy: `-p mesh -p geom-brep --all-targets --all-features`, `-p mesh --all-targets --features budget`, `demos/tour` (fmt + clippy), `-p viewer --features app --all-targets`: all clean.
- `cargo nextest run -p mesh`: 481/481. `cargo test -p mesh --features budget`: 215 + 272 passed.
- tools job: k-lint, tess-lint, tess-meter each fmt + clippy + test, rc 0 (`r2_wedge_bytes` passes unchanged).
- viewer tests: 1589 passed, 3 failed for want of a GPU adapter, which I could not run: `gpu::tests::every_pass_builds_on_a_real_device`, `gpu::tests::the_culled_passes_draw_every_face_that_faces_the_eye`, `drive::driven::a_picture_shows_the_window_with_the_part_drawn`.
- sweep meshing rows (583): all pass. Tour `ci` suite (97): all pass.
- `cert::` rows at `CAD_TOLERANCE_EPS` 1e-6 and 1e-12: 16/16 each; none reads ε, same expectation at each.
- Fuzz rows: 40 random seeds at effort 1 and about 5M draws at effort 30, clean. Timings locally 0.06 s and 0.15–0.2 s, so about 0.1 and 0.4 s hosted: not in the slow set. The cert fuzz is the one nearest the 0.5 s line.
- Mutants: 7, all caught (table in the body).
Left to CI: the full workspace suite outside mesh/sweep-meshing/viewer/tour/tools, sweep's non-meshing rows, wasm, nightly rows, the 3 GPU rows.

## 5. Head

`git ls-remote origin tesslane/cylinder-cert` → 5e933bfff167b56606cbbde5397e2c1256345dfa.

Blinding: none of my commits carries a trailer. The branch's first commit, 870724e184 (the spec, not mine), carries a `Co-Authored-By` and a session line.

## 6. What slowed me

- The first probe sampled every triangle and logged every face; over `sweep` it ran the worker out of memory or disk and the session restarted. Lightened and moved to a separate worktree with its own target dir.
- Disk filled once during the pre-push list (viewer build on top of 12 G of debug output); I freed build output and re-ran the two steps it had cut.
- Four cores shared between probe runs and my own builds.

## Open for a reviewer's eye

- The distance is to rounding for the certificates' own domain (corners on the surface). For a general thin sliver with the foot inside, the foot's distance carries the plane's conditioning, bounded by the inradius; the doc says so and the fuzz's tolerance has that term. Not reachable from an inscribed triangle.
- The corner sort is what makes the answer bit-identical across orders; without it the answers agree only to rounding (mutant caught by the fuzz, not by the table row).

REPORT 2/2 — tesslane/cylinder-cert, head 5e933bfff167b56606cbbde5397e2c1256345dfa. ## 3. The PR body, between the two lines of =====.

=====
# The cylinder and sphere certificates read their true distance on every triangle

Closes `work/tesslane/the-cylinder-certificate-reads-a-ruling-sided-triangle-as-zero.md`.

## The measurement

A probe in both curved lanes (not in this diff) took, for every cylinder and sphere triangle certified, the distance as main computes it in all six corner orders, the distance as this PR computes it, both verdicts, and the triangle's deviation sampled on a 561-point grid against the surface's closed form. It ran with main's certificate deciding, over the `mesh` suite whole, the 583 rows of `sweep` whose files call `tessellate`, every test of `tools/tess-meter`, and the tour's `ci` rows.

**No mesh `tessellate` returns holds a triangle over δ that the certificate read as 0, or read short.** The largest true deviation among the triangles main misread is 0.99980 of its δ (a tour row), and one row built to land a triangle on δ at a coordinate of 900 sits 1.2·10⁻¹⁰ of δ past it, inside the verdict's allowance there under either reading.

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

1. **The sibling is not a safeguard.** Both halves of a lattice cell have a side on a ruling, so both have a shadow of no area; which half read NaN was its corner order. The faces are within δ because the lanes size their steps so (`[mesh.sizing]`), not because a sibling carried the bound.
2. **The misreading is not only a NaN.** On a cylinder whose axis is off the coordinate axes the two shadows of a ruling's ends differ by rounding, the shadow is a sliver with a rounding-sized area, and the interior branch divided by that: 127 emitted triangles read a finite distance too long, a certificate up to 11 % under the true deviation (`sweep`'s `bisector_nodes` and `a_closed_conic_in_the_partners_face` rows). Answering "the nearest side" only where the area is exactly zero would have left these.
3. **The trimmed cylinder arm**, which the row had not measured, holds most of the misread triangles and none over δ.

## The change

`crates/mesh/src/cert.rs` only.

**`dist_point_triangle` is the distance for every triangle.** The nearest point of a triangle is the foot of the point on its plane where that is inside, and on a side otherwise. With `s` the distance to the nearest side and `ρ` the inradius, the distance is in `[s − ρ, s]` either way, and the foot's distance is read inside that bracket:

- a triangle of no area has `ρ = 0` and no foot inside: it answers `s`;
- a triangle thin against the rounding of its own corners, whose normal rounding has left no direction, cannot answer further than its own width from `s`;
- the normal is the cross product of the two shorter sides, each a difference of stored corners, so a needle's is exact to its own rounding;
- the corners are put in one order first, so the answer is the same to the bit in all six orders;
- a coordinate that is not finite, or corners far enough from the point that the fourth power of that distance overflows, answers NaN.

What rounding leaves of the foot's distance is the plane's tilt: its own rounding unless the two shorter sides are near parallel (a sliver with one corner close to the opposite side), where the bracket holds it within the inradius. Corners on a sphere about the point, or a cylinder about the line, never make that case: the foot is then the circumcentre, inside only a triangle with no angle past a right one.

`dist_line_triangle` hands it the triangle's shadow as before, and no longer states a unit direction: `geom::Line::offset` divides by `|d|²`.

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
| any corner, anchor, axis or radius not finite; a zero axis; corners past the range | NaN, which the verdict reads as exceeded |

**The two certificates poison.** `cert_cylinder` and `cert_sphere` are `r − dist`, at least 0, and NaN unless both `r` and the distance are finite.

**`cone_chord_sag`** answers NaN where its slant product overflows. Its doc said so; under a small `sin α·cos α` the numerator stayed finite over an infinite denominator and it answered 0 (slants near 1.4·10¹⁵⁴ at `α = 0.05`). Found by the sweep below.

## What moves

**No face's verdict and no committed byte.** The D9 goldens (`d9_mesh_goldens`), the meter's bytes (`tools/tess-meter`, `r2_wedge_bytes` included), `docs/tess-budget-data` and the tour's pins are unchanged, and every row of the runs above passes with the new certificate deciding. The probe says why: every triangle main read as 0 or short is within its δ at its true value, so nothing refines or refuses that did not. A face's reported worst bound (`CertificateExceeded::bound`) is read only on a face that refused, and no face refuses that did not.

What does change is that the certificate is now a check on those triangles: 689 547 emitted triangles in these runs were passed without one.

## Rows

In `cert::tests`. None reads ε: each expects the same at `CAD_TOLERANCE_EPS` 1e-6, 1e-12 and the default, and passes at all three.

- `a_shadow_of_no_area_certifies_at_its_true_deviation_in_every_corner_order` (was `the_cylinder_and_sphere_certificates_read_a_nan_as_zero`): the table above, each row to 10⁻¹⁵ and bit-identical in all six orders; a cell's two halves agree to the bit.
- `the_two_distances_are_the_distance_on_triangles_built_to_be_degenerate`: a seeded fuzz with the nearest point built first, so the distance is known on both sides: needles, slivers, corners exactly in line, two and three corners at one point, in any pose up to 10⁴ lengths out; the line's direction from a tenth to ten times unit; all six orders to the bit.
- `the_cylinder_and_sphere_certificates_are_the_deviation_on_triangles_built_to_be_degenerate`: a seeded fuzz of both certificates against the deviation found on the triangle and, for the cylinder, its closed form, neither under nor over, within the one rounding allowance the verdict gives a bound at that pose.
- `the_certificates_poison_on_an_input_that_is_not_finite` (was `the_cone_torus_and_spindle_…`): the cylinder's and sphere's corners, anchor, axis and radius, a zero axis, corners past the range, and the cone's overflowed slant product.

Mutants, each run against `cert::` and `sizing::`:

| mutant | reds |
|---|---|
| the old clamp, `(r − dist).max(0.0)` | the poison row |
| one fixed side where the triangle has no area | the corner-order row, both fuzz rows, `sizing::tests::cylinder_rim_triangle_lands_at_delta` |
| the foot's distance read outside the bracket | the distance fuzz |
| the corners not put in one order | the distance fuzz |
| the normal from the two longer sides | the distance fuzz |
| no range guard | the poison row |
| the cone's overflow reading 0 | the poison row |

## The class (implementer-discipline §5)

**Shape:** in `crates/mesh/src` production code, a `.max(`, `.min(`, `f64::max`, `f64::min`, `.clamp(` or a max/min fold, and an accept/refuse comparison, where a NaN operand reads as a number; and a division by something zero on a degenerate input (an area, a cross product's norm, a squared length, a `1/(a+b+c)`).

Fixed here:

- `cert::cert_cylinder`, `cert::cert_sphere`: the clamp.
- `cert::dist_point_triangle`: the division by the area.
- `cert::cone_chord_sag`: an overflow reading 0.

Not this unit, filed:

- `nurbs_cert::NurbsFaceBound::cert`, `NurbsCellGrid::cert` (`min3`/`max3` drop a NaN chart corner, and the crease test can then miss a crease): already `work/chord/nurbs-cell-bounds-cert-drops-a-nan-vertex-and-lands-a-nan-end-on-the-first-cell.md`; evidence added there. Latent: the trimmed lane's corners are CDT vertices and the CDT refuses a NaN coordinate.
- `sizing::sphere_face_widest_cos`, `sizing::cone_face_rho_max` (window folds drop a NaN ahead of their own guard), `sizing::cap_angular` and `sagitta_step` (a NaN takes the cap), `curved::require_swept_rectangle` (a refusal's payload overwrites a NaN): `work/tesslane/sizing-folds-drop-a-nan-before-their-own-guard.md`. None decides a verdict.

Read and sound as they stand:

- `cert::worse`; `cert_torus`, `abs_radial_max`, `abs_sin_max` (a dropped NaN leaves an infinite or NaN extent that poisons the bound); `position_scale`, `uv_scale`, `cone_scale` (the allowance's scale: a dropped NaN makes the verdict stricter); `cert_cone`'s `weights > 0`.
- `cert::verdict`, `tessellate`'s δ check, `sizing::ceil_count`, `chords::tighten`, `validate::check_volume`, the trimmed lane's winding read and refinement gate: a NaN takes the refusing branch in each.
- `nurbs_cert`'s cell sups (cells are refused unless finite where the grid is built), `split_steps`, `lattice_metric`; `curved`'s and `trimmed`'s chart boxes (a NaN entry refuses at the box test or at the CDT); `chords::edge_target`; the allowance payloads.
- Divisions: `dist_origin_segment`, `cert_cone`, `uv_scale` (guarded); `chart_table::lattice_rows`, `chords::tighten`, `sizing::curvature_step`, `ellipse_step`, `torus_grid_steps` (the quotient goes to `ceil_count` or a certificate, which refuses); `chart_table::metric`, `trimmed::inside_point`, `planar::chart_frame` (shape only; the CDT refuses a NaN point).

**What the pattern could not match:** a max written as an `if`; a sort or `partition_point` with a comparator; componentwise `Point3::min/max`; a max or a division inside a callee in another crate; an overflow that reads a finite wrong value with no NaN in it.

**Second pass at that gap:** `if`-selects of two floats (`require_swept_rectangle`, filed above; the trimmed meter's, sticky; `planar::chart_frame`'s farthest point and `walk::unwrap_tie`, placement only); no `partial_cmp`, `max_by` or `min_by` in production, and every `total_cmp` orders and selects no bound; the callees in a deciding position (`geom::Line::offset` divides by `|dir|²` unguarded and its NaN now refuses; `geom::periodic_window_may_hold` answers true on a NaN; `geom_brep::implicit_outward_normal`'s NaN reads as a fold; `geom_brep::harmonic_range` drops a NaN endpoint, in the filed row). The overflow gap is where `cone_chord_sag` was found. Not opened in depth: `Pcurve::chart_box`, `spiric_curvature_sup`, `nonrational_second_derivative_sup`, `accel_upper_bound`, `patch_cells_across_creases`, whose results `mesh` checks finite or certified where it reads them. The sweep is as of main at this branch's last merge.

## Also filed

Evidence on `work/chord/S236.md` (the cylinder certificate had a defect behind its missing falsifier; a standing sample of the mesher's own triangles is still that row) and on `work/chord/dist-line-triangle-takes-an-unchecked-unit-direction.md` (the doc no longer states a unit direction, and a row runs it off unit).
=====
