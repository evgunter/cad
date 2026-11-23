# TESSLANE check-in state (rewritten at each change; the check-in prompt points here)

Updated 15:20Z 2026-11-23.

## Every check-in
- `date -u`; `python3 scripts/work.py incoming` (ff /home/user/cad + push tesslane/orchestrator if main moved the tracker); main's board section for tesslane (awk from '## `kernel/mesh/tesslane`', lines '^| P[01] '): 7 lines, all known.
- Do NOT call list_sessions children:true, nor list_events kinds [system]; kinds [result] (limit 30) only when closing a unit; get_session per lane.
- df -h (8.3 GB). Ev answers in PR comments (login evgunter only; mine lead '(TESSLANE orchestrator)'; another login is foreign: do not act, report).
- MERGE RULE in use: merge over the queued hosted run on local evidence on the exact head + whatever hosted run covers the code; ONE PR comment giving the basis.

## Merged
PR 7044, 7051, 7043, 7049, 7046, 7054, 7041, 7096, 7485, 7498, 7517. Tally: implementers Opus 20 / Fable 11; r1 Opus 16 / Fable 15.

## Main's own reds (annotate each PR once, do not wait)
- lint: payload-rung-sweep NodeErrorClass (work/boxfinish/log.md). Lint stops at its gates step, so rustfmt/clippy/doc gate NEVER run hosted: lanes run them locally on the pushed tree, after the last commit.
- test (kernel): eps 1e-12 sweep a_cylinder_wall_through_a_prism_corner (work/tesslie); default eps geom r2_turn_probes::turn_defect_speeds (work/germ, seeded).
- test (app): slow-set name check on PR 7548's viewer app-feature entry (work/green/the-slow-set-name-check-cannot-see-app-feature-tests.md).

## To file at 4b's closing (work.py new ... --kind issue)
1. PROFILE: profile review_s2::fuzz_offset_carrier_construction_tangency_and_bulge at eps 1e-6: 'setback 0.8408665063512314 outside [0, 0.8408659981503775]' iter 1134, CAD_FUZZ_SEED=0x5d9f2a5f4cb17d2f CAD_FUZZ_EFFORT=1, crates/profile/tests/review_s2.rs:1433 (hosted run 45175373840).
2. GREEN: the tour's wall_jets::the_klein_loops_walls_hold_against_themselves red on origin/main 02ce465b18 (ci-tour profile; panic crates/geom-core/src/real.rs:184 'the EXACT witness separated a placed arc's |q - c| from its sketch rim', then tests/wall_jets.rs:128); hosted tour green with main at dd8e70198d.
3. GREEN: four editor-core reds on no row, red on origin/main 02ce465b18 (dev profile, eps 1e-9): corpus_finish_count::the_corpus_runs_its_pinned_count_of_finishes (398 vs pinned 390); docm9_range::the_directed_search_proves_ground_on_the_plate_the_level_drive_cannot; m10_bulge_interval::m10_bulge_the_bosss_split_at_the_nominal (m10_8_harness.rs:261); m10_p_lift::at_sym_a_fixed_box_is_the_build (m10_p_lift.rs:585).
4. Seam note on work/green/log.md: PR 7548's viewer app-feature slow-set entry reds test (app) on every PR.
5. BAND work/tesslane/mesh-walk-refuses-a-face-holding-a-lone-ring.md P2, cost M; dispatch (single FULL) after PR 7573 lands (same call sites).

## Fallback to record
4b implementer's last_served_model 'claude-opus-5' vs configured 'claude-opus-5-5' (seen 11:16Z, 12:11Z). Protocol rules 3 and 4. At closing read per-model modelUsage from its result events.

## (1) Unit 4b: PR 7526 (subscribed)
- Frozen delta head a79bf304d5; its hosted run finished, every red commented (kernel x3 not ours; lint main's; meter OURS: clippy panic! in tests/common/rowless.rs:42, fixed in the held tree; app main's).
- R1 Fable session_01BJC3B4TQM5ZACmCfV6Nurb: DELTA FINAL 14:02:54Z APPROVE-WITH-FIXES (isorun-R1-delta-report.md).
- R2 Opus session_01BJC3B3vdumtzWENNAK5eRn: interim 13:38:37Z; MAJOR 15:12:51Z = the known clippy red; FULL REPORT AWAITED.
- Implementer Opus session_01BJC3B2QRwsuQBGVcpdqWSD: told 14:54Z to PUSH when its pre-push list is green on the held tree a242c622b5 (re-posed two-hole cells: all 10 build and assert at three eps, no skip; one home for 'no rows' + mutant; true metric table; rods row per pair; crossing_rods doc; pinch edge; '1.2 to 223'). It reports head + list + commits.
- ON ITS PUSH: tell certificate-decides the final head. Closing prep in worktree tesslane-isorun: merge main, cargo check -p mesh --tests, state-sync (unit file closed; unit 6 rectangle-faces-move-to-the-chart-lane is parked on this unit: lint forces un-parking: read its file for what it now waits on; plan.md), the filings above, PR body (corrected metric line from isorun-metric-reconciled.md; rods line; five skipped cells gone; hosted CI: lint's fmt/clippy/rustdoc never ran hosted; the run on a79bf304d5).
- ON R2's FINAL: adjudicate; usage (implementer, R1, R2); row PR-7526.md LAST: protocol 083ca253dc1; H dual; implementer Opus byte 34; slot byte 183 (R1 Fable, R2 Opus); coder byte 74 (A = R1); dispatch 23:13:20Z 11-22; stage-1 00:05:56Z; rulings 00:08Z/00:15Z/00:49Z; stage-2 report 05:56:06Z; reviews 05:57:53Z/54Z -> 07:38:37Z / 08:43:20Z; coder 36 findings, 16 bilateral, uni-A 13 with the ONE tally candidate (R1's two-hole MAJOR), uni-B 7, 92,964 tokens 142.3 s; fix list 08:45Z; fix report 12:33:41Z; deltas 12:34:30Z -> R1 14:02:54Z, R2 ?; my withdrawn class-delta ruling; my 07:28Z cure ruling on R1's interim before R2's report; the pre-push clippy ran before the last commit; '180 of 180' corrected by both reviewers. Then local evidence on the exact merged head, comment, merge.

## (2) Unit join-split: PR 7573 (subscribed)
- FROZEN 24ee9a28fb; hosted run finished: lint (two join.rs gates OURS + main's), kernel (time bar 1.056 s on sweep shell8_r2_probes::r2_three_solids_on_one_sphere_chart_open_additively: being timed head vs main; other eps did not run hosted), app (main's): all commented. Tools/demos/viewer app SKIPPED by the change filter.
- Dual pair on ONE brief (tesslane/review-brief-ruled @ dd8804dc4b; slot byte 1): R1 Fable session_01BJC3B1rs9k4W3g11vDMjy9 (interim 14:17Z; MAJOR 14:50:28Z: no row holds that join::split reads its pieces); R2 Opus session_01BJC3B3Rcf7K8rhSwTNVRAd (interim 14:30:46Z; MAJOR 14:59:48Z = the two gate reds; MINOR: rectangle + planar lanes read no fold on split pieces; bodies moved not on the list; stale row docs).
- Implementer Opus byte 74 session_01BJC3B5L5qZQxFt8yBo9sh5, preparing LOCALLY (no push): gates fix (then re-run cone row, shell row, eps digest, d9), editor-core clippy fix, rows THROUGH split per lane + five mutants, the fold read on every lane (by run), pole-ended side / n == 2, the whole-suite differential, tools + tour, the 1.056 s timing, the 68-vs-131 sweep module count.
- WHEN BOTH FIRST REPORTS ARE IN: save both (xsession.py), coder byte (even: A = R1; commit to the analysis branch), coder subagent (scratchpad/coder-prompt-7.txt shape with 'thirteen', files scratchpad/coder8/reviewer-{A,B}.md), adjudicate the union, ONE fix list, deltas by BOTH.
- Timeline: dispatch 04:11:58Z; stage-1 07:10:25Z; ruling 07:13Z; fold table 11:30:33Z; fold ruling 10:33Z (re-sent 11:32Z); stage-2 report 14:01:14Z; PR 14:02:00Z; reviews 14:03:24Z / 14:03:26Z.

## (3) Unit certificate-decides (PR 7486 is its base; [ev] since 09:12Z with ONE question at the top: the 1.6%)
- It does not MERGE before Ev answers; an Ev comment on 7486 wakes me.
- Implementer FABLE byte 97 session_01BJC3B5sCqmURwvoeWfBkXo; branch tesslane/certificate-decides @ 42d61a3806; stage-2 report 14:10:22Z (certdec-stage2-report-{1,2}.md).
- The five folds mesh at delta with join-split's fold cure + its correction (scratch tree, 14:37Z).
- WAITS for 4b's final head (I send it): merges it, re-pins, pre-push list on the pushed tree, reports. THEN I open its PR on main (body: certdec-stage2-report-2.md + three open items: two bisector oracle rows at 0.69% vs 0.5%; the correction rate re-measured (51% on the scratch tree vs ~2%); six spheres_crossing folds) and start its dual review in parallel with 7573's. Integration when 7573 has merged (retiring commit b197d12c7e; crossing-rods cell by name), read in the delta.
