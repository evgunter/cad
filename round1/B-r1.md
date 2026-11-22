## For Ev

**Recommendation.** Where two faces share an edge that tier 3 has read as a crease, the embedding question should read no angle of its own. It takes that edge's wedge from tier 3's verdict, and over its cells it proves only a sign: which side of a separating surface each face lies on. The band is applied once, to the wedge, where tier 3 already applies it. Tier 3's wedge check should in turn hold over the whole edge, not at sample points. Where there is no crease to take (a smooth seam, an edge tier 3 does not judge, two faces sharing only a vertex, a pole, a face against itself), the question keeps a reading of its own, metered at the faces' reach as PR 7329 builds it. `likely`

This is your "unified" taken literally, and it is what PRs 6821 and 6893 already say ("no second reading"; the question "reads tier 3's facts from its verdict"). It is not the exact-sign option you declined on PR 7095: that dropped the band, and this keeps it and stops applying it twice.

**Terms.**
- *Wedge*: the angle two faces make along a shared edge.
- *Lever*: the length an angle is multiplied by to become a displacement, which is then compared with the band K·ε.
- *Reach*: how far the faces run from the edge. *Curvature arm*: a curved face's radius. Tier 3 levers the wedge at the least of the reach and the two curvature arms.
- *Cell*: a piece of the question's own subdivision of a face.
- *Separator*: a surface through the shared edge; the question shows the two faces lie on opposite sides of it.
- *Station*: one of the sample points along an edge at which tier 3 reads the wedge.

**How the two readings are related.** The separator passes through the stored edge, and the stored edge is only within ε of each face. So "this face is on the + side" is blind within about ε / sin(angle) of the edge. The band's job is to keep that blind strip small beside the faces: ε / sin θ ≤ reach / K is the same statement as sin θ · reach ≥ K·ε, which is tier 3's wedge check. That is the case you asked about (geometry ε off can matter here), and it needs one check per edge, not one per cell. Given a crease from tier 3, a sign over each cell finishes the proof whichever separator is used. `likely` (argued by hand, not run)

**The premise, corrected.**
- The gate does not ask the embedding question yet, on main or on the PR's branch. Its only production caller is the shell's cavity check, which asks before the result's tier 3 and only at the box scalars. Tier 3's verdict keeps no per-edge fact either: `AtRestBody` holds the body and check 7's certificate. Both hand-overs are approved and unbuilt (PR 6893; the program page has them landing "with lane C"). So the PR had no verdict to read a wedge from. `sure`
- What the PR bands per cell is the edge's wedge under only one of its three kinds of separator. `sure` (read in `departure`, `planes_through`, `bisector`)
  - The other face's own surface: the sine is the wedge's.
  - A plane through a straight edge, bisecting the two faces: the sine of half the wedge.
  - A circular edge's own plane, or the edge's graph: unrelated to the wedge. A capsule's cylinder and dome meet smoothly on a circle. Tier 3 reads the angle between them as zero, a smooth seam. The question reads each face leaving the circle's plane at a right angle: "1 × reach", far clear of the band.
- So the number is the angle to a helper the engine picked. Banding it is legitimate (it bounds the blind strip), but it is tier 3's wedge only sometimes, and then with a different lever.
- The disagreement in the reviewers' table is real (`sure`, their probe), and the code already warns against it. `folded_lever_arm`'s doc: "A new site levering against its own fold is a silent non-comparability, so route new callers through this function." `edge_wedge.rs` exists so that "what a constructor stores and what tier 3 accepts are read from one verdict", and a test holds the blend to reading the wedge "through the one fold". The embedding question is the one reader built outside that door.

**The final states.**

*A. One reading (recommended).*
- Tier 3 keeps each edge's wedge verdict on the finished body, and the question takes the finished body (PR 6893's shape).
- At a crease the engine has no lever. "Reach alone, or curvature arms too?" stops being a question about the engine, and so does the apex: in a cell at an apex it proves the sign of one derivative, and nothing is multiplied by an arm that reaches 0.
- A near-tangent edge gets one answer. The reviewers' 1 mm cylinder at a sine of 5e-7: tier 3 says tangent, and the engine cannot say crease.
- The engine cannot refuse as ambiguous a wedge tier 3 accepted, and it forms no second opinion of one.
- Tier 3's wedge check reads the whole edge, by enclosures over stretches of it, as its own filed row proposes. Until it does, a wedge that dips into the band between two stations without crossing passes. That hole is tier 3's today, for every reader of the wedge; under A the engine stops covering part of it with a second lever.
- That whole-edge reading has to say what the lever is on the last stretch before a cone's apex, where the curvature arm goes to 0 and every angle reads in band. Tier 3 passes cones today only because no station lands there. I lean: no reading on that stretch, known by the apex vertex tier 3 already places. `unsure`. Under A this is one question in one place; today it is hidden by where the samples fall.
- Still possible: on an edge tier 3 does not judge (beside a spline face) the engine's own reading is the kernel's only wedge check, as now.
- Reversible: yes. The engine's own reading stays in the code for the cases with no crease.

*B. Two readers, one function.* The engine keeps reading per cell, but calls tier 3's own wedge function on the cell's stretch of the edge, curvature arms included.
- Gives whole-edge coverage of the band now, without changing tier 3.
- Leaves two deciders of one fact: a station's point value and a cell's enclosure can differ, so a body can pass tier 3 and then be refused as ambiguous on the same wedge.
- Needs an ε concession at every apex, inside the engine.
- Only the first kind of separator can use it; the other two keep a different number.
- PR 6893's sentence ("never re-deciding one over a cell") has to change.

*C. As PR 7329 builds it* (the engine levers at the reach alone; tier 3 at reach and curvature arms). Two levers for one angle. No wrong body passes only because tier 3 also runs. I would not keep it as a final state. It is a fair step toward A: it removes the sliver you ruled out and stops levering at a cell.

**Where there is no edge, and the five other margins.** One rule covers them. A sign the question proves needs a rate that an ε error cannot turn. That rate is either a fact tier 3 has decided, read from the verdict, or the question's own reading levered at an extent of the body, never at a cell.
- From the verdict: an edge's crease; a cusp edge, which the engine today re-decides from a cell's size.
- The question's own: two faces' reach from a vertex they share alone; a pole or apex; an edge's length for how an edge advances.
- The PR's vertex and pole readings fit this as built, and the five need no separate ruling. `likely`; I did not read each of the five sites.

**Text I would change.** In `[topology.tiers]`, the sentence "That reading is the edge's wedge as tier 3 reads it, one reading for both: an angle metered at the reach…" is agent-written from your reply "yes i think so, presumably unified with the other case". It lets "the same recipe, decided again in every cell" count as one reading. I would have it say:
- at a shared edge tier 3 reads as a crease, the reading is that wedge, taken from the verdict, and a cell proves only a sign;
- elsewhere the reading is the question's own, an angle metered at a reach of the faces it governs, never at a cell.

And in tier 3's page: the wedge is read over the whole edge.

**The reach's site.** My answer does not depend on how it is settled. Under A the wedge has one lever in one place, and whatever is settled applies there once.

## For the orchestrator

**Errors in the brief.**
- "the verdict keeps tier 3's per-entity facts beside its certificate…" is DESIGN's sentence (PR 6893), not `validate.rs`. `AtRestBody` is `{ body, sign, held }`. `validate_geometric` drops check 4's marks; only the `contact_marks` pass returns them.
- "A finished solid passes tier 3 and then the embedding question" is the ratified target, not the tree. `embedded` has one production caller, `shell::cavity_embedded`, behind `CavityGate` (asks at `Interval` and `Sym` only), on the cavity clone before the result's tier 3. So "check 4 gates first" is true of the gate harness branch, not of that caller: there check 4 runs after, in `shell_open`'s final validation.
- "refuses every cone that owns its apex": the row measured one test (`a_revolved_cone_holds`); the rest is inferred from the arm enclosure reaching 0.
- The table's "check 4" column is `classify_dihedral` at one point with a 1 m reach (the reviewer's probe), not check 4 on a built body.
- Item 2 ("main did the same, with the cell's span") checks out: main's `sign_on` bands `grad·Φ_a × span`.

**Is this a new fork?** Mostly not. Fork 260 (PR 6821: "decisions about the body stay at `T`", no second reading), fork 264 (PR 6893: the verdict is the strict question's input, the cusp reads `End`) and fork 285 (PR 7095: one reading with tier 3's wedge) read together give A. What Ev has not been asked:
- a bare sign at creases, given the verdict. It looks like the option declined on 7095, so the PR body must say why it is not;
- check 4 over the whole edge, and the stretch before an apex;
- the sentence.

**Sequencing (yours, not Ev's).** PR 7329 can land as a step if its docs stop claiming one reading: `sign_on`'s margin paragraph, `reach_over`'s "every reader", the `wedge_reach.rs` module head, the RESTFRONT row. Then:
- A's engine half needs lane B's rest: the wedge verdicts kept, `embedded` taking the bundle.
- Shape for `sign_on`: in place of `arm: Option<Lever>`, a licence that is either a crease built only from the verdict's `WedgeVerdict::Crease` or a lever. Then "a bare sign with no crease" cannot be written.
- Until the shell's check retires into the gate (fork 264's final state), that caller can ask `topo::edge_wedge` once per edge: the same door, so the same answer tier 3 will give.

**The hand argument, for a reviewer to attack.** Rates r_x, r_y > 0 over the two rectangles, with the zero side within ε of the separator. A common point then needs ∫r_x + ∫r_y ≤ 2ε. At a crease the larger rate at the edge is at least sin θ / 2, so a common point lies within about 4ε / sin θ of the edge, at most 4/K of the wedge's lever.
- Weak point: a rate that decays just past the edge on a tightly curved face. The argument then leans on tier 3's lever folding the curvature arms, which is one more reason the engine's reach-alone lever should not be the one that counts.
- It fails at a smooth seam (both rates can be small together), which is why the engine keeps its own banded reading there.

**Measure before building A.**
- How many edges tier 3 classes `Seam` or `End` where today's carrier arm certifies at first order.
- The exact-sign proxy on the sliver row ("no verdict moves in the tour", §7) predates this PR and had the sliver on; re-run it.
- Removing the band at creases can only turn refusals into holds or expose a crossing; count which.

**Off the question.**
- The banded number depends on which separator presented. With the bisector plane each face reads sin(θ/2), so a wedge between K·ε and 2K·ε at the lever certifies by the carrier arm and refuses by the plane arm. By inspection, not run. Belongs on `the-departure-is-levered-at-the-reach-alone…` or a row of its own.
- `wedge_reach`'s "the reach is the farther face's" against `[topology.census]`'s "over the shorter cell": a thin face beside a large one is levered at the large one (review 2's 20ε slab). It belongs with ONEROUND's site fork; say so on that row.
- `cusp` decides "normals opposed" as a `Zero` at the cell ball's reach, the fact tier 3 holds as `End`. Already on the program page and the five-margins row; under A it is the first site to convert.
- Tier 3's own "does NOT yet check" list names the wedge on spline-adjacent edges, and `classify_seam` is called only from `pcurves/mint.rs`. Under A that gap shows in the embedding question; it wants a TOPO or RESTFRONT row if it has none.

**Not checked.**
- I built and ran nothing. Every measurement cited is the rows' or the reviews'.
- In `injective.rs` (13,117 lines on `origin/embed/remeter`) I read the module docs, `sign_on`, `departure`, `arm_of`, `planes_through`, `bisector`, `cusp`, `monotone`, and the tail of `graph_of`. I did not read the `present_*` readers, `leaves_graph`, `leaves_line`, `monotone_run` or `zero_of`.
- I did not read PR 7262. I took review 1's account of `LeverBounds`.
- `git fetch` failed to update two refs unrelated to this (`embed/corner-pieces`, one `analysis/model-compare` branch). `origin/embed/remeter` read at `404bacbf05`, `origin/main` at `e44e340032`.
