# Review of PR 7329 `embed/remeter` at `2c7136591` (reviewer lane `embed/remeter-review-5bf3bb`)

## Verdict: APPROVE-WITH-FIXES

Conditional on the gate, which I verified: run 44981119939 is `completed/success` on head SHA `2c7136591a1c…`; `gate ok`, `lint`, `test (kernel)` and `test (app)` are green at step level (default-eps step, the 1e-6/1e-12 step and the slow-set step each `success`). I did not read test names inside the slow-set step (it ran 1 s).

No wrong `Embedded` found. No MAJOR. The fixes are text and test-strength: two filed rows state their finding inaccurately, one module doc is now false, ONEROUND's row is left stale, and no row pins a real arm near the band.

Blinding: I read no other `embed/remeter-review-*` branch and no PR comment.

## Findings (each claim answered)

**Claim 1, what a departure proves: HELD.**
- The argument, from the code (`sign_on`, injective.rs:5592-5695): `ψ∘Φ` is zero on the zero side; the enclosure of `∇ψ·Φ_a` over the whole rectangle is one-signed; so `ψ` is strictly signed on the rectangle less that side. That holds in exact arithmetic whatever the margin. The presented rectangle is a superset of the face's part of the cell (a plane-corner cell lying off the face is presented as `y = (0, 0)` on the line, injective.rs:4239 and 4273: sound as a superset).
- The margin adds only strictness: the enclosure's near-zero end times the arm's lower end must reach K·ε. A cell far below ε does certify, by design.
- Attacks. Unit (`injective_review_probes.rs`): a cylinder of radius 1e-12 m leaving a plane squarely is certified at a metre's arm in a cell whose face is 1e-15 m from the plane (`probe_a_face_wholly_inside_the_band_of_its_separator`). That is the ruling's consequence; the cells that do not touch the edge still need `ψ` past the band (`Zero::None`), so the body refuses.
- Body (suite file, `mod review_probes`): a fold of 5e-8 rad between metre faces certifies at its edge (14 reads, all decided, arm 1.118); folds of 2e-8 and below, and lenses of sagitta 1e-7 and below, are refused by the profile doors, so nothing nearer a wrong `Embedded` can be built through a door. Lens s = 1e-6 at `Interval`: `Embedded`, correctly (gap 1000 ε).
- Tangent second-order pairs and a wedge passing through zero: the enclosure holds zero, nothing decides (the drum: 56,037 of 58,869 reads straddle).
- NOTE: the "sine" is not bounded by 1. I traced enclosures of `[-1.6e16, -3.5e-17]` (drum). Only the near-zero end decides, so it is harmless, but the quantity handed to `Margin::levered` as a pure number is not one.

**Claim 2, one reading with tier 3: FALSIFIED as stated; the PR discloses the arm but its filed row understates what differs. MINOR, DEMONSTRATED, doc/claim (no verdict moves: tier 3 is asked first).**
- Same sine, same lever source: held by reading and by probe (`probe_check4_and_the_engine_on_one_edge`: `geom_brep::classify_dihedral` beside `sign_on`, one edge, the same 1 m reach, both carrier arm and plane-through-edge).
- Measured, cylinder r = 1e-3 against a plane through its ruling:

| sine | sine·r | check 4 | engine |
|---|---|---|---|
| 1e-4 | 1e-7 | Corner | Positive |
| 5e-6 | 5e-9 | **InBand** | **Positive** |
| 5e-7 | 5e-10 | **Tangent (Zero, smooth class)** | **Positive** |
| 5e-9 | 5e-12 | Tangent | None |

- So the engine certifies as a definite crease an edge check 4 classes smooth at that very point. The filed row (`the-departure-is-levered-at-the-reach-alone…`) frames the gap as "a pinch between stations … passes both"; no pinch and no station gap is needed. The classes differ, not only the margins.
- Each way: engine accepts where check 4 refuses (row 2). Check 4 definite where the engine is undecided happens only by enclosure width or stations (r = 1e3, sine 5e-8: both definite; I found no arm-caused case, since fold ≤ reach).
- Which is right: check 4. Under Ev's test (can geometry ε off turn the angle?) reach alone fails on a curved face: at r = 1e-3, sine 5e-7, moving the plane 1e-9 changes `1 − cos θ` by 1e-6 against 1.25e-13, so the wedge is not determined, while the margin reads 5e-7 m ≫ K·ε. **"Reach alone" is the unsound side wherever a face's radius is far below its reach.** It cannot make a wrong body pass today only because check 4 gates first.
- The row's three facts: (3) stations are interior (`interior_stations`, `1..CERT_SAMPLES-1`): confirmed by reading. (1) the fold refuses `a_revolved_cone_holds` at 3,573 leaves and (2) the arm enclosure `[8.8e-17, 0.35]`: could not tell, not re-measured (no fold prototype built).
- `wedge_reach.rs`'s own safety argument leans on the fold ("the curvature arm … caps the lever at the face's radius in any case", lines 29-31), which the engine does not apply.

**Claim 3, `arm_of`: HELD, with one NOTE.**
- The reach is the faces' (`Piece.boundary`, the whole lifted boundary), never the cell's. No factor that shrinks with the cell remains in the departure margin (mutant M1 restores it and is killed).
- "The least" over near edges: all reaches are over one point set from pivots on it, so they differ by at most about 2×. A long edge's pair cannot get a much shorter arm; it is only a loss.
- Vertex reach: a property of the two faces, recomputed per cell pair.
- NOTE, the 20 ε slab: its end edge's metre reach is the farther face's, which is the recipe's stated choice (wedge_reach.rs:11-15), so it is tier 3's reach. It is not the thin face's tolerance (that face, ε off over 20 ε, turns by 0.05 rad). Inherited from check 4, with the run-along-the-edge over-statement the module already files.

**Claim 4, pole and apex: HELD.**
- The bound is right: `Φ_x(·, y₀) = 0`, so `Φ_x = ∫Φ_xy`, `n·Φ_x ≥ Δy·min n·Φ_xy`, `|Φ_x| ≤ Δy·max|Φ_xy|`; the interval quotient reads it from below. On a sphere and a cone it is tight (`Φ_xy ∥ Φ_x`).
- Unit: cone apex, 6 half-angles (1e-3 to 1.5697) × 3 poses (plain, turned, 1,000 units off) × 3 widths × 8 planes through the ruling, against 200 sampled sines each: 432 asked, 359 answered, **0 wrong signs**. 19 one-signed cases go unanswered, all in the turned frame (enclosure width; a loss). Sphere pole, 4 radii/centres × 9 cells: all Positive.
- Body: cones r/h from 0.01 to 100 and quarter cones, as built, turned, 100 units off, both scalars: `Embedded`, except r = 1, h = 1e-4 turned (pair budget). Balls r = 1, 1e-3, 30: `Embedded` except the full ball turned at `f64`, which refuses `Uncertified/Departure/Edge`. That refusal is identical under main's margin (M1), so it is not this PR's (pole pins it).

**Claim 5, the sliver is gone: HELD.**
- Sites, constant, module-doc entry, audit row, roster name: removed; no `sliver` reader left in `crates/`. `Vacuous` returns remain at 1843, 1949, 2018, 2409, 2497, each a cell off the face; I found none declaring a face absent from a cell it is in.
- The four vertex-alone tests, on the gate harness with this margin (`a246952d3` + the probe commit; it lacks only the head's doc/clippy commits and its main merge): `split_through_an_apex::a_split_through_an_apex_closes…` **passes, and by the arm** (under M1 it refuses again at `Vertex`). The other three still refuse `Uncertified/Departure/Vertex`; their undecided reads straddle zero (93; 13; 1,519) or are one-signed at 1e-15, with arms 0.7 to 7 m. Not an arm shortfall.

**Claim 6, the lever gate: HELD; acceptable with ONEROUND told, one MINOR (doc).**
- Sourced as rulings 289/290 ask: no new source, no `unsourced`, one pivot over both faces' points, built at the verdict, read from below (a definite sign is the only reader). `lever-doors.sh` and `lever-sources.sh` pass on the head; 303 is the true count.
- The gate's header hands wrappers to review and the residue row; the row and the log line are present. No text requires ONEROUND's approval; the note is owed and made (by the lane, not the orchestrator).
- MINOR: the pinned site moved from `WedgeReach::reach` (which holds the carrier and face keys) into `reach_from_vertex(vertex, points)`, a `pub(crate)` pass-through. The row's "Geometry in the function (44)" still lists `WedgeReach::reach` (line 60); the site now belongs under "Points or scalars only (11)". Tallies not updated.
- NOTE: the vertex pivot is a new geometric argument, approved by EMBED's orchestrator only. A zero arm (two disc rims) is documented to refuse and has no row.

**Claim 7, re-pins: HELD.** Line corner: the cell reaching the line from outside is presented `y = (1, 1)` and signed on the line (rule: a strip is read). Strip row: presented as its rectangle. Drum `Interval`: tangent plane/torus seam, the strip is now read and its rate straddles (trace above). Drum row run at default ε, 1e-6, 1e-12: passes at all three, as do the four margin rows, the two new body rows and the two pole body rows.

**Claim 8, measurements: HELD on what I sampled.** Not re-run: the 249/255, the tour, the 275.
- Eight still-refusing census tests through the gate with a margin trace (7 drawn at random, seed 7329, plus the rounded plate): in none is an undecided read one-signed above rounding size. Two have no undecided read at all (refused on presentation: `the_reflex_315_corner…`, `a_thin_slab_reads_out…`). The rest straddle zero or sit at 1e-15. The arm is not too short in any; whether a straddle is true tangency or enclosure width I could not tell.
- The rounded plate at `Interval`: 130,886 reads, 118,615 straddling, 0 one-signed undecided, out of pairs. Likely the drum's mechanism (strips at tangent fillet seams now read and split, where the sliver waved them through); I did not prove it.
- My fold probe shows the same on a body tier 3 takes: the prism on (0,0) (1,0) (1,1e-6) refuses `Uncertified/Departure/Edge` with every margin read decided, and identically under M1.

**Claim 9, mutants: HELD, all killed.** One build, runtime switch `REMETER_MUTANT`, over `injective::` (112) plus the suite file (52): control 164/164.

| # | definition | red |
|---|---|---|
| 1 | margin = rate × rectangle span (main's) | 6: both ring rows, three margin rows, the line-corner row |
| 2 | sine not divided by `|∇ψ||Φ_a|` | 1: `a_departures_rate…` |
| 3 | greatest reach | 1: `the_arm_is…` |
| 4 | first edge's reach | 1: `the_arm_is…` |
| 5 | other face's vertices dropped | 1: `the_arm_is…` |
| 6 | missing arm read as a metre | 2: `a_departure_is_levered…`, line-corner row |
| 7 | pole as `n·Φ_xy/(|n||Φ_x||Φ_y|)` | 5: pole row, cone, ball, two-ball union, filleted corner's ball |
| 8 | collapsed guard removed | 1: pole row |
| 9 | vertex pair's reach from one face | 1: `the_arm_is…` |
| 10 | sine's enclosure → its midpoint | 3 body rows (kiss, pierced cylinder, rod count); no unit row |
| 11 | arm doubled | 1: `a_departure_is_levered…` |
| 12 | any nonzero one-signed margin decides | 4 |
| 13 | margin compared with ε, not K·ε | 2 |

("Compared non-strictly": the funnel already uses `≥`, so I ran 12 and 13 instead.)

**Claim 10, `metre_arm()`: answered; MINOR, test gap, DEMONSTRATED by M3/4/5/9/11.** The 40 hand-stated rows prove nothing about the margin's size: their sines are O(1) at a metre. A real arm is pinned only by `the_arm_is…`, which reads `arm_of`'s number and never a decision. No row through `departure` is sensitive to the arm's value: doubling it, the greatest, the first, one face's all survive every body row. So the join `arm_of → sign_on` near the band is unpinned.

**Claim 11, the five other margins: partly FALSIFIED (the row's wording). MINOR, doc/claim, BY-INSPECTION.**
- Same defect, accurately: monotone (6354-6361), graph (6271), `leaves_graph` (4070-4076), cusp (6311-6313).
- Not a cell of the sweep: `_column` (`monotone_run`, 5017: a piece is 1/64 of the edge's span) and `leaves_line` (4664: 1/64 pieces under its own halving). The row's "a smaller cell makes it worse" is untrue of both. `PIECES`' doc (845-848, "decides nothing") is contradicted by both.
- The "26 sites" is not reproducible: 21 call sites.
- No unlisted cell-levered site in injective.rs; none in `clearance.rs` or `cell_region.rs` (their decisions are distances; a cell's reach is subtracted, never multiplied).
- The cusp decides `Zero` from a cell ball's reach, the dangerous direction, but it only ever becomes a `SecondOrder` split or refusal (clearance.rs:2161), never a certificate.
- Leaving them: DESIGN's sentence covers "a face's own turning", so monotone remains a literal exception to ratified text, and the cusp to "never re-deciding one over a cell". Defensible as staged; the PR cannot claim the sentence holds beyond the departure.

**Claim 12, coupling: HELD (no interference found).**
- `origin/embed/strip` merges clean; 171/171 of `injective::` plus the suite file. I did not run strip's 22 gate tests.
- `origin/embed/pole`: textual conflicts only in `.config/nextest.toml` and a CHARTTAIL row; compiles; 179/181. The two red rows (`a_slender_cone_in_a_turned_frame_refuses_departure_at_its_apex` now Holds at 392 leaves; `singular_points_hold…` frame 3 refuses 18,250 leaves) are **red with identical receipts on pole merged with main `ae076e050` alone**, and under M1. They are not this PR's. Off-target, for the orchestrator: pole's two rows are red against current main in this box's build.

## Spec deviations
- Reported by the author: reach per pair, not on `Rim`; the check-4 row on RESTFRONT; the census selection; the wrapper past `lever-doors.sh`; "not yet one reading".
- Not reported: the filed reach-alone row's framing (claim 2); the five-margins row's two mis-classed sites and its count (claim 11); `wedge_reach.rs`'s module doc left describing a fold the new reader skips; ONEROUND's row tallies; five open EMBED rows and `program.md:43` still describing the sliver as live (`the-filleted-drum…:78-80` says the drum "holds with `present_chart`'s sliver refused … gates that sliver's retirement").

## Style
Exercised: Q1 (prose and constants sweep over the three touched files), Q2, Q3, Q4, Q5, Q6, Q7. Q8 only in part (see "read").

- `crates/topo/src/injective.rs:5532-5536`, `geom-brep/src/dihedral.rs:1041`. The sine is computed twice, in two crates, and a doc comment says they are the same. Nothing compares them; my probe is the only place both run on one edge. There is no one home for "an edge's wedge levered at its faces' reach": one lever source, two computations, two arms. **sure**
- `crates/topo/src/wedge_reach.rs:1-5, 29-31`. The module says it is the home of the reach "every reader … passes to `folded_lever_arm`" and that the curvature arm caps the lever "in any case". The engine reads the reach and passes it to no fold. The doc rotted on the day its premise was removed; this is the code drifting from an intended invariant, not a sentence to delete. **sure**
- `injective.rs:5805`. `arm_of` runs for every cell pair, before the collapsed and vacuous returns that never use it, and walks both faces' vertices each time. A reach is a face pair's (or an edge's); it is recomputed per cell. The PR argues this at length in "Deviations"; the cost claim rests on one tour timing with no guard. **likely**
- `injective.rs:5540-5553` against `5658-5662`. The pole bound is written in the function's doc, sixty lines from the two lines that rely on it, and the guard carries no pointer back. The mathematics is right; its tie to the code is by proximity of names only. **likely**
- `injective.rs:11705-11713`. The re-pinned line-corner row asserts `.is_some()`: either sign passes. A row whose subject is that the strip is read should say which side. **likely**
- `injective.rs:12062`. The replaced row dropped its second half (a lever that does not certify decides nothing). No row now puts an uncertified surface through the departure. **unsure**
- `injective.rs:6366-6371`. `metre_arm` is a `#[cfg(test)]` function in the production region, between `monotone` and the test modules, with 40 callers. A reader of `sign_on` must pass an arm "by convention" (the PR's own words). **likely**
- `injective.rs:5627-5634`. A value called `sine` that is not in [-1, 1] (traced at 1e16). **unsure**
- `injective.rs:845-848`. `PIECES`: "decides nothing", while it sets two levers. Pre-existing; the PR's row repeats the mistake rather than catching it. **likely**
- `docs/predicate-dimension-audit.md:822-831`. The five rows the PR calls defective are still marked "OK", with no pointer to the new row. **unsure**
- `work/embed/five-more…md:20`, `work/oneround/lever-doors…md:19, 60-62`. A hand-written count and a hand-written census, each wrong or stale the day it landed. This PR closes a structural finding and mints two instances of "a count kept by hand". Class: every count in a filed row or a PR body. **sure**
- `work/embed/the-sliver-concession…md:5`. `status: open` under a "Landed" section. **unsure**
- Taste: 835 lines for one margin, most of it `metre_arm()` threaded through 40 call sites. An arm that is `Option` at the signature, where `None` means "decide nothing", makes the absent case a value every test must supply. **unsure**

## Rubric
- Idiom and structure: **3**. The margin itself is small and clear; the arm is an `Option` threaded by hand and recomputed per cell, and the "one reading" is two.
- Test quality: **3**. Every mutant dies, but six of thirteen die on exactly one hand-stated row, and no body row feels the arm.
- Doc and comment honesty: **3**. The PR body is unusually candid; two filed rows and one module doc say things the code does not do.

## Executed, as against read
- Executed (my builds, CI's opt-1 profile): control 164/164; 13 mutants; unit probes (4) and body probes (6 families, both scalars, three poses); the drum and new rows at three ε; margin traces of the drum, the folds, the four vertex-alone tests and eight census tests; the four vertex-alone tests under M1; merges with pole (twice, once with the switch) and strip; pole on main alone.
- Read only: `sign_on`, `Margin`, `Lever`, `arm_of`, `reach_from_vertex`, `wedge_reach.rs` and `edge_wedge.rs` in full; `wedge_of`, `folded_lever_arm`, `edge_wedge_over_as`; the three former sliver sites; `zero_of`, `vacuous`; all new rows; the gate scripts and rulings 289/290 (by a helper, which also ran the two gates).
- Not read: `injective.rs` end to end (13,079 lines; I read about 2,500), `clearance.rs` as a whole, `validate.rs`. Not run: editor-core, the tour, the 275, strip's gate tests.
- Probes: branch `embed/remeter-review-5bf3bb` at `c10dfa760` (`crates/topo/src/injective_review_probes.rs`; `mod review_probes` in the suite file; `REMETER_MUTANT=<n>`, `REMETER_TRACE=1`).
