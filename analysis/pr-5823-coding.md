# PR 5823 dual review — blind coding inputs

The A/B labels were assigned by a recorded byte; the mapping is held by the orchestrator, not in this file.

Frozen head: 0bfb067e682e7e2781809f48239210fe9ccf3340 (merge-base 0bf14bca). Arm: holdout (a concurrent pair), letter M. Both reviewers were dispatched at 03:11Z with one brief. Neither was interrupted, and both report that lane isolation held.

## Review A

ATREST dual review (FULL) of PR #5823 @ 0bfb067e: APPROVE-WITH-FIXES, 1 MAJOR / 3 MINOR / 4 NOTE (+9 style).

Merge-base 0bf14bca. Probe branch: `the reviewer's probe branch` (pushed), file `crates/topo/tests/review_atrest_crossing_probes.rs`, 5 rows: `probe_inscribed_tetrahedron_turned`, `probe_cubes_touching_rest`, `probe_apex_and_box_near_a_face`, `probe_thin_wedge`, `probe_placement_findings_sees_a_pierced_pair`. Lane isolation kept: I fetched only the PR branch and main, read only the PR body and diff, and saw no other review branch or PR comment.

## MAJOR
M1. **A fast-set editor-core row goes red on head and is green on base.** Row: `refusal_concision_at_rest::every_at_rest_finding_renders_to_the_standard` (`crates/editor-core/tests/refusal_concision_at_rest.rs` `LABELS`). The new `TouchInterference` text (`crates/topo/src/validate.rs`, Display arm `TouchInterference`) opens with "two instances overlap:". `LABELS` admits that opening only for `InstanceInterference` and `SameSideInterference`, so every route fails with 'carries the stage prefix "two instances overlap:"'. The row is not in the nextest slow set, so per-PR CI goes red. The fix is one `LABELS` row. Breaks claims 5 and 6 (the rendering), and the PR body's verification list, which names no editor-core rows beyond the goldens. Executed (head binary red; merge-base binary green). sure.

## MINOR
m1. **`placement_findings` / `is_decided_overlap` (`crates/topo/src/lib.rs` test_support) no longer see a pierce-, same-side-cross- or wedge-decided interference.** Before, arm 2's "another finding reports their boundaries crossing" `CensusUndecidable` stood beside those pairs. This PR drops it (the `Said::Pierce | Said::Wedges(None) =&gt; None` arm, plus the `decided` precheck). Rows that assert a pose clears by `placement_findings(..).is_empty()` can no longer go red on a pierce-only overlap: contact1 ×7, contact5 ×4 including the grid sweep's `refused`, bool4/bool4r1/bool4r2, mate9. The PR patched only `contact7_touch_sweeps::refused`, locally. Probe `probe_placement_findings_sees_a_pierced_pair` (a box tilted 1e-3 into a cube along a resting edge, decided by pierces) is green on base and red on head at ε 1e-6, 1e-9 and 1e-12. Breaks claim 4/5 in their test-guard sense. Executed. sure.

m2. **The wider rule (`decided` precheck, `census.rs` arm 2 `if !interfered &amp;&amp; coincident.is_none() &amp;&amp; !decided(..)`) has no witness.** Mutant `(true || !decided(..))` stays green on topo `census::`, `coincident_line_wedges`, contact5 and contact7. The restated rows (sunk beam, sunk ridges, pinwheel 5→4) are restated by `blocks`' own Pierce/Wedges(None) → None change, not by the precheck. The precheck's distinct effect is unpinned: dropping escalation/unexamined/undecided-touch `CensusUndecidable`s and the probe's orderings beside a decided pair. A ready witness: `probe_thin_wedge` at −300ε (some poses) has `CensusEscalated` beside the pierces. Executed. sure.

m3. **"A crossing outranks an earlier undecided reading" has no witness.** Mutant: `Some(why) =&gt; return Some(Blocked::Undecided(why))` in place of `other = other.or(what)`. It stays green on `census::`, `link::` and 9 integration modules (coincident_line_wedges, contact5, contact7, two_copies, a_vertex_read_by_two_sector_passes, bool4*, contact1, mate9, h14). The tetrahedron witness has no undecided reading beside its crossing. Executed (not the full suite). likely.

## NOTE
n1. **Claim 4 is sound: the `decided` skip hides no finding.** The standing findings `blocks` read (escalations, unexamined, curved or unread touches) remain in `errors`. At rest they are kept, or ride the pair with `covered` false (`checks/at_rest.rs` `partition`/`localize`), and the gate's intersection localizes every overlap component. Behaviour change: a pierce- or wedge-decided pair between copies is now quietable by an assertion (before, the solid-pair `CensusUndecidable` held `covered` false). That matches ASSEMBLY A5's intent, but no editor row pins it. likely.

n2. **`blocks`' `Said::Pierce | Said::Wedges(None) =&gt; None` (census.rs ~8717) holds only because the precheck ran first.** If `census_interference_solids` and `solid_of_entity` ever resolve a cell differently, a decided pair clears arm 2 silently. They agree today (both resolve through `he_plus`'s face). A loud arm (unreachable or a refusal) would be fail-loud. unsure.

n3. **`link.rs` `rest::judge` returns `Crossing` on an `In` probe even when that probe set `notes.degenerate`** (a probe on another face of its own link). Pre-existing, but this PR raises its consequence from undecidable to decided interference. A fin (doubled face, no volume) reading `In` would mint a false `TouchInterference`. Not constructed. unsure.

n4. **A crossing now outranks a standing same-solid `BoundaryCrossesItself` (`Undecided::Crossing`) on the pair**, while the link analysis presumes each link bounds one material. The body is refused anyway. unsure.

## Claims
1. Holds. Code read: `crosses` needs `(Pos,Neg)` both decided, `Place::Inside` and `on_cut == Some(false)`, and the probe path needs `reads_over` (None in band) or `On` with `Aligned`, each confirmed in the ball. Executed on head: 10 offsets (0, ±0.3ε, ±3ε, ±30ε, −300ε, ±1e-3) × {apex on face, edge on face, box on face, apex on edge, thin wedge tilted on its edge} × 6 poses (5 `poses()` plus an odd turn), at ε 1e-6, 1e-9, 1e-12. Zero `TouchInterference` in band; in band reads `CensusEscalated`, a `CensusUndecidable` or a rest. sure.
2. Holds on everything I built. Gaps and exact touches never decide interference (face, edge and corner cube contacts, the rows in 1, every pose, three ε). `four_pairs_battery` (sweep, 5340 lines) is byte-identical head vs base: 4208 SOUND / 982 ERR SharedVertexCrossings / 143 SKIP, zero `MixedTouch` or `TouchInterference`. `join_pierce_runs_sweep` non-ignored: 29/29. The turned witness reads `TouchInterference` in all 6 poses. likely (n3 and n4 open).
3. Holds. `MixedTouch` had two producers: `TouchVerdict::Crossing`, now `Blocked::Crossing` → TI; and cross-solid `Said::Wedges(None)`, now decided by `CoincidentWedgesOverlap` through `census_interference_solids`. Neither goes silent; the pinwheel row confirms. sure.
4. Holds (n1); restatements correct. The sunk beam and ridges read 2 and 1 interfering pairs, with no refusal beside them. The pinwheel reads 4 decided overlaps. contact7 `refused` is right, but local (m1). The precheck itself is unpinned (m2).
5. Mostly holds. editor-core `perf12_census_goldens` and `perf12_census_bvh_diff` plus step-import `perf12_census_goldens` are bit-exact at 1e-6, 1e-9 and 1e-12. topo `census::`/`link::` (245) and coincident_line_wedges/contact5/contact7/two_copies/a_vertex_read rows (41) pass at all three ε. An editor-core subset (at_rest|interfer|assembl|census|two_copies|aligned|asm|quiet|overlap|mate|concision|docm9; 418 rows) gives 412 ok, 6 FAILED: 5 `docm9_range::*` reds that are also red on the merge-base, so inherited; and M1, which is this PR's.
6. Placed consistently in `ops.rs`, `finding_solids`, the tags and `TAG_INVENTORY` (alphabetical), `MEMBERS_BOUND_AS` and the samples. `finding_solids` → `census_cell_solid` gives the right two solids. The rendering fails M1. pncad-py and Python not executed.
7. Holds. Mutant `Self::Crossing =&gt; Some(Unrest::Undecided(TouchInBand))` turns the witness red (and `probe_inscribed_tetrahedron_turned`). No other named row catches it: the `a_vertex_read_by_two_sector_passes` allowlist accepts both readings.

## Style
Questions exercised: Q1, Q2 (pattern grep over the diff plus a read), Q3 (three mutants), Q4, Q5, Q6, Q7. Q8 was partial: arm 2 of census.rs (~8580–9090), the `Said`/`Named`/`Unrest`/`Blocked` region, and link.rs `judge`/`crosses`/`reads_over`; census.rs is not read whole.
- S1 (Q1). Two spellings of "decided overlap": `test_support::is_decided_overlap` (3 arms), whose doc claims to be "the one home ... so a new decided verdict reaches all of them", and `census_interference_solids` (6 arms). contact7's `refused` now ORs both, a third spelling minted by the fix (m1's root). Class: look at every `placement_findings` caller. sure.
- S2 (Q1/Q7). `TouchVerdict` / `Unrest` / `Blocked` are three near-parallel enums. `Option&lt;Unrest&gt;` is `TouchVerdict` with `Unread` remapped. `TouchVerdict::refusal()` now returns a decided interference, but its name still says refusal. sure.
- S3 (Q4). `Undecided::Crossing`'s `what()` still says "another finding reports their boundaries crossing ... fix that crossing first". It now fires only for one solid crossing itself, so "their boundaries" misleads the user. likely.
- S4 (Q4). Arm 2's comment ("With none, the findings are read whether or not the probe decided ...") does not mention the new `decided` precheck. Premise (1)'s "the census refuses a solid whose boundary crosses itself (`blocks`)" is now skipped for decided pairs; harmless for clears. likely.
- S5 (Q5). `TouchInterference`'s Display names no location though it carries two cells. SameSide has the same gap. unsure.
- S6 (Q4). README C6's gate sentence ("a pierce and a same-side crossing as interference evidence") omits coincident wedges, which ASSEMBLY.md now lists. ASSEMBLY.md ~588 is an unwrapped &gt;100-column line, and the `census_interference_solids` doc is rewrapped unevenly. sure (nits).
- S7 (Q3). The witness is axis-aligned (cube plus face diagonals). My turned copy passes in 6 poses; the turned form would be the stronger pin. sure.
- S8 (Q6). "Other programs' rows still say MixedTouch ... left as written" (`work/contacthold/declared-only-…`, `…vertex-on-edge-record-alone…`) is stale narrative with no schedule, and the #5810 re-bless is likewise conditional and unscheduled. likely.
- S9 (dispatch premise). Claim 5's list and the PR's verification omit editor-core's concision rows, which is where M1 sits. The lane's "goldens unmoved" framing would have passed it. sure.

## Review B

FULL review of PR #5823 at 0bfb067e (merge-base 0bf14bca): APPROVE-WITH-FIXES. 1 MAJOR / 2 MINOR / 3 NOTE, plus style findings.

Lane isolation: I did not read any other review/atrest-crossing-* branch, the PR's comments or any review scratch. Probe branch: the reviewer's probe branch (crates/topo/tests/review_atrest_crossing_probes.rs, wired into all.rs). It has three probes:
- probe_inscribed_tetrahedron_at_every_pose
- probe_apex_on_a_face_is_no_interference
- probe_thin_wedges_and_coplanar_sheets_are_no_interference

How I ran things: dev profile, nextest --profile ci --ignore-default-filter, private target dirs. Head and base were both built, in worktrees.

## MAJOR
M1. The PR turns CI red: editor-core `refusal_concision_at_rest::every_at_rest_finding_renders_to_the_standard` fails on head and passes on the merge-base (executed on both).
- The new sample (`test_support_samples.rs` TouchInterference) renders "two instances overlap: where they touch, …". The checker reads "two instances overlap:" as a stage prefix on all three wrappers (at rest, product, at rest+product).
- `refusal_concision_at_rest.rs` LABELS admits that opening for InstanceInterference and SameSideInterference only. TouchInterference has no row.
- Claim 6 (placed everywhere) misses this table. The PR body's suite claim covers topo only; editor-core tests were not run.
- Head editor-core: 2864 tests run, this is the only failure.
- Confidence: sure.

## MINOR
m1. The wider rule ("a decided pair is not read again") is written twice, and neither copy is pinned on its own. I executed four mutants on the census, contact5/7, coincident_line_wedges, a_vertex_read…, two_copies and probe rows (141 tests):
- m2, gate removed (`census.rs` `sweep_cross_solid_backstop`, the `!decided(&amp;errors[..standing], sa, sb)` term): all green.
- m3, `blocks`' `Said::Pierce | Said::Wedges(None) =&gt; None` restored to `Some(Undecided::Crossing)`: all green. Under the gate those arms are unreachable. Their comment "read before this" is an invariant the code does not enforce.
- m4, both reverted together: red on sunk beam, sunk ridges and pinwheel.

So the half of the rule that "drops the probe's undecided orderings" (`errors.extend(undecided)` skipped) is exercised by no row: no decided pair in the tree has a probe that left orderings undecided. Claim 4 holds on the rows I read, but that half is unwitnessed. Confidence: sure (mutants executed).

m2. TouchInterference is never read at the assembly layer.
- No editor-core row builds two copies whose only decided overlap is a crossing touch. So `checks/at_rest.rs` `finding_solids`' new arm, `partition` and `localize` run on the new verdict only through the rendering sample.
- The same goes for the behaviour change the PR body states: a pierce-, same-side- or wedge-decided copy pair loses its solid-pair CensusUndecidable, so `covered` can now go true and an assertion can quiet it. No editor-core row pins that pair as quietable (or as loud).
- Confidence: likely (I grepped editor-core tests for TouchInterference and pierce-quiet rows; no assembly probe built).

## NOTE
N1. Claims 1 and 2 held in everything I ran (executed at ε 1e-6, 1e-9 and 1e-12, five poses: rest, turned about z, general turn, flipped, flipped+turned):
- The inscribed tetrahedron is exactly one TouchInterference with no undecidable at every pose and ε, including scaled by 1±0.25ε. Scaled by ±1000ε it becomes InstanceInterference, plus pierces.
- A pyramid apex on a face interior: apex ±0.25/±0.5ε off the face, pyramid height 1, 1e-3 and 1e-5 (thin wedges). Never a decided interference.
- A box on its bottom edge with the far side lifted r·ε, r from 0.25 to 1e6: never interference. Dipped (genuine overlap), it is undecided or escalated for r ≤ 8 and InstanceInterference for r ≥ 1000. Never TouchInterference.
- A flat box turned 0.4 rad about z, inside or overhanging the cube's top edges, sunk 0/0.25/0.5ε: never interference.
- The sweep pin `three_pairs_at_one_vertex_build_on_one_key_and_every_op_is_sound` passes on head, so the old false-MixedTouch pinch does not come back as a false TouchInterference.
- Confidence: likely. I read judge and crosses; every Crossing return sits behind Some(..) readings confirmed in the ball.

N2. Claims 3, 5 and 7 held.
- Claim 3: every former MixedTouch path now returns Blocked::Crossing → TouchInterference, or is skipped by the gate while its own decided finding stands. I found no path that went silent (code read). No other code or golden matches the retired sentence (git grep).
- Claim 5: perf12 goldens (editor-core goldens and bvh_diff, step-import), 7 rows, pass at all three ε on head and base. The topo census/link/coincident_line_wedges/contact5/7/two_copies filter: 355 rows on head and 354 on base (+1 witness), all green at all three ε.
- Claim 7: mutating `Self::Crossing =&gt; Some(Unrest::Undecided(TouchInBand))` turns the witness red. It also turns my pose probe red, and nothing else. TouchInterference is pinned by that one row; the a_vertex_read allowlist admits it but does not require it. Confidence: sure.

N3. Claim 4 edge case (unsure, not executed). `localize`'s `covered` treats any evidence other than CensusUndecidable or UnrecordedContact as covered (`_ =&gt; true`). With the gate, a pierce-decided copy pair carrying a CensusUnsupported FacePair (Said::Unexamined) no longer gets the solid-pair CensusUndecidable that used to keep it loud, so a Gap assertion can now quiet it. It may be fine, since the intersection boolean is the ground truth, but it is a new quiet path on a pair the census never examined.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q7; Q6 lightly. Q8 not done: I read census.rs's touched regions (~6780–7660, 8600–9100, 10170–10290), not the whole 15k-line file.
- S1 (Q1/Q2) `census.rs` `blocks`, `Said::Pierce | Said::Wedges(None) =&gt; None` beside the `decided` closure. One rule has two homes, and the arm's comment asserts an ordering only the call site holds. See m1. sure.
- S2 (Q1) `census.rs` `enum Unrest` and `enum Blocked` are near-parallel ({Crossing, Undecided(Undecided)} vs {Crossing(TouchSite), Undecided(Undecided)}). `TouchVerdict::refusal` now returns an `Unrest` and refuses nothing, so the name is off. likely.
- S3 (Q4/Q5) `census.rs` `Undecided::Crossing.what()` still says "another finding reports their boundaries crossing". It now fires only for one solid's own self-crossing, so "their" reads as the pair's. The variant doc was updated; the user-facing sentence was not. likely.
- S4 (Q4) `editor-core/src/checks/at_rest.rs` `partition` comment: "What the census left undecided … — a crossing standing on the pair …" describes evidence that a decided pair no longer carries. The doc rotted; the code is right. likely.
- S5 (Q3) `contact5_gate_and_beam.rs` `refusals(&amp;errors) == []` on the sunk rows is monotone in the wrong direction (m2/m3 stay green). The restated rows pin the sum of the two copies, not either one. sure.
- S6 (Q7) `contact5_gate_and_beam.rs`: the new `fn interfering` is inserted between the `CROSSING` and `UNEXAMINED` consts. sure, cosmetic.
- S7 (Q7) Unreflowed lines: ASSEMBLY.md ~588 (one overlong line), the census.rs module doc ("…as [`ValidationError::TouchInterference`].\n//!   Why that decides a"), the `census_interference_solids` doc ("…wedges overlap. `None` for every other finding,") and `a_vertex_read_by_two_sector_passes.rs` ~167. sure, trivial.
- S8 (Q5) The PR body says "None moved" for goldens and that the full topo suite passes. That is true as scoped, but it leaves out editor-core's suite, where M1 is red. sure.

The PR's restated rows are right as restated. I checked contact5 sunk beam (2 pairs), sunk ridges (1 pair), contact7 `refused` (a pierce is invisible to placement_findings, so the extra disjunct is needed), the pinwheel (5→4 findings) and a_vertex_read's allowlist.
