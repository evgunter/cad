## For Ev

**1. Recommendation now: one fixed ray for the whole family (likely). My answer moved.** The other answer's third point is what moved it: whether a ray crosses a planar face is read as the turn of the face's loops seen along the ray. The loop's vertices are dropped onto the plane square to the ray, about the ray's own point, which is exact. I had ruled out the fixed ray because I estimated the tilted cap would still read only to about ±0.1. That estimate assumed the turn is taken about a computed crossing point, which carries the moving height a second time. Seen along the ray, each vertex's enclosure enters once against an exact line. The width that is left is the cap's real motion. My reason against the fixed ray does not stand.

Terms. *Member*: the body at one parameter value. *Fixed ray*: an aimed ray whose aim and direction are plain numbers, independent of the document's variables, read against every member at once. *Embedding certificate*: tier 3′'s proof that no member's boundary meets itself except along its own edges.

**2. The reasons given against my first answer (reading the side at one member and carrying it by the embedding certificate).**
- *The gate holds no member.* Half answered. At `Sym<Interval>` a member can be rebuilt by re-valuing each stored expression at a point. But that instrument exists today only in tests and only at `f64`. At plain `Interval` there is no member at all. The point stands as a real cost.
- *A verdict taken from a point is the lane scalar's.* Answered only in form. The member would be read at `Interval` and the claim built from that reading plus the certificate. But it adds a premise that no reader needs today: a leaf is one continuous family. The fixed ray needs no such premise.
- *The other callers have no embedding certificate.* Stands. Shell roles inside Booleans read bodies no gate has passed.
- *Whether embedding passes over these boxes is unmeasured.* Stands. It is a measurement being taken now.

**3. Rivals or layers: layers, with the fixed ray as the rule.** The fixed ray is sound for each member on its own, at every scalar and for every caller, and it rests on nothing but the stored values. The member reading could only ever be a convenience for the finish alone. It would serve the bodies a fixed ray truly cannot read:
- a wall whose edge moves past the aim (beyond about ±0.19 on the unit slab);
- two faces that move together and stand closer than they move.

None of those stands before the switch's deletion. Adopting the fixed ray makes the member reading unnecessary on every body measured. If it is ever wanted, it can be added later without undoing anything.

**At `f64`:** aims, rays, advances and edge gaps are unchanged bit for bit. Only the turn's arithmetic changes. It decides at ±π by construction and is no tie site.

**Proposed design-doc text:** none. `[scalars]` already makes a value that depends on no document variable a choice, not a claim.

**Confidence.** The fixed ray's soundness: sure. The tilted cap reading at ±0.25 with the turn seen along the ray: likely (not built).

## For the orchestrator

**4. What the measurements would have to show for me to change again.**

*The collapsed-ray run.* Note that the hand collapse still reads today's turn: across the normal, about a computed crossing point. So:
- The tilted brick may still stop on `side_at_infinity_winding` at ±0.25. That stop is expected and is not evidence against the fixed ray. It is the row the along-ray turn must close.
- The collapse could leave a ray stopped by a reading other than the turn while the members' true crossings stay clear of every edge — an edge gap, a span test (`pass.reach`, if it is read from vertices against the line's origin), or an advance pair — with a width beyond the face's real motion. That would mean a stored-against-stored difference survives a fixed line. In that case the fixed ray is not the whole answer, and I would want each such reading named before ranking again.
- The slab's caps should read at ±0.25. Its walls should read to roughly ±0.19 and then stop, where a member's wall really ends below the aim. Anything far short of that, unexplained by motion, moves me back toward reconsidering.

*The embedding run.*
- If tier 3′ refuses over ±0.25 on the slab or on the tilted brick, the finish cannot pass there whatever check 7 does. The member reading buys nothing, and that refusal becomes its own blocker row for the deletion.
- If it passes wide, the member reading stays available as a later layer for the finish alone. It does not change the rule.

The only result that would put me back on the member reading: the fixed ray stopped by unexplained dependency on a measured body, with no re-posing, while embedding passes over the same box.

**5. What each caller of the reader is owed under the fixed ray.** Each is the same reader, on stored values alone, with no premise from any gate:
- **Check 7, inside the finish:** fixed rays, then the volume's sign where no ray reads (the volume stays its own question).
- **Check 10 (shell winding):** the same.
- **The standalone `validate_geometric*` doors:** the same reading, so no door at a box scalar says less than the finish does.
- **Shell roles inside Booleans, the pieces sort, the census role gate and `classify_shells`:** the same fixed ray. Each is owed a side that is true member by member, without any certificate. This is the case the member reading could not serve.
- **Point containment's missed-ray side:** it calls the side at infinity of the selection, so the same fixed ray.

**Rows.** I agree with the other report's re-statement of the three items' closing tests:
- The advance item's test retires.
- The wall test becomes "reads at ±0.25; each ray reads up to the width where a member's crossings truly change".
- The tilted test stands, with its turn row taken along the ray.

The re-posings PR 7480 shipped (`planar_cross_dot`, the step's in-plane clearance) are superseded for the ray's readings. `loop_aim` still uses the in-plane form to choose where to aim.

**Off the question.**
- `a-margin-whose-form-is-a-nonzero-constant-escalates-over-a-box` (the co-moving faces' order) deserves a ref from the deletion row. It is not a blocker.
- The soundness note on "a crossing definitely behind the first counted one": it must mean past the first count by more than the band, since crossings in band of each other are summed. It needs a reviewer.
