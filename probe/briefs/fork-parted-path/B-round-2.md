## For Ev

**Recommendation, round 2 (changed; likely).** Gate a fully decided leaf on its **choices and node standings**, not on the build's cell record and not on the whole log.
- **Certified:** every node stands as at the witness, and the leaf's choices equal the witness's, choice for choice. Checks must come back definite and passing, but they are not compared.
- **`FlipCrossing`:** a choice turns (same predicate, same route, other sign), or a node's standing changes with its cause proven. The evidence names this by type, so it can't be empty.
- **`Parted`:** the routes part with no turn. This is a new reason in the unsettled family. It is refused at once, not bisected, and it names the node, the step, and what each run recorded there.
- **The kernel's lanes disagreeing with each other** is a test instrument's question (choice parity at the witness), not a mass of a document.

**What changed and why.** In round 1 I recommended gating on the build: cells, incidence and names. The measurement shows that record cannot be the certificate. Every one of today's 661 `FlipCrossing` leaves has the witness's record, among them the tree's planted notch and a crease that goes from convex to concave. So a cell record would certify a notch as the nominal's corner. And the record adds nothing to the choices: the search for decided leaves with equal choices and different records found none. My premise was that the log can't prove "different" and the build can. It is false: the turned choices are what state the difference, and the cell record does not.

**Terms.**
- *Witness*: the nominal run at `f64`.
- *Leaf*: a sub-box replayed at the box scalar.
- *Choice*: a decision whose other sign can change what a node builds.
- *Check*: a decision whose other sign can only make the node refuse.
- *Standing*: built, failed, poisoned or absent.
- *Turn*: the same predicate, reached by the same route, decided with the other sign.
- *Parting*: any other first difference of two routes.
- *Cell record*: per node, its standing and its bodies' named faces, edges, vertices and their incidence.

**1. Is "a turned choice with the same cells" the witness's part? No (sure).** A crease that is convex at the nominal and concave in the leaf is a corner become a notch. An extrusion whose normal component turns lies on the other side of its sketch plane. A designer holding a tolerance study would call both a different part, and every downstream reader agrees: a blend of that edge goes the other way, and every measure moves through a degenerate value. Topology was the wrong word for what E6 means. The right notion is a part of the same *form*: every decision that shapes the part is answered alike, and only dimensions move. The decision log is the one place the kernel states that, because by DESIGN `[verdicts]` every such decision is a named predicate. Restating it outside the log would mean copying the choices into another record.

**2. What a certificate claims (likely).** In words a tolerance study can be held to: *at every point of this leaf the document builds, and builds a part of the nominal's form. The same cells and incidence; every convexity, side, class and configuration the build decided is decided as at the nominal; only dimensions move. Every measure's enclosure contains its value at every point.* Against M13's list of what the cell record leaves out:
- **Owed, through the choices:** a crease's convexity, the side an extrusion lies on, a measure's configuration (`carrier_pose_*` choices), carrier kinds.
- **Owed, through enclosures, not identity:** positions.
- **Not owed by the certificate:** an assertion's verdict, which is reported per leaf by E10's own rule (the 82 certified leaves reading `Violated` are correct reports, not false certificates). Also not owed: a coincidence's discharge kind (`Proven` against `Numeric`), which the coincidence report owns; the glue is the same.

What becomes of the three cases:
- `m10_3`'s planted notch stays a `FlipCrossing` naming `path_junction_turn`.
- The sign-changing extrusion is a flip naming `extrusion_normal_component`.
- The range search's boundaries stay where they are: where a shaping decision turns.

Proposed E6 text: replace "shares the nominal build's topology" with "builds the nominal's form: every choice and standing as at the witness". Replace the no-flips condition with "a leaf on which a choice turns (`FlipCrossing`, the turn named)". No-flips (Ev's proposal) is otherwise untouched.

**3. What `FlipCrossing` is for (sure).** It is the leaf proven to be another form, named by the decision that turned. That is today's 660 flip leaves, all genuine. The 661st is the plate, which parted in check sequences only and now certifies.
- **Standing changes:** a standing change counts as a flip only when the failure is proven definite over the leaf. No log carries one today.
- **The 27,781 leaves** with a node that fails where the witness builds and no escalation logged are unsettled and should stay so; 27,667 of them are the one Extrude sentence "no definite length across the sketch plane". Their defect is at the source: that failure is an undecided sign that does not go through the funnel. Once it is routed through the funnel it escalates and bisects as what it is, with no protocol change. Calling it "builds another" would refuse 2.02 of mass that certifies below it today.

**4. Partings: why terminal, not bisected (likely).** On decided leaves there was 1 parting in 2,505, and it was checks-only, which the choice gate certifies. None parted at a pruning query. Between the two lanes at one point, choices part in 2,106 builds, at box-lane-only decisions (`session_box_reach`, `mate_chart_slides_span` against `pose_unit_norm`, and others). That is the kernel's lanes disagreeing, and no split ever re-aligns it: bisecting would spend the leaf budget to end as `Budget` mass. So a parting is refused at once, typed and named, with its mass meaning "not shown either way", in a class of its own beside budget and reads-no-variable. If a pruning-query parting that a narrower box cures is ever measured, bisecting `Parting::Mark` alone is a one-arm change.
- **Lane choices:** the one case the choice gate would get wrong is a turned lane choice that leaves the form alone (circle against ellipse). It is unobservable today: the only document that reaches it decides no leaf. I would not design for it until it is seen.

**Masses after.**
- *Certified*: proven the nominal's form.
- *`flip_crossing`*: proven another form, named.
- *`parted`, sliver, budget, reads-no-variable*: not proven.
- E2's containment holds as stated, because only named turns are flips.

**Confidence.** Recommendation: likely. "The cell record is implied by the choices and misses the form": sure, as measured. Terminal over bisect for partings: likely, resting on M13's measured cause.

## For the orchestrator

- **To do first, in order:**
  1. **Choice parity as a test instrument**, at a zero-width box on each driven document's witness. It compares choices and standings and reports checks without failing. Trace M13's 2,106 lane partings to their sources. Each box-lane-only decision is either unlogged under F48's principle, or decided in both lanes. Otherwise every such document parts at every leaf the day it is driven.
  2. **The Extrude failure** "no definite length across the sketch plane", 27,667 leaves. Find the sign decided outside the funnel and route it through it, so the failure carries its escalation. This is the largest mass on the board, and it is a source defect, not this fork's.
  3. **Then the gate:** compare choices and standings, so `certifying_vector` drops checks; build `FlipCrossing` evidence non-empty by type; add `Unsettled`/`RefusalReason` `Parted` (terminal); delete `FlipEvidence::is_empty`, range's `Unfounded`/`FlipUnfounded`, its viewer sentence and Python tag, and `render_reason`'s " unnamed". Re-run M13's flip census: the expected outcome is 660 flips unchanged and the plate certified.
- **Off the fork, likely bigger than it:** the box lane escalates at the nominal point in 22% of builds (54% of the tour's), mostly `curve_pair_pose_parallel`, `germ_side`, `chart_singularity_nearer`. Such documents decide no leaf at any width, under any gate. That deserves its own row, cut by first escalating predicate.
- **The cell record** (`build_record.rs`) should not be built into production: the gate doesn't need it, and by `[simple.checks]` nothing is kept only for a check. It remains a probe for scalar-independence of names if anyone needs that again.
- **The two lanes' `pcurve_chart_placement` turn** at the tour's bracket (Positive at `f64`, Negative at `Interval`, same point) is a turn at one point: a kernel defect by construction. File it.
- **My round-1 fact 3** (a lane choice turns and refuses a same-form leaf) stands as a reading of the code, but M13 could not observe it; drop it from anything that goes to Ev as evidence.
