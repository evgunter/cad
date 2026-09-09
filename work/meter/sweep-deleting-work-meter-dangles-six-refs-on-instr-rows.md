---
id: sweep-deleting-work-meter-dangles-six-refs-on-instr-rows
kind: issue
title: deleting work/meter/ at the sweep leaves six refs: entries on INSTR rows naming ids that no longer exist, and lint requires every ref to resolve
status: closed
opened: 2026-09-08
closed: 2026-09-09
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

Six rows, five distinct dying ids. The form the sweep should use is below,
and it is not a new decision: GATES answered exactly this at its own sweep
on the same day.

## The form is settled — GATES did this at its own sweep today

`work/README.md` already names the shape for `blocked_on`: *"one-file-one-item
means the program closing a trigger cannot un-park another program's rows in
the same PR, so a closing PR can red `main` for rows it does not own."* This is
the same hazard on `refs`, and it bites harder, because a whole directory of
closed rows disappears at once rather than one trigger firing.

**It does not need a new answer. GATES hit it this morning and answered it:
commit `3a8dd05fe`, *"Sweep 7: refs to GATES' deleted ids cite their closing
PRs"*.** Its form, in two parts per citing row:

1. **Replace the dying id in `refs:` with the number of the PR that closed it**,
   in place, leaving the rest of the list alone. `work/README.md`: *"Ints are PR
   or issue numbers and are not checked."* So `refs: [D109, D205, D261, D287]`
   became `refs: [2038, D205, D261, D287]` on `work/code-quality/D208.md`.
2. **One prose section at the citing row**, headed `## Refs at GATES' sweep
   (2026-09-08)`, reading: *"GATES closed and its item files left the tracker
   (`docs/DOC-LEDGER.md`, sweep 7); `D109` is now cited by its closing PR
   2038."* A row substituting two ids carried both clauses in the one sentence
   (`work/code-quality/D68.md`).

The provenance survives, because the PR carries the work and the ledger SHA
carries the deleted file; nothing is stripped; and `lint` is satisfied without
being softened.

**And it settles the ordering, which is therefore no longer a question.**
`3a8dd05fe` edited rows on `work/code-quality/`, `work/meta/` and `work/topo/`
— three programs that were not GATES — from GATES' own sweep. The rewrite is
mechanical and carries no design content, so the only cost is a merge conflict
if an INSTR lane holds one of those six files open, and that conflict is the
handoff surfacing rather than a defect. **The sweep does the rewrite in its own
commit.** It does not need INSTR to land a prior PR.

## METER's mapping — each id and the PR that closed it

| dying id | closing PR | citing row(s), now on `work/instr/` |
|---|---|---|
| `D201` | **2167** | `C15` |
| `cut-prefix-three-unpinned-spellings` | **2151** | `cut-line-commit-names-no-baseline-change` |
| `k-lint-predicate-roster-unpinned` | **2115** | `k-lint-csv-header-unpinned-against-five-producers`, `k-lint-eps-coupled-criterion-unwritten` |
| `k-report-baseline-fold-cert1-roster` | **2140** | `k-lint-gate-described-as-diffing-the-committed-baselines` |
| `tess-lint-twinned-csv-fixture` | **2179** | `tess-lint-recourse-quote-half-pinned` |

**Derived twice, by methods that do not share a failure.** First: each closed
row's `branch:` field matched against the merge subjects on `origin/main`
(`git log origin/main --merges --pretty='%h|%s'`), which named
`meter/split-scan-and-face-name` → 2167, `meter/cut-prefix-pin` → 2151,
`meter/klint-roster-pin` → 2115, `meter/k-report-cert1-fold` → 2140,
`meter/12-twinned-csv-fixture` → 2179. Second, and independent of the `branch:`
field: the commit that first wrote `status: closed` into each file
(`git log origin/main -S 'status: closed' --pickaxe-regex -- <file> | tail -1`),
then the first `Merge pull request` on the ancestry path from that commit to
`origin/main`. The two agree on all five. A third source agrees on one:
`tess-lint-twinned-csv-fixture` carries `pr: 2179` in its own header.

**What that derivation cannot reach**: a row closed with no PR at all (no
`branch:`, or closed as a rider on another program's PR) would produce no hit
rather than a wrong number — none of the five did — and a branch name reused by
two PRs would be mis-attributed by the first method, which is why the second
exists.

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
does not check it, and it goes stale silently rather than loudly. The GATES
form does not fix those either; it fixes exactly what lint can see.

## Closing this row

It closes when the sweep has made the six substitutions and `lint` is green
on a tree with `work/meter/` gone. Nothing here is a design question any
more, so it wants no `[ev]` PR: the precedent is merged and the mapping is
derived twice.

## Closed (2026-09-09) — the sweep made the six substitutions

Discharged by METER's closing sweep, in the commit that deletes
`work/meter/` and `docs/METER-EXIT-WALK.md`
(`docs/DOC-LEDGER.md`, sweep 10). The GATES form was followed as this
row specified: the dying id replaced in `refs:` by the number of the PR
that closed it, and one `## Refs at METER's sweep (2026-09-09)` section
at each citing row saying which id is now cited by which PR.

**The mapping was re-derived by the sweep before it was written**, both
of the methods this row names, and both agree with the table above:
`meter/split-scan-and-face-name` → 2167, `meter/cut-prefix-pin` → 2151,
`meter/klint-roster-pin` → 2115, `meter/k-report-cert1-fold` → 2140,
`meter/12-twinned-csv-fixture` → 2179.

**The citing rows were re-derived too, rather than taken from the table.**
Every `parent`, `blocked_on`, `rides_with` and `refs` entry in the whole
tracker was parsed through `scripts/work.py`'s own `load_tree` and tested
against the ids that die with `work/meter/` — 13 hits, of which 7 are
inside `work/meter/` itself and die with it, and 6 are the rows named
above. No third program cites a dying id. **What that sweep cannot
reach** is unchanged from the one above: an id referenced from an
unmerged branch, and prose citations, which lint does not see. The sweep
PR reports the prose citations it found rather than rewriting them —
`docs/DOC-LEDGER.md`'s *A note on inbound references* is the standing
answer for those, and GATES' sweep left its own (`work/gates/D103.md` is
still cited from `scripts/gates/viewer-module-kinds.sh:85` today).

**One deviation from mechanical substitution, in two rows.**
`k-lint-gate-described-as-diffing-the-committed-baselines` already carried
`2140` beside the dying id and `tess-lint-recourse-quote-half-pinned`
already carried `2179`; substituting in place would have written the same
number twice, so the dying id was dropped and the section at each row says
so.
