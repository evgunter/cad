round 4 (label C): a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing

## For Ev

**Recommendation (likely): the same-lane reference, tied to the `f64` witness by its decision log, with the whole record compared and a parting bisected. I agree with report A's round 3 as it stands, plus report B's rule that there is no second reference.** Stated once:

- **Reference**: once per drive, the nominal is replayed at the drive's own box scalar over a zero-width box.
- **Tie**: the reference must be decided and its choices must equal the `f64` witness's: no turn, no parting. Otherwise the drive refuses whole, at once, naming the node and step, as the kernel's and not the document's.
- **Certified**: every predicate definite over the leaf, and the leaf's record (choices and checks, in order) equal to the reference's.
- **`FlipCrossing`**: at the first difference one question has the other definite sign. Named, role carried, never empty.
- **Parted**: the records differ and nothing turned. Not proven; bisected; named at the floor. Never a flip.

**Terms.** *Witness*: the nominal at `f64`, the build the user sees. *Box scalar*: the interval arithmetic leaves are replayed in. *Reference*: the nominal replayed at the box scalar, zero width. *Record*: per node, every logged definite decision in order. *Choice*: a decision whose other sign can change what is built. *Check*: one whose other sign can only refuse, or a reading taken off the finished body or inside an instrument. *Turn*: the same question, reached the same way, other sign. *Parting*: any other first difference. *Tie*: what makes the reference stand for the witness.

**What changed from round 3, and why.** In round 3 I withdrew the reference on two objections. Both are answered by the other reports' amendments, and one observation I had missed makes the rest follow.

- *The tie.* My objection was to a tie by named cells, which is blind to what a choice decides. A tie by the log is not blind: the measured contradiction (`pcurve_chart_placement`, opposite signs in the two scalars at one point) refuses that drive out loud. I answered in round 3 that such a tie "is the lane parity the reference was meant to make unnecessary". That was wrong about what the reference is for. **With the choices tied, comparing a leaf's choices with the reference's is the same comparison as with the witness's.** So the reference takes nothing away from the round-3 gate I agreed to. It adds two things on top of it: the checks become comparable (both sides are one program), and a parting at a leaf can no longer be the two scalars logging differently, because the tie has already shown they log alike at the nominal.
- *Checks.* My round-3 argument was that checks are not one kind of thing. That stands (below). What it does not settle is which error to risk. Leaving checks out risks certifying a corner become a notch wherever only the finished-body check reads the corner. Keeping them in risks refusing, under a name, a leaf whose form is the nominal's. A wrong certificate breaks the promise; a wrong refusal costs mass. Against the `f64` witness the second risk was large (203 builds differ in checks alone); against the reference it is zero as measured. So the strict comparison, which is E6 as ratified, is the safe default again.
- *A parting.* Terminal was right when a parting could be lane noise. Behind the tie it cannot be, and the driver refuses outright only what splitting is proven not to help.

**Answers to the arguments against my round-3 position.**
1. *"A convexity only the finished-body check reads turns with every choice equal."* Correct, and I had answered it with a repair at the site. That repair needs an audit of every form fact tier 3 reads; the strict comparison needs none. Site: `geom_brep::Corner::convexity` (`dihedral_sign`, convex or concave, both pass). I concede.
2. *"Every reason for terminal `Parted` and for leaving checks out answered lane noise."* Correct; conceded above.
3. *My own check argument, corrected.* The example I gave was wrong in detail: `side_at_infinity_order`'s margin is a magnitude that passes only when positive (`topo::props::AIMED_PAIR`), so it cannot turn between two passing signs. The class of the point survives on the rays' other rows (`side_at_infinity_beside`, `_denom`, `_winding` in `topo::rows::AIMED_RAY`): whether a ray passes beside a face can change when a dimension moves and the form does not. That would be a named flip that is not a wall of the part. I have not seen one; the fourth measurement looks for exactly it. (likely, by reading; not run)

**The recommendation as a function of the four measurements.**
1. *The reference on every driven document.*
   - Decided and tied on all, or failing only where the drive certifies nothing today (expected): adopt as above.
   - Some drives certify mass today while their reference is undecided or parts: the cause is the box scalar failing at the nominal, repaired there. Until repaired those drives refuse whole. I would not add an `f64` fallback: one word, "certified", must not mean two things.
   - If that is many drives (say more than a few of the 339), the reference is not ready: ship the round-3 gate (choices against the `f64` witness, a parting terminal and named) and revisit.
2. *The 2,505 decided leaves re-sorted, whole record and choices only.*
   - The two sorts agree everywhere and match today's but for the plate (expected): checks stay in, at no measured cost.
   - They differ: decided by the predicate, in 4.
3. *The six lane pairs.*
   - All (a), work one scalar logs and the other does not run, steering nothing (expected for most): repair each at its site; the tie passes.
   - Any (b), a decision only the box scalar makes that steers what is built: the tie as stated can never pass for those documents. Then the tie tolerates a parting at that decision and still refuses any turn (report B's tie), and E6 says so: certified means "as the box scalar's nominal".
   - Any (c), a parting a narrower box removes: confirms bisecting.
4. *A decided leaf with equal choices and a turned check.*
   - None (expected): nothing changes.
   - A `dihedral_sign`: checks in the gate are doing their work.
   - A ray row: that leaf is a false flip. Then a turned check is still refused and named, but as "not proven", not as `FlipCrossing`; a one-arm change.

**If no measurement could be had:** the recommendation at the top. It certifies nothing the round-3 gate would not, so it cannot be less safe, and its costs are one replay per drive and whole-drive refusals that fall as sites are repaired.

**What all three agree on regardless** (the list is right; three additions):
- Named cells cannot gate. A turned choice is a named, non-empty `FlipCrossing`. A parting is never a flip and is named wherever it ends. A standing change is a flip only when the failure is proven definite. The extrude's undecided length goes through the decision funnel. Two scalars answering one question oppositely at one point is a kernel defect, refused out loud. `is_empty`, `FlipUnfounded` and " unnamed" go.
- Added: a certified leaf promises the nominal's *form* (every shaping decision answered alike), and "topology" is too weak a word for it in E6. A refusal at the depth or grid floor names the node it stopped on. An assertion's verdict and a coincidence's discharge are not the certificate's.

**Text for E6**: report A's round-3 sentence is right: "A leaf is compared with the nominal's replay at the leaf's scalar over a zero-width box. That replay must be decided, and its log must agree with the `f64` witness's, or the drive refuses naming the node. A record that differs with no decision turned is not proven and bisects."

**Confidence.** Recommendation: likely. "Tied choices make the reference a refinement of the choices gate": sure, by construction. "Strict is the safer error": likely. "A ray row can turn with the form unchanged": likely, unobserved.

## For the orchestrator

- **I agree with A's round 3 as it stands**, with B's no-fallback rule in place of A's fallback gate, and with the tie comparing choices (A's "no turn, no parting"), not cells and not turn-only. The three reports now differ only on what happens under outcomes the measurement is expected not to produce; the function above covers each.
- **Checked this round**: `topo::props::AIMED_PAIR` (the order row is a magnitude, positive passes) and `topo::rows` (`AIMED_RAY`, `SCHEDULE_RAY` row names). Not checked: which scope the schedule rays (`bool_point_in_solid_*`) log in. If they are choices, a ray-route decision turning with the form unchanged would already be a false flip under every gate proposed, today's included; M13's 660 flips hold none, so it is a row to watch and not a blocker.
- **Corrections to my earlier rounds**: round 3's `side_at_infinity_order` example is wrong in detail (above); round 3's claim that a log tie defeats the reference's purpose is withdrawn; round 2's cell tie is withdrawn (it stays out of production, so `build_record.rs` is a probe only).
- **Thresholds I would hold the measurement to**, so no further round is needed:
  - (1) Count drives that certify mass today and would be refused whole. Zero: adopt. One to a handful: adopt, and file each as a box-scalar defect at its node. More: ship the round-3 gate now and re-measure after the nominal-escalation rows land.
  - (3) For each pair, the label and the repair. A pair labelled (b) should come back to one designer for the E6 sentence before it is repaired either way.
  - (2) and (4) are expected null; a non-null result is itself the finding and names its own arm above.
- **Build shape, unchanged from A's**: `drive` replays the reference once and walks its choices against the witness's; `classify_replay` compares each leaf's whole vector with the reference's; one first-difference walk (checks and marks read) returns same, turned or parted; `Unsettled::Parted` joins the existing kinds; `Budget` carries the deciding node and kind.
- **Process**: the E6 change is one added sentence group and one word ("form" for "topology"); still Ev's.
