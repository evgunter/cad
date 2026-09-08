---
id: sweep-deleting-work-meter-dangles-six-refs-on-instr-rows
kind: issue
title: deleting work/meter/ at the sweep leaves six refs: entries on INSTR rows naming ids that no longer exist, and lint requires every ref to resolve
status: open
opened: 2026-09-08
---


Found while executing step 3 of METER's close (the PR that opened
`work/instr/` and moved the twenty rows).

## What

`work/README.md`: *"**References resolve.** Every id in `parent`,
`blocked_on`, `rides_with` and `refs` names a file that exists."*
`scripts/work.py` enforces it as an ERROR, not a warning
(`scripts/work.py:390-392`).

Six of the twenty rows now on `work/instr/` carry a `refs:` entry naming
a CLOSED METER row that stays in `work/meter/` and dies with the
directory at the sweep. Today they resolve, because both directories
exist. The moment the sweep deletes `work/meter/`, `lint` errors six
times on files the sweep PR does not own:

| INSTR row | `refs:` id that dies |
|---|---|
| `C15` | `D201` |
| `cut-line-commit-names-no-baseline-change` | `cut-prefix-three-unpinned-spellings` |
| `k-lint-csv-header-unpinned-against-five-producers` | `k-lint-predicate-roster-unpinned` |
| `k-lint-eps-coupled-criterion-unwritten` | `k-lint-predicate-roster-unpinned` |
| `k-lint-gate-described-as-diffing-the-committed-baselines` | `k-report-baseline-fold-cert1-roster` |
| `tess-lint-recourse-quote-half-pinned` | `tess-lint-twinned-csv-fixture` |

Six rows, five distinct dying ids.

## This is the `parked` cost in a second shape

`work/README.md` already names the shape for `blocked_on`: *"one-file-one-item
means the program closing a trigger cannot un-park another program's rows
in the same PR, so a closing PR can red `main` for rows it does not
own."* It is the same hazard on `refs`, and it bites harder here, because
a whole directory of closed rows disappears at once rather than one
trigger firing. The answer the page gives is the same one: fix the rows,
not the check.

## What the sweep does about it

The sweep PR is METER's and the rows are INSTR's, so it is a
cross-program edit either way. Two shapes, and the choice is the sweep
orchestrator's:

- **prune the six entries in the sweep PR itself**, taking the merge
  conflict if INSTR has a lane in one of those files — the conflict IS
  the handoff surfacing, per the same page; or
- **INSTR prunes them first**, in a PR that lands before the sweep, and
  the sweep then deletes cleanly.

Either way the information is not lost: a closed row's id is recoverable
at the DOC-LEDGER SHA the sweep records, and the prose in each INSTR row
already names its predecessor by title. Do not answer it by softening
`lint`.

## Sweep and its blind spot

Derived over the whole tracker, not just the two directories, at the
commit that opens `work/instr/`: every `parent`, `blocked_on`,
`rides_with` and `refs` entry in `work/*/*.md` and `work/issues/*.md`
parsed through `scripts/work.py`'s own front-matter parser and tested
against the set of ids that stay in `work/meter/`. No program other than
METER and INSTR references a dying id; `D204`, the one non-METER id in
the six rows' `refs`, resolves to `work/code-quality/D204.md` and is
unaffected.

**What that sweep cannot reach**: an id referenced from a branch not yet
merged, and any citation of a METER row that is prose rather than a
header field — a body sentence naming `D206` is not a `refs:` entry, lint
does not check it, and it goes stale silently rather than loudly.
