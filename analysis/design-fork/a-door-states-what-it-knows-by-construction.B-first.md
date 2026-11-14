## For Ev

**Recommendation (likely).** The fault is not in tier 3. The sweep doors work out again, from the 3-D points they have just placed, data that their construction already fixes, and over a box that arithmetic produces width the geometry does not have. The final state has one rule for every door: **each carrier datum (a plane's normal, origin and reference direction, a cylinder's or circle's `u_ref`, a line's direction) is built from the construction's own inputs and moved only by the construction's own rigid motions. It is never re-fitted from the vertices.** The vertices are then certified against the carrier, as they already are. The revolve already works this way: every surface it makes reads one `AxisFrame`, built from the axis the user wrote and turned by the sketch frame (`sweep::revolve::axis`). The extrude and the loft's caps are the exceptions.

**The premise holds, with one thing added.**
- Confirmed in the code: `swept::cap_plane` and `extrude::side_surface`'s line arm call `geom_brep::newell_plane`, which takes a centroid of the corners and sums cross products. The arc arm mints `u_ref: rim.normalize()` with `rim = q − place(centre)`, where both are placed points (sure). The circle verb builds its start vertex as `(cx + r, cy)` (`profile::path::circle_kernel`), so `rim` is a difference of two quantities that carry the same parameters (sure).
- The same mint appears in two more places: the rim circle carriers (`swept::placed_segment_spec`, `Curve3::circle(.., rim.normalize())`), and every line rim and strut (`(q_to − q_from).normalize()`, `w.normalize()`). Check 1 reads circle frames too, so the rim circles probably carry the tour's refusal as well (unsure: the report shows only the face findings).
- The question is "what each door stores, and what the sketch hands it". It is not "what tier 3 asks". Check 1 and check 7 are right to refuse a stored normal tilted by ±0.2. A reader that took midpoints would be unsound, as the probe report says of its own substitution.

**What the extrude knows, and would store (final state, option A):**
- **Caps.** The cap is the sketch plane, moved by `w` for the top cap.
  - normal: ±(the sketch frame's third axis), as `frame.turn(±ẑ)`, which is exact.
  - origin: the placement origin, plus `w` for the top cap.
  - `u_ref`: the frame's first axis.

  A validated outer loop already winds counterclockwise, so the sign is a choice that has already been made. `cap_plane_orientation` and the `CapPlaneError::{Orientation, EdgeOn}` refusals go away, because nothing is left to disagree.
- **Line walls.** The normal is `frame.turn(the segment's unit direction) × ŵ`.
  - On the Distance door, `ŵ = ±frame.turn(ẑ)`. The two directions are an orthonormal pair, so their cross product is exact and needs no normalize.
  - On the Vector door, `ŵ` is decided once from the user's vector.
  - Line rims and struts take the same two directions.
- **Arc walls and rim circles.** `u_ref = frame.turn(the arc's unit start direction in the sketch)`.

**What the sketch must hand the door.** The profile records, for each segment, the unit direction its construction defines, so no door re-derives one from two endpoints:
- a line's director, which the path verbs already hold as `profile::path::Dir` and validation discards;
- an arc's start direction, which the circle verb defines as +x (or the split circle's phase).

`ValidatedSegment::Line` and `Arc2` do not hold these today. This adds no new kind of redundancy: `Arc2` already repeats its endpoints, and the endpoints are certified against the carrier (`arc_reads_back`, `carrier_endpoint_*`). Where the author gave only points (a polyline), the direction is a real function of those points. It is computed once, in the sketch, by a unit map that is tight over a box (next paragraph).

**A tight unit map.** At `Interval`, `v/‖v‖` treats `v` and `‖v‖` as unrelated, so `(W ± δ, 0)` normalizes to roughly `(1 ± 2δ/W, 0)` when the true answer is exactly `(1, 0)`. Over a box that does not contain zero, the range of each component of `v/‖v‖` is exact and cheap to compute: each component is monotone, with extremes only at the box's corners or where another component is zero. Making that the scalar's normalize keeps the `f64` bits and only tightens the box scalars. This piece is unsure. It is a change to the `Real` surface (like `sin_cos` being the primitive), and nothing measured needs it yet. A slab whose width varies would.

**Option B: the box scalar keeps the dependency instead.** `Sym` already knows `z − z ≡ 0` and `(cx + r) − cx ≡ r`, but its numeric channel is `Interval`'s, unchanged (E12). B would narrow each value to its normal form evaluated over the box, plus the tight unit map.
- For: one place, and it covers every door, including blends, which have dozens of `normalize` mints, and doors not yet written.
- Against:
  - it rewrites E12's "numeric channel is `Interval`'s verbatim";
  - it costs a form evaluation per operation;
  - it stops at opaque atoms: `‖v‖ = sqrt(v·v)` stays opaque, and folding `sqrt(R²) = R` was retired as "a sign of the box";
  - the symbolic-off lane gets none of it;
  - and it leaves the doors refitting a plane they already know, which reads a proxy rather than the thing (`[intrinsic.meaning]`).

I lean A as the design and B, at most, as a later backstop for the case A leaves open: a direction between two authored points that share a parameter.

**Option C: change what tier 3 asks.** For example, carriers would store the `UnitVec3` witness so that check 1's length and tilt margins have nothing to read. This is insufficient on its own: the cap's ±0.2 is tilt, not length, and check 7 reads it. Separately, check 1 reads `‖enclosure‖ − 1`. For a direction that really rotates over the box (an angle variable), that reading comes out about δ²/2 from 1 even though every member is unit. Bisection cures it quadratically, so it costs leaves but does not block. That would be its own row, and it would reverse `UnitVec3`'s "What is NOT this type" and `geom`'s at-rest rule. I would not open it here (unsure it is ever needed).

**What A makes true, and what it leaves possible.**
- True: in the slab and the derived-frame cube, every stored normal is exact, so their width is the geometry's own (the cap's origin moves with the depth, which is real). The tour's hole `u_ref` comes out exact, so check 1's margin is ε. The cap code gets shorter.
- Still possible: nothing stops a new door from writing `UnitVec3::new(b − a)` over boxed points. The type can require a witness at door-facing carrier constructors, but it cannot tell a decided normalize of inputs from one of placed points. The rule therefore lives in the sweep README, beside the revolve's frame.
- Reversible: yes, door by door.

**Ratified text.** Nothing ratified blocks A. The `[scalars]` lane clause (Ev's, `[ev]` PR 6912) says a claim "is built at the lane scalar, from its own inputs", which A satisfies more directly than a refit does.
- To change: the `geom_brep::newell` module docs say the sweeps "call this when they create planar caps and side faces … Newell turns vertex data into the certified surface". That is a module doc, not a design page. Under A, Newell remains for data with no construction behind it (the importer's recognizer, test fixtures).
- To add: a short clause in `crates/sweep/README.md` saying that a door's carrier data comes from its inputs and its rigid motions, never from its placed vertices. That clause binds future work, so it is yours to ratify.

**Confidence.**
- A closes the slab and the derived-frame cube: likely. The cap's tilt is what blocks the rays. Even with a constant ray, the root denominator of face 1 (the cap) reads `[-1.357, 0.437]`.
- A closes the tour: likely, inferred, not measured.
- The tight unit map is needed: unsure.

## For the orchestrator

- **Assumed:** the slab's sketch is fixed and only the depth varies (the brief says so; I did not open the fixture). The tour's hole `cy` may be exact. If it is, a tight normalize alone would close the tour without carrying a start direction, but carrying it is still the right final state.
- **Measurements to take on `boxfinish/probe-switch-off`:**
  1. Cap normal = `±frame.turn(ẑ)` with the wall normals left as stored, with and without the aimed-ray fix. The report's `BOXFINISH_PLANE=mid` collapsed every planar normal at once, so it does not separate the cap's tilt from the walls' length.
  2. The tour with `u_ref = frame.turn(+x)` on the hole walls and the rim circles.
  3. A slab whose WIDTH varies, positioned by a variable offset. This would show whether the profile has to carry directions or a tight normalize is enough. Nothing measured covers it.
  4. Check 1 on the hole extrudes' circle edges, which carry the same `u_ref` mint.
- **Scope of a unit for A:**
  - The extrude: caps, line walls, arc walls, line rims, rim circles, struts.
  - The loft's two `cap_plane` calls, which use each section's frame.
  - Profile directions: `ValidatedSegment::Line` and `Arc2` gain a unit direction, fed from `Dir` and the circle verb.
  - Not in scope: the revolve and the tube, which already follow the rule.
- **Off-question:**
  - The blend code (`sweep::blend::{surgery, reach, junction, open::*}`) mints `u_ref`s and normals by `normalize` of differences. These are probably the same class, unmeasured; a P1 inventory row on BOXFINISH or wherever blends are owned.
  - Check 1's δ² reading of a truly rotating unit direction (option C above) is a possible P2 row, unmeasured.
- **Same class as two other rows:** the side-reader row (a ray rebuilt from boxed ends) and the chart-ends row (an exact 0/1 sent through `(1/span)·t1`) are both "a door re-derives what it knows". The README clause above would cover all three.
- **Provenance:** the checkout is shallow. `git log -S` for the `UnitVec3` "What is NOT this type" text and for `geom`'s at-rest rule hits the graft (`6d466484` / `c58a72e3`), so their PRs are unknown. The `[scalars]` lane clause traces to `be9f5573`, "[ev] restate as a [scalars] principle (Ev on 6912)". E12's "numeric channel verbatim" wording is in `editor-core/src/drive.rs`'s header and ERROR-DESIGN; I did not trace its PR.
