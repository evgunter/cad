# QA-1 — gates that report green without running: the #888 residue

Unit spec, S-QA program (`docs/S-QA-PLAN.md`; charter
`docs/WORK-STREAMS-2026-08.md` §S-QA). Binding alongside
`docs/prompts/implementer-discipline.md` — read that in full first.

## Premise, and verify it before anything else

Issue #888's headline defect — a matcher whose exit 2 is swallowed by
the `|| true` an exclusion filter legitimately needs — was fixed on
main by PR #1138: `scripts/gates/lib.sh` now carries `gate_grep`
(read its header comment at `lib.sh:115-140`; it is the ratified
doctrine for this unit) and ten gate scripts adopted it. **Verify
this premise against your merge base**; if the tree has moved again,
report before implementing.

What remains, measured at dispatch (2026-08-29, main = `2ec2c23b`):

- `scripts/gates/probe-suite-census.sh` still carries four
  `grep … || true` matcher/filter sites (~lines 305, 622, 641, 648
  at dispatch), one of them also suppressing stderr
  (`2>/dev/null || true`), which hides the exit-2 diagnostic twice.
- `scripts/gates/gate-roster.sh` carries the shape at ~:119 and
  ~:197, with header prose (~:110, ~:185-193, ~:402) arguing the
  `|| true` is load-bearing and citing S157 (dropping one reds the
  gate's own selftest).
- The probe-suite census's **selftest** flaked red over green
  content on 2026-08-29 (hosted): `SELFTEST FAILED … clean fixture`
  + `printf: write error: Broken pipe`, while the real census step
  in the same run passed. Record:
  `docs/CI-MINUTES-2026-08.md` §"Observed flake" (2026-08-29). The
  existing fix (`selftest_hosted_half_is_large`, a padded fixture)
  covers one `grep -q` path; the flake's broken pipe came from the
  census's own `printf` at the roster-listing site — the class is
  "a producer whose reader closed early", and padding one grep at a
  time does not close it.

## Deliverables

1. **The remaining matcher sites convert to `gate_grep`, or their
   exception is argued at the site.** For each of the five sites
   above: convert per `lib.sh`'s per-stage rule, or leave it with a
   comment stating why this site is genuinely not a scanning
   matcher (a `grep -q` predicate, per the lib.sh header's own
   carve-out, is a legitimate exception). `gate-roster.sh`'s
   load-bearing prose predates `gate_grep`'s existence or not —
   `git log` decides; if it predates, the conversion supersedes the
   prose and the prose is rewritten with the conversion, not left
   contradicting it. Dropped stderr suppression comes back only
   with an argument.
2. **A fresh sweep for the shape**, per the discipline's sweep rule:
   an exit-status-masking `|| true` (or a bare pipeline tail under
   `pipefail`) downstream of a matcher, across `scripts/*.sh`,
   `scripts/gates/*.sh`, `local-scripts/*.sh`, and inline `run:`
   shell in `.github/workflows/*.yml`. Hit list with per-hit
   disposition (converted / exception argued / filed) in the PR
   description. State what the sweep pattern cannot match.
3. **A selftest arm for the malformed-pattern case**: plant a
   pattern `grep -P` rejects (the issue's own reproduction) into a
   selftest fixture and assert the gate REFUSES — red-first: show
   the pre-fix census script passing over it where applicable.
4. **The selftest broken-pipe race closed as a class**: every
   producer in `probe-suite-census.sh`'s pipelines tolerates a
   closed reader, or the readers drain — one mechanism applied
   uniformly (e.g. read-fully consumers, or SIGPIPE-tolerant
   producer wrappers), not another site-by-site pad. The 2026-08-29
   flake's shape (selftest red, real census green, broken-pipe on
   stderr) must be either impossible by construction or loud by
   name. If a deterministic regression test is constructible
   (forcing the early-close), add it; if not, say why in the PR.
5. **PR description** answers: which of #888's asks were already
   paid by #1138 (so the orchestrator can close the issue on the
   combined record), and the sweep's blind spots.

## Out of scope / fences

- `scripts/gates/bounds-allowlist.sh` is CONTESTED (live branches
  add entries). If the sweep hits it, FILE the hit on issue 888 as
  a comment-ready note in your report — do not edit the file.
- No allowlist entry changes anywhere, no `tools/` edits, no
  workflow structure changes beyond what deliverable 2's hits
  strictly need.
- k-lint distribution semantics are not reinterpreted here.

## Verification

- Every gate script you touch: run its selftest (where one exists)
  and the gate itself against the real tree, before and after; all
  green except your planted reds, each red shown then fixed.
- Shell only — no cargo builds are expected for this unit. Run
  `bash -n` on every touched script; keep `shellcheck` clean if it
  is clean at your merge base (check first).
- Hosted CI is the verification of record: the census selftest and
  discipline rows run on code-tier runs. Your PR must show the
  changed instrument firing on a planted defect AND passing clean —
  plant, record the red run, revert the plant in the same PR, cite
  both run IDs in the PR description.
- `python3 scripts/check-ci-mirror-parity.py` if you touch
  `ci.yml` at all.

## Lane discipline

- Branch `qa/1-silent-green` (already created for you); commit and
  push after every coherent step. Open the PR non-draft when ready
  for the gate (drafts run nothing).
- **Blinding**: NO `Co-Authored-By` trailer in lane commits. If one
  lands in a pushed commit, note it in the PR body and carry on —
  never rewrite history, never stop the unit over it.
- Foreground rule, both halves: never arm background waiters or
  chains for your own runs, AND launch any job that could outlive a
  600 s foreground call `setsid`-detached, then poll it in the
  foreground. Never end a turn with background work still active.
- Working artifacts (notes, sweep outputs) go in YOUR worktree or
  lane-private paths, never the shared session scratchpad.
- Final report ≤150 lines: what landed, the hit list summary, what
  #1138 already paid, blind spots, and the run IDs.
