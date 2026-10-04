You are a VERIFIER for the REACH orchestrator in evgunter/cad. Read CLAUDE.md first. You change no code on the PR branch, merge nothing and post no GitHub comments. Disk is limited, so build only what you need.

PR: evgunter/cad#3985 (branch reach/arc-from-pairing). Head to verify: b171d906a4611648f0aee765062463e24c80fe62.

This is a SHORT re-check. An earlier verifier, on analysis/reach-verify/3985, verified the PR at f69ec37c57. Since then, two things changed:
- **The main merge.** The lane merged main, including JOIN's #4008, which ranks a germ's partners along the conic, and #4025.
- **The removal.** Per the orchestrator's ruling, the lane removed the walk filter: `walk_passes`, `bool_join_walk_site` / `bool_join_walk_order` and their offer and audit rows. JOIN's ranking now picks the same partner, and with the filter disabled no row went red.

Check:
1. **The removal is clean.** `git diff f69ec37c57 b171d906a4611648f0aee765062463e24c80fe62 -- crates/`, minus what came from main, removes the filter and its rows and changes nothing else in behaviour, apart from the merge resolution in `boolean/join.rs`.
   - The lane says join.rs is now main's except for the datum. Confirm that.
   - The lane also deleted `wall_region` and `ChordJoiner::fragments` as dead after the merge. Confirm nothing calls them on main.
2. **The chord differential.** Compare origin/main against this head over topo, sweep, mesh, editor-core, step-import (--all-features) and the demo tour, using an env-gated or local-only dump of every chord spec. Report:
   - the number of chords main mints that the head does not (the lane says 0);
   - the tests that differ only by added chords;
   - the renamed tests.
3. **Mutants**, over the touched rows plus JOIN's pocket rows:
   - datum negated;
   - always-ccw;
   - germ dir flipped;
   - partner dir.

   Each must turn rows red. The four-crossing row should be red under germ dir, partner dir, datum negated and always-ccw.
4. **Battery.** Run the touched crates at CAD_TOLERANCE_EPS 1e-9, 1e-6 and 1e-12. Check that every red also fails on origin/main. The lane reports four torn-body rows under per-op-postcondition, JOIN's steep_ellipse_poses at 1e-6, and arc_loft at 1e-12.

Push `verify.md` to branch analysis/reach-verify2/3985. Commits end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. The file gives a verdict of VERIFIED or NOT VERIFIED and lists the blocking points. End your turn with the same.
