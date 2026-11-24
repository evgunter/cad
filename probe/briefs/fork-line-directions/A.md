DESIGNER REPORT — line directions (work/boxfinish/a-door-reading-the-minted-axis-loses-identities-with-quotient-minted-partners)

## For Ev

**Recommendation.** Two changes, neither of which is a rule for "which spelling a door reads":

1. **A validated line side records its direction and its extent together**, as an arc already records its radius beside its radials: `Line { director, extent }`, meaning `end = start + extent · director`. The pair is made once, where the direction is made, and every reader (the sweep doors, the edge's own description, the verbs' junction checks) reads that pair and never the two points.
2. **The reading that asks whether two planes are one plane uses both normals evenly.** Today it decides the orientation (same or opposite facing), then computes `d1 − σ·d2` with `σ = ±1` that decided sign; it becomes `d1 − (n1·n2)·d2`, and its angle term becomes `‖n1 × n2‖` (which the same ladder already decides one rung earlier).

No change to the symbolic tier, nothing registered, no design-doc text needed. Confidence: **likely** on the whole; the second change is measured, the first is partly measured (below).

**Terms.** *Director*: the unit vector along a line side. *Extent*: the signed length along the director from the side's start to its end. *Quotient*: a direction written as chord ÷ chord's length. *Minted axis*: the exact `(±1, 0)` or `(0, ±1)` validation writes instead of the quotient when one component of the chord is zero as an expression. *Symbolic tier*: the expression algebra that proves a margin zero at any box width (ERROR-DESIGN E12). *Seat*: two members' faces lying on one plane.

**Premise check: the framing has two things under one name, and each is a missing piece rather than a spelling.**

- *"The span folds only through a square-root rule" is evidence that the extent is stored nowhere.* The record holds a direction; the length is re-derived by each reader, and each re-derivation is a new spelling: the chord's norm (`√((2 − r)²)`, which is `|2 − r|` and does not fold to `2 − r`), or the chord projected on the director (which needs `√X² = X`). I count four re-derivations of one line: the sweep's chord reader, the frame door (which normalizes again), the edge's description (`SketchSegment::Line { a, b }`, two points and no direction), and the directed-leg verb, which is handed a length, builds the end point from it, then takes the square root of the squared chord to get the length back. The direction was moved into the record; the extent was left behind.
- *"An axis beside a quotient loses the identity" is evidence about the reader, not the walls.* The plane reading multiplies the second normal by the decided ±1. That is an identity of the two expressions only if both normals happen to carry the same sign spelling. They did, by accident, while every wall was a quotient (both carried `(ro − ri)/|ro − ri|`, which cancelled). `d1 − (n1·n2)·d2` is the same real number to second order in the tilt, means "plane 1's distance from the centre, minus plane 2's distance projected on plane 1's normal", and is zero as an expression whichever way each normal is spelled, because each normal enters it twice.
- So the walls were never proved one plane "whichever sides they were built from": today two quotient walls whose chords differ by a non-constant factor also fail, for the same reason. The even reading proves those too.

**The final state.**

- *Stored, and by whom.* The profile owns the pair. A verb that states a side by direction and length (a directed leg, a rectangle's sides) records the pair it was given and derives the end point from it. A side stated by its end point (a hop) has the pair minted from the chord at one site: `(axis, ±component)` where a component is zero as an expression, with the ± a literal as today; `(chord/‖chord‖, ‖chord‖)` otherwise. In both, `start + extent·director` equals the end as an expression with no rule: `σ·σ = 1` for the axis, a plain cancel for the quotient.
- *Derived.* Everything else: a wall's normal and reference direction (the station's frame turned onto the director, no division), a rim's carrier and span (`extent`, read), a wall's middle (`start + extent/2 · director`), the edge's description (`start + t·director`, `t` in `[0, extent]`).
- *At each scalar.* `f64`: the pair to rounding. `Interval`: enclosures of it; with no expression to ask, a hop's director is the quotient and is wider than the geometry, and a verb-recorded one is exact. `Dual`: the same formulas, tangents included. Symbolic: the closed spelling where the expression allows, and the identity above in every case.
- *A side whose record a door cannot use:* there is none. Every validated line has the pair; a door has no fallback to the points. A run of collinear sides joined into one carries its first member's director and the sum of the extents, as an arc run carries its first centre and summed sweep.
- *A direction a test table wrote:* a table carries none. The table door that writes directions is test-only, and the production check that admits its directions within a band (with its own resolution gate and two refusal kinds) exists to serve it ([simple.checks]). Recorded pairs live only in a constructed loop, held by its type; a loop that "gives up its provenance" gives up its recorded directions too and has them minted. This departs from the agreed unit-direction work, which kept that door.

**Worked example** (the ring of wedges). Near side `(ri,0)→(ro,0)`: pair `((1,0), ro − ri)`. Far side between `(ro·c, ro·s)` and `(ri·c, ri·s)`: director `S·(c,s)` with `S = (ri − ro)/|ri − ro|`, extent `|ri − ro|`. The next member's near wall, spun by the step, has normal `(s, −c)`; this member's far wall has `S·(s, −c)`, facing the other way since `S` is −1 in value. Old reading, with the decided `σ = −1`: `(s,−c)·(o1 − ctr) + S·(s,−c)·(o2 − ctr)`, zero only if `S ≡ −1`, a sign the tier does not hold. New reading: `n1·n2 = S`, and `S·S = 1` is algebra, so it is `(s,−c)·(o1 − o2) ≡ 0`.

**Measured** (a local probe on main, not kept): with the doors reading the record and main's plane reading, `a_ring_stepped_by_an_exact_turn…` fails with exactly the item's five unproven seats. With the even reading, all seven ring rows pass. With the extent supplied as above, the sweep's two line-span rows pass under no rule, and the "two blocks on one floor" row keeps its pinned rungs.

**What it leaves.**
- A slanted hop between two authored points whose direction does not in fact move (the wedge's far side) is still stored as a quotient, with the box's width in its values. Only a verb that states the direction, or a mint that could factor a chord's common factor, removes that. I would not build the factoring now. **likely**
- The even reading builds larger expressions: two of the floor row's already-unproven rungs change their reason from "one construction would prove it" to "budget". No proof is lost in what I ran. **sure** (measured)
- One drive receipt (`sym_9`) reads 1349 symbolic decisions against main's 1369 with my stand-in; the item's version read 62 fewer. My stand-in did not convert the edge's description, which still spells the line from its two points; I expect the remainder there. **unsure**

**Alternatives.**
- *Teach the tier that `|x| = sgn(x)·x`* and mint the axis as `sgn(component)`. True at every value, keeps every identity today's quotients cancel, removes the unlogged sign decision in the mint. But it changes E12's algebra, and two axis walls from different sides (`sgn(w1)` beside `sgn(w2)`) still need the even reading. Once the reading is even, this adds nothing to identities. Reversible; I would not do it.
- *Spell every direction with a literal sign,* quotients included, by factoring each chord's common factor. This is E12's "the construction spells the decision as a literal sign" applied fully, and it would also give the wedge's far side an exact value. It needs polynomial factoring inside the structural question and still cannot cover a chord with no common factor, so the even reading is needed regardless.
- *Doors keep reading the points.* Nothing is lost that passes today; a fixed wall keeps the box's width, and the record stays unread.

**Ratified text.** None changed. If a sentence is wanted, in E12 beside "the construction spells that decision as a literal sign": "A reading that compares two directions takes each through a product of the two (a cosine, a cross product), never the decided sign times one of them."

## For the orchestrator

**Probe (local worktree, removed; nothing pushed).** On main `36cc57d5ea`:
- Mutant as the item states it (`sketch_vector` line arm → `director`; `sketch_chord` → director and a span).
- Reader change: `geom-brep/src/plane_eq.rs` `plane_ladder`'s `offset_margin` → `d1 − p1.normal.dot(p2.normal)·d2`; `carrier_eq.rs` `reading`'s plane arm → the same offset and `n1.cross(n2).norm()` for the swing.
- Extent stand-in in `sketch_chord`: `director.x·w.x` where `director.y`'s expression proves zero, `director.y·w.y` where `director.x`'s does, else `w.norm()`.

Results, `cargo nextest run -p editor-core --test all … --ignore-default-filter`:
- Mutant + reader change: `g_ring_closure_measure` 7/7 pass. Control (reader change reverted): `a_ring_stepped_by_an_exact_turn…` red, `N = 5: [(0,1),(0,4),(1,2),(2,3),(3,4)]`.
- `coincidence_door` (of the 56 ring and door rows run with everything applied, 53 pass). `two_blocks_resting_on_one_floor…`: passes in the control (so the item's three-rung move is gone with the extent stand-in); with the reader change rungs 4 and 5 read `unproven/Numeric/Budget` for `OneConstruction`. `two_copies_on_one_axis…`: eight `Meet VertexFusion` rows read `OneConstruction` where the pin says `Budget`, in the control too (the record's doing, not the reader's).
- `a_boss_typed_tangent_to_a_cylinder_glues_one_unproven_tangency` is red with and without the reader change: its union refuses at `Interval`, `section_site_foot_bend` Invalid, with `curve_pair_pose_parallel` at `[-1.0000000050e-8, -9.99999994e-9]` against an escalate band of `1e-8`. It is not in the item's list. It looks like a fixture sitting on the band's edge moved by last bits; not traced.
- `sym_9_the_drive_writes_the_ladders_receipt`: `symbolic 1349, numeric 1813, retried 14, frozen 214`, identical with and without the reader change, and identical with a non-normalizing frame door for the line wall (so the double normalization is not the cause). I did not run main's own row for `numeric` and `retried`, so I cannot say how many of the 20 are proofs lost and how many are questions no longer asked.
- `sweep --lib`: `the_placed_lines_far_end_folds…` and `a_placed_lines_span_is_its_segments_length…` pass.
- Not run: the `1e-12` sweep rows, the pins and digests in the item's list (ii), the slab and driven-leaf closing tests in git history, the tour.

**Errors or gaps in the brief.**
- The brief says no door reads the record and frames the question as which spelling a door reads. The plane ladder is the piece the brief does not name: the loss in (i) is located in `plane_ladder`'s offset and `carrier_eq::reading`, both `σ·n2`.
- The item's (ii) attributes the rows to the projection span alone. With a non-projected extent some remain (`sym_9`), so (ii) has at least one more reader in it.

**Other readers I saw and did not check.** `geom-brep/src/certify.rs` near `gap_orientation` (`r1 − sigma·r2`, tangent gaps); whatever reads two line carriers as one line (the edge-pair counterpart of the plane ladder); `SketchSegment::Line { a, b }.eval`; `profile/src/path.rs` `emit_straight_leg_at` (`(end − head).norm_squared().sqrt()` for a length its caller held); revolve's and loft's line arms (no `direction_and_length` there by grep, not read further).

**Where this meets the parked rows.**
- `a-recorded-direction-claims-a-unit…`: the pair fits its makers (the exact axis with its signed component; `normalized` returning the length it divided by). My table recommendation contradicts its "the test-only table door survives", which was the orchestrator's choice between two leanings; I would reopen that one point. `ConstructedProfile::into_profile` documents "giving up the provenance" and keeps the directions, which is the inconsistency.
- `a-constructed-loop-keeps-its-directions-beside-its-segments`: same type move, now carrying the pair.
- `fillet-side-and-centre-arcs-and-off-axis-hops…`: its off-axis hop half is what my "what it leaves" bullet says stays.
- `a-hop-whose-axis-changes-sign-inside-the-box…`: untouched; the literal sign and its unlogged reading stay as they are.
- The signed-radius branch leaves `sketch_vector`'s line arm on the points and adds the non-normalizing frame door, so the two compose.

**Assumed, not verified.** That the even offset is well-conditioned in band (it differs from `d1 − σ·d2` by `(σ − n1·n2)·d2`, second order in a tilt the parallel rung has already bounded at the extent's arm); that the Boolean's glue goes through the same `plane_ladder`, so a ring's union over a box has the same exposure as the coincidence door; that `Budget` on the two floor rungs is the form budget freezing the larger offset.
