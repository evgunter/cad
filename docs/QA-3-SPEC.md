# QA-3 — the debt-charging class: the tools-scope k-lint path pin (#1023 + D183)

Unit spec, S-QA program (`docs/S-QA-PLAN.md` §QA-3 and §Rulings Q1 —
**Evan's ruling, 2026-08-29, is the charter here**). Binding alongside
`docs/prompts/implementer-discipline.md` — read that in full first.

## Premise, and verify it before anything else

The k-lint job samples ONE of its five feature-unification rows per
run (`scripts/ci-filter.py`'s `KLINT_ROWS`, drawn under its own salt).
The consequence, measured twice: the merge that changes what a row
compiles is more likely than not gated by a different row, so
breakage lands undrawn and detonates in an unrelated PR (#1023's
instance; D183's tess-meter constants case; the banked VERBS finding
that main can be latently red for four of five rows). Verify against
your merge base: the sampling, the five rows, and the absence of any
path-shaped pin on the klint dimension. Note the tree now carries
QA-2's machinery — `_forces_interval` (the surviving arms), the
`--notices` file, `CONFIG_SOURCE` per-dimension `pinned` vocabulary —
which this unit extends to the klint dimension rather than
re-inventing.

## The ruling (fold it in exactly)

A change under **`tools/`** forces the k-lint row that compiles what
changed — D183's mechanism at the `tools/` scope (~7% of code-shaped
merges, measured over 14 days at ruling time). **`demos/` is
explicitly NOT pinned** (~29% — determinism there would erode the
sampling, and the demos failure shape that actually bit is caught by
any row that runs); state that at the site so the scope is a decision,
not an omission. No unconditional row, no scheduled run.

## Deliverables

1. **`_forces_klint(files)` in `scripts/ci-filter.py`**, the
   `_forces_interval` sibling: any changed file under `tools/` pins
   `KLINT_ROW` ahead of the draw. **The path→row mapping is DERIVED
   from what the five rows actually compile** — read the k-lint job
   and `KLINT_ROWS`' own definitions and write the mapping with its
   derivation at the site (D183's named case: `tools/tess-meter/`
   forces the `dev-default` row, whose tests hold the split-scan
   guards). Where a changed path is compiled identically by every
   row (no feature/profile sensitivity derivable), pin the
   `dev-default` row and say why — fail closed into the row that
   runs the most tests, never into the draw. Precedence mirrors the
   lane dimension: an explicit request (dispatch input or
   `CI-Config:` trailer) overrides the pin; the pin overrides the
   draw.
2. **The pin announces itself**: `CONFIG_SOURCE` reports
   `klint:pinned` with the reason (file + row) in the notices file,
   exactly as the lane pin does. Self-tests: restoring the draw over
   a `tools/` diff must red `ci-filter.py`'s selftest; the
   announcement and the precedence order each get a red-able case.
3. **The three owed sentence corrections** (the debt
   `docs/CI-MINUTES-2026-08.md:335` records): `docs/K-REPORT.md:219`
   and `:226` still say "unconditional" about rows the sampling made
   1-in-5 — correct them to the true schedule (which this unit
   improves: unconditional-when-`tools/`-changes is now real for the
   pinned paths); the `KLINT_ROWS` header's own third-instance note
   updates to point at the landed pin; the CI-MINUTES debt line is
   resolved with a pointer, not deleted.
4. **D183 leaves the schedule**: delete the row from
   `docs/SMELL-SCAN-2026-08.md` §Track J in this PR, per §D's
   landing convention. Re-derive the track's count from its table
   after the edit — never transcribe or decrement (two sessions have
   been burned by reconciling counters that hid wrong tables).
5. **PR description** carries: the derivation of the path→row
   mapping; the measured firing rate restated (~7% at `tools/`
   scope) with the demos exclusion and its reason; the accepted
   residue stated plainly (path-uncorrelated breakage still lands
   and persists until a later draw, per the sampling design's own
   argument — this unit narrows the hole, it does not close it);
   what the sweep for other undrawn-row debt shapes could not match.

## Out of scope / fences

- `demos/` pinning (ruled out), any new CI job or schedule, k-lint's
  five rows themselves, `docs/K-REPORT.md` beyond the two named
  sentences (K-telemetry semantics are the K ground), the
  bounds-allowlist scripts.
- Issue 1023 is closed by the ORCHESTRATOR on your record — do not
  close it; reference it non-adjacently ("issue 1023") everywhere.

## Verification

- `python3 scripts/ci-filter.py`'s selftest and
  `python3 scripts/check-ci-mirror-parity.py`, clean.
- Hand-run the filter on synthetic lists: a `tools/tess-meter/`
  diff, a `tools/k-lint/` diff, a `demos/`-only diff (must DRAW),
  a mixed diff, an empty list — read KLINT_ROW/CONFIG_SOURCE/notices
  for each, pre- and post-change.
- Hosted: your own PR touches `tools/`… it does not — it touches
  `scripts/` and docs, so the pin will not fire on your own run by
  path. Demonstrate it hosted the honest way: one commit in this PR
  adds a trivial real change under `tools/` (e.g. a comment line in
  `tools/tess-meter/src/main.rs`) so the run pins and the notices
  show it, then KEEP that commit (a real comment is not a plant —
  make it say something true and useful) or revert it with both run
  IDs recorded, your call — either way the pin must be shown firing
  on hosted output, and the drawn-vs-pinned line cited.
- An instrument change verified only by reading the script is this
  program's defect class — show it firing.

## Lane discipline

Branch `qa/3-klint-path-pin` (already created, from main, carrying
only this spec); commit and push after every coherent step
(`git push -u origin qa/3-klint-path-pin`); open the PR non-draft
when ready. NO `Co-Authored-By` trailer or model identifier in lane
commits (blinding override; if one lands pushed, note it in the PR
body and carry on — never rewrite history). Foreground rule, both
halves: never arm background waiters for your own runs, AND anything
that could outlive a 600 s call runs `setsid`-detached and is polled
in the foreground; never end a turn with background work active.
Notes stay in your worktree. Scan every commit message and the PR
body for issue-closing keyword adjacency. Final report ≤120 lines:
per-deliverable dispositions, the mapping derivation, run IDs with
drawn/pinned configuration, and any CLASS-shaped findings called out.
