round 3: a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing

## For Ev

**Recommendation (likely): compare each leaf with the nominal's own replay in the leaf's arithmetic, with checks included and partings bisected. Tie that replay to the f64 witness once per drive by the decision log, not by the cells.** This is the other position (a "same-lane reference"), with one amendment that it needs.
- **Reference**: once per drive, replay the nominal at the leaf's scalar (`Sym<Interval>` or `Interval`, as the drive runs) over a zero-width box.
- **Tie, once per drive**: the reference must be fully decided, and its log must agree with the f64 witness's: the same choices, no turn, no parting. Otherwise the drive refuses whole and at once. The refusal names the node and the step, and is counted as the kernel's, not the document's.
- **Certified**: every predicate is definite over the leaf, and the leaf's whole record (choices and checks, signs and order) equals the reference's.
- **`FlipCrossing`**: at the first difference, one question is answered with the other definite sign, choice or check. It is named, the role is carried, and it is non-empty by type.
- **Parted**: the records differ and no question turned. The leaf is not proven either way and bisects like an undecided leaf. At the floor it is `Budget` or `ReadsNoVariable`, naming the node and "parted".

**What changed from round 2, and why.**
- In round 2 I compared choices only with the f64 witness, and refused a parting at once.
- Both choices rested on one fact: the two arithmetic lanes log different work at the same point (2,311 of 8,673 decided builds). Against the f64 witness, a parting or a check difference might therefore be the kernel and not the document.
- A reference in the same lane removes that by construction: both sides are one program and differ only by the box. Both of my reasons then go:
  - What can part a route is box width or real geometry, where E6's rule (refuse only what splitting is proven not to help) says bisect.
  - The checks become comparable.
- The amendment: the other report ties the reference to the witness by the record of cells. M13 shows that record is blind to what a choice decides. A lane contradiction at one point, such as `pcurve_chart_placement` (Positive at f64, Negative at `Interval` on the tour's bracket), would pass that tie. The drive would then certify, against the box lane's sign, a part the f64 nominal (what the user sees) builds the other way. Tying by the log makes that a loud refusal of the whole drive instead.

**Terms.**
- *Witness*: the nominal at f64. *Leaf*: a sub-box, replayed over interval arithmetic. *Lane*: the arithmetic a run uses.
- *Record (log)*: per node, every definite decision in order: predicate, sign, role.
- *Choice*: another sign can change what is built. *Check*: another sign can only refuse, or the decision is a reading taken off the finished body.
- *Turn*: the same question, reached the same way, with the other sign. *Parting*: any other first difference.

**A turned check (likely; the site does not depend on the reference).** `geom_brep::edge_wedge_over_as` records a crease's `dihedral_sign` as a check for constructors that store "the same for a crease of either turn". Its other sign is a corner become a notch. That is the part's shape changing, which round 2 agreed is owed. If the only reader of that convexity is a check, a choices-only gate certifies the notch. So checks belong in the comparison.

Against the f64 witness that was impractical: 203 builds part in checks between lanes. Against the same-lane reference it costs nothing measured: 0 decided leaves turn a check today. The residue is a check whose sign is not shape, which would refuse conservatively. No instance is known.

**What the reference costs, and where it loses (unsure until measured).**
- **Drives whose reference is undecided at zero width** (22% of builds escalate at the nominal at plain `Interval`; `Sym<Interval>` measured only at 67 witnesses, 4 of which differ in standing). M13 reads such a document as deciding no leaf at any width. If that holds for every driven document, refusing whole at once, naming the escalating node, is cheaper and truer than burning the leaf budget. If any such drive certifies mass today, the reference loses that mass, and I would fall back to round 2's gate for those documents. That is a second gate, so measure before choosing.
- **Drives whose reference parts from the witness** (24% of builds today, at box-lane-only decisions such as `session_box_reach`): refused whole and named until each site is fixed. Under round 2's gate the same leaves were refused `Parted` one by one, so no mass is lost. The difference is one named, loud refusal per drive instead of mass. Once the sites are fixed, the reference and the witness log alike. What the reference buys from then on is that a future drift between lanes fails one drive loudly, rather than quietly turning leaves into refused mass.
- **One extra replay per drive.**

**What the consumer is told.**
- *Certified*: "at every point of this leaf every question the build asked is answered as at the nominal, and each measure encloses its value".
- *`FlipCrossing`*: "proven across a named wall".
- *Budget/ReadsNoVariable after a parting*: "not proven; the decision routes part at <node>".
- *A drive refused at the tie*: "the kernel's two lanes disagree at <node> on this document", the kernel's defect, never mass.
- The range search loses `FlipUnfounded`. The viewer and Python tags follow.

**The parting: terminal or bisect.** Against the same-lane reference it bisects: there is no proof that splitting fails. What decides it is the re-sort of today's 2,505 decided leaves against the reference: the share of parted leaves whose halves certify. The expected count is 0 parted (after the plate's repair). In that case the choice is one of principle and costs nothing.

**E6 text.** Keep the gate as ratified (record equality, a turned sign is a named `FlipCrossing`). Add: "A leaf is compared with the nominal's replay at the leaf's scalar over a zero-width box. That replay must be decided, and its log must agree with the f64 witness's, or the drive refuses naming the node. A record that differs with no decision turned is not proven and bisects." This is smaller than any earlier proposal. Ev's no-flips policy is untouched.

**Agreement.** I now agree with the other position's lead, with two additions: tie by the log, not by cells; and run the reference at the drive's own scalar. On the fallback, the third report and I agree (choices against f64, a parting terminal).

**Confidence.** Recommendation: likely. "A turned check can be the part's shape": likely, resting on the `edge_wedge` site. "A cell tie hides a lane contradiction": sure (M13 §2 and the measured `pcurve_chart_placement` turn). The cost of an undecided reference: unsure, measured next.

## For the orchestrator

- **Measure first, in order:**
  1. **The reference exists.** At the drive's scalar, at zero width, for every driven document (305 drives, 34 range searches, the tour): is it decided, and is its log equal to the f64 witness's (equal, a turn, or a parting at which node and step)? Count the drives that certify mass today while their reference is undecided. That count decides between refusing the whole drive and the fallback.
  2. **The six lane pairs traced to source** (in flight). Each fix shrinks the tie's refusals, and none should be accepted as a "parting allowed" at the tie.
  3. **The re-sort** of the 2,505 decided leaves against the reference, whole record and then choices only. Expected: 660 turned, the plate certified, 0 parted, and both sorts agreeing. Any leaf that moves class is the finding.
  4. **The extrude's escalation** ("no definite length across the sketch plane"), as before. It is the largest mass on the board, and it is independent of this fork.
- **File regardless of the gate:** the `pcurve_chart_placement` turn between lanes at one point. Under the recommendation it becomes the tie's refusal on the bracket drive, so it blocks that drive until fixed.
- **Build shape:** `drive` takes the reference once and compares its log with the witness's using `first_turn` with checks read. Then `classify_replay` compares each leaf's whole vector with the reference's, and one first-difference walk returns same, turned (choice or check, role carried) or parted. `Unsettled::Parted` joins the existing kinds, and `Budget` should carry the deciding node and kind. With one lane on both sides, the marks become comparable.
- **Correction to round 2:** its case for terminal `Parted` and for checks out of the gate rested entirely on comparing across lanes. I withdraw both under this reference, and keep them only as the fallback.
