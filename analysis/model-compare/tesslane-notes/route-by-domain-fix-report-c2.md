Part (c) of 4, piece 2 of 2: the PR body, continued from piece 1.

## No face that meshes on main changes lane

- A face main meshed on the curved lane passed both doors there, and stores no non-iso form (main's sphere test sent a projected image to the trimmed lane; on a cylinder, cone or torus a circle or line with a projected image is off the surface). It routes to the curved lane again.
- A face main sent to the trimmed lane either:
  - stored a non-iso form (a projected sphere circle, a cone's focal section, a torus spiric, a spline's projection) → trimmed by fact 2; or
  - carries an ellipse whose image is harmonic → the door reads the ellipse's kind first, with no decision, and the domain read is the lane's own, so it is trimmed again or refuses with the lane's `DomainUnread`.
- **Armed runs:** `Routes::of` panicked wherever main's kind route said "trimmed" and the new route differed, or main's said "curved" and the doors passed a face the new route sent to the trimmed lane.
  - mesh suite: 484/484 green, no panic. The 97 faces that move from main's curved lane to the trimmed lane are faces the doors refused on main.
  - sweep suite: 3762 of 3764 green, no panic, 0 route differences; the two reds are main's (`mass_props_are_thread_count_invariant`, red on `origin/main` too).
  - tour: 106 of 106, 0 route differences.
- **Byte and K-stream evidence:**
  - The reviewer's eight cells (ball less a slab turned 2–8e-9 rad, x0 0.3 and 0.6) mesh again, to main's bytes (`route_rows::a_section_tilted_within_the_band_meshes_to_mains_bytes`).
  - A nine-body corpus records main's K stream on a run and on a memo hit, and main's bytes (`route_rows::a_body_that_meshes_records_mains_k_stream`).
  - Both rows' constants were read on a worktree of `a56edec9d6` at ε 1e-9, 1e-6 and 1e-12 and agree with this head at all three.
- **D9** (20 bodies × 2 δ): 30 meshing pairs and 6 refusal pairs unmoved; 4 refusal pairs move (keyway, oblique lens). The four hand-built refusal rows now carry their images minted (`witness_bodies::minted`), as a finished body does:
  - The half cap and the bow tie digest to main's constants again (the doors' verdicts).
  - The lens's outer loop is open in the chart, the answer main gives a minted lens.
  - The keyway face meshes on the trimmed lane. The sheet's other face — the U's complement on the unbounded wall, its loop wound against it — refuses `TrimLoopOrientation`, the fixture's winding. This is the one row whose answer differs from main's for the same body.
- **step-import `split_iso_side`:** red on an intermediate head (3a2d0aa0b5), where the route read a harmonic image with nonzero coefficients as moving. Under the oblique placement the rims carry 1e-17 sine terms, so rectangles went to the trimmed lane ([16, 16]). From 2bf2bfe084 only non-iso forms route; back to [28, 64], the armed mesh run's same finding on a posed tube.

## Notched faces per chart

- Cylinder: the pipe bars through and half in, ∪ both orders and pipe ∖ bar, mesh at δ 5e-3, 1e-3, 2e-4 and pass `geometry` (`a_notched_or_holed_pipe_wall_meshes`).
- Sphere: ball ∖ a quarter-turn wedge, from below across and beside the seam and from above through the pole. The faces are decided `props_rim_level`; every op meshes at every δ (`a_notched_sphere_face_meshes_on_the_sphere_row`).
- Cone and torus: no door builds the notch (the boolean refuses `CurvedPierceUnsupported` for a box through either wall).
- Triangles of no area: 0 over the 30 notched bodies at nine δ from 5e-2 to 1e-4.

## Mutants (mesh suite whole each)

| mutant | red | caught by |
|---|---|---|
| a notch sent to the rectangle lane | 6 | both notched rows (walk-consistency `UnsupportedCurvedDomain`), the two keyway rows, D9 ×2 |
| a rectangle sent to the chart lane | 38 | D9, route_rows, the slit, the split-seam donut, … |
| the branch door skipped | 6 | the half cap's three rows, the hemisphere pair, route_rows' K stream, D9 ×2 |
| the image rule removed | 5 | the tilted cells, the K stream, the lens, D9 ×2 |
| the kind pre-pass removed | 1 | `geom-brep` `iso_rectangle_door::a_carrier_of_no_iso_kind_is_decided_before_an_edge_in_band` (no mesh row: in every body built, the ellipse precedes any edge in band) |

## Cost (this container, 4 vCPU, nextest)

- mesh suite, measured 2026-11-17: 184.4 s on main (483 tests), 193.8 s on this head (489 tests); the two notched rows are 5.1 s and 5.6 s of it and sit in the slow set.
- Tess-budget sweep (release, every tour scene, `--sizing-only`): main 256 s and 255 s, branch 274 s and 258 s. The meter's 1678 rows are byte-identical. [These are 2026-11-16 numbers from the first head; the re-measurement on the frozen head will replace this line.]

## §5 sweep

Shape grepped: a face routed by a match on its carriers' or images' kinds, `Curve3::{Ellipse,Nurbs,Spiric}` or `Pcurve::Projected` in `mesh/src`.

| hit | disposition |
|---|---|
| `tessellate::has_ring`, `trimmed::has_trim_carrier`, `has_tilted_circle` | fixed (gone) |
| `memo.rs` `curve3_kind`, its image tag | key encoding |
| `chords.rs` chord counts and speed bounds | sizing per carrier |
| `trimmed.rs` image check | which images the lane reads |
| `sizing.rs` sphere/cone extents | per carrier |
| `chords::tighten` non-circle skip | reads the route |

The pattern cannot see a route spelled without those variants. Second pass on `Lane::`, `route_of` and the door calls: every lane read goes through `Routes`; the doors are called in `iso_rectangle_shape` alone. Beyond `mesh`, `topo::props` routes faces to quadrature by carrier kind (`trims`, `curved_rim`): a route by integrand, whose closed forms read any loop of iso carriers, so not this defect.

## Ratified pages

None re-worded.

## Filed

- `work/conearm/the-branch-door-reads-an-escalated-apex-or-pole-decision-as-one-branch.md`
- `work/tesslane/a-seam-cornered-cylinder-notchs-rectangle-neighbour-exceeds-its-certificate.md`
- `work/ciw/a-chart-branch-guard-row-is-red-locally-and-green-on-the-hosted-kernel-job.md`
- `work/tesslane/a-rectangle-the-walk-refuses-has-no-witness-the-chart-lane-can-read.md`: the walk-refused rectangles; minted, the slit still goes to the walk, and what is missing is a route override.
- A note on `work/tesswalk/apex-crossing-branch-premise-has-no-body-level-witness.md`.
=====
