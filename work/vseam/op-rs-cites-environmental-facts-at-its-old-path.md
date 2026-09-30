---
id: op-rs-cites-environmental-facts-at-its-old-path
kind: issue
title: CancelDoor's doc comment cites work/view/environmental-facts-... which now lives at work/vnews/
status: open
opened: 2026-09-19
priority: P4
cost: E
---

Filed by a VNEWS census lane (`work/vnews/a-disabled-control-says-why-
in-four-shapes`) at merge base
`2654cc111417da806d9786c40136106469096fec`. An announced crossing:
`crates/viewer/src/session/op.rs` is VSEAM's ground, so this is filed
and not fixed. **Nothing under `crates/` was touched.**

## The citation

`crates/viewer/src/session/op.rs`, in `CancelDoor`'s doc comment (the
paragraph that draws the precedent split between a `&'static str` on
`on_disabled_hover_text` and a `Refusal` rendered by the control):

> … which is the shape
> `work/view/environmental-facts-answer-usable-as-a-bool-with-the-
> reason-elsewhere.md` is open about.

That row moved to `work/vnews/` in VIEW's re-scope of 2026-09-17, by
`git mv` with its body and history unchanged. The path in the comment
resolves to nothing on this tree:
`work/vnews/environmental-facts-answer-usable-as-a-bool-with-the-reason-
elsewhere.md` is where it is.

## Why it matters more than a stale path usually does

The comment is one of the nine sites that state the disabled-control
rule's first limb, and the row it points at is the one the census had
to rule *disjoint* from that rule — so a reader following the citation
to decide whether the two classes overlap is the exact reader this
breaks. The row is also still open, so the pointer is not merely
historical.

## The sweep, and the class it found

`grep -rno 'work/view/[a-z0-9-]*' crates/`, with each hit's id checked
against `work/view/` on this tree: **26 citations are stale, in 13
files across 5 crates**, and only 6 still resolve. This is not one
rotted path; it is the 2026-09-17 re-scope's `git mv`s and the
`props` cut leaving every in-tree pointer behind. The class has its own
row, `work/vdoc/crates-cite-work-view-rows-that-moved-in-the-rescope`,
with the full hit list. This row is the one instance on VSEAM's ground,
kept separate because the instance is a live pointer in a doc comment
that states a rule a census just ruled against.

**What the pattern could not match**: a citation written as a bare item
id with no directory (nothing distinguishes one from prose), a comment
naming a row by its title rather than its path, and a path **wrapped
across two comment lines** — this very citation is one, which is why
the raw grep reports its id truncated at `…-with-`.

## Home

VSEAM's: `crates/viewer/src/session/op.rs` is in that program's
`paths`. The VNEWS census that found it repoints nothing across the
fence.
