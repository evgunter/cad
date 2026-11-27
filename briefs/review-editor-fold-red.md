# Review: PR 7825 (`heldrecords/editor-fold-red`), frozen head `c088350de7`, base main — single FULL review, P0

**Probes:** only to `heldrecords/review-editor-fold-red-<6 hex>`.

**Read first:**
- `CLAUDE.md`;
- `docs/prompts/reviewer-style-lane.md`, in full;
- the row `work/heldrecords/main-red-an-editor-core-fold-refuses-its-plates-face-touch-unread-since-7745.md` on the head;
- `crates/topo/README.md`, the tier 3′ carry paragraph.

**Review** `git diff origin/main...c088350de7`.

**Cause, per the lane.** Since PR 7745, the plate face's division names its section edges. The output stage's coplanar merge killed two of them, and `Descendants::live` followed their substitution rows to faces. So the division's minted cells included result faces. The record landed on a kept piece, but a (rod wall, merged face) pair outside the census lanes refused `Unread` first.

**Fix.** `ops.rs` `fate_of` drops a minted cell whose live image is a face.

**Claims to falsify:**
1. **No silent drop.** Dropping a face-imaged minted cell must never remove the only landing of a record whose touch lies on that absorbed section.
   - The lane says such a touch refuses `Stale`, which is loud. Build or find that pose.
   - Also check that a merged face which holds the cell's interior is still read as a piece, through the pieces path.
2. **No valid body moves.**
   - Run a full default profile, slow set included, per test, for topo, sweep and editor-core: head vs origin/main.
   - Also run `ci` at 1e-12.
   - Built→refused must be empty. The lane reports only the target row and a seeded fuzzer differ.
3. **Re-run the carry mutants** from PR 7799's probes (`heldrecords/review-removed-members-red-798492-r2`) and PR 7808's (`heldrecords/review-minted-door-7c2e5a`), plus R1's PROBE_GAP rows, and compare the counts.
4. **No fast pin exists.** Check whether a topo or sweep row can reach the merge's face substitution, so the fix gets a `ci` pin and not only an editor-core slow row.

**How to run.** Use your own `CARGO_TARGET_DIR` outside the checkout, per crate. Foreground rule: never end a turn with background work. Main's sweep slow rows are 10–20× slower since INSIDE's PR 7801. If a full run is infeasible, say exactly what you ran.

**Report.** At most 150 lines:
- verdict;
- claims held or broken;
- MAJOR / MINOR / NOTE;
- `## Style`, each sure / likely / unsure;
- probe rows.

Send it by `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`, then stop. No email or personal identifier anywhere; no model names in commits.
