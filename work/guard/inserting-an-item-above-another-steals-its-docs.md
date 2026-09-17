---
id: inserting-an-item-above-another-steals-its-docs
kind: issue
title: inserting a type above an existing one silently moves the existing item's doc comment onto the new one
status: open
opened: 2026-09-05
---


Third instance this session, found by the #1953 style review:
`PartCensus` was written directly above `PartChooser` and took the
first paragraph of `PartChooser`'s doc comment with it. The earlier two
were `draw_badge`/`chrome` and `Disagreement::notice`/`disagreement`.

## What happens

A doc comment binds to the NEXT item. Insert a type between an existing
`/// …` block and the item it documents, and the block silently becomes
the new type's — rustdoc publishes the old item's summary as the new
one's, and the old item is left opening on whatever paragraph the split
happened to leave behind. Both renderings are plausible prose, so
nothing reads as broken:

- the new type gets a summary about something else;
- the old type gets an orphan sentence, usually mid-argument.

`cargo doc` is silent. Clippy is silent unless the leftover happens to
be an empty line before the item (`empty_line_after_doc_comments`),
which is why one of the three instances was caught by a warning and two
were not.

## Why it recurs

Every instance came from the same motion: writing a value beside the
consumer that takes it, at the point in the file where the consumer is.
The natural insertion point is exactly the wrong one, and the diff
looks correct — the new item is complete and the old item is untouched.
What moved is the boundary between them, which no hunk shows.

## What resolving it looks like

**A grep is not enough.** The shape is "a doc block whose prose is
about an item other than the one it precedes", which is semantic. Two
mechanisable proxies are worth costing:

- **The first-paragraph/name mismatch.** `crates/viewer/src` names
  items and their summaries in a house style regular enough that a
  summary naming a DIFFERENT item in this crate, in backticks, above
  an item it does not name, is a strong signal. Cheap, noisy, and
  probably worth it as a warning.
- **The rustdoc JSON output.** `cargo doc --output-format json`
  carries each item's docs; a check could compare a doc block's first
  backticked identifier against the item it landed on. More faithful
  and much heavier.

The gate that already catches this ONE LEVEL UP is
`scripts/gates/viewer-module-kinds.sh`'s check 2: a module header that
contradicts what the module does is refused, because rustdoc publishes
it. The same argument applies to item headers and nothing enforces it.
