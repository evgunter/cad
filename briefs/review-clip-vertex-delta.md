**Delta re-read, PR 7464 `embed/clip-vertex`.** You reviewed this unit's frozen head and reported APPROVE-WITH-FIXES with findings M1, M2, m1 to m5, n1, S1 to S15 and your claim answers; the fix pass ran off your findings as the orchestrator adjudicated them. Its head is **`388ee3ef82`** (frozen; `git fetch origin embed/clip-vertex && git checkout --detach 388ee3ef82`). It holds a merge of the landing branch `origin/embed/wave3-land` at `8d34076a9e` (commit `661a0abfe8`: approved units waiting to land together, among them PR 7322, collinear edges joined at a vertex by key, and PR 7320); main's later changes are NOT in it. Read the lane's own commits since the head you reviewed, not the merge's. Hosted CI is not waited on in this program (the owner's ruling: a unit lands on local runs listed in its PR body): say what you ran yourself. Rules as before: `docs/prompts/reviewer-style-lane.md` for anything new, every finding with `sure` / `likely` / `unsure`, probes on your own review branch, no model named in the report or a commit, no `Co-Authored-By` trailer, builds in the foreground (anything over 600 s `setsid`-detached and polled; never end a turn on a detached job).

**Rulings made since you read it (the orchestrator's; hold the PR to them, and say if one is wrong):**
- The circle plane (your m2) is removed, not pinned: a reading no body reaches is deleted.
- The off-grid wall (your M2) is to be pinned HOLDING, not refusing; the lane says the earlier f64 refusal was the cover's one-ulp joint, closed by PR 7322 in the landing branch, and not this unit's.
- A row may be pinned refusing at a named exit where the cause is another program's and is filed there; it must go red when the cause is fixed.
- `embed/corner-pieces` (PR 7627, not on this base) deletes the straight run; this PR owns `Zero::Through` and `through_read`, and whichever lands second merges onto the other's spelling. Say if what this PR keeps of them could not survive that.

**The lane's report, verbatim (its claims; a correction to one is a finding):**

(clip-vertex) FIX PASS DONE. PR 7464, head `388ee3ef82`, FROZEN for the delta re-read. Merged with `origin/embed/wave3-land` @ `8d34076a9e`; main not merged by me. PR body rewritten (the full account; this is the short one). Tree clean, pushed.

HEADS
- `661a0abfe8` the landing merge; `5b5ab29652` census blessing; `388ee3ef82` moves the mirror sighting to PCERT's row (row text + one doc comment in the suite file; no code). Every run below is on `5b5ab29652`'s code; the suite file was rebuilt and green (92/92) after `388ee3ef82` inside the mutant run.

RUNS ON THE MERGED TREE
- fmt ok; `cargo clippy --workspace --exclude viewer --all-targets --all-features -- -D warnings` clean.
- `injective::` 203/203 and the suite file 92/92 at default ε, 1e-6, 1e-12.
- topo roster 11/11; `scripts/run-gates.sh` clean; `work.py lint` ok.
- Endpoint census 5/5 after blessing `injective.rs` 208 → 209: one read in `zero_of` (a merged clip line's lower end, to ask which coordinate line the side lies on), dispositioned in the census entry.
- topo + sweep slow sets: 5 red, each matching a main-red row on GREEN's slate, none touching this change: `bisector_nodes::the_near_node_poses_at_the_witness_band`, `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`, `section_end_poses::a_section_end_at_the_band_edge_…`, `section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`, `face_normal::tests::no_source_file_folds_the_bit_by_hand` (names `step-import/src/one_record.rs`). NOT re-run on the landing branch alone, so "same as the landing tip" is by name, not by a control run.

OFF-GRID PIN (M2): HOLDING. `a_wall_holds_against_a_face_at_a_vertex_mid_ruling` builds on the grid and off it (r 0.7, block [0.13,0.7]×[-0.61,0]×[0.3,1.7]); in three poses the wall pairs and the whole body hold at f64 and Interval, `Embedded` at the verdict door. The earlier f64 refusal was the cover's one-ulp joint (0.3 vs 0.30000000000000004), closed by PR 7322 in the landing branch.

MUTANTS, once on the merged tree: 20; 19 caught, 1 survivor argued at its site.
- Both sets (unit 203 + suite 92): A (3 unit, 4 suite), B (4 unit, 2 suite), F, G, N, R06, R07, R09, R11, R13, R15 (unit rows only; the suite stays green under those nine).
- Unit set only: C, D, H, I, J, K, L, M caught; **E survives** (the `Along::whole` filter on `Through`): no fixture has a line edge off its side's own line; what it leaves out is a spline chart, said at the site, no row builds one.
- R13 (anchor 1e-3 off the line) is caught by value in two rows; the reviewer's "None partner" negative is still NOT built.

MIRROR → PCERT (your question): IT IS THAT CAUSE at f64. Probe of `Body::pcurves` on this head: the generic mirror's re-mint stores the ring's four rim rows with `b: 1.39e-17 / 2.78e-17` in the axial channel (exact 0 as built), and the block's two rim half-circles with `a: 2.78e-17`. Appended to `work/pcert/a-cylinder-rim-on-a-turned-frame-stores-a-rounding-in-its-axial-channel-at-f64.md` as a third sighting, TRACED, with the table, cost, controls. My EMBED mirror row is WITHDRAWN (git rm); its two references repointed. So this PR touches one PCERT row file: a seam for you.
- NOT covered: the ring also refuses at Interval and the block refuses at Interval (Budget) while holding at f64. I did not print the Interval rows; the PCERT row's cause is an f64 point off zero, so those may be another cause. Said on the row.
- Pinned as it reads: `the_ring_and_the_interval_block_refuse_mirrored_across_a_generic_plane` (goes red when fixed).

MEASUREMENTS (unchanged from my last message; taken on the fix-pass code BEFORE the landing merge, same base main `a85cfefaa` + harness `ebb2e6f8e`): 55 tests / 5,006 asks: Embedded 4,101 → 4,165, Departure 745 → 681, Crosses/budget/second-order 64/95/1 unchanged; 5 tests fully embedded, 4 with fewer refusals, 46 unchanged, 0 worse. Tour 85 stops: verdicts unchanged (78/4/3), splits 86,569 → 86,159. NOT re-taken on the merged tree (the harness does not carry the landing branch). The bushing is closed by the cover's joint alone, with or without this PR.

REVIEW FINDINGS
- Done: M1 (drum row restored; swept by fn name and removed lines), M2, m1 (plane by value), m2 (circle plane removed), m3, m5, n1, claim 1, claim 9's circle-vs-corner split pin, S1–S4, S11, S12.
- NOT done, said in the PR body: m4 (the one corner among several loose vertices is signed only where it is the first shared vertex; on the row's "what is left"); the `Zero::None`-partner negative; a row for two circles through one vertex; S8, S9 (reasons in the body).

SEAMS
- `embed/corner-pieces`: same parent; this PR owns `Zero::Through` and `through_read`; second to land merges onto the other's spelling.
- Landing branch's one-shared-vertex arm can now reach `Through`; its call is `planes_through(x, y, &alone, rx, ry)` and its note says so.
- Suite file (ciw, green own it): rows added, none of main's edited. Census file: one entry.

UNSURE / NOT CHECKED
- Hosted CI: red on the pre-merge head `ed6af1ddc4` (`test (app)`, `test (kernel)`, `gate ok`), not investigated per your word; GitHub reports a conflict with main. Not checked on `388ee3ef82`.
- The sweep for sibling refusals was done on main `a3d5ce89c`, not redone on the merged tree.
- The mirror `curved_walls` exit trace is from before the merge; on the merged tree I printed the stored rows only.

Not merged, no reviewers requested. Waiting for the delta re-read.

**Answer:**
1. Per finding of YOUR report: fixed, answered (and whether the answer holds), or still open, with the row or mutant that shows it.
2. **M1.** Is the drum row restored as it was, and does the lane's sweep (by fn name and removed lines) cover what its pattern could miss: a row whose assertions were weakened in place rather than removed?
3. **M2.** Re-run the off-grid wall at both scalars in its three poses. Is "closed by PR 7322's joint alone" right: does the row hold on the landing branch WITHOUT this unit? If it does, the row pins the landing branch and not this unit; say what in this unit it can still turn red.
4. **The survivor.** Mutant E (the `Along::whole` filter on `Through`) survives every row: "no fixture has a line edge off its side's own line; what it leaves out is a spline chart, no row builds one". Is the filter a guard a built body can reach (then build the body), or one no body reaches (then it is deleted, by the same rule as the circle plane)?
5. **Not done.** m4 (the one corner among several loose vertices is signed only where it is the first shared vertex); the `Zero::None`-partner negative; a row for two circles through one vertex; S8, S9. Does each stated reason hold, and is each on a row rather than only in the PR body? Is the missing `None`-partner negative a soundness gap or a completeness one: build it if it is an hour's work.
6. **The mirror.** The lane traced its mirrored ring and block to PCERT's stored axial rounding at f64, withdrew its own EMBED row, and pinned `the_ring_and_the_interval_block_refuse_mirrored_across_a_generic_plane` as it reads. Is the trace right for the f64 refusals? The `Interval` refusals (the ring, and the block at `Budget` while it holds at f64) are untraced and may be another cause: is withdrawing the EMBED row then right, or does the `Interval` half still need a row of ours? A body that holds at f64 and refuses at `Interval` is a scalar split that this suite's own rule calls a finding.
7. **The census entry.** One new endpoint read in `zero_of` (a merged clip line's lower end, to ask which coordinate line the side lies on): is its disposition true, and is a lower END the right thing to read there, or a comparison in disguise that a rounding flips?
8. **Mutants.** 20, 19 caught, on the merged tree; nine of them caught by unit rows only, the suite green under them. Re-run a sample by their definitions and your own from the first pass. Which of the unit's readings does a BODY row hold?
9. **Measurements.** 55 tests / 5,006 asks: `Embedded` 4,101 → 4,165, `Departure` 745 → 681, 0 worse; tour verdicts unchanged. Taken on the fix-pass code BEFORE the landing merge and not re-taken. Is anything in the merge able to move them (PR 7322's cover; the tangent-corner run)? Spot-check three of the five tests that became fully embedded on the frozen head.
10. Anything new the fix pass introduced, with a short `## Style` section.

**Verdict** on the ladder APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT, findings MAJOR / MINOR / NOTE with file and line, each MAJOR marked DEMONSTRATED or BY-INSPECTION with its defect class, and what you executed as against read. At most 130 lines, to the orchestrator by `send_message` (`session_018u3gVFQSECnnYvnvggrF2a`).
