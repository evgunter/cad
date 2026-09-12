---
id: ledger-recovery-recipe-fails-in-a-shallow-clone
kind: issue
title: The ledger's recovery recipe fails in a shallow clone with a message that reads like a wrong recipe
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the S-TCOST orchestrator, out of hitting it while
checking that S-TCOST PR 2426 shipped a recipe that works. **I concluded
the recipe was broken. It was not — my checkout was.** That
misdiagnosis, by a reader who was deliberately verifying the recipe, is
the whole of this row.

## The defect

`docs/DOC-LEDGER.md` recovers a deleted doc with

```
git show <sha>:<path>
```

In a repository cloned with `--depth`, the commit object is absent, and
git reports:

```
fatal: path 'work/gui/log.md' does not exist in 'f955ddc7...'
```

**That message names the PATH, not the missing object.** It is
indistinguishable from the recipe citing a path that was never at that
sha — which is precisely the defect
`ledger-recovery-recipe-misses-renamed-then-deleted-docs` is about, and
which a reader of the ledger is primed to expect. So the failure mode is
not "the recipe does not work", it is **"the recipe reports the other
known defect"**, and a reader confirms a bug that is not there.

Verified on this container, on `origin/main`:

```
$ git rev-parse --is-shallow-repository
true
$ git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:work/gui/log.md
fatal: path 'work/gui/log.md' does not exist in 'f955ddc7...'
$ git fetch origin f955ddc75cda454a268f9214d2a753ae1a9bbd0f
$ git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:work/gui/log.md | wc -c
33562
```

The recipe is correct and the document recovers in full. One `git fetch`
of the sha is the entire difference.

## Why it is worth a row rather than a shrug

**Shallow is the default where this recipe is most needed.** Every
remote agent container in this project starts from a shallow clone, and
those sessions are exactly the ones that cannot ask a human what a
deleted doc said. A recipe that works on a maintainer's laptop and fails
in every container is a recipe for the readers who need it least.

The blast radius is the ledger, not one entry: **every `git show <sha>:`
recipe in `docs/DOC-LEDGER.md` has this property**, including the six
CIW landed citation sites and the two S-TCOST landed at PR 2426, all of
which were written to the house spelling and all of which inherit it.

## The fix

One sentence where the ledger states the recipe: in a shallow clone,
`git fetch origin <sha>` first. Optionally give the recipe in its
self-sufficient form —

```
git fetch origin <sha> && git show <sha>:<path>
```

— which costs nothing in a full clone and removes the failure entirely.
Prefer that: a recipe with a precondition in a neighbouring sentence is
a recipe whose precondition gets copied without it, and this project has
eight copies of the short form already.

## What this row is NOT

Not the rename defect
(`ledger-recovery-recipe-misses-renamed-then-deleted-docs`, filed
2026-09-10). That row is about recipes naming a path the sha never had;
this one is about a correct recipe failing for a reason in the reader's
checkout. **They produce the identical error message**, which is why
this row exists and why fixing this one makes that one easier to
diagnose rather than harder.

Not S-TCOST's: `docs/DOC-LEDGER.md`'s accuracy is META's by
`work/meta/program.md`'s `paths`, and the keep_out's carve-out (sweep
entries belong to the program being swept) does not reach the recipe
itself.

## What I did not check

Whether CI's own checkouts are shallow, and so whether any gate or
script in `scripts/` depends on a `git show <sha>:` that would fail the
same way. Worth one grep by whoever takes this; I did not run it.
