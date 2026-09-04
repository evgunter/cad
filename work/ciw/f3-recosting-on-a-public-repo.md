---
id: f3-recosting-on-a-public-repo
kind: unit
title: F3 and the nightly demotions rest on an Actions allowance this repo no longer has: re-cost them on a public repo
status: open
opened: 2026-09-04
---



Opened on Ev's direction (in-chat, 2026-09-04: CIW opens the re-costing
as a unit) out of the closure of `main-latently-red-at-tier-all`, whose
class half this is.

## The fact that moved

`evgunter/cad` went **public** on 2026-09-03 (the repository's
`visibility` is `public`; the tree's own record is `5cc16e81` through
`483212ef`, and `483212ef` restates the runner spec). Two things follow
and neither is a matter of opinion:

- **Standard-runner minutes are free.** `docs/CI-MINUTES-2026-08.md`
  opens with *"the Actions allowance was being consumed faster than the
  work justified"*. That sentence is the premise of the whole document
  and of every trim it licensed.
- **The runner is 4 vCPU / 16 GB**, up from 2 vCPU / 7 GB. `483212ef`
  says so at the site and marks the document's timings as predating it.

The same day, and before the visibility change, the account's Actions
spending limit denied job starts outright for two and a half hours
(`work/issues/actions-budget-denies-job-starts`, closed). That is the
old regime's last data point, not this one's.

## What is therefore open

Three ratified or landed decisions were bought with billed minutes and
have not been re-read since the price went to zero:

1. **F3** — a `push: main` run is reduced to `filter` +
   `rebuild-latency` + `renders`, skipping build, test, clippy and
   k-lint (`docs/CI-MINUTES-2026-08.md` §F3, commit `0768882`, Ev
   authorised 2026-08-20). Its stated cost is that *"the landed main
   commit is then never itself tested"*, and the compensating control
   is the next PR's merge ref. The scheduled full run on main that
   would have paired with it was declined by Ev on 2026-08-22, on the
   same cost grounds.
2. **This month's demotions to the nightly** — TCOST-C1, C2 and C3 move
   `corrupt input (release profile)`, the rustdoc gate's excluded roots
   and third pass, and the python suite's ungated re-take out of the
   per-PR gate. Each argued a billed-minute saving.
3. **The declines that named a minute as the reason**, chief among them
   `doc-gate-two-unread-axes` axis (b): a `--release` doc pass was
   declined because it is a fourth compilation on a job F6 had fought
   back inside two billed minutes.

## What this unit does, and what it does not

**Measures first.** What a restored full `push: main` run now costs in
**wall clock** on the 4-vCPU runner — not in minutes, which are no
longer the currency — against what it buys: the landed commit tested as
landed, rather than a merge preview of it. The build jobs are the
critical path (F4: `build + archive (interval)` alone is ~88% of a
13.75-minute critical path, at 2 vCPU), so the re-take is a re-take:
every figure in `CI-MINUTES-2026-08.md` predates the runner change and
none of them may be quoted forward.

**Then proposes, on an `[ev]` PR.** F3 is Ev's ruling and the declined
scheduled run is Ev's decline; neither is reopened by a lane deciding
it now costs less. This unit's deliverable is the measurement plus a
recommendation, and the change — if any — lands after Ev answers.
`work/README.md`: the question rides an `[ev]` PR and this item sets
`needs_ev` when it is asked.

**Out of scope**, so that the unit does not become the whole board:

- Cache and build knobs stay S-TCOST's under this program's `keep_out`,
  including the finding that makes the wall-clock number worse than it
  looks (`work/tcost/rust-cache-never-restores-across-branches`). This
  unit may cite that measurement; it may not fix it.
- The change filter's tiering is not re-opened wholesale. F3 is one
  decision with one written argument, and that is the subject.
- Nothing here reads on whether the demoted rows WORK — that is
  `work/ciw/nightly-demotions-have-never-run`, and it is a different
  defect with a different fix.

## Inherited class: "2 vCPU" asserted in prose, 2026-09-04 (from unit 5)

Unit 5 (`perf-history-cannot-identify-its-host`, PR 1722) edited six
files and found that each still asserted the old runner three lines from
the paragraph it had just added saying the runner changed. It fixed the
six **in its own diff** and stopped there; the rest is this unit's,
because the subject is the same one — what the 2026-09-03 runner change
invalidates.

**The count, measured at that PR's merge base.** `2 vCPU` or `2-vCPU`
appears **51 times across 43 files**, excluding the two occurrences in
`docs/perf-data/criterion/README.md` that correctly *describe* the change.
It is not one sweep but three sub-classes, and only the third is likely to
be a mechanical edit:

* **~26 hits, `crates/*/Cargo.toml` + `crates/*/tests/all.rs` (13 crates,
  one pair each).** These cite the 2-vCPU runner as the *reason* for the
  one-test-binary layout ("on the CI runner (2 vCPU) the per-binary
  codegen+link…"). **The number is load-bearing on a decision**, so these
  are not text fixes: the layout's justification has to be re-checked at
  4 vCPU / 16 GB before the sentence is rewritten, and re-checking it is
  this unit's kind of work.
* **~10 hits in costing prose** — `docs/CI-MINUTES-2026-08.md` (×3),
  `docs/GENERICS-BUILD-COST.md` (×2), `docs/PERF-SCAN-2026-08.md`,
  `.github/workflows/ci.yml` (×5), `.github/workflows/nightly.yml`, plus
  this item's own two. Same status as every other figure in
  `CI-MINUTES-2026-08.md`: predates the change, not quotable forward.
* **the remainder** — `memories/perf-measurement-lane.md:25`,
  `scripts/doc-gate.sh`, `scripts/check-ci-mirror-parity.py`,
  `benches/benches/kernel.rs`, a few `crates/*/tests/*.rs` headers and
  three `work/` logs. These are variance/fat-tail asides where the vCPU
  count is incidental to the point; unit 5's repair spelling ("a shared
  hosted runner has a fat tail") drops the stale number without
  claiming a new one, and it applies unchanged here.

`memories/perf-measurement-lane.md:25` is worth calling out on its own:
it is the file the perf READMEs cite as their authority, so it is the
one whose staleness propagates.

Recorded here, not fixed by unit 5, because a 43-file sweep inside a
two-field emitter change would have buried the change it was reviewed
for — and because the first sub-class is a re-costing question, which is
this unit's whole subject.
