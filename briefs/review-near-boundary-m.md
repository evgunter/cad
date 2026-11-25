# Review: PR 7757 (`heldrecords/near-boundary-m`), frozen head `89673eecb4`, base main — single FULL review

Probes go only to `heldrecords/review-near-boundary-m-<6 hex>`.

Read first:
- `CLAUDE.md`
- `docs/prompts/reviewer-style-lane.md`, IN FULL
- the row `work/heldrecords/the-junction-side-reading-assumes-no-other-target-boundary-within-its-reach.md` on the head
- `crates/topo/README.md` tier 3 *The junction's side*
- D10

Review `git diff origin/main...89673eecb4`.

**What it builds:** `reduce::junction_window` asks `junction_end_boundary_clear` of every window, the order's own pivot m included. Before this, PR 7731 asked it only past pivot m. Where the guard fails, the end refuses `JunctionEndUnread { NearBoundary }`. The lane's measurement found 1226 pivot-m reads across topo, sweep and editor-core at ε 1e-9, 1e-6 and 1e-12. Every one was decided clear, so no verdict moved.

**Claims to falsify:**
1. **No valid body moves.** Re-run the head against main at three ε: topo, sweep, editor-core `ci`, plus the slow elbow and junction rows. Diff the verdicts.
2. **The guard is sound at pivot m.** If another boundary of the target lies within the spread's reach, the side reading's verdict cannot be trusted, so the guard must not pass it. Attack the boundary-clear test itself:
   - a far rim just outside its stated reach;
   - a boundary vertex, not an edge;
   - a cone apex;
   - a seam of the target face.
3. **The guard is not over-strict in a way that hides a regression.** The swept 3σ and quarter-turn poses build, at the closed form, and are sound at t3 and t3′.
4. **The pin catches the mutant.** Run the "past pivot m only" mutant. The lane reports that its poses refuse later in band rather than building wrong. Check whether any pose builds a wrong body under the mutant, which would make this a soundness fix and not a tidy-up.
5. **The docs say what the code does.** The README sentence was the belt lane's own, not Ev's; check with `git log -S`. Also check the NearBoundary doc and the refusal message.
6. **The swept 1σ–1.5σ `VertexOnCurvedFace` refusals.** Look at them briefly and say whether they are in band, a separate defect, or fine.

Running it:
- Own `CARGO_TARGET_DIR` outside the checkout; per crate.
- Foreground rule: never end a turn with background work.

Report, ≤150 lines:
- verdict;
- claims held or broken;
- MAJOR/MINOR/NOTE;
- `## Style`, as sure/likely/unsure;
- probe rows.

Send it by `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`, then stop. No email or personal identifier anywhere; no model names in commits.
