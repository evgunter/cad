## For Ev

**Recommendation.** One rule (`sure`): **every direction or frame a carrier stores is built from its construction's own inputs and moved only by that construction's rigid motions. It is never re-fitted from the vertices the construction placed. The chain starts where the direction is first defined, which for a profile segment is the sketch verb that drew it.** This is your `[scalars]` sentence from PR 6912 ("a claim (a frame…) is built at the lane scalar, from its own inputs") and `[edges]`' "What is true by construction is not measured", applied to faces and edge carriers. Three pieces follow, in this order:

1. **The sweep doors keep the sketch frame they are handed.** Needs no ruling and can be built now. Closes the slab and the derived-frame cube. `sure` it is right, `likely` it closes them.
2. **The profile records each segment's unit directions.** Needs your sign-off: it changes the ratified "Profile format" clause. Closes the tour's certified stop. `likely`
3. **Separately: what check 1 may ask of a frame that truly turns over the box.** Nothing this program measured needs it. I lean to a carrier storing the witness, and recommend it gets its own row and one measurement before it comes to you. `likely`

Terms. *Box scalar*: `Interval` or `Sym`, a number held as a bracket over the parameter box. *Carrier*: a stored `Surface` or `Curve3`. *Door*: an operation that returns a body. *Witness*: a type whose only constructors establish a fact, here `UnitVec3` (a unit direction) and `RigidMotion` (an origin and an orthonormal frame). *Check 1*: tier 3's first check, which among other things reads each stored frame's length and squareness back from its numbers. *Newell*: fitting a plane to a loop's vertices. *M1*: this program's measurement with the switch off. *Fork N*: row N of the design-fork log.

### Premise check

- **It is one act, not three data.** `extrude` is handed `profile.plane()`, a `RigidMotion` witness. Its first statements take `.placement()` and `.normal().get()`, which drop it. Everything after rebuilds directions from placed points: `newell_plane` for caps and line walls, `rim.normalize()` for the cylinder's `u_ref` and each rim circle's, `w.normalize()` for struts, `(q_to − q_from).normalize()` for line rims. `sure` (read)
- **It is not one door.** The revolve's surfaces already read one frame (`AxisFrame`). But the partial revolve's caps and the loft's go through the same `cap_plane`, and the revolve's latitude circles through the same `rim.normalize()`. `sure` (read)
- **The tour's width is also spurious, and its source is one level up.** The circle verb builds its two vertices as (cx ± r, cy) (`profile::path::circle_kernel`), so each arc's start direction is exactly ±x by construction. The profile stores centre, radius and sweep and drops that direction; the door recovers it as `place(cx + r, cy) − place(cx, cy)` divided by its own length. `sure` (read)
- **A frame that truly turns over the box is a different case.** It is not what the tour measured. It is on record as fork 259's case (a): a cylinder moved by a boxed motion refuses "axis not unit length" and finishes only at a box half-width of 1e-11.

### Piece 1: the sweep doors keep the sketch frame

- **Caps.** A cap is the sketch plane's own frame, moved by the sweep: normal ± the sketch normal with the sign read from the profile's winding, origin the sketch origin plus the sweep, reference direction the frame's first axis. `newell_plane` and the `cap_plane_orientation` decision lose their production callers. One function (`swept::cap_plane`) serves the extrude, the loft and the partial revolve, so all three change together.
- **Walls, rims, struts.** Each direction is the sketch frame turning a 2-D direction decided once in the sketch, as the revolve's `AxisFrame::build` already does ("a rigid motion carries a unit vector to a unit vector, so nothing is decided in space"). A depth extrusion's strut runs along ± the sketch normal. A vector extrusion's sweep direction is decided once from the vector the user wrote.
- **Makes true:** the slab's top cap normal is (0, 0, 1) at every depth and its walls' normals are exact, at `f64` as well as over a box. The only width left in a cap is its depth's own.
- **Leaves possible:** an arc whose centre and endpoints carry the box still gets a `u_ref` that is a quotient (piece 2). A new door can still normalize placed points; no type can tell that from normalizing inputs, so the rule needs a written home (below).
- **Reversible:** yes, door by door.

### Piece 2: the profile records each segment's unit directions

- A line records its director; an arc records its unit radial at each end, so reversing a loop swaps them and rotates nothing. The path verbs already hold these (`profile::path::Dir`, kept exact across every hop) and validation discards them. For a bulge arc both radials are rational in the bulge and the chord's unit direction (my derivation, not in the code). Where the author gave only points, the direction is decided once, in the sketch.
- The door's `u_ref` is then the sketch frame turning the recorded direction.
- **Makes true:** the tour's hole `u_ref` is the frame's first axis, exactly, and check 1's margin is ε. The rim identities the sweep registers by hand today (`register_rigidity`, `register_rim_identity`) become plain polynomial identities, since no square root is left in them (inferred).
- **Leaves possible:** a direction between two authored points that share a variable keeps a quotient's width. Whether that matters is unmeasured.
- **Ratified text:** `crates/profile/README.md`'s "Profile format" says a segment is "a carrier plus a signed interval on it" with vertices authoritative, and that every redundancy is "checked at validate … or holds by construction". The recorded direction is one more such redundancy and takes the same rule.
- **Reversible:** yes, but it is a stored-form change every profile consumer sees.

### Piece 3 (separate): a frame that truly turns over the box

- **The case.** A hole of radius 5 mm on a sketch plane turned by θ = 30° ± 1°. The stored direction is (cos θ, sin θ, 0) with two independent brackets. Since cos²(θ − δ) + sin²(θ + δ) = 1 + sin 2θ · sin 2δ, its length reads 1 ± δ·sin 2θ: first order in the box, so [0.985, 1.015]. Check 1's margin is ±7.6e-5 m against ε = 1e-9 and decides only below about 1e-5 of a degree. Every frame in the family is exactly unit. Only where the nominal direction is axis-aligned in the computing frame is the excess second order.
- **Why it is a question.** `RigidMotion`'s docs since PR 5068: "nothing re-derives rigidity from the entries … at a box scalar no door re-derives a motion's rigidity from its entries". Check 1 does that to the same numbers once they sit in a carrier.
- **Three final states.** (a) A carrier stores the witness, as `SketchPlane` does, and check 1's frame margins go. A plane's normal, a line's direction and a cone's frame, which nothing certifies today while readers take them as unit (open row `unlevered-frame-conventions-are-uncertified-at-rest`), become unit by type. It gives up the `f64` at-rest measurement of a wrong mint, and a carrier's structural lift between scalars would have to carry the witness. (b) Check 1 asks through the decision funnel, so `Sym` discharges the identity where its rules reach; small, but certification then rests on the symbolic budget. (c) Leave it: such documents certify at tier 3 only at boxes near ε.
- **My lean is (a).** With the switch on, a document of sweeps alone on a turned sketch plane certifies at tier 2 today; with the switch deleted it meets this check (inferred). So it bears on the deletion even though no row measured it.

### Not recommended

- **Repair the three sites in the extrude.** Leaves the other caps, every rim circle and the profile's dropped directions.
- **Have `Sym` narrow each value from its expression.** It compensates downstream for a datum the door should not have widened, rewrites E12's "numeric channel is `Interval`'s, verbatim", and stops at the square root inside every normalize.
- **Tighten the scalar's normalize now.** Over a box, v/‖v‖ of (W ± δ, 0) reads (1 ± 2δ/W, 0) where the truth is (1, 0), and the exact range is cheap. It is right in itself, but after pieces 1 and 2 nothing measured needs it. `unsure`

### Text to change or add

- **The rule's home.** One clause, yours to ratify because it binds future doors. The same rule covers the side reader's ray and the chart ends (both "a reader or door re-derives what it holds"), which live in `topo`, so I lean to a sentence in `docs/DESIGN.md` beside `[edges]`' "What is true by construction is not measured", with `crates/sweep/README.md` stating the sweep's instance.
- `crates/profile/README.md`, "Profile format": gains the recorded directions (piece 2).
- `geom_brep::newell`'s module doc says the sweeps call it for caps and side faces (agent text): re-worded with piece 1.
- Piece 3 (a) would replace `geom`'s "conventional data, unchecked here" paragraph and the "What is NOT this type" paragraphs of `UnitVec3` and `RigidMotion`. Provenance: check 1's frame margins are PR 3238, an agent unit merged with no question to you; the "NOT this type" text is from PR 5068, where your words were about crossings and never about carriers. `git log -S` ends at the shallow checkout's graft, so I read both PRs.

### Confidence

- The rule, and piece 1 as its first instance: `sure`.
- Pieces 1 and 2 with the aimed-ray row close the slab, the derived-frame cube and the tour: `likely`, each unmeasured at the mint.
- Piece 3's width is first order and blocks: `sure` of the arithmetic, `likely` of the consequence (fork 259 measured it on a moved cylinder; no sweep-door case is measured).
- (a) over (b): `likely`.

## For the orchestrator

**1. What moved me, and what I now recommend that differs.**
- **The tour closes under the first change once the arc's start direction is data; it does not need my change 2.** I checked `circle_kernel`: the vertices are `(center.x ± radius, center.y)`, and `circle_loop`'s doc says "each vertex is the centre plus the radius along a unit direction". The hole's `u_ref` is a constant direction with a spurious quotient's width, not a turning frame. My first report filed the tour under "a claim read back from numbers" and said a fix at the door could not reach it. That was wrong for the tour: the fix is one level above the door.
- **The profile carrying directions moves from "candidate, unmeasured" to part of the recommendation.** It is the same rule, started where the direction is first known.
- **Change 2 is demoted** from a recommendation in this question to a separate row with its own measurement. I still lean to it.
- **`Extrusion::Vector`:** I withdraw "needs `work/strut/extrude-arc-walls-are-ruled-in-n-not-w.md` settled". The sweep direction decided once from the user's vector, and each line wall's normal decided perpendicular to it, keeps today's geometry and needs no ruling.
- **A written home for the rule:** adopted. I differ only on where (below).

**2. My arguments the other report touches.**
- *"Check 1's length reading fails for any frame wide over the box."* The other report answers that the excess is about δ²/2 and "costs leaves but does not block". **Not answered.** The identity in piece 3 gives δ·sin 2θ, first order; δ²/2 holds only at θ a multiple of 90° in the computing frame. Fork 259's row records the measured margin as "exactly [0, 2h]", finishing at h ≤ 1e-11. The argument stands, with its scope cut down: it is not the tour.
- *"Two descriptions of one frame; readers take unchecked fields as unit."* Not addressed there beyond "would not open it here". Stands, as the case for piece 3 (a) on its own row.
- *"It is not one door."* The other report puts the revolve and the tube out of scope as already following the rule. Half right: the revolve's surfaces do. `revolve/partial.rs` calls `cap_plane` twice, and `revolve/surfaces.rs`, `full.rs` and `turn.rs` mint latitude circles with `rim.normalize()`. Fixing `cap_plane` covers the partial revolve for free.
- *"Newell goes."* The other report keeps it for the importer's recognizer. `step-import/src/recognize.rs` has its own cross-sum and does not call `newell_plane`; after piece 1 only `topo::test_support_fixtures` calls it. Minor.
- *"Nothing ratified blocks A."* True of the doors. The profile half touches the ratified "Profile format" clause, which the other report does not mention. I did not trace that bullet's own provenance (shallow checkout).
- *"A type cannot tell a decided normalize of inputs from one of placed points."* Agreed; my first report said the same under "leaves possible".

**3. Where we still differ, and the one question beneath it.**
Does check 1's unit-length reading of a truly turning frame **block** (first order) or only **cost leaves** (second order)? One measurement settles it: on the probe branch, extrude a circle on a sketch plane turned by 30° ± 1° (a nominal that is not axis-aligned), read check 1's root margin and the width at which it decides. If it is first order, deleting the switch regresses every variable-angle document with a round feature, and the ruling needed is only whether that is a deletion blocker or its own row. The home of the rule's clause (DESIGN or the sweep README) is a smaller difference, Ev's to pick.

**Scope of a unit a lane can build now (piece 1, no ruling).**
- `swept::cap_plane` takes the cap's frame: extrude (2 calls), loft (2), partial revolve (2).
- `extrude::side_surface` (line and arc arms), `swept::placed_segment_spec` (line and arc rims; shared with the revolve's meridian edges), `strut_carrier`: each direction is `plane.frame().turn(…)` of a 2-D direction decided once in the sketch from the segment as stored today.
- Distance arm: sweep direction ± the sketch normal, exact. Vector arm: decided once from the vector.
- Revolve latitude circles: at the start station `rim` lies along `frame.u3`, so `u_ref` is ± `u3` (`likely`, not traced for sign); the end station's is that turned by θ.
- Out: the profile (piece 2, after Ev), blends (inventory row, as the other report says), check 1.
- This unit closes the slab and the derived-frame cube and leaves the tour's stop red until piece 2. Say so in the unit's spec so the tour row is not expected of it.

**Piece 2's open sub-design, for its spec:** one direction per arc or one per end (I say per end, for reversal); on `Arc2` or on the segment (`Arc2` is shared with `geom-brep`'s sketch segment); what a table-authored profile records (decided at validation); how `ValidatedSegment::lift` carries it.

**Measurements (probe branch, none run).** The other report's four are right and I adopt them: cap normal alone from the frame, with and without the aimed ray; the tour with `u_ref = frame.turn(±x)` set by hand on walls and rim circles; a slab whose width varies; check 1 on the hole's circle edges. Add the turned-plane circle above, and `cut_plate` on main at a macroscopic box (inferred: `topo::take` runs the same check 1 with the switch on).

**Unchanged from my first report:** I ran nothing; provenance searches end at grafts; M1's `BOXFINISH_PLANE=mid` was read-side and left the plane's origin at Newell's centroid, which piece 1 moves.
