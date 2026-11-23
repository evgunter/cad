**Rows 1 and 28: I move. They are not legitimate `Centre` reads.** The caller list reads "reporters and mints of a new entity's datum"; a conic's own edge description is not on it. What I read this round, which the other two should weigh:

- **The description is a centre-form restatement of the edge itself.** `MappedSource::RevolvedPoint::eval` is `Affine3::rotate_point_about_axis`, spelled `q + R·(p − q)` with `q` the axis origin. Fed from the circle's centre, the centre is literally a term of the edge's points. My round-5 label "reporter" was wrong for it.
- **What it is checked against.** Certification compares `carrier(t)` with `description.eval(s)` sample by sample (the `mapped.rs` module doc). Both rows synthesize the description from the carrier; `arc_of_circle`'s doc calls it the pushforward "about the carrier's own axis". So the check meters one statement of the circle against another statement of the same circle.
- **At what scale on an arc-held circle.** The carrier's points hold at the arc's scale. The description's round at `u·r` whatever supplies the axis, because `q + R·(p − q)` sums from the far axis. So the residual reads the description's own rounding, about 1e-10 m at `r` = 1e6 (my arithmetic, not measured).
- **Final state.** A `RevolvedPoint`'s axis is the revolution's datum, not the circle's.
  - Row 28: the axis is that of the surface the rim lies on, which `step_rim_on_wall` has already held the rim to. The check then compares two entities.
  - Row 1: a transient scaffold has no other entity, so its description is built from the conic and evaluates as the conic does.
- **A defect beside it, in `mapped.rs` and not in the conic interface.** A `RevolvedPoint` is a point of the arc and an axis, which is already the arc-held shape. Evaluated as the start point plus the rotation's displacement, `p + (R − I)·(p − q)`, the axis origin enters at the arc's scale. Evaluated as today, it does not. That belongs on a row for the unit that first holds a circle on its arc.
- **Why row 57 stays a reporter.** `Arc2 {centre, radius, sweep}` is the profile's own arc form, and its centre is the arc's by the format's definition, as STEP's placement is. `RevolvedPoint`'s axis is another entity's datum. That gives the rule: a reporter writes a format that is centre-form by its own definition; anything else restating the conic is built from the conic.

`Centre`'s callers are then: the STEP writer, the sketch re-author (57), the two blend surface mints (38, 39) and the section loop's turning axis (51).

**The three refinements** are mine; I sign (a), (b) and (c) as you state them.

**The two additions:**
- **Rational-quadratic control points:** signed as one door on the conic, built from the point it stores. My `point + derivative·tan(h/2)` is how that door computes the corner inside; it is not something a reader writes.
- **The circle's point at another geometry's direction:** signed. The parameter of a direction is a direction-only reading beside `azimuth` (`unit_of`'s job, returning an angle and reading no position), then `point_at`.

If we still differ on rows 1 and 28, the deciding fact is whether anything downstream reads a `RevolvedPoint`'s axis as the circle's own axis rather than the surface's. I did not trace the pcurve derivations that consume the description class.