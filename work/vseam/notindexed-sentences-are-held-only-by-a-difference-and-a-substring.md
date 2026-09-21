---
id: notindexed-sentences-are-held-only-by-a-difference-and-a-substring
kind: issue
title: NotIndexed's three refusal sentences are held only by a difference check and a substring
status: open
opened: 2026-09-21
---

priority: P3
cost: E
---


## What

`NotIndexed`'s `Display` (`crates/viewer/src/pickcache.rs`, the
`impl core::fmt::Display for NotIndexed`) writes the three sentences a
reader is shown when a click is refused for want of an index. The only
rows that read them are in
`a_click_with_no_index_refuses_typed_and_a_hover_stays_quiet`
(`crates/viewer/tests/frame_policy.rs`), and between them they assert
two things:

- `Building.to_string() != Absent.to_string()`;
- each of the two contains the substring `"index"`.

`AnotherPicture`'s sentence is read by neither.

**So a wrong sentence passes.** Found by mutation while closing
`work/view/the-quiet-seam-half-of-pickcache-indexing-has-no-shipped-producer`:
that unit DELETED a clause from `Absent`'s sentence — *"or the index
seam has stopped answering"*, a cause the panic ruling made
unreachable — and the suite was green before and after. Replacing the
whole sentence with `"not picked: mutated sentence"` was what finally
reddened a row, and it reddened it for lacking the word `index`, not
for being wrong.

## Why it is filed rather than fixed

The unit that found it had a reason to change the sentence and no
mandate to decide how user-facing text in this crate should be held.
That is a question with a population: these three sentences are one
instance, and `EditError`'s `Display` and the badge and status-line
vocabularies are the neighbours to sweep before choosing a shape. A
guard per sentence is the obvious answer and is probably the wrong one
— it would pin wording nobody wants frozen.

## What it would cost to measure

Nothing to measure. The blind spot is visible by reading the two
assertions against the three arms.
