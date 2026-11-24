round 2: a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing

## For Ev

**Recommendation (likely): a leaf certifies on its route of structural choices, not on the cells it builds.**
- **Certified**: every predicate is definite over the leaf, every node has the witness's standing, and every node's choices match the witness's one for one, signs included. A leaf's checks are its own: each must be definite and passing over the leaf. They are not compared with the witness's checks.
- **Turned**: the first difference is a choice decided with the other sign. The leaf is refused as `FlipCrossing`, named, and the evidence is non-empty by type.
- **Parted**: the first difference is any other difference. The leaf is refused at once as **`Parted { node, step, what each lane logged there }`**: a refusal class of its own, counted as mass of the kernel, not of the document. It is not bisected.

**What changed since round 1, and why.**
- In round 1 I moved to "certify when the leaf builds the witness's cells". The measurement withdraws that move.
- The cell record is the same at both scalars. But it holds the witness's cells on all 660 of today's turned flips. That includes the project's own planted flip (`m10_3`'s notch: a profile vertex that is convex below zero and a notch above it) and an extrusion that changes which side of its sketch plane it lies on.
- A tolerance study that called the notched part "the nominal part" would be wrong in the words an engineer uses. The record of cells is missing exactly what a structural choice decides. Adding that back means adding the choices, which is the decision log again.
- So I am back to my first gate, with one change. In the first report a parting bisected, and was a kernel defect at the floor. Now a parting refuses at once, with its own name. The measurement shows partings never occur at a pruning query: 0 of 2,311. They already appear at a zero-width box on the nominal point, so no narrower box can cure them.

**Terms.**
- *Witness*: the nominal run at f64. *Leaf*: a sub-box, replayed over interval arithmetic.
- *Choice*: a decision where another sign changes what is built or named. *Check*: a decision where another sign can only make the node refuse.
- *Standing*: built, failed, poisoned or absent.
- *Parting*: two runs' logs first differ in something other than one predicate's sign: another predicate, or one log ending first.
- *Lane choice* (DESIGN `[verdicts.consumers]`): a reading that only picks the closed or general form of one result.
- *Mass*: the share of the box's probability, counted per reason.

**1. Is the measured outcome right? No (likely).** A leaf across a turned structural choice is not the witness's part, even when its named cells are unchanged. Its convexities and its sides are different, and those are what the recipe decided. The record of cells is complete about combinatorics and silent about orientation, so it is the wrong notion of "the same build". Under my gate the planted flip, the sign-changing extrusion and the range search's boundaries stay what they are today: named `FlipCrossing`s and the edges of proven ground.

**2. What a certificate claims.** Proposed E6 wording: "At every point of the leaf the document takes the nominal build's decisions: the same cells, names, convexities and sides. Every measure enclosure contains the true value at every point." Against M13's list of what the cell record leaves out:
- *A crease's convexity, the side an extrusion lies on, which configuration a measure reads (`carrier_pose_*`)*: **owed**, to every consumer. They are choices, so the route carries them.
- *Positions*: **not owed** as equal. They are what the measure enclosures bound.
- *Carrier kinds or closed-form spelling (lane choices), and a coincidence's discharge (`Proven` versus `Numeric`)*: **not owed**. They say how a result is stated or proven, not what the part is. STEP export sees the form, but a certificate is not a promise about an export spelling.
- *An assertion's verdict*: **not owed by the certificate**. E10 reports it per leaf, as today.

So a lane choice must not be logged as a choice. Today `geom_brep::intersect::closed_form_shown` reads through `decide` in the caller's role, which defaults to `Choice`. That is a site defect against `[verdicts.consumers]` ("records nothing"), fixed where it is read. It is not a reason to change the gate. M13 could not observe one turning on a decided leaf. (sure that the site logs; likely that it is a choice at its callers)

**3. What `FlipCrossing` is for.** A turned choice: 660 of today's 661 refusals. The class does not empty out. The 27,781 leaves where a node fails while the witness builds are not "builds another". No failure on them is proven definite, and 13 such boxes hold certified mass below them, so they keep bisecting as today (`Unsettled::Failed`). 27,667 of them are one sentence: an extrude with "no definite length across the sketch plane", failing with no escalation on its log. That decision should go through the funnel, so that it escalates where it straddles and reads definite where it does not. Then the drive reads it as it reads any other. A failure proven definite over a leaf where the witness builds would be a finding of the document, like E8's `Infeasible`. No log carries one today, so I would not build a class for it yet. (likely)

**4. The fallback, and why `Parted` is terminal and named.**
- The measurement moves me to the fallback, with a terminal parting. At one point the two lanes' logs differ in 27% of fully decided builds: 2,106 at a step or an end, 203 in checks only, 0 at a mark.
- A parting is therefore the kernel's two lanes logging different work. The first differences seen include a box-lane-only `session_box_reach` where f64 logs `pose_unit_norm`, and the plate's premise checks. It is not geometry that a narrower box resolves.
- Bisecting such a leaf burns the leaf budget down to `Budget`, which lies about the cause. Calling it a kernel bug (`unreachable!`) at a 27% rate would crash ordinary documents.
- A named refusal says the true thing: "not proven, because the kernel disagrees with itself here, at this node and step". It costs one replay, and its mass is a kernel column the K-style receipt can track down to zero.
- Checks stay out of the gate. Of the 203 builds that part in checks only, every one already has the witness's choices and cells. 0 decided leaves turn a check.
- **E6 bullets to change**: the certified bullet becomes "choices and standings matching; checks are the leaf's own". The no-flips bullet becomes "a choice turned (`FlipCrossing`, named); a log that parts with no turn is refused `Parted`, the kernel's, never bisected". Ev's no-flips policy itself is unchanged.

**Consumers.**
- The range search's `FlipUnfounded` becomes the driver's `Parted`: a floor, named. `flip()`'s filter goes.
- The viewer's sentence becomes "the kernel's two lanes disagree at <node>". The Python tag `flip_unfounded` becomes `parted`.
- `render_reason`'s " unnamed" and `FlipEvidence::is_empty` go.
- `drive::contained` reads `FlipCrossing` only, which is then true as stated.
- The tour does not change: M13 shows it has no flips and no partings.

**Confidence.** The recommendation: likely. "The cell record cannot be the gate": sure, by M13 §2. "Partings are lane disagreement, not width": likely, since 0 at a mark and they are present at zero width. The 2,106 first differences are not yet traced to source (below).

## For the orchestrator

- **Correction to my own round-1 report:** its "move to B" is withdrawn on the measurement. Its three tree facts stand: a check passes with either sign; a box query's extra pairs decide; a lane choice is logged in the caller's role.
- **To measure or build first, in this order:**
  1. **Trace the parity instrument's first differences to source**: `pose_unit_norm`/`session_box_reach` 1,348, `mate_chart_slides_span` 625, `ssi_hull_sup`/`pcurve_projected_lever` 51, `bool_point_in_solid_*` 31, `path_leg_length`/`trace_plane_*` 21. Classify each as lane-specific logging, fixed at its site as the F48 ruling did, or as something else. If any is a parting a narrower box cures, the terminal `Parted` is wrong for it, and that should be known before the build.
  2. **Land the zero-width parity row as a test** on the corpus witnesses, whole log and choices. It is the instrument that keeps `Parted` mass falling.
  3. **The extrude's "no definite length across the sketch plane"**: find why it fails with no escalation, and route it through the funnel (a row, P1).
  4. **Audit the `closed_form_shown` lane-choice sites** for the role they log in, and take them out of the choice log (a row).
  5. **Then build**: the choices gate, `Turned` as a named `FlipCrossing`, terminal `Parted`, checks out of the gate, and the consumer changes above. Re-run M13's §4 table under it. The expected result is `FlipCrossing` 14.92 unchanged, with the plate's one leaf moved to certified.
- **Not mine, but noted:** 22% of builds escalate in the box lane at the nominal point (54% on the tour), so no leaf of them decides under any gate. That is the undecided-at-rest row's territory, and it bounds what this fork can move.
- The 82 certified leaves whose assertion reads `Violated` against `Holds` at the witness are E10's by rule. They are worth one line in this item's PR body, so nobody reads them as a certificate defect.
