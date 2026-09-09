---
id: cert1-notes-pr-body-tracked-on-main
kind: issue
title: a lane's PR-body draft is committed on main as .cert1-notes/pr-body.md
status: closed
opened: 2026-09-08
refs: [k-report-baseline-fold-cert1-roster, 1220, 2140]
closed: 2026-09-08
---

Found by unit 7 while reading PR 1220's argument for the
`props_meridian_pole` fold.

`.cert1-notes/pr-body.md` (75 lines) is in `git ls-files` on `main`: a
CERT-1 lane's draft of its own PR description, committed as a stray
dotfile directory rather than posted and discarded. S-CERT is closed.

**Why it is not merely untidy.** This repo's stated convention is that
*"the sanitized/logical documentation of a change lives in the PR
description, not in commit messages"* (`CLAUDE.md`); a second copy in
the tree is a document with no owner, no lifecycle entry in
`docs/DOC-LEDGER.md`, and no rule saying when it goes stale — and it
already has, since it describes `crates/geom-brep/src/props/curved.rs`
as PR 1220 left it and MESH-12 has since rewritten that file. A
dotfile directory also escapes every doc sweep that walks `docs/`.

**Disposition wanted**: delete it (the PR description is the record),
or, if any of it is worth keeping, fold it into
`crates/geom-brep/README.md` or the predicate-dimension audit and
record the deletion in the ledger.

Unit 7 did not delete it: the path is outside its fence and the file's
provenance belongs to whoever closed S-CERT.

## Closed (2026-09-08) — the disposition, and it was `delete`

The row asked for a disposition, not for work, so the disposition closes
it. Ev ruled on 2026-09-08 in PR #2212's thread, one word: *"4. delete"* —
against `work/issues/`, which METER's exit walk had offered as the
genuine last-resort case (`docs/METER-EXIT-WALK.md` §5, third row, and §8
question 4).

`git rm .cert1-notes/pr-body.md` executed in the successor PR that opened
`work/instr/`. `.cert1-notes/` held that one file and is gone from
`git ls-files` with it. `git grep -n cert1-notes` at the commit that
removes it finds no live pointer at the file: four hits in
`docs/METER-EXIT-WALK.md` and two on `work/meter/k-report-baseline-fold-cert1-roster.md`
(both deleted by the sweep), and one row of `work/STATUS.md`, which CI
regenerates on the next push to main. That grep matches the directory
name and would not catch a pointer that spelled the path some other way.
Nothing was folded into
`crates/geom-brep/README.md`: the file described `props/curved.rs` as PR
1220 left it, MESH-12 has since rewritten that file, and the PR
description remains the record of the change per `CLAUDE.md`.

No `docs/DOC-LEDGER.md` entry: the ledger records the lifecycle of
documents the project owns, and this was a lane's uncommitted-by-intent
draft that was never a document of record — it had no ledger row to
close. The walk is the citation.
