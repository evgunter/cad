round 4: a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing

## For Ev

**Recommendation (likely): I now agree, as it stands, with the gate that compares each leaf with the f64 witness on choices and standings.**
- **Certified**: every predicate is definite, every node stands as at the witness, and the choices match the witness's one for one. The leaf's checks must be definite and passing, but they are not compared.
- **`FlipCrossing`**: a choice turned. It is named, and non-empty by type.
- **`Parted`**: the choices part with no turn. The leaf is refused at once, as its own class, naming the node, the step and what each run logged there. It is never a flip.
- **Kernel self-disagreement**: two lanes that disagree at one point are a test's question. A test at a zero-width box on each driven witness catches it.
- **Shape as a choice**: anything that is the part's shape is recorded once as a choice where it is decided, never only as a check.

The same-lane reference is not needed unless the trace now running finds a decision that only the box lane makes and that steers the build (below).

**What changed from round 3, and why.** Two of the other side's arguments are right, and I cannot answer them.
1. *My tie was lane parity under another name.* I tied the reference to the witness by the log: same choices, no parting. When that tie holds, comparing a leaf with the reference on choices is exactly comparing it with the witness. So on choices the reference bought nothing. The other report's tie tolerates partings, and that admits a reference whose route the f64 build never took. What was left was checks and partings.
2. *Checks are not one kind of thing (sure, by reading).*
   - `topo::side_reader::side_at_infinity` casts its rays in a check scope. Its own doc says that `side_at_infinity_order` "is a pair's gap, geometry, and its other sign moves a ray between reading and refusing on that pair".
   - So two unrelated faces can swap order along one ray while the side read, recorded once as the choice `side_read`, is unchanged. In a whole-record gate that turn of a check is a false `FlipCrossing`.
   - Meanwhile a crease's `dihedral_sign` read as a check (`geom_brep::edge_wedge_over_as`, the constructors' `Role::Check`) is shape.
   - The role cannot tell these apart. The tree already shows the right repair: where something the part depends on is a function of checks, record it once as a choice (`k_stats::answered`).
   - So a convexity that only a check reads is a site to fix, not a reason to put checks into the gate.

With both gone, the case for bisecting a parting goes too. Against the f64 witness the measured partings are the two lanes logging different work at one point, at zero width, none of them at a pruning query. No split re-aligns that.

**Terms.**
- *Witness*: the nominal at f64, the build the user sees. *Leaf*: a sub-box, replayed over interval arithmetic (the box lane). *Reference*: the nominal replayed in the box lane at zero width.
- *Choice*: another sign can change what is built. *Check*: another sign can only refuse, or an instrument's reading on the way to an answer. *Standing*: built, failed, poisoned or absent.
- *Turn*: the same question, reached the same way, with the other sign. *Parting*: any other first difference.

**The recommendation as a function of the four measurements.**
- **(3) the six lane pairs, labelled:**
  - *All (a)* (one lane logs work that steers nothing): each is fixed at its site, by logging it in both lanes or in neither. The gate above stands, and `Parted` mass falls to zero as they land.
  - *Any (c)* (a narrower box removes the parting): bisect that parting's kind only, `Parting::Mark` if it parts at a mark. Everything else stays terminal. One arm.
  - *Any (b)* (a box-only decision that steers the build): first make that decision in both lanes. The f64 lane can decide any question the box lane asks at a point. If it cannot be made in both, then for that document the f64 build is not a faithful reference, and the same-lane reference, tied by choices, becomes the coherent answer. That goes back to the designers with the site in hand.
- **(1) the reference on every driven document:** it matters only if (3) finds a (b). Then, if the reference is undecided on drives that certify mass today, the same-lane reference costs certificates. Weigh it against making the decision in both lanes instead.
- **(2) the re-sort, whole record against choices only:**
  - *The sorts agree on every leaf*: checks out costs nothing today. The choice is then one of principle, and the principle is the side reader's.
  - *They disagree*: read each leaf that moved. A `dihedral_sign` turn means a site to record as a choice. A ray turn confirms checks out.
- **(4) a decided leaf with equal choices and a turned check:** a `dihedral_sign` is a P1 site row (until it is fixed, that leaf certifies a notch). A ray decision is no defect.
- **What I expect:** every pair (a) or close to it, the sorts agreeing, 0 turned checks. That gives the gate above.
- **If nothing could be measured:** the gate above, today.

**What all three agree on: the orchestrator's list, corrected.**
- Cells cannot gate, and cannot tie a reference to the witness.
- A turned choice is a named, non-empty `FlipCrossing`.
- A parting is never a flip and is named wherever it ends.
- A standing change is a flip only when the failure is proven definite. None is today.
- The extrude's undecided length ("no definite length across the sketch plane") goes through the funnel.
- `is_empty`, `FlipUnfounded` and " unnamed" go.
- *Correction:* "the lanes' contradiction at one point is refused out loud" holds only partly under the agreed gate. `pcurve_chart_placement` (Positive at f64, Negative in the box lane, at one point) refuses the leaves around the nominal, but as `FlipCrossing`, which is a document's finding, misattributed. Add: a turn on a leaf that holds the nominal point is a kernel defect (`[failure.taxonomy]` row 5, a `debug_assert` in the drive), and the parity test lists turns between lanes separately from partings. Away from the nominal, a lane turn cannot be told from a wall without a reference. The parity test is what finds it.
- *Addition:* the certified claim is "the nominal's form", not "the nominal build's topology". That means the same cells, every decision that shapes the part answered alike, and only dimensions moving. Each measure encloses its value. Not promised: an assertion's verdict, a coincidence's discharge kind, a lane choice's spelling.

**E6 text** (the others' wording):
- Certified: "choices and standings matching the witness's; checks are the leaf's own".
- No-flips: "a choice turned is `FlipCrossing`, named; a record that parts with no turn is refused `Parted`, never bisected".
- Ev's no-flips policy is untouched.

**Confidence.**
- The gate above: likely.
- "Checks mix shape and an instrument's route": sure (the two sites).
- "A log tie makes the reference redundant on choices": sure (by construction).
- "Terminal over bisect": likely, pending (3).

## For the orchestrator

- **I agree with C's round-3 report as it stands**, adding the correction above: a turn on a leaf that holds the nominal is a kernel defect, and the parity test separates turns from partings. My round-3 reference and its tie are withdrawn.
- **Ask the trace for a, b or c per pair**, not only a source, as C asks. For any (b), add whether the f64 lane can decide the same question at a point. That answer decides between "decide it in both lanes" and the same-lane reference.
- **Rows, agreed across the reports:**
  - `drive::contained` trusting an empty flip.
  - `Budget` dropping the deciding node and kind.
  - The extrude's failure outside the funnel.
  - `VerdictVector`'s false doc sentences ("decide nothing", "cannot explain").
  - `closed_form_shown` logging a lane choice as a choice (unobserved on a decided leaf).
  - A convexity read only by a check, to be recorded as a choice where it is shape.
  - `pcurve_chart_placement` turning between the lanes at one point (P0-shaped).
- **Not checked this round:** whether any finished-body convexity is read only in a check scope with no choice deciding it upstream. That is (4)'s question, and an audit of the `edge_wedge_over_as(..., Role::Check)` callers would answer it by reading.
