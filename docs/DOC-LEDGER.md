# docs ledger — deleted historical documents

This is the permanent index of documents deleted from `docs/` and — since
sweep 5 — of closed programs' directories deleted from `work/`. It replaces
`docs/archive/`, whose method was to **move** dead documents aside and index
them; the method now is to **delete** them and record where they are
recoverable. Git is the archive — the repo is merge-only and never rewrites
history (`memories/git-workflow.md`), so every file the ledger names is still
reachable, byte-for-byte, at the commit its note gives.

Nothing the ledger records is normative, and nothing it records was normative
when it was deleted. The living contract is `docs/DESIGN.md` plus the
companion design docs its table lists.

**This page is the convention; the entries are `docs/doc-ledger/`, one file
each.** It does not list them, deliberately: every program appended to this
file at the same anchor at the end of every unit, which made it the
highest-collision file in the tree, and a list here would put that back. A new
entry is a new file.

## What a note contains, and what it does not

A note is a **pointer**: the document that was deleted, one line saying what it
was, the PR, and the command that recovers it. Nothing else. The account of
what a document got right, what it got wrong and what the unit built instead
lives where the tracker puts it — the PR body, the unit's item file and its
program's `log.md` (`work/README.md`) — and a fourth copy here only rots. Where
a program has since left the tracker, its `log.md` and its exit walk are
recoverable at that sweep's SHA, which its sweep note gives.

**A document that nobody would want to read later gets no note at all.** The
ledger exists to point at archival material someone might actually want; a spec
the tracker records as *wrong* is not that (Ev, 2026-09-22). A spec that was
superseded but sound, or that records a design that shipped, keeps its note.
Nothing is lost either way: this file's own git history holds every entry as it
was written, verbatim, and history here is permanent —

```
git log -p --follow -- docs/DOC-LEDGER.md
```

## Naming

```
docs/doc-ledger/sweep-05.md              a sweep, zero-padded
docs/doc-ledger/tint-4-spec.md           a per-merge deletion, the document lowercased
```

A sweep note is `sweep-NN.md`, so a citation reading "DOC-LEDGER sweep 5" lands
on `sweep-05.md`. Sweep numbers 6 and 7 were each used by three distinct
sweeps; those three share the one file, each as its own section, so that the
number still resolves to exactly one place. Every other note is named after the
document it points at, lowercased: `TINT-4-SPEC.md` → `tint-4-spec.md`.

## Recovering a deleted file

```
git show <sha>:docs/<NAME>            # print it
git show <sha>:docs/<NAME> > /tmp/<NAME>   # restore a copy
git log --diff-filter=D -- docs/<NAME>      # find the deleting commit
git log --all --full-history -- docs/<NAME> # ...and if that is empty, this
```

**Check the last one before concluding a document is gone.** A file this ledger
does not name may have been MOVED rather than deleted (the tracker migration
moved every plan and log out of `docs/`, sweep 4), and `--diff-filter=D` does
not see a rename. `git log --follow --stat` from either path prints the move and
whether the content changed with it. A citation to a path that no longer exists
is evidence of nothing until that has been run: the first reader to follow
`docs/PERF-PLAN.md` filed it as a document deleted without a record, and it had
been `work/perf/plan.md`, byte for byte, since the day it left.

Files listed under sweep 1's `docs/archive/` group need that prefix in the path:
`git show <sweep-sha>:docs/archive/<NAME>`. The tracker directories the sweeps
from 5 onward deleted take their own prefix — `git show <sweep-sha>:work/<program>`
lists one, and `git show <sweep-sha>:work/<program>/log.md` prints a file from
it. Every deleted path is still greppable across history with
`git log -S<string> --all`.

## A note on inbound references

Some surviving documents and a number of source comments cite files deleted
here — mostly "see also" pointers in append-only logs, which state what was true
when written and are not edited in place. Those citations are not broken: the
filename plus the recovery recipe above resolves any of them. No file was
deleted that a *live* pointer depends on for its content.
