## For Ev

**Recommendation, round 4 (changed back; likely). I agree with the final state the third report reached in round 3, with one addition that removes the reason I left it.** Unless the measurement in flight finds a box-scalar-only decision that steers what is built, the final state is:
- **Reference:** the `f64` witness.
- **Certified:** every predicate is definite, every node stands as at the witness, and the choices equal the witness's, one for one. Checks must be definite and passing, and are the leaf's own.
- **`FlipCrossing`:** a choice turns. It is named and non-empty by type.
- **`Parted`:** any other first difference among choices. It is refused at once, in its own class, naming the node, the step and each run's entry. Its mass means "not proven: the kernel's two lanes recorded different work here".
- **The addition:** a reading of the part's own form that is taken inside a check scope is recorded once as a choice, the way the side reader records `side_read` (`k_stats::answered`). The first such reading is a crease's convexity in the finished-body check.

**What changed from round 3, and why.** In round 3 I moved to the same-lane reference and put checks back in the gate. My reason was that a convexity read only by the finished-body check would turn with every choice equal and certify a corner become a notch. Two things answer that better than checks in the gate.
- **Checks are not one kind of thing (sure, read this round).**
  - `geom_brep::dihedral`'s `convexity` decides `dihedral_sign` through `decide_nonzero_reported`: convex or concave, both pass, and the sign is the part's shape.
  - `side_at_infinity_order` (`topo::rows`, the aimed ray's `order` row, read in `side_reader.rs`'s pair reading) is two faces' order along a ray. Its other sign moves the instrument to another ray, with the part unchanged.
  - The role marks both as checks, so comparing checks would refuse some unchanged parts as flips.
- **The right fix is at the site, not in the gate.** Record the convexity as the answer it is, once, as the side reader already records its side. Then the choice gate sees the notch, and no instrument's route can refuse a leaf.

The reference objection also stands. My round-3 tie tolerated a parting between lanes, so it would certify against a route the `f64` build did not take. And a tie strict enough to see `pcurve_chart_placement` compares the lanes' choices, which is the parity the reference was meant to make unnecessary.

**Terms.**
- *Witness*: the nominal run at `f64`, the build the user sees.
- *Leaf*: a sub-box replayed at the box scalar.
- *Choice*: a logged decision whose other sign can change what is built.
- *Check*: a decision whose other sign can only refuse, or an instrument's reading on the way to an answer.
- *Answered*: an instrument's result recorded once as a choice.
- *Standing*: built, failed, poisoned or absent.
- *Turn*: the same question, reached the same way, with the other sign.
- *Parting*: any other first difference.
- *Same-lane reference*: the nominal replayed at the box scalar over a zero-width box, compared with leaves instead of the witness.

**The arguments against my round-3 position, answered.**
- **The cell tie hides a lane contradiction.** It does; I withdraw that tie. Under the `f64` reference the contradiction shows in the leaf holding the nominal as a turn, and a turn in a leaf that holds the nominal point is a kernel defect by construction. So it is refused as the kernel's: a `debug_assert`, and a refusal naming it, never document mass. The range search already does half of this.
- **Checks are not one kind:** accepted, above.
- **"Every reason for terminal `Parted` answered lane noise, a fact about the kernel":** true. The answer is that the kernel's mass is shown as the kernel's (`Parted`, named, falling as the sites are fixed) rather than moving the reference to make it disappear.
- **A box-only decision that steers:** this is the one case that brings the same-lane reference back (below).

**The recommendation as a function of the four measurements.**
- **(3) The six lane pairs, labelled.**
  - All (a), work one lane logs that steers nothing: each is fixed at its site by deciding it in both lanes or unlogging it in one, and the state above stands. This is the expected outcome.
  - Any (c), a parting a narrower box removes: bisect that parting kind only (a `Parting::Mark`, or the named step's class), terminal for the rest.
  - Any (b), a box-only decision that steers what is built: adopt the same-lane reference for every drive. Its tie to the witness is: decided, equal standings, no choice turned against the witness, lane partings allowed. Compare choices only, with the convexity answered. Partings then bisect, since lane noise can no longer cause them. One reference for all drives, never per document.
- **(1) The reference on every driven document.**
  - Under the state above this decides only how much `Parted` mass the kernel owes. A drive whose zero-width reference parts from the witness refuses its leaves `Parted`, named.
  - If (3) forced the same-lane reference and (1) shows drives that certify mass today while their reference is undecided, the reference loses that mass. Fix the escalations at their sites first (`curve_pair_pose_parallel`, `germ_side`, `chart_singularity_nearer`); do not keep a second `f64` reference beside it.
  - A turned choice between the lanes at the nominal is the kernel defect above in every branch: refused out loud, filed P0-shaped.
- **(2) The re-sort, whole record against choices only.**
  - If the two sorts agree on every leaf, which I expect, keeping checks out costs nothing measured and the choice is by principle, as above.
  - If they differ only at shape readings (`dihedral_sign`), the convexity addition is confirmed; land it before the gate.
  - If they differ at an instrument's route (a ray's `order`, `advance`), the comparing-checks answer would refuse unchanged parts, which confirms checks out.
- **(4) A decided leaf with equal choices and a turned check.**
  - Any `dihedral_sign`: the addition is required, not optional.
  - Any ray decision: as in (2).
  - None, the expected outcome: the addition still lands, as a site fix that closes the notch case before a document reaches it.
- **With no measurement at all:** the state above, the convexity answered, `Parted` terminal.

**What all three agree on (the list, corrected).**
- Cells cannot gate, and cannot tie a reference.
- A turned choice is a named, non-empty `FlipCrossing`.
- A parting is never a flip and is named wherever it ends.
- A standing change is a flip only when the failure is proven definite.
- The Extrude's undecided length goes through the funnel.
- A contradiction between lanes at one point is a kernel defect, refused out loud.
- `is_empty`, `FlipUnfounded` and " unnamed" go.

Add three:
- A refusal at the floor carries its deciding node and what it stopped on; `Budget` names nothing today.
- The cell record stays a probe, not production.
- The parity instrument lists turns between lanes separately from partings.

Correct one: "the shape is owed through the choices" is true only once a shape reading taken in a check scope is answered as a choice. That is the addition, and it should be on the list.

**E6 text.** The third report's round-3 wording. Certified: "choices and standings matching the witness's; checks are the leaf's own". No-flips: "a choice turned (`FlipCrossing`, named); a record that parts with no turn is refused `Parted`, not proven either way". Replace "shares the nominal build's topology" with "builds the nominal's form". No-flips (Ev's proposal) is untouched.

**Confidence.** Recommendation: likely. "Checks are not one kind": sure. "A crease's convexity can be answered once as a choice without changing what tier 3 decides": likely; it is the `side_read` pattern. "Expected outcomes, all (a) and the sorts agreeing": likely, from M13's 0 turned checks and 0 mark partings.

## For the orchestrator

- **Plainly:** I agree with the third report's round-3 state, plus the convexity addition. I withdraw my round-3 same-lane reference and checks-in, except as the (b) branch above.
- **Ask the trace** to label each pair (a), (b) or (c), and for (b) to name what the decision steers. That is the only outcome that changes the reference.
- **New row (P1, E/M):** tier 3's crease convexity (`dihedral_sign` from `Dihedral::convexity`, through `edge_wedge_over`) is the part's shape, read only in a check scope. Record it once per edge as answered, the `side_read` pattern, so a convexity no construction choice decided still reaches the certificate. Land it with or before the gate.
- **The drive owns the turn-at-nominal rule:** a turn on a leaf holding the nominal point is the kernel's, a `debug_assert` and a named refusal; the range search's copy moves into `classify_replay`.
- **Rows already agreed:** file them if not yet filed (`contained`, the deciding node on `Budget`, the Extrude funnel, `VerdictVector`'s doc sentences, `closed_form_shown`'s role, `pcurve_chart_placement`).
