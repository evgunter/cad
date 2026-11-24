## For Ev

**Recommendation, round 1 (unchanged, likely; the facts below strengthen it).** Gate a fully definite leaf on **the build**: per node, the same standing and the same name tables as the witness.
- **Equal:** certified.
- **Unequal:** `FlipCrossing`. The evidence names, by type, the first node that builds otherwise and what differs there (standing, or names gone or new). It also names that node's cause: the decision that turned there, or where its path parted (step, and query where a mark shows it), or the profile lift's structure flip.
- **The log** gates nothing and explains everything.
- **The kernel disagreeing with itself** is the zero-width parity instrument's to catch, plus a `debug_assert` for a leaf that holds the witness point and builds otherwise.
- **No parted class, no bisect-versus-terminal choice.** A leaf whose route differs but whose build is the same certifies after one replay.

**Terms.**
- *Witness*: the nominal run at `f64`.
- *Leaf*: a sub-box replayed at the box scalar.
- *Choice*: a decision whose other sign can change what a node builds.
- *Check*: a decision whose other sign can only make the node refuse.
- *Turn*: the same predicate, reached by the same route, decided with the other sign.
- *Parting*: any other first difference of two routes.
- *Mark*: a note of which candidates a pruning query handed on.
- *Name table*: the names each node output gives its cells, minted from lineage and structure, never from floats.
- *Lane choice*: a decision that picks one of two formulas for a result whose cells are the same either way.

**Three facts the reports disagreed on, read in the tree (all by reading; nothing run).**
1. **A check that passes with either definite sign exists (sure).** Tier 3 validates every finished body inside `k_stats::checking` (`topo::validate`, `AtRestBody::validate`). Its edge reading (`wedge_reading`) accepts a `Crease`, convex or concave alike; the sign behind it is `geom_brep::DIHEDRAL_SIGN`. So "choices agree, one check has the other sign" comes from geometry wherever construction asked nothing of that corner. Contrary to the earlier "no call site found", there is one. The convexity also steers nothing later: the contact mark it yields is read only by the at-rest must-carry rule and by tests.
2. **The extra candidates a box-scalar pruning query hands on are decided, and logged as choices (sure).** In `topo::boolean::ops::walk_pairs`, a face pair past the marked box test goes to `pair_verdict`, whose section predicates log in the default choice role. So `VerdictVector`'s doc ("decide nothing about them") is false, and the slid-block passage in `resolve/vdiff.rs` is right. Also, the next pruning step there, `separating::apart`, returns a bool with no mark. So a parting "elsewhere" (not at a mark) has a source today that is geometry or box width, not a kernel defect.
3. **A lane choice is logged as a choice (sure), so a turned one refuses a leaf whose cells are the witness's (likely).** Only five scopes in the tree record checks (`finish.rs`, `validate.rs`, `side_reader.rs`); everything else is a choice. Example: `geom_brep::intersect`'s plane×cone section decides `pn_axis_normal` through `closed_form_shown`. Zero gives the circle, definite gives the ellipse, one section curve either way (its own docs call it a lane choice). Take a document whose tilt parameter is 0 at nominal. Every leaf off 0 turns `pn_axis_normal` Zero→Positive and is refused as "proven different". That happens under the choice gate and the whole-record gate alike; only the build gate certifies it.

**On each other answer.**
- **The choice gate** (certify on equal choices; bisect at a pruning parting; any other parting a kernel defect).
  - I accept that checks out of the gate is right, and that kernel parity belongs to the instrument.
  - I reject two things, by fact 3 and fact 2. First, a turned choice is not a proof of a different build: a choice *can* change the build, and a lane choice never does. Second, a parting away from a mark is not yet a kernel defect, because `separating::apart` steers unmarked today. Calling it a defect means failing loud on documents until those rows close.
  - It also bisects pruning partings that geometry makes permanent (a slide that moves which boxes overlap), down to the budget.
- **The whole-record gate** (checks included; a turned check is a refused flip; partings bisect).
  - I accept its corrections: E6 is silent on, rather than decisive about, the parted case; `contained()` makes a false claim on empty flips; a `Budget` refusal drops the node it stopped on.
  - I reject a turned check as a finding (fact 1: a convex-or-concave crease is the witness's build).
  - I also reject "bisect every parting": a parting caused by geometry never re-aligns, so it spends depth-many replays per leaf and ends as `Budget` mass that means nothing.
  - Its role-stamp worry is answered in the tree for its own example. The side reader's rays and volume are checks, and the side it reads is recorded once as a choice, `side_read`, with its argument beside it in `side_reader.rs`. Under the build gate, role stamps don't carry the certificate at all: a wrong stamp can't produce a false certificate.
  - What the whole-record gate buys over the parity instrument: only defence against a check whose answer steers unlogged. Fact 1's search found none, and the build gate catches one anyway, because a steered build differs.
- **What a consumer is told about partings:** nothing separate. A parting with the same build is certified. With another build it is a `FlipCrossing` whose cause field says "parted at step k (query q)" instead of a turn. Floor cost: none beyond today's one replay.

**If the name tables do not state the whole build** (the one load-bearing unknown, unsure): then "the same build" has no record in the tree, and E6's "lineage-scoped key identity" claim has nothing under it either. The gate becomes a structural record built for this purpose: per node and output, standing, cell names, each cell's bounding cells, and the glued coincidences (`NodeValue::coincidences`). If that is judged too much, I'd fall back to the choice gate's final state with the whole-record gate's naming: certify on equal choices; a turn is a flip; any parting bisects with `Unsettled::Parted` carried to the floor. I'd accept its lane-choice false refusals as a known cost, recorded as a row.

**E6 text (short)**, replacing the leaf protocol's first two conditions: "Every predicate definite → certified if the leaf builds the witness's build (per node: standing and names; assertions excluded), refused mass (`FlipCrossing`) if it builds another, naming the first node that does and the decision that turned there, where one did." No-flips (Ev's proposal) is untouched: the far side is still refused unanalysed; only "far side" now means another build, not another route.

**Masses after.**
- *Certified*: proven the witness's build.
- *`flip_crossing`*: proven another build. This is the finding E6 means, and E2 containment is true as stated.
- *Sliver, budget, reads-no-variable*: not proven.

## For the orchestrator

- **To measure before a lane builds, in this order:**
  1. **Name-table parity.** On every corpus document, the witness's tables against a zero-width box-scalar replay at the nominal. Same standing, same tables? If not, the tables are scalar-dependent and the build gate is off (run the parity instrument for logs in the same pass).
  2. **No false "same".** For leaves either side of the corpus's and tour's known flips, every leaf whose route turned on a topological predicate has different tables. This is the false-certificate test, and it decides the unknown.
  3. **The payoff.** Today's `FlipCrossing` leaves on the corpus and tour, split into equal tables (would certify), turned, and parted (and at what: mark or unmarked step). Include a plane×cone with a tilt at nominal 0 if no corpus row has a lane-choice turn.
  4. Only if the gate falls back to choices: the share of parted leaves whose halves certify (the whole-record report's own deciding measurement).
- **Rows worth filing (off the question, agreeing with the other reports):**
  - `VerdictVector`'s doc claims are false: "decide nothing about them" by fact 2, and "cannot explain a difference it detects".
  - `separating::apart` is an unmarked pruning step that steers `pair_verdict`. Add it to the doctail list of unmarked filters if it is not there.
  - A `Budget` refusal drops the deciding node and the kind of unsettledness it stopped on.
  - `classify_replay`'s `_` arm sends a leaf that is definitely invalid to bisect and then to `Budget`.
  - `drive::contained` reads an empty `FlipCrossing` as containment.
- **Not checked:** whether `closed_form_shown`'s in-band arm leaves an escalation on the frame that `classify_replay` reads as a cue to bisect, which would make the in-band ellipse lane unreachable over a box (`geom_core::ladder::in_band` after `decide`). Worth a look by whoever owns `intersect.rs`.
- **F48** (the rounding lane's pass unlogged) stays right; under the build gate it carries nothing for certification, only log parity for the instrument and K.
