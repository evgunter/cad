---
id: ledger-notes-have-no-guard-that-their-recovery-command-resolves
kind: issue
title: Nothing checks that a doc-ledger note's recovery command resolves
status: open
opened: 2026-09-22
---


A note in `docs/doc-ledger/` is a pointer, and its whole job is the one
line that recovers the deleted file:

```
Recover with `git show <sha>:<path>`.
```

**Nothing runs it.** `scripts/work.py lint` reads `work/` and says
nothing about `docs/`; no CI row opens a note; the gates under
`scripts/gates/` have no ledger arm. A note whose SHA is wrong is
indistinguishable from a note whose SHA is right, and it fails at the
one moment it exists for — when a reader needs the file back.

## The instances, and they were found by hand

Of the 144 notes migrated out of `docs/DOC-LEDGER.md` into
`docs/doc-ledger/` on PR 3063, **three name a commit at which the file
was already deleted**:

| note | cited SHA | the file at it |
| --- | --- | --- |
| `docs/doc-ledger/tint-3-spec.md` | `da1b20f85` | gone |
| `docs/doc-ledger/tint-4-spec.md` | `5494b9927` | gone |
| `docs/doc-ledger/tint-5-spec.md` | `8e7bcc02b` | gone |

All three were carried verbatim from the old single-file ledger, where
they are equally broken on `main` today, so the migration did not
introduce them. The cause is one habit: S-TINT's entries named "the
fix-pass head" as the recovery SHA, and each spec was deleted **in that
same fix pass**, so the commit the entry points at is the first commit
without the file. PR 3063 repoints them at `387600f79`, `72555a2fb` and
`82f956582`, each verified to hold the file and to be an ancestor of
`origin/main`.

They were found by an audit run by hand on the branch, not by any
check. That is the defect this row is about: a second reader happened to
look. The same audit ran all 139 recovery commands in the directory:
the other 135 resolve, and one is unverifiable here for the clone reason
below.

## The check, which is cheap

For every `` `git show <sha>:<path>` `` in `docs/doc-ledger/*.md`, run
`git cat-file -e "<sha>:<path>"`. It needs no build and no network.

**The one class it must not fail on is a commit the clone does not
have.** `docs/doc-ledger/topo-d265-spec.md` names
`7253e5efb66c479aa27c7381b878be0d1a2bd52a`, whose commit object is
absent from a shallow checkout; `git cat-file -e <sha>^{commit}` tells
that case apart from a path that is genuinely gone at a commit the clone
does hold, and only the second is a failure. A check that cannot make
that distinction would red `main` on CI's fetch depth rather than on
anything about the tree.

The sweep notes' template lines (`git show <sweep-sha>:work/<program>/<FILE>`)
carry placeholders and are not resolvable commands; the check keys on a
literal hex SHA.

**It must not key on the backticks.** A sweep note's concrete pointers
sit in an indented block with no backticks around them, and eighteen of
the directory's 139 runnable commands are of that shape — the exit-walk
and design-doc recoveries in `blend-`, `cite-`, `docm-`, `door-`,
`eval-`, `fillet-`, `fix-`, `gates-`, `m10-`, `meter-`, `s-bool-`,
`s-cert-`, `s-mate-`, `s-mesh-`, `seat-` and `verbs-leaves-the-tracker`.
A pattern requiring a closing backtick reads 121 of the 139 and reports
a clean sweep over the rest, which is the failure mode this row exists
to prevent: a check that passes because it did not look.

## Disposition

This row proposes the check; it does not write it. `docs/doc-ledger/`
is META's by `work/meta/program.md`'s `paths`, so a META unit takes it.
Where it belongs is the question the unit answers — a `scripts/gates/`
member, or an arm of `scripts/work.py --selftest`'s neighbours — but it
is a docs-tier input, so it must run in a job the docs tier reaches.
