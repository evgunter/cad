# The findings register and the stream cut, replaced by the tracker — 2026-09-03

Two documents deleted, their content carried into `work/` (contract:
`work/README.md`).

    git show <this deletion's commit>^:docs/SMELL-SCAN-2026-08.md
    git show <this deletion's commit>^:docs/WORK-STREAMS-2026-08.md

- `SMELL-SCAN-2026-08.md` — the 2026-08 structural-findings register and
  its Tracks K–X schedule. Its rows became item files under
  `work/code-quality/`, which itself left the tracker at
  `code-quality-leaves-the-tracker`; the rows
  are on the eleven programs of the 2026-09-11 cut
  (`docs/WORK-TRACKS-2026-09.md` addendum 3). The register's `D<N>`/`S<N>`
  numbering scheme is retired, not relocated — see the amendment in
  `code-quality-leaves-the-tracker`.
- `WORK-STREAMS-2026-08.md` — the 2026-08-29 stream cut. Every stream it
  proposed graduated to a program, and each `work/<program>/program.md`
  carries the charter and territory it assigned.

## Moved, not deleted

Every `docs/<NAME>-PLAN.md` / `docs/<NAME>-LOG.md` pair became
`work/<program>/plan.md` / `log.md` with git rename history intact, in
`4916f90cfc5cd45c0092b9464fd1fed604f93140` (PR #1619) or its siblings, with
no content change — `git log --follow -- <new path>` walks through the
rename. The nine `SMELL-*-LOG.md` track logs went to
`work/code-quality/logs/` and left the tree with
`code-quality-leaves-the-tracker`.
`docs/MODEL-AB-LOG.md` stays in `docs/`. `scripts/work.py lint` refuses a
plan or log reappearing in `docs/`.

`docs/PERF-PLAN.md` is the one readers have followed and failed to find; it
is `work/perf/plan.md`. The open questions about its citations are
`work/meta/perf-plan-citations-name-a-path-that-moved.md` and
`work/meta/perf-plan-is-cited-by-twenty-nine-files-and-absent-from-tree-and-ledger.md`.
