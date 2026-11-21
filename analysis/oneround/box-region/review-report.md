# Review of PR 7225 (`oneround/box-region`) at `5c3bb9b9`: reviewer r1

**Verdict: APPROVE-WITH-FIXES.**

I found no MAJOR. Across 22 540 placing-door asks, 240 posed tori and balls checked against their analytic reach, and arm-by-arm reading, the predicate never admitted a body that reaches past the box. It also never refused a body inside the box as past it. Every point here comes from the frozen head. Base is the merged main `1d320404`, the second parent of `4874552f`.

What I found is weaker than the PR claims in three ways:
- two cases where a body truly past the box is refused as a limit, against the README's sentence;
- three mutants that survive every row the PR adds;
- a third spelling of the face-region bound.

My branch `oneround/box-region-review-r1` carries two rows the author can keep, which kill those three mutants. It also carries my probes and the mutant script.

## What I executed and what I only read

**Executed** (in this container, debug test profile, my own target directories):
- The PR's session-box rows in `topo` and `sweep`: 21, all green.
- With `editor-core`'s placed-copy row and the two edited `tcost_k3_certificate` slow rows at ε = 1e-9: 26 rows, all green on the head.
- Reviewer probes in `crates/sweep/tests/review_box_region_r1_probes.rs`:
  - `probe_c4_door_equals_gate_on_image`: 22 540 placing-door asks against the gate on the moved image.
  - `probe_c1_analytic_reach_oracle`: 240 random poses.
  - `probe_c2_nearly_axial_torus` and `probe_spindle_point_past_reads_as_limit`: both print only.
  - Two asserting rows: `a_zone_whose_pole_alone_is_past_the_box_is_admitted` and `an_arc_whose_middle_alone_is_past_the_box_refuses_past_it`.
- Ten planted mutants (`analysis/oneround/box-region/r1-mutants.py`), each over those rows.
- A timing of the box's ask and of the gate, base against head (`review_box_region_r1_timing.rs`, `#[ignore]`d).
- The CI run resolved to step level through the jobs API.

**Only read:**
- `session_box.rs`, `cover.rs` (topo), `geom/src/cover/readings.rs` and `contain.rs`'s placement.
- The `validate.rs`, `finish.rs` and `boxes.rs` diffs, and the editor-core and pncad-py arms.
- The readers ledger, the two `work/boxes/` rows and the PR body.

**Not done:**
- The whole fast set or slow set at any ε.
- The `Interval` and `Sym<f64>` lanes, re-timed or re-run.
- The author's verdict differential, re-run.

So I did not re-measure claim (a) "every finished face has a cover", the differential behind C5(c), or the C6 ledger. I read those.

## Findings

### MINOR 1: a torus whose axis is tilted only slightly off a world axis loses its inside read, so a body truly past the box is refused as a limit
DEMONSTRATED, `sure` on the behaviour, `likely` on the cause. Claim C2.

**The probe** (`probe_c2_nearly_axial_torus`):
- A full torus, R = 300 and r = 100, its seams at the inner and outer equators.
- Its axis is turned θ off `+y` about `x`.
- It is placed so the true largest `y`, `r + R·sin θ`, is past the box by half of `R·sin θ`, while every vertex and edge is inside.

**Results:**

| θ | verdict at the gate and at the door |
|---|---|
| 0.3 down to 1e-5 | `OutsideSessionBox`, the margins exact (1.5e-3 m past at 1e-5) |
| 1e-6 | `SessionBoxUnshown(0.00015)` |
| 1e-7 | `SessionBoxUnshown(1.5e-5)` |

At 1e-6 and 1e-7 the stage-2 cover decides definitely past the box, so these margins are out of band. But no point is kept, so the body is told it "may lie inside the box" when it is 0.15 mm past.

**Cause** (by reading, `likely`):
- `face_extreme_points` takes `off = e − a(a·e)` (`crates/topo/src/cover.rs`, the torus arm).
- Its component along a nearly parallel `a` is `1 − a_y² ≈ θ²`, which keeps about 1e-16 of absolute error.
- So `off/|off|` carries an `a` component of about 1e-16/θ.
- The candidate then sits `R·1e-16/θ` off the torus: 3e-8 m at θ = 1e-6.
- The placement reads the carrier first, finds the point definitely off it, places it Out, and the filter drops it.

The compass branch only runs when `off` is exactly zero (`is_direction` is "not the zero vector"). So the regime between "nearly along e" and "along e" that the brief asks about is exactly where the read fails.

This is the safe direction: a limit, never an admission. It is a wrong refusal under the README, though, and nothing pins or files it.

### MINOR 2: no inside read for spindle or spline faces, so a body with a point past the box is refused as a limit
DEMONSTRATED, `sure`. Against the README's sentence; disclosed, but not scheduled. Claims C1/C3, questions Q4 and Q6.

The README says: "One with a point past it is refused as past the box." `face_extreme_points` has no candidate for a spindle or a spline face. The PR body says so ("Spindle: none … Spline: none").

**The probe** (`probe_spindle_point_past_reads_as_limit`):
- The conical-ended football: a spindle with R = 30 and r = 50 between two cones.
- Its equator has radius 80 and lies inside the spindle face, except at the revolve seam, which points along `x`.
- Moved along `z`:

| centre at z | equator reaches | verdict at the gate and at the door |
|---|---|---|
| 930 | 1010 | `SessionBoxUnshown(10)` |
| 950 | 1030 | `SessionBoxUnshown(30)` |
| 960 | 1040 | `OutsideSessionBox(8.12)`, read off the rim edges, not the true 40 |

A body 30 m past the box is told "the body may lie inside the box, but a tighter bound of its faces is not built yet". The family's recourse (place it nearer the origin) is still right, but the sentence is false of the body.

A spline face whose interior bulge is the extreme has the same shape. The loft prism's flare sits on its corner edges, so the PR's row does not show it.

**Scheduling:**
- The filed spline row (`a-spline-faces-cover-is-its-whole-control-net`) schedules a tighter cover, not an inside read.
- Its sentence "the box's verdict on a body past it is unchanged, since that is read off points on it" reads as though spline bodies past the box are refused as past it. For a spline face no points are read.
- I found no row in `work/` for either kind's inside read. Q6: the deviation owes one.

### MINOR 3: three of my mutants survive every row the PR adds; two new rows kill them
DEMONSTRATED, `sure`. Claim C7, question Q3.

| mutant | the author's rows (26) | with my two rows |
|---|---|---|
| **stage 2 never admits**: past stage 1, refuse as a limit | all green | `a_zone_whose_pole_alone…` red |
| **the placement filter keeps every candidate** | all green | `a_zone_whose_pole_alone…` red |
| **stage 3 reads no edge points** | all green | `an_arc_whose_middle_alone…` red |
| uncovered face dropped at the gate and tier 3′ | all green | all green: no row reaches `SessionBoxUncovered` at any door |
| a Zero read as past at every box decision | all green | all green: no row at the box's band edge |
| the cone arm of stage 1 without its apex | all green | all green: every cone fixture has its apex as a vertex |
| the torus's compass points dropped | all green | all green: see the NOTE below |
| the torus extreme's minor term reversed | `a_torus_whose_bulge…` red | |
| a placed carrier left unmapped (the author's) | 5 red, as the PR body says | |
| "stage 2 reads only the vertices" (my first, mis-aimed spelling of "stage 2 skipped") | 7 red | |

**The two new rows:**
- **The barrel.** A sphere zone |y| ≤ 30 of a radius-50 sphere, capped by discs, centred at y = 965.
  - Its ball reaches 1015 m and its +y pole is not on the face.
  - So it is admitted only through stage 2 (the zone's own cover), and only if the off-face pole is dropped.
  - At 975 m it refuses `OutsideSessionBox(5)`.
- **The arc cylinder.** A cylinder of quarter arcs whose ends sit on the diagonals, centred at x = 905.
  - Its +x arc's middle reaches 1005 m, while the vertices reach 975.7 m.
  - It refuses `OutsideSessionBox(5)` at the extrude; at 895 m it builds.

The placement filter is what guards C2's hazard ("a candidate wrongly placed In would refuse a valid body as past the box"). No row of the PR's goes red when the filter is removed.

### MINOR 4: the gate's deferral of `SessionBoxUncovered` behind tier 3 is not the README's "before anything is decided", and nothing pins it
BY-INSPECTION `likely`, plus the surviving mutant above. Claim C3.

**What the code does.** Every combination of the three stages refuses a body with an uncovered face, so I found no admission path:
- stage 1 reads `Err(face)` as not within;
- stage 2's `Err(face)` becomes `Uncovered`;
- stage 3 picks between past the box and the limit.

But at the gate and at tier 3′, the box's limit is reported only where tier 3's checks find nothing. Tier 3's decisions are therefore taken on a region the box did not admit. The ratified sentence says the door asks the box "before anything is decided on that region".

**Why it matters:**
- The author's reason is sound: check 1 names the fault better. But it is a deviation from ratified text, argued only in the PR body. The README bullet is unchanged, and `[ev]` was not asked.
- The refusal arm `(true, Some(uncovered)) => Err(vec![uncovered])` in `structural_via` is reached by no row. The mutant that drops it survives.
- `SessionBoxUncovered` refuses directly at the move and placing doors. There its recourse is the family's "place nearer the origin", which is wrong for every cause the author measured: poisoned, placeholder or seed nets, and a datum that describes no locus. The author says no finished body reaches it there.

### MINOR 5: `face_cover_unproven` is a third spelling of "a ruled face is in its boundary hull, a cone with its apex, any other kind in its carrier"
BY-INSPECTION, `sure` on the restatement, `likely` on breaking the rule. Claim C8, question Q1.

**The three spellings:**
- `geom::Surface::face_region` (`crates/geom/src/cover/readings.rs`, read by `step-import/src/reach.rs`). It is the same rule, looser: it hulls the boundary with a sphere's or torus's carrier.
- `face_cover_in`'s plane, cylinder and cone arms.
- The new `face_cover_unproven`, which restates their dispatch. It repeats the lone-vertex `bounded` test and the cylinder's `is_direction` test, and sits beside the `boundary_cover_by` split.

**What the doc claims.** "So it holds `face_cover`'s region." That superset property is held by hand across two separate matches. It is what makes the PR body's "the staging changes no verdict" true, and nothing checks it per kind.

**The rule.** The gate's stated rule (`[one-way.places]`, quoted in `cover-kind-match.sh`) is "one cover per entity … never a second reading". Stage 1 is a second reading of a face's region, outside `face_cover`, written for cost. The readers ledger registers it as a door rather than asking whether it should be one. Q1's trap: this is the cover home's own structural fix minting a new copy beside the two it already has.

`boundary_cover_by` restates nothing: `boundary_cover` now delegates to it. `face_extreme_points` matches kinds inside `cover.rs`, so the counter does not see it, which is allowed.

### MINOR 6: the two `tcost_k3_certificate` slow rows lose their measured arm at the default ε
BY-INSPECTION, `sure`. Claim C5(c), question Q6.

- At ε = 1e-9 the exhausting arc prism's net (1272.8 m) passes the box, so both rows now assert the limit and stand down.
- Their purpose (a certificate split across rounds; a refusing arm returning no properties) now runs only at 1e-12, which in practice means the nightly slow set. It already stood down at 1e-6.
- The fixture could have been scaled by its net's reach. `box_unit` already takes a reach.
- The narrowing is scheduled only through the P3 spline row, so the coverage stays lost until that row lands.

### NOTE A: what C4 and C1 measured (DEMONSTRATED; `sure` within the sample)

**C4 (the door against the gate on the image).**
- 7 bodies: the PR's torus; a seam-at-equators torus; the ball poled on `y` and on `z`; a cone with its apex a vertex; the loft prism; the football.
- 5 rotations, 4 directions, and translations stepped 2.5 m from 700 to 1100 m (0.125 m from 990 to 1002 m for the loft).
- In total 22 540 asks. `topo::session_box_at` and the gate on `transform_rigid`'s image agreed in refusal kind on every one: 0 disagreements, and no other tier-3 finding on any image.
- Verdicts covered Ok, Outside and Unshown (football 58, loft 12). No Uncovered occurred.
- The to-the-bit row pins only stage 1 equality, on a brick and a sphere cap. Stages 2 and 3 at the door run on the built image, so this differential is the only evidence for them, and it is a sample.

**C1 (the analytic oracle).**
- 240 random poses of three tori (including R = 600, r = 20) and a ball, placed with their analytic largest `|coordinate|` (`|c·e| + R·|ê| + r`) at 1000 + d, with d ∈ {−0.5, −0.01, 0.01, 0.5, 3} m.
- No pose past the box was admitted, none inside was refused as past it, and every pose past it was refused as past it.

**By reading, arm by arm.** I found no way to make stage 1 contain less than a face:
- plane and cylinder: rulings leave a bounded face at its outer or inner loops, seams included, and a lone-vertex outer loop falls to the carrier's refusal;
- cone: the hull with the apex holds either nappe;
- sphere, torus and spindle: whole tubes, the spindle's about `|R|`;
- spline: the net, with weights held strictly positive (`geom/src/curves/nurbs.rs`).

**The clockwise torus window** (BOXES, P3) is now read on a soundness-bearing path: stage 2 admits on it alone. It stays latent at the doors. The gate asks before tier 3, but tier 3 then refuses a clockwise face, and the move and placing doors see only finished bodies. I did not build a clockwise face.

`boxes.rs`'s new sentence, "never admits one past it", claims more than that open row allows. `likely`.

### NOTE B: two of the inside read's arms do nothing in practice (`likely`)
- **The torus compass points.** Every full-turn torus the kernel mints has a meridian seam edge, which reaches the top circle's height. Any face holding an arc of the top circle inside it has a boundary that crosses that circle. So the eight compass points are never the only read. Their mutant survives, and I could build no body that needs them.
- **The cone's apex candidate.** Every cone fixture's apex is a vertex.

Both are sound. Neither has a row.

### NOTE C: stage 1 admits on an escalation
`within` admits on anything but a definite Negative, as main did for the points. Now an escalated *cover* margin admits without the inside read running at all. At the `Interval` lane, a cover margin that straddles the band could admit a body whose point margin would decide past it.

I could not build one: the window is the enclosure's width. Theoretical, `unsure`.

### NOTE D: stage 2 runs every face's `face_cover` after the first uncovered face
Their constraint decisions land on the open log and decide nothing. One name, `session_box_reach`, logs both the stage-1 and the stage-2 decision, so a verdict log cannot tell them apart. Q7, taste.

### NOTE E: what moved, re-measured (C5(b))
f64 only, debug profile, mean of 200 asks and of 20 gates, this container, base then head. My gate is `validate_geometric`, not the PR's "structural gate", which explains the loft's larger gate figure.

| body | ask, base | ask, head | gate, base → head |
|---|---|---|---|
| brick | 34–38 µs | 97 µs | 3.4–3.8 → 3.1–3.6 ms |
| ball, r = 50 | 20–26 µs | 24 µs | 2.8–3.3 → 2.6–2.7 ms |
| torus R 450, r 400 | 32–36 µs | 40–45 µs | 7.0 → 7.4–7.8 ms |
| loft prism | 226–241 µs | 342–347 µs | 42 → 42–49 ms |

This agrees with the author on shape: the brick's ask is about 3×, the curved bodies' 1.0–1.5×, and the ask is 1–3% of the gate. I did not re-time `Interval` or `Sym<f64>`, and I have no figure for the whole suite.

### NOTE F: the question behind C5(c), how wide the spline class is (`likely`)
**Which bodies it refuses.** A rational quarter arc's middle control point stands `√2·ρ` from its centre, only along the arc's bisector. A body loses the box where it stands within about `(√2 − 1)·ρ` of the box's face along that bisector. A 120° rational segment, with its control point at `2ρ`, loses up to `ρ`.

So lofts and sweeps of arc sections need large arcs within about 0.4·ρ to 1·ρ of 1 km. The likelier source is imported trimmed NURBS. Their base surface's net can reach far past the trim (`face_cover` uses the whole net), so the loss is the distance from the trim to the net, not a constant factor.

**Whether one fixture measures it.** One suite fixture is not a fair measure: the suite places almost no spline bodies near the box's face.

**Whether P3 is right.** P3 is defensible for parts modelled near the origin. It is too low for site-scale models, which are the case the box exists for. The row should name imported trimmed surfaces, since its own measurement cannot see them.

### NOTE G: the C6 ledger (BY-INSPECTION, `likely`)
Stage 1 makes no decision. The stage-1 sample is the one `decide_reported(SESSION_BOX_REACH, …)` in `within`, as main made it, so the call counts hold.

The forms grew because `face_cover_unproven` builds each face's boundary hull and `convex()` at `Sym`, which mints forms without asking. The small increments (+2, +14, +14, +20) fit the Sym arena's forms being shared across asks. "Costs what main's did" is true of decisions and logs and false of time (NOTE E).

### NOTE H: the brief's premises and the PR body's corrections (C10)
- **"The torus example's edges reach 700 m, not 806."** True: 250 + 450 = 700. The 806 is `√(700² + 400²)`, the old Euclidean reading from before the box became a cube.
- **"`curved_face_placement` reads no spindle."** True: `contain.rs` arm `Plane | Spindle => Trim(None)`.
- **"The common path costs and logs what main's did" (C6).** Holds for the logs only (NOTE G).
- **C9.** Both new variants are placed in every exhaustive match: editor-core `at_rest.rs`'s geometry-only arm; pncad-py's tags, `TAG_INVENTORY` and binding census; topo's family map and samples; the recourse roster. Each is in `FindingFamily::SessionBox`, which is the right family. Its recourse is wrong only for `SessionBoxUncovered`'s measured causes (MINOR 4).

### NOTE I: CI run 44859392555, step by step
**Kernel and app jobs.** Ran: build; the tests the diff adds; the default ε; ε 1e-6 and 1e-12 over the touched crates and files; doc-tests; the slow-set name check. Two steps were skipped: "the slow set did not run" and the failing-tests report.

**The slow-set step.** "The slow-set tests this diff adds or edits" (kernel job, step 14) finished within the second it started, 03:29:24. I could not read its output, so I cannot say whether the two edited `tcost` slow rows ran in hosted CI. I ran them here at 1e-9: green.

**Jobs skipped:** `interval`, `tools (k-lint, tess-lint, tess-meter)`, demos, step import (freecad), viewer, and the cache primer.

**Jobs that ran:** lint, every gate step included; python; corrupt input; the mesh budget meter. All green.

The `interval` lane, where `SessionBoxUnshown` is reachable through its own numbers, had no hosted run on this head.

**Reds.** I met none in the rows I ran, so C11 was not exercised.

## Style

**Questions exercised:**
- Q1: MINOR 5. I swept `face_region|face_cover_unproven|boundary_cover_by`, and checked the readers ledger against the gate's rule.
- Q2: MINOR 5's superset invariant, held by hand; `boxes.rs`'s "never admits", in NOTE A.
- Q3: MINOR 3. Mutants planted and survivals shown.
- Q4: the README sentence the spindle, spline and near-axial-torus reads break (MINOR 1 and 2); the spline row's "read off points".
- Q5: the module docs of `session_box.rs` and `boxes.rs` against the code. Accurate apart from NOTE A's sentence. `validate.rs`'s "reported alone, before any check reads a margin" is now false where stages 2 and 3 run: `face_cover`'s window decisions and the placements are taken before tier 3.
- Q6: MINOR 2 and MINOR 6, unscheduled or under-scheduled.
- Q7: NOTE D. Also the five-closure `reach_of`, where stage 1 runs through the fold off the unmoved body while stages 2 and 3 run on a built image. Two paths for one predicate, held equal only by the differential.
- Q8: I read `session_box.rs` whole, and `cover.rs` through the face arms and the new doors.

**Ratings:**
- **Idiom and structure, 3/5.** One fold and three stages read cleanly. But stage 1 is a second hand-kept reading of a face region, stages 2 and 3 take a different path at the door than at the gate, and `body_reach` survives as `#[cfg(test)]` only.
- **Test quality, 3/5.** The rows pin the two measured examples and the limit's margin exactly, and the author's mutants red as claimed. Stage 2's admission, the placement filter, stage 3's edge points, the uncovered refusal and the band edge are pinned by nothing (MINOR 3).
- **Doc and comment honesty, 4/5.** The PR body discloses the spline, spindle and torus-window gaps and both corrections, and its mutant table reproduces. Two overclaims: `boxes.rs`'s "never admits one past it", and the spline row's "read off points on it".

*Reviewer r1. I did not read or fetch the other reviewer's branch, artifacts or comments.*
