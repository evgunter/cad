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
