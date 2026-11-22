REVIEW of PR 7329 (`embed/remeter`) at frozen head 2c7136591, merge base ae076e050. Probe branch: `[this reviewer's probe branch]` ([probe commit]: a runtime mutant switch `REMETER_MUTANT` and two probe rows in `injective.rs`; not for merge). I read no other review branch and no PR comment.

## Verdict: APPROVE-WITH-FIXES

The code is the ruling as the orchestrator approved it, and every mutant I ran (the author's nine and four more) goes red. The fixes are to what the PR says and files: its soundness sentence is false for the lever it adopts, and two couplings it does not name are live (merging with `embed/pole` turns three rows red, and ONEROUND's PR 7262 rewrites `wedge_reach.rs`). Conditional on gate run 44981119939, which I resolved myself: `completed/success` on head `2c7136591`. Its steps I know only from the PR body's account. The drum row, which the gate did not run, I ran myself.

## MAJOR

**M1. The arm is the reach measured from the edge's middle. That reach over-states a face's run along its edge, the unsafe side of a lever, so the departure signs apart a strip whose faces never part by ε.** DEMONSTRATED (a red probe), defect class: contract/doc, with the code inherited.
- `crates/topo/src/wedge_reach.rs:17-23` (the module doc this PR edits) already says "One over-statement remains: the reach is measured from the edge's middle, so a face's run ALONG the edge counts as lever, though two faces separate only with distance from the edge", and cites `work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md` (P2, which calls it "the unsafe direction for a lever").
- Probe `injective::review_probes::review_strip_arm_reads_the_run_along_the_edge` (red on this head): two plane strips, 4 along their shared edge and 0.05 across, folded at φ = 0.9ε/0.05 = 18ε. Their far edges part by 9.0e-10, under ε = 1e-9, at every point. `arm_of` reads 2.000625 (half the edge, not the width). sine × arm = 3.6e-8 against K·ε = 1e-8, so `sign_on` returns `Some(Negative)` over the strip's whole width. With main's margin restored (mutant `span`) the same probe passes, i.e. refuses: this PR is what newly signs it.
- Ev's own test on PR 7095, as the row quotes it: "a wedge that opens by less than about 2ε at the faces' far side could really open the other way". This wedge opens 0.9ε at its far side across the edge, and is read as opening 72ε at the far side along it. The PR body's and `sign_on`'s argument ("two faces each ε off over their reach r meet at an angle uncertain by about ε/r", `injective.rs` doc above `sign_on`) holds only where r is a run away from the edge.
- No wrong body-level `Embedded` was found. The backstop is the distance test: a strip that never leaves the band has a far edge within the band of the other face, and that pair refuses unless the far side is itself a face narrower than the band (a thin face, which `embed/strip` presents). Tier 3 over-reads the same way ("one reading" holds here: both are wrong together).
- Fix asked: say so at `sign_on`'s margin paragraph and in the body, and put this reader on the honeone row with a priority it now earns (the row names tier 3 and the profile, not the embedding certificate). Whether the reach moves to the line (ONEROUND's open "site" fork, `work/oneround/log.md` 2026-11-21 09:11Z: "a point over-reads the faces' run along the edge") is not this unit's to do.

## MINOR

**m1. Merging with `embed/pole` (b28f47a2d) turns three of pole's rows red at every ε.** DEMONSTRATED; class: test/coupling. The merge is textually clean in `injective.rs` (conflicts only in `.config/nextest.toml` and a charttail row, both taken ours). Run over `injective::` + the suite file + `shell_cavity_embedded`, 191 rows:
- `singular_points_hold_far_from_the_origin_and_in_turned_frames`: "the cone in frame 3" refuses `Uncertified {1, 2, Edge(4), Departure}` (18,250 refused, depth 40) at 1e-9, 1e-6 and 1e-12, where pole's own row says it holds. It still refuses with this PR's margin reverted to main's (mutant `span`) and with the collapsed-side guard removed (mutant `guard`). So the cause is neither the margin nor the guard; by elimination it is the sliver's retirement meeting pole's run-on presentation. I did not restore the sliver to confirm this.
- Two of pole's refusal pins now hold, both improvements and both reading "red when it holds": `a_slender_cone_in_a_turned_frame_refuses_departure_at_its_apex` (Holds, 392 leaves) and `the_unit_cone_on_the_first_turned_plane_holds_or_refuses_by_eps` at 1e-6 (Holds, 5,303).
- The PR names the pole coupling as a change of form at merge. Measured, it is one valid body lost and two re-pins owed by whichever lands second.

**m2. ONEROUND's `oneround/turn-lever` (PR 7262, f0302665e) rewrites `wedge_reach.rs` under this PR's `reach_over`.** BY-INSPECTION; class: contract/coupling. 7262 makes `wedge_reach` and `edge_wedge_reach` return `LeverBounds` (both bounds, from below off vertices and from above off covers) and threads a `Band` through `WedgeReach::new` (+110 −61 in that file). After both land, `arm_of` must pick a bound for a definite sign: from below, by ONEROUND's own rule (log 2026-11-21: "a definite sign is proved from below"). And "the reach's one recipe" splits again unless `reach_over` follows. The PR's Seams and coupling lists name EMBED lanes only. Claim 6's question, whether ONEROUND's word is needed, comes down to this: the wrapper is disclosed, but the doors it wraps are mid-redesign.

**m3. DESIGN `[topology.tiers]` (docs/DESIGN.md:907-908) says the question "reads tier 3's facts from its verdict, never re-deciding one over a cell".** BY-INSPECTION; class: contract. After this PR the engine re-decides the wedge, a tier 3 fact, in every cell near the edge. The PR is open about "not yet one reading" (the RESTFRONT P2 row), but neither the body nor that row cites this sentence, which the code now contradicts on its face. Whether the sentence or the code is the one that moves is Ev's.

**m4. Check 4's own doc instructs this site the other way.** `crates/geom-brep/src/dihedral.rs` above `folded_lever_arm`: "A new site levering against its own fold is a silent non-comparability, so route new callers through this function." The departure is such a site (reach alone, by the approved choice, filed P1). The instruction stands unamended, and `wedge_reach.rs:29-30` still says "the curvature arm (folded_lever_arm) caps the lever at the face's radius in any case", which is false for its new reader. Class: doc.

## NOTE

- n1 (test gap). Mutant `mid` (each sine enclosure replaced by its midpoint, a point interval) is red in no unit row. Three body rows kill it: `a_declared_wedge_0_kiss_refuses_naming_the_second_order_arm`, `a_seam_crossing_cylinder_pierced_by_a_face_on_its_edge_never_holds` (both of which then read a wrong hold or a wrong verdict) and `a_rod_less_a_tilted_slab_holds_across_its_curved_corners`.
- n2. The pole reading's enclosure tops 1 (`[-1.139, -0.878]` on a sphere cell 0.5 rad tall). This is harmless, since only the lower end decides, but a reader expecting a sine will be surprised.
- n3. My `span` mutant, which also restores main's corner margin (mixed rate × both spans), reds `a_pole_is_read_by…` as well. The author's table lists five reds for that mutant: theirs left the corner alone.
- n4 (pre-existing). In the carrier arm (`on = None`) a zero side is "zero" when ψ along it is within the band (`read` → `is_zero`), not exactly zero. So the sign proved over a strip thinner than ε/sine is the departure's sign, not ψ's. This was as true under main's margin for strips wider than the band. The ruling accepts it; worth one sentence at `sign_on`.

## Claims

1. **What a departure proves. HELD, with M1.** The one-signed `∇ψ·Φ_a` over the rectangle, with ψ zero on the side (key membership, or in band in the carrier arm, n4), gives ψ∘Φ one strict sign on this cell's rectangle, exactly. Normalising does not change that sign. The margin adds only that the sine clears K·ε at the arm. A cell below ε then certifies wherever the sine is one-signed, by design. I built no crossing body. By the argument below the pairs that cross are caught elsewhere: a crossing inside a near cell makes the sine straddle; one outside meets the distance test; a curl back within the band reaches a tangent direction inside the rectangle, where the sine encloses 0. Nearest wrong `Embedded`: M1's strip, at unit level.
2. **One reading. HELD at first order, FALSIFIED on the arm.** With the face's carrier as ψ, `n₁·(n₂×t̂)` is check 4's `|n₁×n₂|` (identity checked). The arm is always ≥ check 4's (`min(reach, κ-arms)`), so the only arm disagreement is the engine accepting where check 4 refuses. "Reach alone" is therefore always the permissive side, and the filed row itself calls the fold "the more faithful tolerance". Each way:
   - Engine refuses, tier 3 passes: exists, the filleted drum at `Interval` (a tangent seam check 4 settles at second order; the engine has no second-order arm).
   - Engine accepts, check 4 refuses: needs a curvature radius below Kε/sinθ at a station. Every plane/convex-face case I tried makes a face about Kε wide, so I built none.
   - The cone row's fact 3 holds in code: `interior_stations` is `1..CERT_SAMPLES-1`, so no apex station. Facts 1 and 2 I did not re-measure.
3. **`arm_of`. HELD, with M1.**
   - "The least" only loses. Both reaches read the same vertex set and differ only by the pivot (each edge's middle), so a long edge never receives a much shorter arm.
   - The vertex reach is a property of the face pair and the vertex, recomputed for every cell pair.
   - No cell factor enters by another door: I read every `sign_on` arm, and only the enclosures widen with the cell.
   - The slab's end-edge arm of about a metre is the run along the edge, the over-statement in M1. It is not that faces' reach in Ev's "far side" sense.
4. **Pole/apex. HELD.** The bound is right: Φ_x(·,y₀)=0 ⇒ sine ≥ min n·Φ_xy/(|n| max|Φ_xy|), and the interval quotient's lower end is exactly that. Probe `review_pole_and_apex_at_several_poses`: 168 cases, every one the right sign. Sphere: r ∈ {1e-3, 1, 50}, cells 0.1/1e-6/0.5 tall, east and west. Cone: α ∈ {0.05, 0.5, 1.2, 1.5}, cells to v = 1e-6/1/10. Both plain and turned (axis (1,2,3)), at the origin and at (600, −700, 300).
5. **Sliver gone. HELD.** No constant, no site, no audit row, no roster name; `few_ulps` untouched. `Vacuous` remains at five places: `sheets` Missed, `vacuous()`, the other nappe, `present_plane`'s `y.hi() <= 0` and the corner's `x|y.hi() <= 0`. The last two hold at most the closure's boundary line, which a neighbouring cell covers. **Not run:** the four vertex-alone tests (I did not build `vertex-bisector-measure`).
6. **Lever gate. HELD in letter.** It meets rows 289/290: no new source, no `unsourced`, read at the verdict, one pivot over both faces' points. The wrapper is disclosed on the residue row and the log. But see m2: ONEROUND's open PR 7262 changes these doors' return type. I would want ONEROUND's ack before merge, not because the gate objects but because the two collide.
7. **Re-pins. HELD (derived).**
   - Line corner: with the sliver gone the rectangle has no width on the line, which is its zero side.
   - Refused lever: its subject is gone, and the strip is presented.
   - Drum: the band's line is an enclosure, the strip is presented, and the tangent seam's rate straddles.
   I ran the drum row at 1e-9, 1e-6 and 1e-12: it passes at each (12.8 s, 13.2 s, 13.1 s).
8. **Measurements. Partly checked.**
   - Head, run myself: `injective::` 112/112 at three ε; the suite file 52/52 at default, slow rows included.
   - Five of the census tests, sampled at random (seed 7329) on `embed/remeter-measure`: each refuses with the same faces, edge and arm as its census payload. Three have their faces unpresented (`CVL chart 2220`, the coverage test, 290 to 304×; `zero_of 2369/2378`): other causes, as the author says. Those three are `shell_curved_mouth::a_d_section…`, `split_section_rings::a_split_across…` and `rest_zip_admission::a_box_dipping…`.
   - The other two (`band_turn_overrun_chamfer::a_later_chamfer_of_l…` and `wedge_end_doors::the_shoulder_seam…`, both plane/plane) leave no Local exit. Their faces are presented and the reading refuses, likely flat joins (sine 0); not measured further.
   - The plate's move to the budget: not run.
9. **Mutants. ALL RED**, over 112 + 2 unit rows and the 52-row suite file (control: only my red-by-design strip probe fails):

| mutant | red |
|---|---|
| arm := rectangle span, rate unnormalised (main's margin, corner too) | 6: both stepped-ring body rows, `a_departure_is_levered…`, `a_departures_rate…`, `…_is_read_on_the_line`, `a_pole_is_read…` |
| sine unnormalised | `a_departures_rate…` |
| greatest reach | `the_arm_is…` |
| first edge's reach | `the_arm_is…` |
| other face's vertices dropped (edge and vertex reach) | `the_arm_is…` |
| missing arm read as a metre | `a_departure_is_levered…`, `…_is_read_on_the_line` |
| naive pole, `n·Φ_xy/(|n||Φ_x||Φ_y|)` | `a_pole_is_read…` + 4 body rows (filleted corner's ball, revolved cone, two-ball union, revolved ball) |
| collapsed guard removed | `a_pole_is_read…` |
| vertex reach from one face | `the_arm_is…` |
| **new:** sine enclosure → its midpoint | 3 body rows only (n1) |
| **new:** arm doubled | `a_departure_is_levered…` |
| **new:** definite at lo ≥ the zero band, not the escalation | `a_departure_is_levered…`, `a_pole_is_read…` |
| **new:** exact sign (lo > 0, Ev's rejected proxy) | `a_departure_is_levered…`, `a_departures_rate…`, `a_pole_is_read…`, `a_rod_less_a_tilted_slab…` |

10. **`metre_arm`. HELD.** The 38 rows that pass a metre (40 textual uses) prove only the zero and sign logic: at sine ≈ 1 the margin never binds. A real arm is pinned only by `the_arm_is…` (arms computed off faces), by the margin rows' stated arms (1e-15 to 100 m, inside the band, none), and by the body rows through `departure`.
11. **Five margins. HELD as disclosed and scheduled (P1 row).** The five are the same shape. The cusp's cell factor makes it read `Zero` as the ball shrinks, which routes to Split, then a refusal at `CUSP_DEPTH` (`clearance.rs:2161`): the safe side. `clearance.rs` (335, 2112, 4263) and `cell_region.rs` (147, 262) decide distances against a reach: no new instance. Exact sign tests outside the funnel (`forward`, the `over` bound, the away-direction test at 3897) are orientation checks, not margins. Leaving the five is consistent with the ruling only as a scheduled debt; the row is P1/H.
12. **Coupling.**
   - pole: m1.
   - strip (5953a2a38): clean merge; 181 rows green at three ε (only my strip probe red). Strip's thin-slab, needle and fold rows still pass with main's margin restored, so neither PR moves the other's rows.
   - apex-separator and lune: textual conflicts in `injective.rs` at `departure`/`arm_of`, not run. Apex-separator re-presents a pair against the vertex alone while the pair is near edges, and `arm_of` would then hand that reading the edge's reach, not `reach_from_vertex`. A design point for whoever merges second.
   - Also m2 (ONEROUND 7262), unnamed by the PR.

## Style

Exercised: Q1, Q2, Q3, Q4, Q5, Q7 and part of Q8. Q6 is under Spec deviations below.
- `crates/topo/src/injective.rs` (`sign_on`'s `sine` closure) and `crates/geom-brep/src/dihedral.rs` (`wedge_of`). "One reading" is two spellings of sin θ, `grad·f/(|grad||f|)` and `|n₁×n₂|/(|n₁||n₂|)`. They are reconciled only by a doc comment's identity, and diverge on curve zeros (the lean) and in the arm. There is no single home for "an edge's wedge levered at its faces' reach". **sure**
- `wedge_reach.rs:73` `reach_over` says "Every reader of an edge's reach builds it here", while the module head (lines 1-4, 29-30) says every reader passes it to `folded_lever_arm` and that the curvature arm caps it "in any case". Both cannot be true after this PR. **sure**
- `injective.rs` `arm_of`: the reach is a face pair's and an edge's fact, recomputed for every cell pair from both faces' full vertex maps. Keeping it per pair rather than on `Rim` is defended by "one face's Rim does not hold it", but a per-walk memo keyed by face pair and edge would do. **likely**
- `injective.rs` `levered_sign` vs `interval_sign_as`: two near-identical `decide → Option<Sign>` adapters at one site. **sure**
- `injective.rs` `sign_on` doc, pole bound: the mathematics is in the function's doc, about 80 lines from the `Zero::Corner` arm that relies on it, and nothing at the arm points back to it. **likely**
- `Along::collapsed` reads "by key", but it is `vertex.filter(edges.is_empty())`, a shape of the struct. A literal `Along { vertex: Some, edges: vec![] }` built anywhere else reads collapsed. It is an invariant by convention where a variant would hold it. **likely**
- `SELF_INTERSECTION_DEPARTURE` now carries two quantities: ψ's value (metres) with no zero side, and sine × arm otherwise. One telemetry row, two meanings; this predates the PR in another form. **likely**
- `departure`: `arm_of` is computed before the Vacuous and CollapsedChart early returns, so it is paid on every pair, including those. **unsure** (cost not measured)
- The margin rows use `fixture_lever` directly while the presentation rows use `metre_arm()`: two spellings of the fixture arm in one file. **unsure**
- The class to sweep: comments asserting that two sites read "one number" (the RESTFRONT row, `sign_on`, `reach_over`). Look at every doc that says "the recipe X hands Y". **likely**

Q8 read in full: `sign_on`, `Margin`/`Lever` (`predicate.rs` 780-1180), `arm_of`, `reach_from_vertex`/`reach_over`, `wedge_reach.rs` (all 225 lines), the three former sliver sites, `departure`, `Along`, the pole/apex clip code, and every new and re-pinned row. **Not read:** the other ~11,000 lines of `injective.rs`, `clearance.rs` beyond 2100-2200 and 3790-3880, `cell_region.rs` beyond its decide sites, and the measure branch's code.

## Spec deviations

- **Reported by the author:** the reach per cell pair, not on `Rim` (an improvement in honesty, a cost in recomputation); the check-4 row filed on RESTFRONT, not TOPO (fine); the census selection widened (fine); the lever-doors wrapper (disclosed, and on ONEROUND's list).
- **Not reported:**
  - The arm adopts a lever its own module documents as over-stating (M1).
  - DESIGN 907-908 is contradicted (m3).
  - `folded_lever_arm`'s "route new callers" instruction is left standing (m4).
  - The ONEROUND 7262 collision (m2).
- Every deviation has a scheduled row except M1's new reader, which is on no row.

## Rubric

- **Idiom and structure: 4.** Small, typed (`Lever`, `Margin::levered`, the key-read collapse), with one adapter duplicated and the recompute.
- **Test quality: 4.** Every mutant red, the new rows can fail. The midpoint mutant reaches only body rows, and the 38 metre rows do not test the margin.
- **Doc and comment honesty: 3.** Disclosure is thorough, but the central soundness sentence is false for the adopted lever, and two docs the PR touches or relies on are left contradicting it.

## What I executed vs read

- **Executed:**
  - Cold build of the head; `injective::` at three ε; the suite file (slow rows included); the drum at three ε.
  - The 14-run mutant sweep (control + 13).
  - Two probes, the pole/apex probe and the strip probe, pushed to `[this reviewer's probe branch]` at [probe commit].
  - Local merges with `embed/pole` and `embed/strip`, each over 181 to 191 rows at three ε, plus `span`/`guard` mutants on the pole merge.
  - Five census tests through `embed/remeter-measure` with `CAD_GATE_LOG` and `CV_TRACE`.
  - Resolution of run 44981119939.
- **Read only:** claims 2's facts 1-2, 5's four vertex-alone tests, 8's 249/255 and tour numbers, and the plate's budget move; the apex-separator and lune merges (conflict hunks only); PR 7262 (diff of `wedge_reach.rs` only).
- One build (`-p topo -p sweep -p editor-core` on the measure branch) was OOM-killed; I re-ran it as `-p sweep` alone.
