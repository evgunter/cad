## For Ev

**Yes, your worry is right.** Edit 2 as sent fixes the lever by what the datum is (two directions out of one point). The README fixes an extent by what a verdict is used over. Those agree only for a reader that uses the corner's verdict over the corner's own neighbourhood, out to the shorter edge.

**What is true of each sign.**
- A definite sign, served at the lesser far distance read from below, is right for every reader whose region reaches at least that far. A reader that uses it over less reads its own region.
- A Zero, served at the lesser far distance read from above, is right only for a reader whose region reaches no farther. A reader that uses it over more reads its own region.

**Worked case, the Zero.** A flat face with a vertex in the middle of a straight side. The two edges at the vertex are each 1 long; the face runs 100 deep. A plane contains the side's line and is tilted off the face by 0.5ε per unit length.
- The divider, square to the side and into the face, reads 0.5ε at lever 1: a Zero.
- At depth 100 the face stands 50ε off the plane.
- A reader that took the Zero as "this face lies in the plane" would be wrong. A reader that took it as "nothing parts from the plane near this vertex" is right.

**Worked case, the definite sign.** A touch reading inside a ball of radius 0.001, at a corner whose shorter edge reaches 0.01. A sign that is definite at 0.01 can be within the band everywhere inside that ball.

**What holds the invariant: today, the functions; in the final state, the types.**
- Today's readers are safe, and the three that claim a whole face each read the face: the boolean's glue door, the split's coplanar rule ("metered at the face's extent from the base vertex"), and the link's fan.
- But nothing in the types holds that. Two readings of the tree show it:
  - The boolean's whole-face verdict is `Coincident::Glued(CarrierRelation)`. `CarrierRelation` is a plain public enum, and `fan::meeting` takes it as an argument its caller supplies. Any function could write one from a corner's Zero.
  - The split's whole-face verdict is `ParallelFace`, documented as decided "over its face extent". `apex_reading` mints the same type at a bound's lever (`bound.reach.lever()`), not the face's.
- The final state has two type statements:
  1. The corner's arm is a named type, built only from the corner's two departures. Sector predicates take it. A Zero decided at it is a neighbourhood Zero.
  2. Whatever acts on a whole face (keeping one of two coincident faces, treating a face as lying in a plane) takes a whole-face verdict in its signature. That verdict has private fields and one mint, a door that takes the face and levers at the face's far distance. The tree has this pattern already: `geom_brep::ParallelNormal` has `normal_alignment` as "its only mint" and carries the arm it was decided at.
- Then a neighbourhood Zero is a different type from a whole-face verdict and cannot be handed in.

**The three edits to `crates/geom-core/README.md`, "Float rounding".**

1. **Unchanged.** In **A verdict's lever arm**, after "…or the session box where no geometry is in hand.":
   > The read runs one way: a length that places or sizes geometry is a read of that geometry, not of a lever.

2. **Changed.** In **The consumed extent**, after "…the least over all points is not owed.":
   > A verdict on the turn between two directions out of one point, each with its own extent, when used over the part where both run, is levered at the lesser of the two extents' far distances from that point, each read from below and from above. A verdict used over more or less reads the extent it is used over. A direction drawn between them has no extent of its own, and a verdict on it reads the extent it is used over: that same part, or the face the direction lies in.

   - The first sentence no longer asserts the use; it is conditioned on it.
   - The second covers both worked cases.
   - The third no longer gives a divider the corner's lever unconditionally. The first worked case is a divider.
   - "Point" is the bullet's word on main today. If PR 7619's "site" merges first, the edit follows the page.

3. **Changed.** In the same bullet, the closing clause becomes:
   > so a record whose verdicts all read from one point over one extent may hold that extent's lever, read from below and from above, and a record whose verdicts read from different points, or are used over different extents, holds its region, not a lever.

   - The same flaw sat here. As sent, a record whose readers share a point could hold one lever for readers that use it over different regions.

**What else changes in the final state.**
- **The divider.** It carries its corner's arm, as the named type. Its verdicts are used over the corner's neighbourhood. A reader that needs a divider's side over the whole face reads the face through the whole-face door.
- **Item 2's divider line** now reads: "A divider takes its corner's arm: its verdicts are used over the corner's neighbourhood. A claim about a whole face is a different type, minted only by a door that takes the face."
- **The both-bounds door has no opt-outs.** The lever text you approved on PR 7619 says a verdict is served "whatever acts on the verdict" and "no consumer takes one bound". Reading both serves a definite sign at the bound from below and a Zero at the bound from above, which is what each sign needs. So no reader gets a more correct verdict by reading one bound. The `.below()` / `.above()` exits and their swap rows come out.
- **The link's fan** is unchanged. It is the pattern.
- **Unmeasured:** nothing was built. No reader moves to a longer lever, so I expect no verdict to move from this addendum; the one exception to check is the split's graze at a cone's apex, below.

## For the orchestrator

**The five points, each by its test.**
1. *The definite sign.* No contradiction; the two sentences above are both of ours. Test: where the witness is. A definite sign's witness sits at the lever's length, so it serves any region that reaches it and none that does not.
2. *What holds it.* Test: can a function write the whole-face verdict without the face? Yes, in both lanes (`CarrierRelation` is a public enum handed in as `glued: Option<CarrierRelation>`; `ParallelFace` is minted by `apex_reading` at a bound's lever). So A's "no door turns a corner's verdict into one" is a fact about today's functions, and in the split it is not quite true. The types are the final state.
3. *Edit 2.* A's second sentence taken verbatim; my conditioning for the first and third. A's third sentence ("takes that lever") is unconditional, and the divider is the worked case.
4. *Edit 3.* The flaw is there; one text above.
5. *Opt-outs.* Test: is there a sign for which reading both serves the wrong bound? No. A definite sign at the bound from below is also definite at the bound from above unless that bound is unreadable, and then refusing is right. In `sector_shape`, a straddle is "in band", and an in-band rung 2 already falls through to the subdivision, so reading both changes nothing there.

**New findings.**
- `splitting::rules::apex_reading` calls `normal_alignment("split_sector_coplanar", …, bound.reach.lever(), band)` and returns `WallReading::Grazed(ParallelFace(..))`. Rule (a)'s tangent branch mints the same type at `face_extent`. One type, two extents. A graze along a ruling is a claim over the ruling. For a line edge on the ruling the bound's reach is that; for a curved edge at the apex it is not. The unit gives it its own verdict or the ruling's reach. Not filed; I am read-only.
- `Coincident::Glued` wraps a plain value. The unit replaces it with a token that `GluedPairs` alone mints (its `verified.one_carrier` set is the certificate; today it is reached through a `bool`).
- `face_extent` returns a bare `Lever`. In the final state it is the named face reach that the whole-face mints demand.

**What the unit must check.**
- List every function in the boolean and split lanes that keeps, drops, merges or lumps a face, or reclassifies a sector as lying in a plane. Each must take a whole-face token in its signature. I read `recl`'s lump, `pair_search`, `fan::meeting`, `apply_rule_a` and `apply_rule_b`. I did not read `overlap`, `holdings`, `slit`, or the split's `finish`.
- A's item: list every reader of the corner's arm that uses it inside a smaller ball. A says the link's touch readings use their own `clear`; I did not check `BoolSector.arm`'s readers for one.
- A reading that is not a plain product with its lever (a rise plus a parting) can agree at both bounds and differ between them. PR 7619's "holds at every length between the two" covers it; `decide_at_lever`'s closure form does not prove it. The unit lists which sector readings are not monotone in the lever.

**Nothing was built or run.**
