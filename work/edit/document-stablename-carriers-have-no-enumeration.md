---
id: document-stablename-carriers-have-no-enumeration
kind: issue
title: Which carriers hold a StableName is answered in four places and enumerated in none
opened: 2026-09-16
status: open
refs: [2784, stranded-appearance-keys-are-not-reported-by-dm7]
---

(Found by the style review of PR 2784, which added the document's
SECOND `StableName` carrier — the appearance store — to DM7's report.)

## The finding

`Node::payload_names` is the one exhaustive answer to "which PAYLOADS
carry a name", and a new payload kind that forgets it fails a census.
There is no such answer to the wider question — **which of the
document's own fields hold a `StableName` at all** — and four sites
now spell that list by hand, each in its own order, each a place a
third carrier can be forgotten silently:

- `edit.rs`, `stranded_names` and `stranded_appearance_keys` — DM7's
  report, the payload walk and the store walk, adjacent and
  independent. A third carrier is a third function nobody is told to
  write.
- `refactor.rs`, `part_names_are_self_contained` — the split door's
  containment check: the `payload_names` loop and the
  `doc.appearance().keys()` loop, one after the other.
- `refactor.rs`, `inline_part`'s foreign-name classification — the
  same two loops again, for the other refactoring door.
- `persist/check.rs`, the snapshot validator — `payload_names` inside
  the per-node walk, `doc.appearance.keys()` in its own pass much
  further down, far enough apart that neither reads as half of one
  list.

Four hand-written spellings of one list. Nothing fails if a fifth site
spells three of four, and nothing fails when a new `Doc` field holds a
`StableName` and no site is updated at all.

## The shape

One named answer to "which carriers hold a `StableName`", exhaustive
the way the census's `Walk` became on PR #2780: a `Doc` field added
without being placed does not compile. Each of the four sites above
then reads that answer rather than re-deriving it, and the third
carrier arrives at one edit instead of four.

Note the two carriers are NOT the same shape — a payload name has a
carrying node and a store key has none, which is exactly why DM7's
report has two arms — so the enumeration has to carry that difference
rather than flatten it. That is the design question inside the
mechanical one, and it is why this is a row rather than a sweep.

## Why it was not built at PR 2784

That unit adds the second carrier and is the change that makes the
list worth naming; building the enumeration in it would be a fourth
site's worth of refactoring riding a clause implementation. Filed at
the moment it was disclosed.
