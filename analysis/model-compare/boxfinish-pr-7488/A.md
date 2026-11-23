REVIEW — PR 7488 at frozen head 4c17b5428 (merge base 6c5ea3ec3baa). Probes: branch boxfinish/review-mint-X, commits 19b4028d2 (P1–P4), then P5 on top.

**Verdict: APPROVE-WITH-FIXES.** The mechanism is correct and does what E12 ratified. All fixes are to text and to one test, none to the mint. Hosted CI on the head: "change filter" queued, "prime the build cache" skipped. Nothing ran, so the gate claim rests only on my local runs below.

## MAJOR
None.

## MINOR
1. **Claim 1 is overstated in the title, the body and audit row F43** ("one document takes one branch at every valuation", "the same at every valuation and in an eager and a deferring session"). DEMONSTRATED, sure. Probe P1 (`review_mint_probes::p1_nodes_minted_before_the_session_take_the_other_branch`): the same document `(x,0)→((x+w)−w,1)` stores the axis when built inside the session. If `(x+w)−w` is computed before the session is installed and the hop inside it, it stores the quotient (2.78e-17 at (0.1, 0.3)), eager and deferring. So the branch depends on the evaluation history, not only on the document. `sym/question.rs`'s header admits this ("a node the session did not record is an unknown"), and E12 itself words the guarantee conditionally (the zero "holds at every parameter value at which the build takes the decisions this replay took"). The unconditional sentences in the title, the body and F43 should carry that qualifier. The unit's own Ev-revision section already withdrew "the same at every valuation" as an overstatement. I found no production path that mints profile nodes outside the leaf's session; this is about honesty of the claim, not a live bug.
2. **`docs/predicate-dimension-audit.md` F43 names the wrong door.** It says the reading "runs under `k_stats::check_unlogged(.., "F43")`". The code (`seg.rs:802`) runs `k_stats::unlogged("F43", ..)` around `decide(..)`, and `check_unlogged` is a different door (k_stats.rs:776). BY-INSPECTION, sure.
3. **`question::tests::nothing_that_reads_a_value_is_proved` cannot see T1, and the body's reason for that is wrong.** DEMONSTRATED, sure. The body says T1 passes it because "the tier does not prove a `max` hop on any ladder". In fact the row asks before any decision has set the session's witness kind, and the decision read is gated on `sess.witness == Some(Exact)` (sym.rs:3523). Probe P5 (`p5_max_after_a_decision`) takes one decision first and then asks `max(a,b)−b`: on the head it is not proved (green); under T1 it is proved (red). The `max`/`min` half of the row pins only T3 as written.
4. **The two rule switches in `question::RULES` are pinned by no row.** DEMONSTRATED, sure. Mutant R1 turns `decision_read` and `registered` back on, and every geom-core question row, every `recorded_directions` row and my probes stay green. The question's state holds no registry, no params map and no witness, so those rules cannot fire there. The real protection is the empty state, not the switches. Sound today, but the doc ("without the two rules that read a value") credits the switches, and a later change that lends the registry or the brackets would go unseen.
5. **The body's differential is stale and incomplete.** It names merge base `e1830a1c` and says the base `sweep`, `topo` and ε rows are "still running … completed when they land". The frozen head's merge base is `6c5ea3ec`. Between the two, only `crates/viewer`, two `editor-core` docs, `topo/README` and `work/` changed, so code-wise they are equivalent for the five crates. I ran the missing parts (below). The inherited reds for `sweep` and `topo` are still unlisted in the body.

## NOTE
- **Claim 7.** A hop proved zero in both components (P3: `(W,0)→((W+1)−1,0)`) falls to the quotient `0/0`. At a box, that NaN reaches `path_junction_turn` as `MarginDiag(Invalid)` and is refused as an Escalation, not by `vertex_separation`. At a point it is `JunctionTurnWithinTolerance`. It is refused either way. By inspection, base does the same (both read as proven Zero there, so also the quotient). The sentence "for the gates that refuse a degenerate chord by name" holds only loosely. Mutant R3 (an axis in the both-zero arm) is seen by no row. likely.
- **Claim 6.** `ProvedZero` is `Copy` and names no expression, so it is a bool with a private constructor and can be carried anywhere. Harmless today: nothing takes it as input. sure.
- **Claim 13.** Mutant R2 (budget 3) is red only on `the_answer_does_not_read_the_sessions_dials`. No corpus guard pins 64; the doc comment gives the measured reason. The form's size reads no value, so the budget costs no second branch. sure.
- **Claim 3.** The question's state persists per installed session, and P2 (an identity asked cold vs after large non-identities that share its subgraph) gave the same answers. The only cross-ask coupling I found is the retry growth guard (`RETRY_FORMS` = 200,000 forms, across every ask of a session), which can in principle make a late ask skip a retry. Not reachable on anything measured. likely.
- **Brief correction:** the gate script is `scripts/run-gates.sh`; `scripts/gates/run-gates.sh` does not exist.
- **To file (not this PR's ground):** these slow-set rows are red on base and head with the same payload, but I found no open row naming them:
  - topo `face_normal::tests::no_source_file_folds_the_bit_by_hand` names `step-import/src/one_record.rs` `from_n`; only closed rows hold it.
  - sweep at 1e-12: `a_ring_on_a_cone_face::the_rings_volume_brackets_its_closed_form_at_the_certified_scalar`, `band_closed_mixed_chain_shapes::{a_four_arc_barrel_fillets_whole, rounded_polygons_fillet_whole, turned_rounded_outlines_fillet_whole}`, `band_open_mixed_chain::an_l_plates_rounded_corner_run_fillets_whole_at_interval`, `band_open_mixed_chain_shapes::an_s_run_of_a_convex_and_a_reflex_arc_builds`, `section_ellipse_seam_poses::{a_slab_face_nearly_along_a_ruling_builds, planes_through_the_axis_nearly_along_it_split}`. The band ones may sit under `work/band/every-mixed-rims-section-escalates-its-touch-reading-at-interval-at-eps-1e-12.md`; triage needed.

## Claims
1. **Broken as worded** (MINOR 1). Held for every document built inside its session.
2. **Held**, BY-INSPECTION: brackets are read only by `signed` (decision read); constants fold as exact rationals; `Opaque` is keyed by sequence number, not value; the state has no params, registry or witness (R1).
3. **Held**: P2; the footprint row; nodes are not minted (the row pins `nodes.len()`).
4. **Held** BY-INSPECTION: drop guard ordering, `aside` is a marker only, no session installed gives None. Pre-session nodes: P1.
5. **Held** BY-INSPECTION; not run across threads.
6. **Held.** Doctests run: 3 compile_fail E0599 plus 1 that compiles. The seal is a pub trait in a `pub(crate)` module.
7. **Held, loosely** (NOTE).
8. **Held**; M3 is red only on the sign row. The driver bisect row is the author's; I did not run it.
9. **Held** BY-INSPECTION: the only outside reader of recorded directions, `editor-core/src/mate/reach.rs`, reads arc radials only. I did not attack the fillet-trim bits.
10. **Held, DEMONSTRATED.** Head: `m10_9_what_the_registry_buys` green. Tip handed `chord_director(d)` again: the link door-shut is (695, 843), refusing at the extrude (Euler) op, with frozen 2600, exactly the body's experiment. I did not check that each new certification is true.
11. **Held** by arithmetic on three pins: m10_8 slab −4 symbolic (4 sides × 1 proved-Zero), m10_bulge [3,3]→[0,3] and [1,1]→[0,1], sym_9 pad −12/−7 (11+1, 6+1). Not re-derived by running.
12. **Held** BY-INSPECTION: E12 explicitly shuts the registered door; I did not measure the class.
13. **Held** (NOTE).
14. **Partly.** T1 and T2 match the body's red sets exactly; M3 is red on the sign row only. Mutants no row sees: R1, R3, R4 (fresh state per ask, a cost-only change). The `max` row is MINOR 3. Base-red of the new rows not checked.
15. **Held.** Each pair below has the same red set with byte-identical panic text, thread ids stripped:
   - sweep default: 4 reds;
   - topo default: 6 reds (`square_search` is a seeded fuzzer, the same kind of failure under different seeds);
   - sweep 1e-6: 8 reds;
   - sweep 1e-12: 19 reds.
   profile at 1e-6 and 1e-12 is green on base and head. Rows held: see the to-file NOTE.
16. **Partly.** The dispositions I read are right. I did not run my own pattern sweep; my sweep-shaped probe is P1/P5.
17. **Held.** EX1's "not yet of that shape" matches the unit ("moved later: not this change"), which Ev accepted on 6993. "Not yet" is status wording in a present-tense design page (style). F43 has the wrong door (MINOR 2).

## End-to-end
It solves the right problem: the axis spelling no longer depends on the rounding of the value channel or on the session kind, and the link certifying door-shut is a real gain. Missing in practice: a guard that the question's state stays empty of value channels (the switches alone are untested), and the qualifier on the guarantee.

## Style
Exercised: altitude/one-home (BUDGET, RULES, RETRY have one home; sure); honesty of docs (MINOR 1, 2, 4; sure); dead weight. Findings:
- `AxisUnsigned.zero` exists only to make the error uninhabited at numeric scalars, kept with an `#[expect(dead_code)]` (a type trick, sure).
- `Ask::drop` allocates a fresh empty `Session` on every ask just to `mem::replace`; `Option<Box<Session>>` would avoid it (likely).
- "not yet" in EX1 (sure).

## Rubric
- Idiom and structure: **4**. The sealed-trait plus uninhabited-type surface is clean, as is the irrefutable `let Ok` at `f64`.
- Test quality: **3**. T1/T2/M3 rows bite; the `max` half is vacuous against T1, and the switches and the both-zero arm are unpinned.
- Doc honesty: **3**. The overstated guarantee, the wrong F43 door and the stale differential.

## Spec deviations
- Reported by the author: the budget at 64 not 4096; escalation refuses instead of falling back; the door-only radial lost.
- SILENT: evaluation-history dependence for nodes minted before the session (in code docs only, contradicted by the title and F43).

## Executed
- `cargo fmt --check`: OK. `work.py lint`: OK. clippy `--workspace --exclude viewer --all-targets --all-features -D warnings`: OK. `scripts/run-gates.sh`: EXIT 0. `doc-gate --workspace-pass --skip-viewer-toolkit`: OK.
- Mutants T1, T2, M3, R1–R4, TIP.
- Probes P1–P5.
- Differential: sweep default, 1e-6 and 1e-12, base and head; topo default, base and head; profile 1e-6 and 1e-12, base and head.
- geom-core doctests for `proves_zero`.
- Read only: the receipts' derivations, the driver row, the sweep dispositions.
