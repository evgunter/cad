---
id: tree-rs-header-growth-moved-five-cited-subjects
kind: issue
title: Five cited subjects in viewer/src/tree.rs moved when CHROME grew its module header; one of them had its text rewritten
status: open
opened: 2026-09-15
priority: P4
cost: E
---


Filed by CHROME's `chrome/band-refusal-badging` lane, on VIEW's slate
because the citations are VIEW's to repoint. **No line numbers are
given below on purpose**: both programs measured shift maps naming the
wrong subject about half the time, so this row names SUBJECTS and lets
the owner locate each one. Do not apply an offset to the old numbers —
one of the five did not move by the same amount as the other four, and
three further citations that look like they belong to this set were
already stale before the change (last section).

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

## Five is not the whole population

Three further `tree.rs` citations in the tracker no longer locate their
subject. **None is caused by this change** — all three were already
wrong on `origin/main` before it — and none is VIEW's, so they are not
this row's to carry. Named here only so a reader does not take five for
the total:

- `work/ciw/ci-draw-rows-tree-rs-citation-does-not-locate-the-unleverable-arm`
  and
  `work/msolve/memo-key-rows-tree-rs-citation-now-lands-on-the-opposite-claim`
  — filed on their owners' slates rather than listed here, because a
  residue in another program's row dies when that row closes.
- `work/msolve/mate-fault-subject-spelled-in-three-crates.md` cites a
  pre-split range for `blamed_mates` and **already says so in its own
  text**, so there is nothing to schedule and it gets no file.

## Why this is a file and not a sentence in a PR

`work/README.md`: *"a residue a lane discloses inside its own prose
reads as a record of work done, not as an open thread."* The shift was
disclosed to the CHROME orchestrator in a lane report and nowhere in
the tree, which reaches VIEW not at all.

Signed: (CHROME implementer lane, `chrome/band-refusal-badging`)
