1. **The 26 CONSTRUCTION rows (22 dominant, plus rows 12, 18, 21 and 75 as a second label).** By what each does, excluding row 75, which is test support:
   - **Map, 7 rows** (the conic's own door would do it):
     - 4 `starting_at`: the owed re-seam at a point.
     - 26 `half_turn_curve`: an isometry.
     - 30 `column_circle`: `reversed`.
     - 40 `rim_trim_circles`: the owed homothety.
     - 41 `rim_carrier`: a translation.
     - 62 `homothety`.
     - 61 `transport_curve`, its cylinder and cone arms: the owed concentric offset, then a slide.
   - **Record that should hold the `Circle`, 7 rows:**
     - 52 `SphereCutIn`; 66 and 74 `Rim`; 78 `MeetSegment::Arc`; 18 `SphereArc` and 21 `TorusArc`, both second label.
     - 46 `StripLevel` mostly. Its height test is a test on the position, and its wedges, like row 42's, are planes through the arc's own end points.
   - **Mint of a new entity's datum, 5 rows:**
     - 38 `frustum_arm` and 39 `curved_arm`: the surface built about the circle's axis line.
     - 61's torus arm.
     - 51 `pt_germ_frame`: the axis a section loop turns about.
     - 53 `pair_seam`: reads `u_ref` only. It is the seam for the general-anchor row, not `Centre`.
   - **Reporter, 1 row:** 57 `reauthor` writes the user's sketch arc back, which is a description shown to a person.
   - **Should read the conic, 5 rows:**
     - 8 `critical_gap`: it is `cover_over`.
     - 42 `circular_reach` and 46's wedges: planes through the arc's end points.
     - 67 `plane_torus`: which loop is which side is a test on the position.
     - 12 `torus_oblique_circle` (second label): the Villarceau gates are tests on the position, and the pcurve phase is a mint.
   - **Something else, 2 rows: 1 `arc_of_circle` and 28 `mapped_self_description`.** Both build the `RevolvedPoint` scaffold of the same edge about the circle's axis line, and certification then checks that scaffold against the carrier.

   **This changes the caller list, though not what `Centre` is.** "Mint" has to mean a datum of a new entity. A restatement of the conic's own edge does not qualify: on an arc-held circle it is the derived centre used as a position through the back door. So rows 1 and 28 are owed a scaffold built from the conic (its position, or the axis of the surface the edge lies on), not a `Centre` read. "More than a handful" turns out to be maps and records, not a question about what a derived `Centre` means. Confidence: likely. I read the classification, not the bodies.

2. **The three EVALUATION rows that form centre + foreign direction × radius (32, 36, 37)** are the circle's point at another thing's azimuth. Each becomes `point_at` at the parameter of that direction, read as a point-against-conic reading: an azimuth taken of a direction rather than a point (`unit_of`'s job, which takes no position, renamed for it). Row 11's rational-quadratic control points are the arc as a rational quadratic, and should be one conic door built from the stored point.

3. **No agreed item reopens. Every reader is served, with two additions:**
   - Rows 3, 69 and part of 7 read the centre as a rounding magnitude. That is the polynomial's metered magnitude.
   - Row 77 (two circles nested, tangent or crossing) is the third compare question's polynomial: its range and its roots.
   - Rows 54 and 76 are a test on the position and a point read against the conic.
   - Row 24 (`Curve3::data`) is the stored-data walk, which stays.
   - Addition 1: the `RevolvedPoint` scaffold from item 1 is built from the conic.
   - Addition 2: the rational-quadratic door from item 2.
   - Agreed items unchanged: the three compare doors, circle decisions as tests on the position, `u_ref`/`v_ref`, and `Centre`'s two doors.

4. **Changed lines in For Ev:**
   - "Who reads `Centre`: code that reports it (the STEP writer, a sketch written back, a description shown to a person) and code that mints a datum of a new entity from it (a surface about a circle's axis). Never a restatement of the conic itself: an edge's revolved-point scaffold is built from the conic."
   - Add: "A record that carries a circle holds the `Circle`, not its centre, axis and radius."
   - Add: "An arc as a rational quadratic is the conic's own door, built from the point it stores."
