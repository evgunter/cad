# PROPS k-lint baseline — the instrument fired after going dark

**Binding at dispatch** (PROPS program; item
`work/props/klint-dev-probe-fires-after-going-dark.md` — read it in
full). Difficulty logged at spec: **M / MEASUREMENT-THEN-FIX**.
**Review tier: SINGLE review, FULL** (`memories/orchestration-model.md`).
Reason, recorded here so the call is not invisible: the change itself is
small and mechanical either way, but the CORRECTNESS question is which
way it goes — re-baselining over a real defect hides it, and chasing a
real defect that is actually an instrument artefact wastes the fix and
leaves the gate red. That is a meaningful chance of getting it wrong by
reading alone, so the reviewer carries claims to falsify, not only the
style questions. Read `docs/prompts/implementer-discipline.md` in full.
Branch `props/klint-baseline`, cut from `main`.

## What this is

`k-lint (dev-probe)` failed the 2026-09-29 nightly with 35 flags. The
item carries the per-eps table and the two halves of the situation: the
flags, and the fact that the instrument had not run over main for an
unknown window before firing.

## The order, which is the whole design of this unit

**Measure first; decide second; change last.** The one forbidden move
is changing geometry to get under a threshold, and the second-worst is
re-deriving a baseline over a real defect. So:

1. **Date the darkness.** Find when `dev-probe` last actually EXECUTED
   and passed — not when it was configured, when it ran. The item
   records that nightlies 2026-09-22..28 ran zero `k-lint` jobs and
   that no main commit in 09-25..28 carries a `dev-probe` check run;
   extend that backwards until you find the last green execution, and
   name the commit it ran on. That commit is the comparison point for
   everything below.

2. **Separate a population change from a distribution change.** Re-run
   the probe sweep at the last-green commit and at today's main
   (`scripts/k_probe_sweep.sh`), and compare the CSVs: total samples,
   the per-rule counts, and — this is the one that decides it — whether
   the 27 rule-1 rows are margins that EXIST in both sweeps, or rows
   that only the new sweep records at all. If a margin is newly
   recorded, the geometry did not move and the population did.

3. **Identify the nine, by name.** Rule 1 is 9 at EVERY eps row, so
   those nine are eps-independent: undecided or invalid margins, not
   threshold-crowding. Name the predicate and the site for each. Nine
   is small enough to enumerate and an enumeration is the receipt.
   Then do the same for rule 2's eight, which appear only at 1e-12 and
   are the eps-coupled family the rule exists for.

4. **Then, and only then, choose the recourse** from the lint's own
   list: re-derive the baseline and thresholds per `docs/K-REPORT.md`'s
   "M7 addendum (2026-08-07): the large-K lint's floor refresh", or
   demote the row to advisory with a recorded justification. **If the
   measurement says some of the 35 are real geometry that regressed,
   neither recourse applies to those** — stop and report, because that
   is a kernel defect and it is not this unit's to fix blind.

## Rulings

- **ENCL's PR 3418 is a hypothesis with a mechanism, not a finding.**
  The item explains why it fits (every classify outcome now carries a
  reporting margin, so newly-visible `Invalid` margins would land in
  rule 1 eps-independently). Test it; do not inherit it. If it is the
  cause, say so with the sample-count evidence; if it is not, say that
  just as plainly — a hypothesis handed down by an orchestrator and
  confirmed without evidence is worse than no hypothesis.
- **Do not change geometry.** Not to silence a flag, not "while you are
  there". If geometry looks wrong, that is a report and a filed row.
- **The stale recourse text is yours to fix.** The lint prints
  `ci.yml + local-scripts/ci-local.sh` as the pair that must not drift;
  after `49d5b2aee` the hosted row is in `nightly.yml`. Correct it in
  `tools/k-lint/src/main.rs` with the rest.
- **The darkness itself may deserve a row.** An instrument that stops
  running over main without anyone noticing is a finding about CI, not
  about K. If the gap is long, file it on the owning program's slate
  (`python3 scripts/work.py territory --files -` says who owns
  `.github/workflows/`), per discipline §6.

## Posture

- eps posture: none — this unit does not move a band or a comparand.
  If your recourse changes a THRESHOLD, that is a threshold change and
  it is stated with its derivation, not folded in quietly.
- **Verification is hosted CI** (discipline §2). The probe sweep is
  expensive; the nightly is where it runs. You may dispatch the nightly
  on your branch if that is the only way to get the row green, and say
  so. Do not sleep on a CI wait — poll the jobs API in the foreground.
- This is an **8-core 9 GB box with a machine-wide build mutex**: read
  `memories/agent-lane-operations.md` §Build concurrency, wrap heavy
  cargo calls in `local-scripts/with-build-slot.sh`, pass no `-j`, and
  never run two batteries at once. Use your own `CARGO_TARGET_DIR`
  outside the worktree.
- Review: SINGLE, FULL (above).
- **Landing: the item gets `pr:` and `status: review`. DO NOT MERGE.**
  No `Co-Authored-By`, no `CI-Config:` trailer, no empty commits.

## Acceptance

The darkness dated with the last green execution named by commit; the
population-vs-distribution question answered with sample counts; the
nine eps-independent rule-1 margins and the eight rule-2 margins each
named by predicate and site; the recourse chosen from the lint's own
list with its derivation stated, or a stop-and-report if the
measurement says a real regression is in there; the stale recourse text
corrected; `work.py lint` clean; the gate's own row green on hosted CI.
