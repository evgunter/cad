---
id: gui-log-citations-do-not-resolve
kind: issue
title: Twelve live citations of docs/GUI-LOG.md, and the ledger's recovery recipe does not resolve any of them
status: open
opened: 2026-09-04
refs: [geom-brep-test-unused-edgedescription-import, 1795]
---


Found by the CIW lane for the `--all-features` clippy row, whose brief
cited `docs/GUI-LOG.md` for Ev's viewer-CI-posture ruling and whose
style review counted the citation dead. It is dead, and the reason is
one layer deeper than a stale filename.

## The file moved before it died, so the ledger's recipe misses it

`docs/DOC-LEDGER.md` opens with the recovery recipe
`git show <sweep-sha>:docs/<NAME>` and states, under **Inbound
references**, that "append-only logs ..., source comments, and completed
rows in live plans still name deleted files. Those are not broken: the
filename plus the recovery recipe at the top of this document resolves
any of them."

For this file it does not resolve. `docs/GUI-LOG.md` was **renamed** to
`work/gui/log.md` at `4916f90c` ("work: migrate m10, pcurve, verbs, lib,
gui, gauth, seat, blend, perf"), and only then deleted, at
`a1425f92`, recorded in ledger sweep 5 as the program directory `gui`.
So:

```
git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:docs/GUI-LOG.md
fatal: path 'docs/GUI-LOG.md' does not exist in 'f955ddc75cda454a268f9214d2a753ae1a9bbd0f'
```

The content is there, under the name sweep 5 recorded:
`git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:work/gui/log.md`. Nothing is lost;
what fails is the documented lookup, for every citation that spells the
old path.

Sweep 3's "Kept, and why" section still reads `GUI-PLAN.md` /
`GUI-LOG.md` as kept, with two file-specific reasons. Sweep 5's own
prose says it supersedes that decision, so the two are consistent — but
a reader who greps the ledger for `GUI-LOG.md` lands on the "kept"
paragraph first.

## The live citations, derived on this tree

`git grep -n "GUI-LOG.md"`, excluding `docs/DOC-LEDGER.md`'s own three
(which are the record of the deletion, not pointers into it): **twelve
occurrences across nine files.**

| file | n |
|---|---|
| `.github/workflows/ci.yml` | 3 |
| `scripts/ci-filter.py` | 2 |
| `.github/workflows/nightly.yml` | 1 |
| `local-scripts/ci-local.sh` | 1 |
| `scripts/doc-gate.sh` | 1 |
| `docs/MODEL-AB-LOG.md` | 1 |
| `work/chrome/viewer-first-light-on-real-hardware.md` | 1 |
| `work/docm/certify-locally-valid-range-instead-of-sampling.md` | 1 |
| `work/view/focus-marking-is-per-node-not-per-segment.md` | 1 |

Not all twelve are the same kind of claim, and the fix is not uniform:

- **Eight are in CI code** (`ci.yml` x3, `ci-filter.py` x2,
  `ci-local.sh`, `nightly.yml`, `doc-gate.sh`) and cite the file as the
  SOURCE OF A LIVE RULING that gates
  what runs today. Those are the ones that matter: a reader who cannot
  reach the ruling cannot check whether the code still applies it, which
  is exactly the failure this lane hit. `scripts/ci-filter.py`'s is the
  fullest paraphrase, and ci.yml's `clippy` job already points there
  for the full argument.
- **Four** — three tracker items and `MODEL-AB-LOG.md` — cite it as
  provenance for something said at a moment in time. Those are the
  append-only case the ledger's Inbound-references note is about, and
  re-pointing them buys little — but they still spell a path the recipe
  cannot open.

## What this lane did and did not do

The `--all-features` clippy row's own citation was the same mistake,
made fresh, and is fixed on that PR: it now names the ruling's words, says the log left the
tracker in sweep 5, and gives the `work/gui/log.md` path at the sweep
SHA. **All twelve above are untouched** — the count was taken after that
fix, so none of them is mine. A lane fixing its own line is not a
sweep, and the ledger question above is the part worth deciding before
anyone edits eight CI comments.

Two candidate fixes, not chosen here:

1. Add a **rename note to sweep 5's table** — `gui` was
   `docs/GUI-PLAN.md` / `docs/GUI-LOG.md` before `4916f90c` — so the
   documented recipe resolves the old spelling. Cheapest, and it fixes
   every citation at once without editing any of them.
2. Re-point the eight CI citations at `scripts/ci-filter.py`'s
   `RUN_VIEWER_TOOLKIT`, which is where the ruling is paraphrased in
   live code, and leave the provenance mentions alone.

They are not exclusive and (1) is a precondition for leaving anything
un-repointed.

## Scope this did not sweep

Only the exact string `GUI-LOG.md`. A citation spelling it "the GUI
log", "the GUI program's log", or naming a different deleted `docs/`
file that also moved before it died would not match, and no count here
covers those. Whether other sweep-5 directories have the same
rename-then-delete shape is unchecked.
