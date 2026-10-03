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

PR: evgunter/cad#3980
Branch: reach/rest-mate-intersect-diff
Frozen head to verify: bef41106a99ce3618280108558ebf55591dd3b41
Review to read for the findings: the dual review on analysis/reach-dual/3980-r1 and -r2 (frozen 5a5c62365b)

Lane claims (last fix pass). This PR exempts declared Rest pairs on the no-crossings path, so check most carefully that the exemption cannot admit an UNDECLARED or unverified pair.
- The exemption now reads only Rest declarations that the declaration check verified (rest_contacts). It no longer reads pairs that merely share a recipe source. The lane says those cases refuse earlier with CurvedPierceUnsupported. Confirm this, and try to build one that reaches the exemption.
- One Exempt type holds both rules: "declared pair" and "verified Rest".
- The sphere rule skips only when EVERY face on that sphere is a verified Rest against the partner face. Two guards were removed: "at least one face reaches", and a box filter. The lane says no case reaches them. Try one.
- Mutants it says are killed: (1) the exemption answers nothing; (2) the key reads the A face alone; (3) all -> any; (4) operand order swapped; (5) continuations let into the Rest list. Run all five.
- New rows in rest_mate_every_op.rs:
  - one undeclared sphere pair in the ball-in-cavity, where every op refuses SpheresMeet;
  - a pebble buried in the collar wall, where intersect, both differences and union in both orders are checked against closed form (main refused this).
- Filed items: an undersized shaft (union refuses, intersect and difference build), and an equator inside a hole (refuses FallbackExtentUnsupported).
- The .config/nextest.toml slow-set comment lost its hand-kept count. Check that the slow set is still well formed.
- topo, sweep and editor-core pass 6912/6912 at 1e-9, 1e-6 and 1e-12.
