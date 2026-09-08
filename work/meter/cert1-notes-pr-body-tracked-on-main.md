---
id: cert1-notes-pr-body-tracked-on-main
kind: issue
title: a lane's PR-body draft is committed on main as .cert1-notes/pr-body.md
status: open
opened: 2026-09-08
refs: [k-report-baseline-fold-cert1-roster, 1220, 2140]
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
