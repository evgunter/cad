## For Ev

**Recommendation (likely).** Two fixes, one upstream and one downstream of where the question was asked. After both, the sweep doors read the profile's record of a line and never work out a direction from its points again.

1. **A line's record holds its direction and its length, minted together.** `SegmentKind::Line { director, length }` replaces `Line { director }`. Whoever mints the director also mints the length, spelled so that *start + length·director* is the end vertex as a formula. That means it holds at every value with no rewrite rule.
   - **Axis case.** Where the structural question proves one component of the chord zero, the director is the axis `±e` with its sign as read today. The length is `±` the other component, which is a polynomial. Example: a side `(0,0)→(w,0)` records `(1,0)` and `w`.
   - **General case.** The director is the chord over its norm and the length is that norm, taken in one read (`direction_and_length`, which is what the doors compute today).
   - **Verbs.** A verb that records a direction records its length as well. For the rectangle that is the chord projected on its literal axis, a polynomial. For a leg written with an angle, it is the authored length.
2. **The plane ladder's offset rung stops putting a decided sign into a zero claim.** The plane ladder (`geom_brep::plane_eq`) is the check that decides whether two faces lie on one plane.
   - Today the rung asks `n₁·(o₁−c) − σ·n₂·(o₂−c) = 0`. Here σ is the orientation the previous rung decided, stored as a literal ±1. So the zero is an identity only if n₁ = σ·n₂ as formulas, and that needs the sign of whatever factor sits in one normal and not the other.
   - The rung should ask instead whether each plane contains the other's origin: `n₁·(o₂−o₁) = 0` and `n₂·(o₁−o₂) = 0`. Each claim reads one normal only, and is unchanged if that normal flips or is scaled by any non-zero factor.
   - The parallel rung (`‖n₁×n₂‖`) already has that property. The orientation rung stays the numeric decision it is today and only labels the verdict (same or opposite facing).

**Premise check (§1).**
- The question asked which spelling of a direction a door should read. Neither measured failure is about that.
- **Failure (i)** is in a reader. The wedge seat needs `(ro−ri)/|ro−ri| ≡ 1` only because the offset rung multiplies one plane's offset by the decided σ.
  - Main has the same exposure today without any door change. A rectangle verb records literal axes after its gate decides the width positive (`rectangle_kernel`). Its wall beside a table polygon's wall on the same plane is a literal normal against a quotient normal. That pair goes through the same rung.
  - So making every door read the quotient would not end this failure, and neither would any other single spelling. Plane normals arrive literal from verbs and frames whatever the mint does. (likely; the rectangle-against-table pair is reasoned from the code, not run)
- **Failure (ii)** is in a door. The doors recover the length by projecting the chord on the director, `(b−a)·d`. That folds to the length only through rule A (`√X² → X`). With the length minted beside the director, nothing projects. (sure for the axis and quotient spellings, from the measurement's own arithmetic)
- With both fixed, the doors can read the record (one statement), and the record can keep its exact axis (no width from the box).

**What each scalar's record means** (validation runs at `f64`, `Interval`, `Sym<f64>` and `Sym<Interval>`):
- `f64`: the chord's unit and its norm, rounded. The axis case cannot be reached, because a number carries no formula.
- `Interval`: enclosures of the unit and the norm of every chord the box allows. No component is proved zero, so a moving side's width is honest.
- `Sym<·>`: the axis case where a component is proved zero and the other's sign reads definite (as today), else the unit and norm from one read. Either way, start + length·director is the end as a formula.
- A scalar that cannot mint has no sentence here. Lifting a record into such a scalar re-mints it from the lifted points. That is the agreed unit-vector row's rule (`a-recorded-direction-claims-a-unit-and-is-held-as-two-numbers`), and this design slots into it: `UnitVec2::normalized` returns (unit, length), and the axis maker pairs with the signed component.

**What each layer owns.**
- **Stored:** the director and the length, in the profile, beside the vertices.
- **Derived by the doors:**
  - the wall normal: the right normal of the director;
  - the wall frame: turned onto the director;
  - the rim: a carrier through the start along the director, spanning the stored length.
- `sketch_vector`'s and `sketch_chord`'s line arms go. No validated line lacks a record, so the door never has a side whose record it cannot use. The type is total, and no fallback is written.
- The plane ladder owns the sign-free statement "one plane".

**A direction a test table wrote** (the test-only door): the length is `chord·d`. For a literal axis that makes the end an identity. For a written oblique unit it holds only within the band that the `segment_direction` check accepted, and certification then decides the rim's end numerically. That is honest for a test-only input. The stretched-director row, where the stored normal's length is `1 + ε/5`, becomes unwritable when `UnitVec2` lands. This design does not need that row first.

**What it does not reach.** A sloped side built from points whose direction does not change over the box (the wedge's far side) keeps the width of its quotient. A formula over points cannot show that the direction is fixed without its sign. Only a construction that holds the direction can record it exactly: the open fillet/side/hop row (`fillet-side-and-centre-arcs-and-off-axis-hops-record-no-direction`), unparked by this work. (sure)

**Alternative weighed: no sign in any director.**
- How it works: components proved zero are spelled as the literal 0, every director is the quotient (symbolically `x/|x|`), and a new primitive gives `Interval` a tight enclosure of a vector's unit.
- For: it needs no plane-ladder change for table-against-table pairs, and the unlogged sign reading behind the axis (audit row F43) goes.
- Against: it still needs the ladder fix, for verb literals and frame normals beside quotients. It adds a primitive per scalar, and it fills residuals with `x/|x|` atoms the tier cannot fold. I lean against it. It is the fallback if the F43 reading proves unsound.

**Ratified text.** None changes. The exact-axis spelling is E12's own example of the structural question, and this keeps it. A short sentence in the profile README: "A line records its director and its length, minted together so that its start plus its length along its director is its end as a formula." The audit row for `plane_eq_offset` changes its margin's description. Reversible: yes. Each half is local, and the ladder change is a margin re-posing.

**Confidence.**
- That (i) is cured by the ladder: likely. I read the coincidence door down to `plane_ladder` but did not run it.
- That (ii) is cured by the paired length: likely for the plane-ladder and fold rows. The frozen counts, digests and low-bit goldens will move and are re-baselines.
- That `sym_9` gets back to main's symbolic count or better: unsure.

## For the orchestrator

- **Assumed, not run:** the ring seats and `two_copies_on_one_axis` reach `geom_brep::plane_eq::plane_ladder` through `coincide::replay::carrier_verdict` → `topo::boolean::seated_pair_decided`. Check that no other plane identity (e.g. `topo::injective::one_plane`, the stated-axis arm) bundles σ the same way. If one does, it gets the same re-posing.
- **Measure first** (cheap, and decides the fork): PR 7384's mutant, plus (a) the ladder rung re-posed and (b) `sketch_chord` returning the paired length (`chord·d` when d is a literal, the norm from the same read otherwise). Expected: kind-(i) rows green; the kind-(ii) ladder rows and the plain-tier fold back to main; the 1e-12 `band_*` carve rows unknown. Then run the slab and two-row tests from `32eb1f08`.
- **Possible pre-existing defect** (not run): a rectangle-verb member beside a table polygon sharing a wall, at `Sym<Interval>` over a box. By the same rung it should leave the seat unproven on main today. A row that shows it would be evidence for half 2 independent of this item. File it if it reproduces.
- **Riser row** (`a-hop-whose-axis-changes-sign-inside-the-box-gets-no-flip-verdict`): untouched by the recommendation. The F43 reading is sound, because a straddle refuses and a definite read holds over the box, but it is not logged. With the sign-free readers above no symbolic Zero leans on it, which makes logging it (a flip verdict) safe to do there.
- **Arc half consistency:** this is the same pattern as signed-radius. The literal the construction decided is stored, readers read a form that has no sign in it, and nothing is registered. `mint_radial` shares `axis_or`, so a radial's paired quantity is the arc's stored radius, which that unit already has.
- **Sequencing:** the ladder half can land first, alone, and should not move any existing row except the pre-existing defect above. The record half then lands with the doors reading it, and closes this row's tests. `UnitVec2` can come before or after.
- **Brief:** accurate. I did not read `chord_director`'s non-finite `atan2` arm closely; a hop of non-finite length takes a direction from an angle there, and its paired length needs a sentence when built.
