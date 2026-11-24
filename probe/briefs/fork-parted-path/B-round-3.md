## For Ev

**Recommendation, round 3 (changed; likely): compare a leaf with the nominal replayed in the leaf's own lane, and compare the whole record.** This agrees with the same-lane proposal, with two additions.
- **The reference.** The nominal document is replayed once per drive at the box scalar (`Sym<Interval>`) over a zero-width box at the nominal point. Every leaf is compared with that replay, not with the `f64` witness.
- **Certified:** every predicate is definite over the leaf, every node stands as in the reference, and the leaf's record equals the reference's: choices and checks, in order.
- **`FlipCrossing`:** at the first difference, one question is answered with the other definite sign, whether a choice or a check. The evidence names it by type (node, predicate, role, both signs), so an empty one cannot be built.
- **Parted** (the records differ and nothing turned): the leaf is not proven either way. It is unsettled and bisects like an indeterminate leaf. At the floor its refusal (`Budget` or `ReadsNoVariable`) names the node, the step and each run's entry there. It is never a flip.
- **The tie, once per drive.** The reference must be decided, must build what the `f64` witness builds (per node: standing, and cells by name), and must not turn any choice against it. Otherwise the drive refuses whole and at once, naming the node and predicate. The last condition is my first addition: two lanes answering one question oppositely at one point (`pcurve_chart_placement` on the tour's bracket) is a contradiction, and it should be refused out loud, not hidden. A parting between the two lanes is tolerated, because legitimate lane-specific logging produces one.

**What changed from round 2, and why.** In round 2 I compared choices with the `f64` witness, left checks out, and made a parting terminal. All three choices answered one problem: the two lanes do not log alike (27% of decided builds at one point). They were not facts about the document. The same-lane reference removes that problem by construction, since both sides are one program in one lane and differ only by the box. Once it is gone:
- I can no longer defend leaving checks out. A convexity that only the finished-body check reads is still the part's shape, and I said in round 2 that convexity is owed.
- A parting can no longer be lane noise, so the case for making it terminal goes too. What is left to part a route is box width and real geometry, which is what bisecting exists for.

The reference also states the nominal's form in the certified lane. A definite box-scalar answer is proven true at the nominal point, whereas an `f64` answer is true only under the float-conditioning rule.

**Terms.**
- *Witness*: the nominal run at `f64`, the build the user sees.
- *Reference*: the nominal replayed at the box scalar over a zero-width box.
- *Leaf*: a sub-box, replayed at the box scalar.
- *Record*: per node, every logged definite decision in order: predicate, sign, role.
- *Choice*: a decision whose other sign can change what is built.
- *Check*: a decision whose other sign can only refuse, or a reading taken off the finished body.
- *Turn*: the same question, reached the same way, answered with the other sign.
- *Parting*: any other first difference.
- *Standing*: built, failed, poisoned or absent.

**Checks: a turned check on a decided leaf is the part's shape changing (likely).** The site is the finished-body validation, `topo::validate`. It runs inside `k_stats::checking` and reads each edge's `geom_brep::DIHEDRAL_SIGN`. Its `wedge_reading` accepts a `Crease` that is convex or concave alike. So a check that passes with either sign is reading a property of the part. If it reads another definite sign, the part differs in that respect: a corner has become a notch. The meaning does not depend on the reference; whether checks can be compared does. Against the `f64` witness, 203 builds part in checks alone through lane noise. Against the same-lane reference they cannot. M13 finds no decided leaf that turns a check today, so keeping checks costs nothing measured. Where only a check reads a convexity, leaving checks out would certify the notch. (Sure about the site; likely that no check reads something other than shape. The side reader's rays are the case to watch. Their answer is logged once as the choice `side_read`, but a ray's own checks can change with which face it aims at. That shows as a parting, which bisects, and not as a turn.)

**Partings: bisect, and name them at the floor (likely).** Today they barely exist on decided leaves: 1 in 2,505, the plate, which the reference certifies. So the choice is by principle, and E6's principle is "not decided at this width → split". The fact that would reverse it: among same-lane partings on decided leaves, how many have a half that certifies (box width) and how many stay parted to the floor (geometry, such as a slide that changes which boxes overlap, or the unmarked `separating::apart`). If the second kind dominates, refuse at once as `Parted`; that is a one-arm change.
- **What a consumer is told.** Bisected: "not proven here; the kernel's route depends on the box at node N, step k", with the depth spent. Terminal: the same sentence after one replay. Neither may read as "different".

**The reference's costs, and what I'd do (unsure until measured).**
- **Undecided at zero width.** 22% of builds do so at plain `Interval`; at `Sym<Interval>` it is measured on only 67 witnesses, 4 of which differ in standing. Where the reference is undecided, the nominal's form is not proven in the certified lane. Any leaf holding the nominal is undecided at every width anyway, so refusing the drive whole, naming the node, is honest.
- **I reject a fallback to the `f64` reference** for those drives. One drive whose certificate means "the box lane's nominal" and another whose certificate means "the `f64` nominal" are two semantics behind one word.
- **What would change that:** measuring that many drives certify real mass today while their reference is undecided. Then the defect is the box lane failing at the nominal, and it is fixed there, not by a second reference.
- **`pcurve_chart_placement`.** Under the tie above it is seen, refusing the drive whole, rather than hidden.

**What a certified leaf promises.** Over this leaf the document builds, and every question its build asked is answered as at the nominal, read in the certified lane. That covers the same cells, convexities, sides and measure configurations. Only dimensions move, and every measure's enclosure contains its value at every point. Not promised: an assertion's verdict (E10's own column), a coincidence's discharge kind, or a lane choice's spelling.

**Text for E6.** Add to the leaf protocol: "A leaf is compared with the nominal's own replay at the box scalar over a zero-width box, which must be decided, build the `f64` witness's cells and turn none of its choices, or the drive refuses whole. A leaf whose record differs with no decision turned is not proven either way and bisects; a turned decision of either role is `FlipCrossing`, named." No-flips (Ev's proposal) is unchanged.

**Confidence.** Recommendation: likely. "Same-lane removes lane parity from certification": sure, by construction. "A turned check is a shape change": likely. "Bisect over terminal": likely, pending the first fact below. "Refuse the whole drive on an undecided reference": unsure until measured.

## For the orchestrator

- **I now agree with the same-lane report (C).** My additions: (1) the tie also refuses a choice that turns between reference and witness, naming it; (2) a parted leaf's floor refusal carries node, step and both entries, since a `Budget` refusal today names nothing; (3) no `f64` fallback reference: a second reference means two semantics behind one word.
- **Measure first, in order:**
  1. **The reference at `Sym<Interval>` on every driven document** (the 305 drives, the 34 range searches, the tour): decided? cells equal to the witness's? any choice turned against the witness? Also count the drives that certify mass today while the reference is undecided; that decides whether a whole-drive refusal costs real certificates.
  2. **Re-sort M13's 2,505 decided leaves** against the reference on the whole record. Expected: 660 turned, the plate certified, 0 parted. Any other change of class is the finding. Repeat with choices only: if the two sorts agree everywhere, checks-in costs nothing measured.
  3. **For any same-lane parting found:** do its halves certify? That decides bisect against terminal.
  4. **The six lane pairs being traced:** they no longer gate certification; they go to the parity instrument and K. Only an opposite answer to the same question at one point (`pcurve_chart_placement`) is a kernel defect of the first rank. File it now.
- **Unchanged from round 2:** the Extrude "no definite length across the sketch plane" goes through the funnel (P1, the largest mass, independent of this fork). `FlipEvidence::is_empty`, " unnamed", range's `Unfounded`/`FlipUnfounded` and their viewer sentence and Python tag go.
- **The naming walk** must read checks as well as choices, and marks, along the first difference. With one lane on both sides the marks are comparable.
- **Cost:** one extra replay plus one cell-record comparison per drive. `build_record.rs` on the probe branch becomes production code for the tie only.
