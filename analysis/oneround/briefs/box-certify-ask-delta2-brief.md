# Second delta read: PR 7272 (`oneround/box-certify-ask`), the pass N1 to N6

You reviewed this unit on `34c966f6` and read its fix pass on `1ff43e30` (NOT-MERGEABLE-AS-IS for one mechanical reason, the code at APPROVE-WITH-FIXES). Its last pass is in. Read it now, on the FROZEN head `1e656c82f7824382141a023c71f084d84559e45a` (whatever is pushed later). The same rules as before hold: your branch `oneround/box-certify-ask-review`, pushed as you go; never push to the unit's branch; your own `CARGO_TARGET_DIR` outside the checkout; no waiters or background chains, a long job `setsid`-detached and polled in the foreground; no trailer in your commits and no model named anywhere; do not look at any other session; no helper subagent on a model other than your own, and any helper named in your report.

The head merges its base `oneround/box-region` (PR 7225) at that PR's final head `606af1e32c`, then `origin/main` twice (to `a7b44d8e`). PR 7225 has since been approved by Ev and will merge to main after one more merge of main by its own lane; this unit then retargets to main. One hosted run read an earlier head (run 45291124288 on `95a45dc2`, some jobs still queued when read); none has read the frozen head, which differs from `95a45dc2` by test configuration only (`.config/nextest.toml`'s slow-set names). Your own runs are the evidence.

## What you are asked

Section A is the rulings on your delta read, as sent to the implementer. Section B is the implementer's two reports of the pass. Both are claims. For each of N1 to N6: is it carried out as section B says, on the tree; did the fix mint a fresh instance of what it closes (`docs/prompts/reviewer-style-lane.md`, the stance's last bullet); does anything you raised remain open. Report a ruling you think wrong as a finding.

The claims that matter most, to falsify by execution:

E1. **The re-pins are main's reading plus the box's move, and nothing else** (N1). Each was measured "with the ask off and on in one binary" by an instrument that is not in the tree. Re-take it your own way for every re-pinned row: the value on the frozen head with certification's box ask on, the value with it off (your own switch, uncommitted), and main's pin; each on-minus-off difference is the box's move and each off value is main's reading at this merge. The rows: `asm2a` direct_total 988 → 1012; `m10_8` numeric 496 → 520; `m10_bulge` ×3 `session_box_reach` [0,1] → [0,25] and [0,37] ×2; `sym11` link frozen ceiling 2600 → 2604; `revalue_corpus` plate_param apart 3204/7068 → 3282/7146 (PINNED 3282/7306) and boss_union apart 2384/3826 → 2435/3868 (PINNED 2423/3898); `m10_6` certifying keys re-blessed; sweep's washer 458 → 492 and 608 → 632. **The two revalue pins are not the measured values** (7306 against a measured 7146; 2423/3898 against 2435/3868): the report says they are "main's pin plus the box's move" on rows that are red on main by drift (a filed GREEN row), "so neither pin is read green". Is a pin that no tree produces a pin at all; what does each row assert today, on main and on the head; and is "main's stale pin plus our delta" the right thing to commit, or a number chosen so that the row stays red for main's reason. Say what is true; this is the finding most worth your time.
E2. **The conflict that carried code** (N1's merge): the base's last fix, "a point that is no finite number is no point past the box", met this unit's routing of the fold through the geom-brep core; the guard now lives in `region_in_box` ("a non-finite inside read leaves the cover's refusal, Unshown or Uncovered"), with a new core row `an_inside_read_past_every_number_leaves_the_covers_refusal`, and "certification's inside read already filters non-finite points, so no edge moves". Falsify: a body or an edge with a NaN or an infinite coordinate at each of the five certification doors and at the gate: is any told "past the box by inf m" or admitted; does the base's own row (the brick with an infinite corner; a net with an infinite control point; `UncoveredCause::InfiniteNet`) hold on this head at tier 3 and tier 3′; did the union of the two `UncoveredCause` changes (the base's P4 and this unit's N5) lose or duplicate a variant or a sentence.
E3. **N2's three rows, each red on its mutant** (the order: the span must not be read first; the non-finite filter; the `finish.rs` arm), and the five mutants it lists (span read first: 2 red; the non-finite filter removed: 2; any not-forward span replacing the box: 1; the `finish.rs` arm removed: 1; the ellipse coordinate case removed: 1). Re-plant; add your own in the unsound direction (an edge past the box certified; a boxed refusal filed under family `Certificate`).
E4. **N3 changed code**: only a span decided reversed, or exactly empty (`is_exact_zero(t1 − t0)`), replaces the Uncovered refusal; an in-band nonzero span keeps the box's refusal; a non-finite end is the door's defect culprit "wherever it came from" (argued at `own_ending`: a build door cannot tell a caller's point from one an operation constructed). Falsify each sentence against the body it names: an exactly empty span on a covered edge past the box; a reversed span on an uncovered edge; is "the door's defect" true of a point the CALLER handed a build door, and is the argument for not telling them apart sound or a convenience.
E5. **The far peg.** Main's test `mate5_cyl_eps_rung::a_far_peg_levers_its_tilt_from_its_own_origins` put a 1 mm peg at 1 km, about 1 mm past the box, and was refused at its first edge; the unit re-placed it at 990 m, having "measured on and off in one binary at 500, 900, 950, 990, 999 and 1000 m" that the test reads its subject as before. Is the test's subject (a tilt levered from the peg's own origins) still exercised at 990 m, or did moving it change what the row can catch (plant the defect the row was written for).
E6. **The answer Ev is owed from PR 7140**, at the top of the PR body: "does any construction certify helper or cutter edges past 1 km on purpose?" answered No, with what was searched (the instrumented whole-suite refusal log at three ε: 228 refusals at 1e-9, "every one a test's own far body or a poison fixture, none a cutter, scaffold or helper"; the demos; the five doors' callers; the fixture dispositions; what was not searched). This sentence goes to Ev. Falsify it: look for a production door that builds a helper, a cutter, a scaffold, a tool body or a section's working edge at or past 1 km and certifies it (half-spaces and cutting planes built "large enough"; a sweep's or a revolve's scaffold; a boolean's tool extended past its operand; an offset's or a shell's working surface; STEP import's unbounded carriers trimmed late), by reading the doors and by running your own far-placed bodies through them. One such construction is a MAJOR. Say also what the 228 refusals were judged against.
E7. **N4, N5, N6**: the row renamed for three doors with the reason the other two cannot be asked; `cover_is_exact(carrier, CoverRead::{Distance, Coordinates})` as the one home read by both readers ("it keeps two kind arms for the cover-kind-match gate": is that one statement or two under one name); `UncoveredCause::Carrier`'s doc; the body's one section per head with no hosted claim that is not true.
E8. **What ran, and what did not.** On `95a45dc2`: the fast set of the workspace at 1e-9 (16,900 of 16,900), the re-pin set of 89 tests at three ε with the ask on (79 pass; the 10 reds main's filed revalue drift) and off at 1e-9 (72 pass; the 10 plus exactly the 7 re-pinned rows), plain clippy, fmt, the gates, the tracker lint. NOT run by the lane: the clippy block as `ci.yml` spells it (both lines with `--force-warn clippy::disallowed_methods` and the conic-centre census, under `bash --noprofile --norc -eo pipefail`); `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`; the recourse rosters of the crates it touches (it adds decision names; the report says geom-brep's roster is "the union"); `cargo test -p mesh --features budget`; the fast set at 1e-6 and 1e-12; the slow set. Run those on the frozen head and report each as passed, red and the unit's, or red on main with its row. Its account of hosted run 45291124288's reds: its own `check-slow-set-names` slip (fixed in the frozen head), and main's (`a_cylinder_wall_through_a_prism_corner…` at 1e-12; the `square_search` fuzzer; topo's `recourse_roster`; the ten `revalue_corpus` rows; pncad-py's `prose_census`): confirm from the run or from main.

Verdict on the ladder APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT; findings MAJOR / MINOR / NOTE, each DEMONSTRATED or BY-INSPECTION with `sure` / `likely` / `unsure`; a `## Style` section naming the questions exercised; the three ratings 1 to 5 with a line of evidence each, for the unit as it now stands; what you EXECUTED and what you ONLY READ, and for every count what it is judged against. Commit the report as `analysis/oneround/box-certify-ask/delta2/delta2-report.md` on your branch, push, and send the orchestrator its text (the verdict, every MAJOR and MINOR in full, the rest in brief, with the commit sha). Then stop.

---

# Section A: the rulings on the first delta read, as sent to the implementer

(ONEROUND orchestrator) PR 7272: the delta read on `1ff43e30` is NOT-MERGEABLE-AS-IS for one mechanical reason, with the code at APPROVE-WITH-FIXES (report `analysis/oneround/box-certify-ask/delta/delta-report.md` on `origin/oneround/box-certify-ask-review` at `b7fb8db2`; read it). Every finding of the first review is closed, all seven of its mutants are dead, the fast set is 16,586 of 16,586, and nothing is red with the ask on that is green with it off. The coincident-point change (an uncovered edge whose span runs no length is the caller's input refusal) stands as you built it. Rulings, N1 to N6; one pass, the same reviewer reads it:
N1 (the MAJOR, demonstrated). Your head conflicts with main and NO CI has run on it: run 45187025826 is the `merge ref` workflow reporting the conflict, and the last CI run is on `34c966f6`. The conflicts are nearly every file you re-pin (`.config/nextest.toml`, `asm2a_instantiate`, `m10_6_certifying_keys.txt`, `m10_8`, `m10_9`, `m10_bulge`, `m10_sym_profile`, `revalue_corpus`, `sym11_exact_channel`, `sym_9_retry`, geom-brep `recourse_roster`, `a_sweep_keeps_the_frame…`). Your base's lane (`oneround/box-region`) is merging main now; WAIT for its push (I will tell you the sha), merge it, then `origin/main`, and RE-MEASURE every re-pin on the merged tree with the ask off and on in one binary, each count as main's move plus the box's with its account, as before. The body's "hosted run … queued" and "hosted CI covers every golden this PR re-pins" come out: say what is true (no hosted run has read a head carrying these pins; what you ran locally, by command, with counts).
N2 (d1, three mutants that survive both fast sets): rows for (a) the ORDER: an edge the box shows inside or past it with a span that is not forward (the span must not be read first); (b) `edge_points_reach`'s non-finite filter (a NaN end beside an end 5 km out is past the box by the far end); (c) the new `finish.rs` arm (a boxed `EdgeCertification` is not in family `Certificate`). Each row red on its mutant.
N3 (claim 2, two wording gaps, hand-built specs only): "decided to run no length" is in code any `IntervalNotForward`, so a NaN-direction line over a 5e-10 m span is told to tighten the tolerance, which only uncovers the defect; and a NaN end on a good line with a forward span ends "kernel defect; report it" at a build door though the point is the caller's. Make each sentence true of the body it names, or say in one line at the site why it cannot be told apart there and put it on a row.
N4: `a_splines_cover_alone_…_is_the_limit_at_every_door` asks three of five doors (`.take(3)`): all five, or the name says three.
N5 (style): `extent.rs`'s `cover_is_exact` is documented as the one home of the exact split and the fix widens it at one of its two readers only (`|| carrier.conic().is_some()`): one statement, read by both. `UncoveredCause::Carrier` shows the first unknown's cause but stands for all of them: say so or carry them.
N6: the PR body interleaves measurements from three heads: one section per head, or re-take on the final one and drop the rest.
Until your base's push arrives, do N2 to N5 on your current head (commit locally or push; the reviewer reads the final head by sha). Final report: the full sha, each N with its commit and a line, the re-pin table, the mutants, what ran with counts. No hosted wait.

(After that message the implementer was told the base's final head, `606af1e32c`, and to put the answer Ev is owed from PR 7140 near the top of its body.)

---

# Section B: the implementer's reports of the pass

## B1. The first report, on `cad9265ca4` (before the base's final head was merged)

N2 (`058953a8`), each red on its mutant:
- (a) `certify::tests::an_edge_past_the_box_is_refused_as_past_it_whatever_its_span`: a line 2 km out over a reversed span, and a point 2 km out chorded with itself, are OutsideSessionBox.
- (b) `session_box::tests::a_nan_end_beside_an_end_5_km_out_is_past_the_box_by_the_far_end`: Point, about 4 km past.
- (c) `topo finish::tests::a_boxed_edge_certification_is_the_boxs_finding`: over the sample list, the 5 boxed EdgeCertifications are SessionBox (not up to isometry); every other one is Certificate.

Mutants, planted on `058953a8`. geom-brep lib is 1,545 tests; topo lib is 2,941: span read first: 2 red; the non-finite filter removed: 2 red; any not-forward span replacing the box: 1 red; the finish.rs arm removed: 1 red; the ellipse coordinate case removed: 1 red.

N3 (`058953a8`):
- (a) Changed the code. Only a span decided reversed, or exactly empty (`is_exact_zero(t1 - t0)`), replaces the Uncovered refusal. An in-band nonzero span keeps the box's refusal. The check-0 doc and comment say so. Row: `a_span_the_band_reads_as_zero_leaves_the_boxs_refusal_standing`. A NaN-direction line over 5e-10 m is SessionBoxUncovered, with no "tighten"; reversed, it is StatedInterval.
- (b) Argued at `own_ending`: a build door cannot tell a caller's point from one an operation constructed, so the ending is the door's defect culprit (D4 ¶1 (iv)). Row: `a_non_finite_end_is_the_doors_defect_wherever_it_came_from`, at Build and AtRest.

N4 (`058953a8`): renamed `a_splines_cover_alone_past_the_box_is_the_limit_at_the_three_minting_doors`. Its doc says why the recertify doors aren't asked there: no held spline edge can be minted on a line description (`Unimplemented`). The five-door row pins check 0 at all five.

N5 (`058953a8`): `cover_is_exact(carrier, CoverRead::{Distance, Coordinates})` is the one home, read by both readers. It keeps two kind arms for the cover-kind-match gate. `UncoveredCause::Carrier`'s doc and fields say it shows the first unknown and stands for all of them.

New in that round: main's topo `mate5_cyl_eps_rung::a_far_peg_levers_its_tilt_from_its_own_origins` (rewritten by another unit) puts a 1 mm peg at 1 km, about 1 mm past the box, and is refused at its first edge. Re-placed at 990 m (`cad9265c`). Measured on and off in one binary at 500, 900, 950, 990, 999 and 1000 m: main's 1 km peg with the ask off stands down at 1e-6 and 1e-9 by its own rule and reads its subject at 1e-12. At 990 m it does exactly that with the ask on.

The growth guard that replaced the plate ledger (main's PR 7501): the ask adds calls (plate assertions 1051 to 1099, door 554 to 578; slab assertions 992 to 1040) and forms; it adds no larger form and no freeze; guards 8/76 unchanged, nothing to re-pin.

## B2. The final report, on `1e656c82f7`

box-certify-ask (PR 7272): N1 and N6 are done on the final base. Head 1e656c82f7824382141a023c71f084d84559e45a, pushed. Tree clean, nothing running.

Merges
- Base 606af1e32c (6639bbbd), then main 99e230c9 (86e2f31c), then main a7b44d8e (95a45dc2).
  - The middle merge was clippy-red: Circle::frame in end_root.rs, a main-vs-main crossing. Main had already fixed it by a7b44d8e, so main was merged again.
- One conflict carried code: the base's last fix to its fold, "a point that is no finite number is no point past the box" (4ed06054).
  - This unit routes the fold through the geom-brep core, so the guard now lives in `region_in_box`: a non-finite inside read leaves the cover's refusal (Unshown or Uncovered).
  - Certification's inside read already filters non-finite points, so no edge moves.
  - New core row: `an_inside_read_past_every_number_leaves_the_covers_refusal`. The base's topo row (the brick with a +inf corner) passes in the fast set.
- The green work row is the base's file whole. The geom-brep roster is the union (main's OVAL_IN_BAND plus the box's two). The lever pin is 507: the base's and main's 501 plus this unit's six (d819f778).

Re-pins, re-measured on 95a45dc2 with the ask off and on in one binary (instrumentation uncommitted, now removed). Every value is the same as on the previous two merges:
- asm2a 988 to 1012
- m10_8 numeric 496 to 520
- m10_bulge session_box_reach [0,1] to [0,25] boss and [0,37] d-tab x2
- sym11 link frozen ceiling 2600 to 2604
- revalue plate_param apart (compared, checks) 3204/7068 to 3282/7146, pinned 3282/7306
- revalue boss_union apart 2384/3826 to 2435/3868, pinned 2423/3898
- m10_6 keys re-blessed
- sweep washer 458 to 492 and 608 to 632
- The growth guard's largest form is plate 28 and slab 4, both on and off (main moved the plate from 38). Nothing re-pins there; the plate ledger is retired.
- m10_9 and sym_9: main's pins stand.

Re-pin set, 89 tests: ask on: 79 pass at each of 1e-6, 1e-9 and 1e-12; the 10 reds are main's filed revalue drift; ask off at 1e-9: 72 pass; red are the 10 inherited plus exactly the 7 re-pinned rows.

What ran on 95a45dc2: fast set at 1e-9, `cargo nextest run --workspace --exclude viewer --exclude pncad-py --profile ci`: 16,900 of 16,900; `cargo clippy --workspace --exclude viewer --exclude pncad-py --all-targets -- -D warnings`: clean; fmt: clean; run-gates.sh: all OK; work.py lint: ok. On 1e656c82: check-slow-set-names over the workspace listing is clean (2,383 of 2,441 entries checked). Not run on this head: the whole slow set, the demos, the cost figures and the whole-suite differential (those are 83aaad76's, labelled so in the body).

Hosted run 45291124288 on 95a45dc2 (lint, demos, mesh meter and viewer were still queued). Its reds:
- test (kernel): check-slow-set-names, the unit's: one entry named main's spelling of the far-placement row this unit renamed, and the strut row was listed twice. Fixed in 1e656c82, which is test-config only. Main's: `a_cylinder_wall_through_a_prism_corner…` at 1e-12 (filed, `work/tesslie/`), red locally both with the ask on and with it off; `square_search`'s seeded fuzzer; topo's recourse_roster sites.
- test (app): the 10 revalue_corpus rows (filed) and pncad-py prose_census (main's, filed by PR 7667).
- No hosted run has read 1e656c82.

N6 and Ev's PR 7140 question: the body now opens with "For Ev (from PR 7140): does any construction certify helper or cutter edges past 1 km on purpose?". The answer is No, followed by what was searched: the instrumented whole-suite refusal log at three ε (228 refusals at 1e-9, every one a test's own far body or a poison fixture, none a cutter, scaffold or helper); the demos; the five doors' callers; the fixture dispositions; and what was not searched. The rest of the body keeps one section per head.
