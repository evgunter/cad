# Sweep 4 — 2026-09-03: the work tracker replaces the register and the survey

The tracker under `work/` (contract: `work/README.md`) is now the one
home of live work, and two documents whose content it carries are
deleted. Recover either with `git show <this sweep's SHA>^:docs/<NAME>`.

- **`SMELL-SCAN-2026-08.md`** — the structural findings register and
  its Tracks K–X schedule. Every live row is an item file
  `work/code-quality/<ROWID>.md` (109 rows, the row id kept as the file
  name), every Ev-only decision a `ruling` item there, every live
  unrowed finding an `issue` item there, and the four ordering rules,
  the partition rules, the territories table and the seams are
  `work/code-quality/plan.md`. §C's process observations are
  `work/code-quality/process-observations.md` verbatim. The census
  that reconciles all 94 finding headings against the tree is
  `work/code-quality/logs/migration-census-2026-09-03.md`; nothing
  was dropped.
  **Re-aimed at sweep 11 (2026-09-11), because that paragraph's three
  live pointers all went that day**: the rows are on the eleven programs
  of the 2026-09-11 cut and no longer in one directory
  (`docs/WORK-TRACKS-2026-09.md` addendum 3); the rules, the process
  observations and the migration census went to the archive with the
  directory and are recoverable at sweep 11's SHA. **The register's
  numbering scheme is retired, not relocated** — see sweep 11's
  amendment. One source defect is carried as a flagged
  reconstruction: partition rule 4's opening sentence was already
  missing from the document (its text began mid-sentence), and the
  plan states it as "A style review runs on every unit against …".
- **`WORK-STREAMS-2026-08.md`** — the 2026-08-29 stream cut. Every
  stream it proposed graduated to a program, and each program's
  `work/<program>/program.md` now carries the charter and territory
  the cut assigned it. Plans that cite the cut as their charter keep
  the citation; it resolves here.

### Moved, not deleted

Every `docs/<NAME>-PLAN.md` / `docs/<NAME>-LOG.md` pair is now
`work/<program>/plan.md` / `log.md` (git rename history intact). The
nine `SMELL-*-LOG.md` track logs were under `work/code-quality/logs/`
and **left the tree at sweep 11** (2026-09-11), recoverable at the SHA
that sweep names; they are closed tracks' execution records, and the one
standing rule they were cited for — *the fix mints a fresh instance of
the defect it closes* — is now a bullet in
`docs/prompts/reviewer-style-lane.md` §1, where every reviewer reads it.
`MODEL-AB-LOG.md` stays in `docs/` as the
experiment log it is. `scripts/work.py lint` refuses a plan or log
reappearing in `docs/`.

**The moves, by name**, because a class rule does not answer a
by-name lookup and the one document this ledger exists to serve is a
reader holding a stale citation. Every pair below moved in
`4916f90cfc5cd45c0092b9464fd1fed604f93140` (2026-09-03, PR #1619) or
its siblings in the same migration, each with **no content change** —
`git log --follow -- <new path>` walks straight through the rename:

| was | is now |
| --- | --- |
| `docs/PERF-PLAN.md` | `work/perf/plan.md` |
| `docs/<NAME>-PLAN.md` (every other program) | `work/<program>/plan.md` |
| `docs/<NAME>-LOG.md` (every program) | `work/<program>/log.md` |
| `docs/SMELL-{C,E,F,G,H,I,KPW,T,UV}-LOG.md` | `work/code-quality/logs/` |

`PERF-PLAN.md` is named in its own row because it is the one readers
have followed and failed to find. Measured 2026-09-11: **32 tracked
files mention it, and five cite it by the dead PATH** — the other 27
name the DOCUMENT, which still carries that name as its own title at
`work/perf/plan.md` and needs nothing done to it. Repointing the five,
and deciding whether a document keeps a name its path no longer
carries, is `work/meta/perf-plan-citations-name-a-path-that-moved.md`.
Two further mentions in this ledger (sweep 1's archive note, DOCM-5's
per-merge record) were left as written: they record what those documents
said at the time.
