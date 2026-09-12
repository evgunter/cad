---
id: ci-filter-cites-a-path-the-ledger-recipe-cannot-open
kind: issue
title: ci-filter.py's two docs/GUI-LOG.md citations name a path the ledger's recovery recipe cannot open
status: closed
opened: 2026-09-10
closed: 2026-09-12
---



Routed from CIW unit 7 (`work/ciw/gui-log-citations-do-not-resolve.md`).
`scripts/ci-filter.py` is this program's, named in CIW's own `keep_out`,
so unit 7 fixed the other six sites and left these two.

## The two sites

Line numbers on `origin/main` at `c5558def5`:

- `scripts/ci-filter.py:1420` — the `VIEWER_TOOLKIT_SEEDS` header:
  "(Ev's viewer-CI-posture ruling, 2026-08-27; docs/GUI-LOG.md)".
- `scripts/ci-filter.py:2197` — the `RUN_VIEWER_TOOLKIT` decoration:
  "(Ev, 2026-08-27, ruling recorded in docs/GUI-LOG.md: …)".

`docs/GUI-LOG.md` does not exist, and — this is the part a filename
alone does not tell you — the ledger's documented recovery recipe does
not open it either. The file was **renamed** to `work/gui/log.md`
before the `gui` directory was deleted, so:

```
$ git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:docs/GUI-LOG.md
fatal: path 'docs/GUI-LOG.md' does not exist in 'f955ddc7...'
$ git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:work/gui/log.md | wc -c
33562
```

The ruling is lines 88-112 of the recovered file.

## Why these two matter more than the other ten

`scripts/ci-filter.py` carries the **fullest paraphrase** of the ruling
in live code, and every other site in the repo — including all six unit
7 rewrote and `.github/workflows/ci.yml`'s own — points here for the
argument. A reader who follows this file's citation to check whether the
code still applies the ruling is the last one who can, and it is the one
that dead-ends.

## The fix

The house spelling landed at `.github/workflows/ci.yml` and is now on
all six CIW-owned sites: name the ruling, say the log left the tracker
with the closed GUI program's directory in DOC-LEDGER sweep 5, and give
`git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:work/gui/log.md`.
Two comment lines; no behaviour changes, so `--selftest` is unaffected.

Cheaper alternative, if the ledger fix lands first: leave the filename
and let the recipe resolve it —
`work/meta/ledger-recovery-recipe-misses-renamed-then-deleted-docs.md`
asks for the rename note that would make that true. The two are not
exclusive and the ledger note is the precondition for leaving any
citation un-repointed.

## Closed 2026-09-12 (branch `tcost/ci-filter-gui-log-citations`)

Both sites carry the house spelling now, matched against the landed
copy at `.github/workflows/ci.yml`'s `run_viewer_toolkit` output header
rather than re-invented: the ruling named and dated, the statement that
the log left the tracker with the closed GUI program's directory in
DOC-LEDGER sweep 5, and
`git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:work/gui/log.md`.
Line numbers re-derived on this branch's merge base rather than taken
from `c5558def5`: the `VIEWER_TOOLKIT_SEEDS` header had not moved, the
`RUN_VIEWER_TOOLKIT` decoration had gone from 2197 to 2438. Comments
only; `--selftest` unaffected and re-run.

The cheaper alternative is declined and stays available: repointing
does not depend on the ledger note, and
`work/meta/ledger-recovery-recipe-misses-renamed-then-deleted-docs.md`
is still the right fix for citations nobody re-points.

**What the sweep found, re-run on the merged tree.** `git grep -n
"GUI-LOG.md"`, less this file, `work/STATUS.md`, `docs/DOC-LEDGER.md`,
the originating CIW item and `work/ciw/log.md`'s account of it. **The
CI-CODE class this item is about is now closed, 8 of 8**: CIW unit 7
took six, this takes the last two. What remains is the other class the
CIW item separated out and declined — **four provenance citations**,
naming the file for something said at a moment in time rather than
pointing a reader at a live ruling: `docs/MODEL-AB-LOG.md`'s banding
entry and three tracker items (`work/chrome/`, `work/docm/`,
`work/view/`). All four sit outside S-TCOST's fence, and the CIW item
already ruled that re-pointing them buys little against the ledger note
`work/meta/ledger-recovery-recipe-misses-renamed-then-deleted-docs.md`
asks for. Untouched here, and reported.

**What the sweep could not match.** The same blind spot CIW unit 7
recorded still holds here: only the exact string `GUI-LOG.md` was
swept, so a citation spelling it "the GUI log", "the GUI program's
log", or naming a different `docs/` file that was renamed before it
died would not match.
