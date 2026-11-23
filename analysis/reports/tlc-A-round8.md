## For Ev

Changed. Two things move.

**1. The text.** Read against your answer on PR 7410, passage (A)'s last sentence, "No consumer takes one bound.", points at a mistake (each caller choosing a bound) rather than saying what a lever is, and so does the phrase "whatever acts on the verdict". The sentence before them already says it all: "a verdict is served only where it holds at every length between the two".
- I would drop both. (A) then reads, after "no longer than the site's own measure of it:"

  > a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points of the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one. Where a face is not built yet nothing proves a Zero over it.

- (B) and (C) stand as agreed. Each sentence of (B) says what a lever is and why a point proves only a Zero. Its last sentence, on records, is your PR 7141 text with "points" made "sites".

**2. What the change does: the line site needs a better bound from below.** The run that reads straight edges from their line (G1) found something the earlier measurement at the edge's middle could not.
- A face whose corners all sit on the edge's line reads a lever of about 0 from that line, however far its curved boundary runs from it. Examples:
  - a full cylinder wall whose only vertices are on its seam;
  - a plane face bounded by arcs whose ends sit on the line.
- So the bound from below must read each boundary edge's farthest point from the line, not just the corners. This is closed form for lines, circle arcs and ellipse arcs, and sampled otherwise; every sample is a point of the face, so it stays sound.
- I had recommended not building this, on a measurement taken at the middle, where it cured nothing. At the line site it is needed, and it belongs inside the same decision as the line site.

**What you would be shown with the question.** One rerun first (G2, below), then each class of moved verdict, each judged against its truth:
- **Two planes with straight boundaries.** The truth is exact. Every moved verdict moves toward it. The faces of no width (all corners on the line) now refuse at the arm check, which is right; 7 tests that pin the old refusal change.
- **Two planes with arc boundaries.** The truth is exact: an arc's largest distance from the line. Today 516 verdicts move away from it, all refusals the truth would not make. G2 must show these gone.
- **A cylinder or cone wall along a straight edge.** No truth was measured. A closed-form truth exists for walls bounded by circles and rulings (the farthest ruling from the edge across the wall's angle), and G2 builds it.
  - In G1 these served wrong smooth verdicts (a corner read as a seam on a lever of 6e-7). That came only from G1 letting every caller read one bound. Under the two-sided rule, which lands first, they become refusals.
  - G2 shows how many remain once the curved boundary is read.
- **The open row's extrude profile** (`honeone/the-wedge-reach-counts-a-faces-run-along-its-edge`).
  - The line site reads the expected lever, 0.05. The wall join is then read as the declared tangency.
  - At 15ε and 19ε, where today a sharp edge is built and tier 3 passes it, the edge is now refused by certification's own normal-parallel check, levered at the arc's 1 m curvature arm.
  - You should see that this move is from built to refused.

**Order of work: unchanged, with one addition.**
1. The two-sided rule, the typed not-built face and lazily read covers land first, with no README change. G1 shows why this must come before the line site: with one bound, the line site serves wrong verdicts.
2. The line site, with the curve-reading bound from below, joins the `[ev]` PR after G2 and your answer.
3. Curved edges and the joint turn follow once their truths exist.

The coarser gate stays not built.

## For the orchestrator

- **What moved me:**
  - G1 §1 and §4: the lever reads 0 on faces whose vertices sit on the line, and the two bounds then disagree on exactly those bodies.
  - G1 §2: wrong smooth verdicts were served because callers still took `.below()`.
  - The M-d "not built" ruling rested on the middle site, where a vertex on the line still stands away from the pivot. It does not transfer.
- **What does not move me:**
  - (B)'s inequality: G1 confirms it exactly on straight-edged plane pairs.
  - (C).
  - The order: unit 1 before the line site is now evidenced, not just argued.
- **What (A)'s "from below off points of the region" owes at the line site:** soundness is met by any points, the vertices included. Completeness is not met by vertices alone. The farthest point of each boundary edge from the site is what the bound from below must read. That is code, not text.
- **G2, needed before the `[ev]` PR opens.** G1's build with:
  - (i) every caller deciding at both bounds (unit 1's rule);
  - (ii) the bound from below adding each boundary edge's farthest point from the line (closed form for line/circle/ellipse, M-d's 17-point sample otherwise);
  - (iii) the closed-form truth for cylinder and cone walls bounded by circles and rulings.

  Report:
  - test outcomes changed, against the middle-site run;
  - moved actions per class, against the truth wherever one exists;
  - straddles per scalar decision;
  - the symbolic lanes' cost of (ii).

  What would change my recommendation:
  - any served verdict past the truth (expected 0): withdraw the line site until found;
  - arc-plane moves away from the truth staying above a handful: narrow (B)'s scope to straight-bounded plane pairs;
  - the cylinder/cone class refusing many bodies valid by the new truth: narrow the scope to planes and put curved faces along a line under (C).
- **The 80 reads where the bound from above fell below the truth:** attribution already under way. If they are covers that do not hold their arc, that is a defect in `topo::cover`, and the bound from above at the line site is unsound on arcs until it is fixed. In that case the line site's code waits for that fix.
- **Possible defect, unchecked:** `tangent_normal_parallel` is levered at 1/κ_rel = 1 m on an arc 0.05 long. The README says a lever is "no longer than the site's own measure of it". If that check's arm is not capped by the region's reach, it over-states the lever for a definite claim, which is the same defect this fork fixed for the wedge. One read of `run_checks`' tangent arm settles it. If confirmed, file it on the slate that owns certification.
- **Plane-plane-straight is not exact on 6,486 reads,** where a boundary edge's carrier ends are not its face's vertices (M-c's case). This rides on M-c's attribution.
