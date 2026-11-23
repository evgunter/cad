My earlier worry was that more than a handful of constructions reading a centre would make `Centre`'s meaning the main question. Sorted, the 26 construction rows are mostly maps and records; only 5 legitimately read the centre, so the worry does not hold. Row numbers below are the classification table's.

1. **The 26 construction rows, by what each needs** (each row counted once; 26 in all):
   - **Maps of the circle, 7:**
     - 4 `starting_at`: a re-seam at a point, owed.
     - 26 `half_turn_curve`: `placed`.
     - 30 `column_circle`: `reversed`.
     - 61 `transport_curve`: a concentric offset plus a translation.
     - 62 `homothety`: owed.
     - 40 `rim_trim_circles`: callers scale the trim arcs, a homothety.
     - 41 `rim_carrier`: `frame.with_origin`, a translation, and its record holds the circle.
   - **Records that should hold the `Circle` itself, 7:** 18 `SphereArc`, 21 `TorusArc`, 51 `SectionLoop`, 52 `SphereCutIn`, 66 `Rim`, 74 `Rim`, 78 `MeetSegment::Arc`. The tangents row 18 forms are `derivative_at`.
   - **Mints of a new datum, the legitimate `axis_line()` callers, 4:** 1 and 28 (a `RevolvedPoint` axis), and 38 and 39 (the frustum or torus minted about a circle's axis). The coaxiality decided inside 38 and 39 is a test on the position.
   - **A reporter, 1:** 57 `reauthor`, which writes the user's sketch arc and reads the coordinate door.
   - **Served by the position, 5:**
     - 8: `cover_over`.
     - 12: the chart image, built about the stored point.
     - 42 and 46: each wedge plane is the plane through the arc's end point normal to the end tangent, so `point_at` and `derivative_at`; 46's height test is the position's.
     - 67: a side test.
   - **A direction, 1:** 53, which reads `u_ref` only.
   - **Test support, 1:** 75.

   So `Centre` keeps its meaning and its two doors. Its callers are the STEP writer, row 57 and the four mints. What Ev is asked does not change.

2. **The three evaluation rows that form centre + direction × radius** (32, 36, 37) become a parameter read against the conic, then `point_at`.
   - 36: the parameter is `param_in(p)`.
   - 32 and 37: it is the parameter of a direction, a new direction-only reading that is a sibling of `azimuth` and reads no position.
   - Row 11's corner control points, which lie off the circle, are the conic's own rational-quadratic statement of a span. That belongs on the conic and is stated about the stored point; it is not something a reader rebuilds from the frame.

3. **Nothing reopens an agreed item.**
   - The classification supports the three compare doors: 23 is the content compare; 55, 65, 66 and 77 are the banded or locus questions.
   - It supports circle decisions as tests on the position: all 39 decision-first rows are of that shape.
   - It supports `u_ref` staying for now: rows 32, 33, 53 and 63 read directions only.
   - One reader none of the agreed readings names: 3, 69 and part of 7 use the centre only as a rounding scale. The position's metered magnitude serves them, and that is already agreed for the polynomial; it needs exposing on the position too.
   - Row 24, `Curve3::data`, already walks the stored datum by name.

4. **Changed For Ev lines only:**
   - "A conic made from a conic is a map on the conic: the existing ones, plus a homothety, a concentric offset and a re-seam at a point."
   - "A record that carries a circle's centre, axis and radius (`Rim`, `MeetSegment::Arc`, `SphereCutIn`, `SphereArc`, `TorusArc`, `SectionLoop`) holds the `Circle` instead."
   - "`Centre` is read by the STEP writer, the sketch re-author, and the mints of a surface or revolved point about a circle's axis: six callers."
   - "The circle's point at another geometry's direction is `point_at` at that direction's parameter, and a span's rational-quadratic control points are the conic's own statement."
