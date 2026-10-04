You are a VERIFIER for the REACH orchestrator in evgunter/cad, working on the PR named below. Read CLAUDE.md and docs/prompts/implementer-discipline.md first.

The implementation lane has finished its LAST fix pass after review. Your job is to check, independently, that what the lane claims is true. You change no code on the PR branch, you merge nothing and you post no GitHub comments.

Steps:
1. `git fetch origin <branch>`, then check out the frozen head given below. If the branch head has moved past it, say so and verify the new head as well.
2. For each mutant listed, apply it by hand, run the named rows, and record red or green. Then revert it.
   - A mutant the lane says is killed but which stays green is a FINDING.
   - Write each mutant as the smallest source edit that matches the description.
3. Run the PR's new or changed test rows at CAD_EPS (or the repo's ε switch, see `scripts/` and the nightly workflow) 1e-9, 1e-6 and 1e-12.
   - Run the changed crates' nextest suites at 1e-9.
   - For each red, check whether it is also red on `origin/main`. If it is, it is not this PR's.
4. Read the diff of the last fix pass (the commits after the review's frozen head) adversarially against the review findings listed. Check each claim below. Report any claim that is false, overstated or unsupported.
5. Push a short evidence file, `verify.md`, to branch `analysis/reach-verify/<PR>`. Make it an orphan or off-main branch, with only that file.
   - Contents: a table of mutant → row → red/green; the ε results; the claim checks; a verdict of VERIFIED, or NOT VERIFIED with the blocking points.
   - Commits end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
6. End your turn with the same verdict and table as plain text.

Disk is limited. Build only the crates you need, and run `cargo clean -p` between ε runs only if space runs short.

PR: evgunter/cad#3985
Branch: reach/arc-from-pairing
Frozen head to verify: f69ec37c57a1ea13796038e65182c28aa653f338
Review to read for the findings: the dual review on analysis/reach-dual/3985-r1 and -r2 (frozen 7e33abf087). Read the fix brief with `git show FETCH_HEAD:briefs/fix-3985-last.md` on analysis/reach-briefs/2026-10-03. Its F-numbers are the findings.

Lane claims (last fix pass). The PR derives the chord's arc from the section's pairing data and retires the two arc-side selectors. Check hardest that no chord main builds is lost or moved.

- **The differential (most important).** Re-run it independently. Use an env-gated or local-only dump of every chord spec (carrier, params, endpoints, bits) on origin/main and on this head, across topo, sweep, mesh, editor-core, step-import and the demo tour. The lane's claims:
  - no chord main mints is missing on the head;
  - every other difference only adds a chord where main refused;
  - the one exception is the round-boss row in reach_slab_cut_sector_side, where 16 chords move in operand orders that stop at the ringed-wall door on both trees, so no built body changes;
  - in the tour, only the renamed snowman row differs, by 24 added chords.
- **Mutants against the 271 touched rows.** The lane reports red counts in brackets; run all of them and report red or green for each:
  - datum negated (92)
  - partner dir (24)
  - germ dir flipped (50)
  - split_leave flipped (40)
  - split chord datum only (41)
  - walk_passes off (3)
  - site rung (69)
  - order rung (13)
  - always ccw (91)
- **F4.** New row four_crossings_on_one_section_circle.rs. It turns red with walk_passes disabled, with the site rung alone, with the order rung alone, and on main.
- **F1.** The Undecided endpoint placement is never reached outside the truth-table rows. Check this with a counter.
- **ε reds.** The only reds at the three ε are main's: the euler paired-tear row under per-op-postcondition, and arc_loft at 1e-12. Confirm both are red on origin/main.
- **New bodies are correct.** Check them against closed-form oracles: tilted sphere pairs, plane×sphere, star plates and four crossings on a circle, each with every op in both orders.
