---
id: tree-rs-header-growth-moved-five-cited-subjects
kind: issue
title: Five cited subjects in viewer/src/tree.rs moved when CHROME grew its module header; one of them had its text rewritten
status: open
opened: 2026-09-15
---


Filed by CHROME's `chrome/band-refusal-badging` lane, on VIEW's slate
because the citations are VIEW's to repoint. **No line numbers are
given below on purpose**: both programs measured shift maps naming the
wrong subject about half the time, so this row names SUBJECTS and lets
the owner locate each one. Do not apply an offset to the old numbers —
one of the five did not move by the same amount as the other four, and
a sixth citation that looks like it belongs to this set was already
stale before the change.

## What moved

`chrome/band-refusal-badging` grew `crates/viewer/src/tree.rs`'s module
header — the `MateFault::Band` carve-out needed a section of its own,
because the header's claim that *"every `MateFault` arm but `Band`
names its subject"* was false. Everything below the header moved.

Four citations were CORRECT before the change and name a subject that
still exists unchanged:

| Subject to repoint to | Row |
| --- | --- |
| `RowStatus::badge` (the `pub fn badge` signature) | `work/view/four-badges-five-spellings.md` |
| the `` [`pncad::document::ClassAdmission`] `` link in `TreeRow::note`'s doc | `work/view/renderer-free-cross-crate-links-are-ungated-off-the-seed-set.md` |
| `Node::Measure`'s arm in `node_kind` | `work/view/the-gui-shows-no-measure-value-and-no-clearance.md` |
| the `` [`crate::app::indeterminate_wording`] `` link in `downstream_wording`'s doc | `work/view/named-not-linked-is-a-silent-disposition-at-eleven-of-thirteen-sites.md` |

## The fifth is not a shift — the text was rewritten

`work/view/comment-symbol-names-outside-rustdocs-reach-have-no-gate.md`
cites the **unlinked `SolvedPoses::placement` mention in a comment**
inside `blamed_mates`. That comment survives as an instance of the
class the row is about — a type-qualified symbol named in a `//`
comment, outside rustdoc's reach, ungated — but **its sentence is not
the one the row read**. The arm it sat in was split in two (`Band` and
`PosesOfAnotherDocument` shared one arm and one comment, which is what
let a live carve-out read as settled), and the surviving comment now
reasons from DI3 rather than from the fault map. So this one wants a
re-read, not a repoint: the citation still lands on an instance, and
the row's census of instances may or may not still be the right size.

## Three that look like they belong here and do not

Named so nobody attributes them to this change and so the count above
is not read as the whole population. **Each is its owner's, not
VIEW's, and none is caused by this diff** — all three were already
wrong on `origin/main` before it:

- `work/ciw/ci-draw-can-hide-a-compile-break-on-main.md` cites a line
  for the `MateFault::Unleverable` arm that was already several lines
  short of it on `main`. **CIW's.**
- `work/msolve/mate-memo-key-does-not-carry-the-solve.md` cites a line
  range for *"take the first blamed mate the fault names"* — that
  range held the `Band` arm on `main`, not the first-blamed-mate line,
  which is in `downstream_of_mate`. It names the wrong subject, which
  is the failure this repo's citation rule exists to prevent. **MSOLVE's.**
- `work/msolve/mate-fault-subject-spelled-in-three-crates.md` cites a
  pre-split line range for `blamed_mates`; that row already says so in
  its own text and needs nothing from here. **MSOLVE's.**

## Why this is a file and not a sentence in a PR

`work/README.md`: *"a residue a lane discloses inside its own prose
reads as a record of work done, not as an open thread."* The shift was
disclosed to the CHROME orchestrator in a lane report and nowhere in
the tree, which reaches VIEW not at all.

Signed: (CHROME implementer lane, `chrome/band-refusal-badging`)
