# Your review: PR 6519 (`splice/teapot-1e6-floor`), a single FULL review

Frozen head: `9aba71aee14c162601b21b717866b4e6eb2d44ca`. Item: `work/splice/teapot-wall-3-at-1e-6-floors-the-meridian-against-a-spout-wall.md` (P0). Evidence branch: `splice/review-6519`.

At ε 1e-6, teapot wall 3 stopped at `SplineCrossingUnsettled { Floor }`. The spline-face walk (`geom_brep::curve_face::spline_face::isolate`) reads a window END_REACH·escalate/pace past each end of the span it is asked about, 10.2 mm of arc at 1e-6, and it refused on a floor box 4.47 mm past the span's end, where the meridian crosses the bore wall's seam edge. The window scales with ε and the floor with √ε, so only the 1e-6 row's window held the crossing. The PR settles a floor box (no meeting, no refusal) only when two things hold. Its curve-parameter range lies wholly past an end. And the curve over it is decided, through the band door, farther than the band from the end's point (`Pair::past_an_end`, K name `spline_face_past_the_end`). Every other floor box refuses as before. Teapot wall 3 at 1e-6 now reaches `GermFrameUnsupported { 6v1 Sphere, 3v3 Nurbs }` like the other rows.

Claims to falsify:
1. **Settling is sound.** A settled box can hide none of the span's answers. Attack it:
   - Can a box wholly past an end, and decided beyond the band of the end's point, still hold a meeting the caller needs? For example, a meeting the span's end vertex is responsible for, or one that a later split or join of the span would bring inside.
   - Is "decided farther than the band from the end's point" the right test? A curve can pass far from the end's POINT yet near the face's boundary there. What exactly is compared with what?
   - Does `past_an_end` read the curve's parameter range against the span's ends correctly for both ends, for reversed spans, and for a closed carrier whose window wraps the seam?
2. **The crossing is read by the piece that holds it.** The PR's trace shows continuation piece 21v1's boundary pass meeting it at t = 0.343435 and splitting there. Check this is general, not a coincidence of this fixture. If the meeting past the end lies in the INTERIOR of the next piece's face, not on its chart edge, does the next piece's walk find it? Is there any configuration where a span's past-end box settles and NO piece reads the meeting?
3. **ε-scaling.**
   - Do the rows' offsets scale with the band, so the witness is red without the fix at every ε?
   - The third row says a floor box that straddles an end still refuses; that residue is filed at P3. Is that residue reachable by a demo today?
   - Does the new rule change any built result anywhere (a stop moving from a refusal to a build), and is each such build right? Run the teapot, the lily and the klein at 1e-6.
4. **The rows can go red.** Run mutants: drop the "wholly past" condition; drop the band decision, or compare raw; flip which end. Each must turn a row red.
5. **The sibling sweep is honest.** The dispositions for `composite::crossings`, `plane_crossing` and `circle_torus` (the last added to HONEROOT's row) should hold. Look for other curve × face arms that read past a span's end by a window: the circle and ellipse arms, and `spline_face` against analytic faces.
6. **Inherited reds.** The lane names these slow-set failures as inherited from main:
   - sweep: `a_lofted_arc_wall_is_a_function_of_the_sections_bulge`; `the_seam_split_census_reads_no_side_inside_out` (`Unmet { stop: Floor }`); `the_walk_matches_the_serial_golden_at_four_threads`;
   - topo: `every_door_that_reads_a_box_is_inventoried`; `a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void`;
   - geom-brep: `span_reach_hunt` at seed 0x7236d51c917fecd0.
   Reproduce `the_seam_split_census_reads_no_side_inside_out` on main before accepting it. It is a `Floor` stop, so check it is not this PR's class, and that this PR does not change its outcome either way.
