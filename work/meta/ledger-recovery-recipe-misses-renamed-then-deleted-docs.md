---
id: ledger-recovery-recipe-misses-renamed-then-deleted-docs
kind: issue
title: The ledger's recovery recipe does not resolve for any doc renamed into work/ before it was deleted — five sweep-5 programs, ten spellings, 51 live citations
status: open
opened: 2026-09-10
---



Routed from CIW unit 7 (`work/ciw/gui-log-citations-do-not-resolve.md`),
which fixed the seven citations inside CIW's own fence and could not fix
the cause. `docs/DOC-LEDGER.md` is this program's `paths`; a recovery
recipe that does not recover is the ledger's **accuracy**, which
`work/meta/program.md`'s `keep_out` keeps here even though the sweep
entries themselves belong to the program being swept — and the five
programs in question are closed and cannot write them.

## The defect

`docs/DOC-LEDGER.md:440-442` states the recovery recipe for sweep 5:

```
git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:work/<program>/<FILE>
```

and the **Inbound references** section says the old
`docs/<NAME>-PLAN.md` / `-LOG.md` citations "were already historical
after sweep 4's move; they resolve here as before."

They do not resolve. Sweep 4 **renamed** each pair into `work/`, so at
the sweep-5 SHA the `docs/` name no longer exists:

```
$ git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:docs/GUI-LOG.md
fatal: path 'docs/GUI-LOG.md' does not exist in 'f955ddc7...'
```

A reader holding an old citation has the filename and the recipe, and
neither tells them the new name. The content is intact under it.

## It is all five sweep-5 programs, not `gui`

The originating item left this unchecked. Derived on `origin/main` at
`c5558def5`:

```
$ git log --diff-filter=R --find-renames --name-status --all \
    -- 'docs/*.md' 'work/*'
```

Every sweep-5 program's plan and log was renamed out of `docs/` before
the directory was deleted, in three migration commits — `51e190112`,
`d7a8c3605` (`qa`), `4916f90cf` (`blend`, `gauth`, `gui`, `pcurve`).
The old spelling is not the directory name in four of the five, so a
reader cannot even guess it:

| sweep-5 program | old `docs/` spelling |
| --- | --- |
| `blend` | `S-BLEND-PLAN.md` / `S-BLEND-LOG.md` |
| `gauth` | `GAUTH-PLAN.md` / `GAUTH-LOG.md` |
| `gui` | `GUI-PLAN.md` / `GUI-LOG.md` |
| `pcurve` | `PCURVE-PLAN.md` / `PCURVE-LOG.md` |
| `qa` | `S-QA-PLAN.md` / `S-QA-LOG.md` |

Live citations of those ten spellings, `git grep -o "<NAME>.md" | wc -l`
on the same tree, after CIW unit 7's seven fixes land:

| spelling | n | | spelling | n |
| --- | --- | --- | --- | --- |
| `S-BLEND-PLAN.md` | 0 | | `GUI-PLAN.md` | 5 |
| `S-BLEND-LOG.md` | 2 | | `PCURVE-PLAN.md` | 2 |
| `GAUTH-PLAN.md` | 6 | | `PCURVE-LOG.md` | 1 |
| `GAUTH-LOG.md` | 4 | | `S-QA-PLAN.md` | 1 |
| `GUI-LOG.md` | 20 | | `S-QA-LOG.md` | 3 |

**44 after unit 7's seven fixes; 51 before them.** Of the 20 remaining
`GUI-LOG.md`, 10 are the originating item's own body, 1 is
`work/STATUS.md`'s generated title line, and 3 are
`docs/DOC-LEDGER.md`'s own record of the deletion — so 6 are live
pointers: `scripts/ci-filter.py` ×2, `docs/MODEL-AB-LOG.md`, and one
item file each in `work/chrome/`, `work/docm/` and `work/view/`. The
non-`gui` spellings sit in `docs/MODEL-AB-LOG.md` ×13,
`crates/viewer/src/session/{op,refuse}.rs`, `work/perf/plan.md`,
`work/code-quality/logs/SMELL-T-LOG.md` and `docs/DOC-LEDGER.md` ×2.

## What it wants

A **rename note in sweep 5's table** — one column, or one sentence per
row, giving the `docs/` name each directory carried before sweep 4 —
so the documented lookup resolves the old spelling without editing any
citation. That is the fix the originating item called "cheapest, and it
fixes every citation at once", and it is a precondition for leaving any
un-repointed citation standing anywhere in the repo.

The **Inbound references** paragraph's claim that these "resolve here as
before" is the sentence to correct: it is what tells a reader not to
worry.

Sibling in class, same slate:
`perf-plan-is-cited-by-twenty-nine-files-and-absent-from-tree-and-ledger`
— a citation whose target the ledger does not record at all. This one is
the target the ledger records under a name the citation does not use.

## Not this item

Repointing the 44 remaining citations. Most are append-only or
provenance mentions, which the ledger's own Inbound-references note
says are fine once the recipe resolves; the ones in live present-tense
code are their owners'.
