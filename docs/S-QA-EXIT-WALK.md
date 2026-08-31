# S-QA exit walk — criteria vs evidence

**STATUS: DRAFT — awaiting Evan's ratification. Exit walks are
design conversations (CLAUDE.md's own exception to agent
self-merge); the program stays open until sign-off, and on
ratification this document becomes S-QA's done-state of record.**
S-QA = the gates-that-lie program (`docs/S-QA-PLAN.md` /
`docs/S-QA-LOG.md`; graduated from `docs/WORK-STREAMS-2026-08.md`
2026-08-29; A/B band 800–899). Criteria are quoted **verbatim**
from S-QA-PLAN's exit-shape paragraph, one clause per row,
dispositions per the M5–M8/ASM convention: MET /
MET-WITH-RECORDED-HONESTY / CARRIED (named owner).

## The walk

| # | Criterion (verbatim from S-QA-PLAN) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "the allowlist gates fail loud when their matcher fails" | MET | **QA-1 (#1237)**: five census matcher sites + the roster sites converted onto `gate_grep` (drops verified byte-identical on the real tree); `gate-roster.sh` ends at `gate_ok` with matcher-death arms — the dual's unilateral MAJOR was exactly this gate printing its own death diagnosis and exiting 0, fixed in the fix pass; the malformed-pattern selftest arm reproduces the silent green at base and reds it at head (hosted, planted run). |
| 2 | "the census selftest cannot red over green content" | MET | **QA-1 (#1237)**: the broken-pipe class closed structurally (no `head -1` mid-pipe under pipefail); the deterministic regression arm builds the race in 400 files / 3.0 s and runs 3/3 red at base, 3/3 green at head — the "too expensive to regression-test" costing was measured ~10x high by both reviewer arms independently. |
| 3 | "a red run reports its whole failure surface and says which matrix point and mode it ran" | MET | **QA-2 (#1232)**: `--no-fail-fast` on both sharded nextest rows (the 0.9.140 fail-fast default MEASURED, not assumed), the mode echoed where a reader of a red run lands, `CONFIG_SOURCE` naming every dimension's provenance; seven ci-local.sh rows took the flag in the fix pass so the mirror cannot re-open the drift. |
| 4 | "a pinned lane says so and adds coverage rather than substituting it" | MET | **QA-2 (#1232)**, Evan's Q2 ruling executed: `CONFIG_SOURCE` says `lane:pinned` (it said `sampled` before — the unit's own end-to-end finding); the basename substring pin is GONE, replaced by the request convention in `docs/prompts/implementer-discipline.md` §2 (the ruling's load-bearing half, per Evan: the doc implementers read matters more than the advisory) plus the `CI-Config:` trailer door verified end-to-end (#1051); the `interval-transcendentals/` pin and fail-closed arm stay. |
| 5 | "the k-lint rows cannot be dodged by the changes they compile" | MET-WITH-RECORDED-HONESTY | **QA-3 (#1297)**, Evan's Q1 ruling executed: `_forces_klint` pins the row that runs a changed `tools/` crate's own suite ahead of the draw, announced via `klint:pinned` + the notices relay; the pin caught D183's failure shape on its own PR three times before merge. **The honesty**: `demos/` is excluded by the ruling (~29% firing rate vs ~7% at tools scope — the recorded cost), and the compiles-vs-runs distinction the dual forced into the record is guarded by `_selftest_klint_workflow` parsing ci.yml (mapping, roster, fallback — three mutations red). |
| 6 | "tess-lint refuses corpus growth it is not comparing" | MET | **QA-5 (#1310)**, Evan's Q3 ruling (issue 1038 option 2): an uncovered fresh-sweep scene is a FAILING finding with its own Kind, harness voice, and literal recourse; the baseline records its cut commit derived by the sweep's own tooling and REFUSES a re-stamp of unmoved data (mutation-selftested), so "never seen" and "outgrown" are distinguishable and the record cannot age silently; red-first shown both directions on real data by both reviewer arms. |
| 7 | "the `test-utils` harness cannot flake by construction" | MET-WITH-RECORDED-HONESTY | **QA-4 close-out (#1300)**: the panic-hook race (issues 882+1134) verified structurally unrepresentable, 0-in-200 empirically plus a load-shaped leg, against ~15-in-200 pre-fix. **The honesty**: three test files still do per-call hook take/set swaps (each single-binary-safe today, all issue-882's shape) and `caught`'s `unwrap_or_default()` is the named reintroduction path — recorded in the log for a future sweep, not silently absorbed. |
| 8 | "the ε-battery's generators satisfy their own premises" | MET | **QA-4 close-out (#1300)**: issue 774's fix verified complete (generator docstring + pinned counterexample); the close-out also landed the ratified proptest-regressions convention as nine doc-comment lines on the pinned fixture (a written-out fixture over a `cc` seed, because the seed re-derives its input only through the strategy the rewrite replaced). |
| 9 | "the J-fence measured claims are guarded, scheduled, or excused at the claim site" | MET-WITH-RECORDED-HONESTY | **QA-6 (#1311 + #1331)**: every measured claim on the swept surfaces bucketed AT the site, the PR 2 tally regenerated with per-bucket columns and a no-bucket column reading 0 on every leg; the instrument committed with variants and blind spots in its own header. **The honesty, two carries by written disposition**: `crates/*/tests` is CARRIED to Track W (its fenced ground; instrument proven ready; class home issue 651, which stays open by design), and `docs/` prose stays deferred on the plan's recorded reason. |
| 10 | "CI prints what each PR costs the suite" | MET | **QA-7 (#1312)**, issue 469: slowest-~20 per test job (per-test cpu-s summed across the job's legs, header naming which — the issue's two traps enforced in code) and "this PR adds N tests costing X cpu-s" via tree-keyed listing artifacts, both deliberately NOT a gate with the reason in-step; verdict-safe by construction and demonstrated on a live red run; the interval-lane copy proven by a requested-lane run before merge. |
| 11 | "the rustdoc gate's `not(feature)` blind spot is closed or named at the gate" | MET-WITH-RECORDED-HONESTY | **QA-8 (#1313)**: closed AND named — pass 3 re-documents every derived `not(feature)`-family root at `--no-default-features` (root set greps the tree, no roster to rot; the grep widened to the family it pretended to be), red-first over mesh's three link errors at the intermediate commit; the residual in-half-link hole is named at the gate. **The honesty**: issue #1317 registers the two remaining axes (in-half links, 15 sites; `not(debug_assertions)`, 16 sites) WITH measurements, so the deferral has a home a future lane will find. |
| 12 | "Track J is empty in §D" | MET | **QA-8 (#1313)**: J 3→0 with heading and fence intact, D301 leaves Track R, every track's count re-derived from its table (never decremented); both reviewer arms recounted all twelve tables digit for digit. |
| 13 | "Every unit merged on its own green hosted head; the walk convention applies at exit" | MET-WITH-RECORDED-HONESTY | Nine merged unit PRs (#1237, #1232, #1297, #1307, #1311, #1310, #1313, #1312, #1331), each gated by a green hosted run on its merge head, several at requested or re-drawn matrix points so the changed instrument itself ran. **The honesty**: QA-6 PR 1 merged with ONE red row established as main's (the sampled k-lint probe panic, issues 1296/1304, M10's ground) and deliberately NOT re-rolled — re-drawing a known red into a green by changing the SHA is the class this program closes; the refusal is the compliance. |

## Walk evidence beyond the criteria

- **The A/B record**: ordinals 800–808 claimed at dispatch on main,
  samples #53, #54, #56, #57, #58, #59, #61, #63, #65 assigned at
  merge in main's merge order (QA-4 was verification-shaped by
  protocol — no row); v6 cross-model duals on every implementation
  row; every fix pass on the implementer's arm (QA-6 PR 2 as a
  logged unit-continuation). Two merge-window ledger races (CERT-4
  took #62, CERT-7 took #64) resolved by the rows' own merge-order
  hedge — the procedure is now precedent.
- **The program's signature, §D rule 5**: in every reviewed unit,
  the dual found the unit's own closed class re-minted in its new
  code or prose — QA-1's roster gate exiting green past its own
  diagnosis, QA-2's skip step asserting a false cause, QA-9's
  status line predicting "can take it" about a slot it had just
  failed to take, QA-6 calling a live register a dated write-up,
  QA-8's pinned count drifting before merge, QA-7's sweep pattern
  blind to the subshell twin, QA-6 PR 2's five record MAJORs. The
  dual review is the instrument that caught all of it; none
  reached main.
- **Dispatch discipline, calibrated**: five dispatch-premise
  corrections across the program (QA-5's inverted mechanism, two
  brief wordings, QA-7's fixture-provenance conflation, and QA-6
  PR 2's spec-level register rule — the one that reached a
  ratified spec), each caught by a reviewer arm treating the
  dispatch as a hypothesis. The orchestrator's errors are in the
  record beside the lanes'.
- **What the program found beyond its slate**: main latently red
  TWICE, both invisible to the schedule that existed — issue 1240
  (TIER=all, QA-1's lane) and the BLEND-5 census trip that every
  docs-tier main run skipped until QA-8's run was the first
  code-tier look (Evan fixed it directly, #1323); the
  Actions-budget ghost-red class (zero-step "failures" recorded in
  two unit PRs); the parity checker's flag blindness (issue 1295).
  Issues minted with homes: 1240, 1295, 1317.
- **Parked, with reasons, at exit**: #470 (its own "why not now" —
  revisit when `docs/perf-data/opt-level/` has runner samples) and
  #466 (a "consider"; its own text schedules the uniform-harness
  PR first). Issue 651 stays open as the measured-claim class
  home by design. Charter issues closed on the program's record:
  888, 1023, 1038, 1051, 1102*, 1122, 1128, 1139, 1204, 774, 808,
  882, 1134, 469, 681 (*1102 and three others were found already
  fixed by PR #1138 at opening — the charter-staleness correction
  is the program's first logged lesson: a cut surveyed at T is
  stale by T+1).
