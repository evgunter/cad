---
id: inherited-red-is-not-attributed-to-its-merge
kind: unit
title: A lane cannot tell in one command whether its PR's red tests are already red on main
status: open
opened: 2026-09-07
priority: P2
cost: E
---

A red on a PR's gate is often not the PR's. In the last month lanes merged
over inherited reds and annotated them by hand at least nine times
(`work/green/log.md`, 2026-11-14: PRs 6921, 6932, 6934 on main's three sweep
reds and the 1e-12 bead; 2026-11-22: PR 7359 on CHARTTAIL's torn-body fuzz
row; `work/band/log.md` 2026-11-12, PR 6514; `work/sect/log.md` 2026-11-21,
PR 7236; `work/parting/log.md` 2026-11-01, PR 6225; `work/spiricarm/log.md`
2026-11-03, PR 6473). Each time the lane first had to establish
"inherited or mine" by hand, and GERM's U3a fix pass spent about two hours
doing that for seed reds alone (`work/germ/log.md`, 2026-11-21 11:50).

Every input to that question already exists without a build:

- the PR run's failing test names (its junit artifact, or the
  `check-fast-set-times` / nextest summary in the job log);
- the last scheduled nightly **of main**'s failing test names per ε row
  (its junit artifacts; note GREEN's 2026-11-22 entry: dispatched nightlies
  can run on a branch, so the script filters `event == schedule` and
  `head_branch == main`);
- the reds the tracker already holds (`pinned_red` call sites and the open
  `main-red-*` rows under `work/`).

What is owed is a read-only script, `scripts/inherited-reds.py <run-id>`
(no workflow change, no runner minutes), that prints each failing test of
the run as `inherited (red on main nightly <run>, held by <row>)`,
`inherited (red on main nightly, held by no row)` or `new on this PR`, and a
line in `docs/prompts/` lane discipline pointing at it where the
inherited-red annotation rule is stated. Building main's tree, bisecting,
or a gating job are out of scope: the original row's own "cheap version"
is the whole of the value.

What would close it: the script exists, its selftest runs in nightly's
`gate self-tests`, and one lane's red PR has been annotated from its output.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): Frequent, and recent. At least nine PRs since 2026-10-22 merged over inherited reds that were diagnosed and annotated by hand: green/log 11-14 (6921, 6932, 6934), 11-22 (7359); band/log 11-12 (6514); sect/log 11-21 (7236); parting/log 11-01 (6225); spiricarm/log 11-03 (6473); boxes/log 11-02 (6383). GERM's U3a spent about 2 h proving seed reds inherited (germ/log 2026-11-21 11:50). As written, though, the row builds main's tree on red runs, which costs runner minutes and queue time. The reformulation is a read-only script comparing a run's failing tests with the last scheduled main nightly's junit and the held rows: no build, no workflow job. `reform/inherited-red-is-not-attributed-to-its-merge.md`.
