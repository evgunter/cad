## For Ev

**Yes: as sent, edit 2 is false for a valid consumer, on a Zero.** The worry is right, and the sentence should change.

**The definite sign is safe for every consumer.** It is served at the lesser far distance read from below. Both directions have points of the body that far out, and those points stand apart by more than the band. A consumer that uses "apart" over anything reaching at least that far is right.

**The Zero is not.** It is served at the lesser far distance read from above, so it proves "within the band" only out to the shorter direction's end. A consumer that uses it over more is under-levered.

*Worked case.* A flat face with a vertex in the middle of a straight side. The two edges at the vertex are each 1 long. The face runs 100 deep. A plane contains the side's line and is tilted off the face by 0.5ε per unit length.
- The divider (square to the side, into the face) reads 0.5ε at lever 1: a Zero.
- At depth 100 the face stands 50ε off the plane: definitely apart.
- A consumer that takes the Zero as "this face lies in the plane" (keep it as a wall, merge it, drop one of two faces) is wrong. A consumer that takes it as "nothing parts from the plane near this vertex" is right.

**Such consumers are valid, and the tree already has three.** Each levers at the face, not at the corner:
- the split's coplanar rule (`splitting::rules`, rule (a)): "sin of the normal angle metered at the face's extent from the base vertex";
- the boolean's glue door, whose extent is "a ball enclosing both faces"; `fan::meeting` says of the corner's own reading "an unglued Zero acts over no extent";
- the link's fan, sized by the face's far distance, "so a coincidence read there holds over the whole face".

So the invariant holds today, and the PR body's sentence was a true report. But it was a fact about readers. Edit 2 as sent fixed the extent by what the datum is (two directions out of a point). The bullet it sits in defines the extent by what the verdict is used over. Those disagree for exactly the consumer above.

**What should carry it: the type, in two places.**
1. *The corner's arm is a named type*, built only from the corner's two departures. It is the lever of the corner's neighbourhood: the part within the lesser of the two edges' far distances from the vertex. Sector predicates take it, not a bare `LeverBounds`. A Zero decided at it is a neighbourhood Zero and nothing more.
2. *Whatever acts on a whole face takes a whole-face verdict in its signature*, and that verdict is minted only by a door that takes the face and levers at its far distance. The boolean's lump already works this way: it accepts only the glue door's one-carrier verdict. The split's rule (a) and the link's fan are the same shape. A neighbourhood Zero is a different type and cannot be handed in.

With both, a reader cannot use a corner's Zero as a face-wide fact: there is nothing to pass.

**Edit 2, as I would now put it** (same place: **The consumed extent**, after the sentence that defines the lever as the extent's far distance):
> A verdict on the turn between two directions out of one site, each with its own extent, that is used over the part where both run has that part as its extent: its lever is the lesser of the two far distances from the site, each read from below and from above. A direction drawn between them has no extent of its own, and a verdict on it has the extent it is used over: that same part, or the face the direction lies in.

It is true of every consumer by construction, because the lever follows the use. ("Site" is the word of the lever text you approved on PR 7619.)

**Edit 3 changes with it.** A record may hold one lever only if its verdicts share a region as well as a site:
> so a record whose verdicts all read from one site over one extent may hold its lever, read from below and from above, and a record that verdicts read from different sites, or uses over different extents, holds its region, not a lever.

Edit 1 is unchanged.

**What else changes in the final state.**
- *The divider.* It still carries its corner's arm, now as that named type. A reader that needs a divider's side over the whole face reads the face's far distance through the whole-face door.
- *Item 2's last line* ("no reader takes it as two faces coinciding without the glue door's verdict") is replaced by the two type statements above.
- *The both-bounds door loses its opt-outs.* The lever text you approved on PR 7619 says "no consumer takes one bound" and that a verdict is served "whatever acts on the verdict". My `.below()` / `.above()` opt-outs and their swap rows contradict that and come out. Every sector predicate reads both.
- *The link's fan* is unchanged. It is the pattern, not an exception.
- *Cost, unmeasured:* none from this change on today's bodies, since no reader moves to a longer lever. It closes a door; it does not move a verdict.

## For the orchestrator

**Read this round.**
- `splitting::rules`' module doc and `apply_rule_a`'s imports (`face_reach::face_extent`). Rule (a)'s coplanarity gate is `split_sector_coplanar`, "metered at the face's extent from the base vertex". Neither of us had cited this. It is a third whole-face consumer, and it is correct.
- `apply_rule_b`: a divider `On` after its descent takes its neighbours' side; consecutive `On` entries refuse (`ConsecutiveOnSectors`).
- `fan::meeting`, `PairRecord::coincident` and `recl` again: the lump requires `Coincident::Glued`.
- PR 7619's lever and extent bullets, read from commit `baf9df0720` (`git show`), since the tree at `63d0479875` holds the earlier text. I took your word that Ev approved that wording.

**Your hypothesis: checked and right**, with one addition. The first sentence of edit 2 has the same flaw as the second. A consumer could use "edge b is parallel to short edge a" as "b lies on a's line for b's whole length". So the new text conditions both sentences on the use.

**New, for the unit.**
- The named arm type is small: a newtype over `LeverBounds` in `topo`, built only by the corner (two departures). A's round-1 `CornerArm` was this name; its upper bound was wrong then, the name was right.
- The whole-face verdict types exist in pieces: `fan::Coincident::Glued(CarrierRelation)`, the split's `ParallelFace` / `geom_brep::ParallelNormal`. The unit makes each acting function take one, and checks no function that keeps, merges or lumps a face is reachable from a bare `SideCode::On` or `PlaneSide::On`.
- `PairRecord::coplanar()`'s doc says "the pair lies in one plane" of four neighbourhood codes. I found no non-test caller in `boolean/` (`slit.rs`'s `coplanar()` is another function). Its doc over-claims; file it.
- The opt-outs coming out also removes the "swap row" merge gate from the measurement list.

**Not checked.**
- Whether every face-wide action in the split and boolean lanes is already behind a whole-face verdict in its signature. I checked the lump and rule (a) only.
- Whether "site" or "point" is the word on main when this merges; the two edits should follow the README as it then stands.
- Nothing was built or run.
