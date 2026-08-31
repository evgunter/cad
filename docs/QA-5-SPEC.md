# QA-5 — the comparison gate that stops comparing (#1038, gate side)

Unit spec, S-QA program (`docs/S-QA-PLAN.md` §QA-5 and §Rulings Q3 —
**Evan's ruling, 2026-08-29: the issue's option 2**). Binding
alongside `docs/prompts/implementer-discipline.md`.

## Premise, and verify it before anything else

`tools/tess-lint` compares a fresh tessellation sweep against the
committed baseline (`docs/tess-budget-data/tess-budget-baseline.csv`)
and treats a fresh-sweep scene absent from the baseline as "new
coverage, not a finding" — so a scene the corpus gained weeks ago
that nobody folded is swept, measured, and compared against NOTHING,
forever, while the gate reports clean (#1038's measurement: 146 face
rows across five scenes, at the time). Verify at your merge base:
the benign-absence branch, the current baseline's coverage against a
scene roster (the tour's registry is the producer side), and whether
the VERBS five-scene audit fold has landed (issue 1023's thread
carries the handoff; if uncovered scenes still exist on main, your
red-first case is free — if not, you construct one).

**Fence coordination, binding**: `tools/tess-lint` is SMELL Track K
ground. Verified at dispatch (2026-08-30): no live branch touches it,
and Track K's remaining rows (D201 stable face identity, D202,
D203, D204 `CHART_TAGS`) are NOT this unit's — do not take them, do
not redesign the join, do not mint a face-identity scheme. This unit
changes exactly the uncovered-scene disposition and the cut-point
record. Say in the PR body that Track K's rows are untouched, so the
K-schedule owner can verify the seam at a glance.

## The ruling (fold it in exactly)

**An uncovered fresh-sweep scene FAILS the gate** — corpus growth
forces the baseline re-cut in the growing PR (the panic-on-move
analogue), instead of a benign aside. At current churn that fires on
~2-3 scene-adding PRs a week, each firing being the PR folding its
own rows — which well-behaved scene PRs already do voluntarily.

## Deliverables

1. **The uncovered-scene branch becomes a failing finding** (its own
   `Kind`, harness-breakage-shaped voice: the gate cannot compare
   what the baseline lacks), with the message telling the author
   exactly what to do (re-run the sweep, fold the rows, commit —
   name the script/steps the repo already documents in
   `docs/TESS-BUDGET.md`). The genuinely-new-scene case and the
   outgrown case are the same case under the ruling — both fold in
   the growing PR; say so where the old "new coverage, not a
   finding" prose lived.
2. **The baseline records its cut commit** (a header line or
   sidecar the lint reads and re-writes at re-cut), so "never seen"
   and "the cut predates it" stop being indistinguishable and the
   report can say how long a scene sat uncovered. Regenerate with
   the repo's tooling, never by hand.
3. **The reverse direction reviewed while you are in the branch**:
   a baseline scene absent from the fresh sweep is already a
   Vanished finding — confirm it still fires and that your change
   does not shadow it.
4. **Red-first**: the gate red over an uncovered scene (real if main
   still has one; constructed otherwise), green after the fold; the
   old behavior (clean + aside) reproduced at the merge base.
   `docs/TESS-BUDGET.md` updated in the same PR (its own rule says
   the doc and the gate move together), including the standing
   sentence #1038 planted there ("coverage restored is not coverage
   verified") kept intact.
5. **If the fix forces a baseline re-cut of currently-uncovered
   scenes**: fold them additively with the audit caveat stated
   per #1038's caveat (current-state, not verified-optimal), or —
   if the VERBS audit landed — verify nothing is uncovered and the
   red-first case is synthetic. Either way the PR body says which
   world it found.

## Out of scope / fences

Track K's D201-D204 (join key, face identity, CHART_TAGS); the
sizing rules themselves; `tools/tess-meter` beyond reading;
`demos/` scene content; k-lint anything. Issue 1038 is closed by
the orchestrator on your record.

## Verification

`cargo test` in `tools/tess-lint` (own CARGO_TARGET_DIR in your
worktree, deleted at the end); the gate run against the real tree
both directions (red case + clean case); hosted CI — note the
budget-gate row (`release-budget`) is SAMPLED 1-in-5 and your diff
under `tools/` now PINS `dev-default` (QA-3's pin — which runs
tess-lint's own tests but NOT the budget sweep row), so to show the
changed gate running hosted, request the row: `CI-Config:
klint=release-budget` on the head commit, and cite the run + its
CONFIG_SOURCE line. An instrument change verified only by reading
is this program's defect class — show it firing.

## Lane discipline

Branch `qa/5-uncovered-scenes` (created, from main, own spec only);
push after every coherent step; NO Co-Authored-By or model
identifier in lane commits (the spec commit carries one by
orchestrator convention — note that in the PR body); foreground
only, setsid-detach + poll anything >600 s; notes in your worktree;
scan for issue-closing keyword adjacency ("issue 1038"); PR
non-draft, body ends with a --- rule then
_Generated by [Claude Code](https://claude.ai/code)_; confirm a run
starts and poll to conclusion. Final report ≤100 lines:
per-deliverable, which world (uncovered scenes live or synthetic),
run IDs with drawn/requested config, CLASS findings called out.
