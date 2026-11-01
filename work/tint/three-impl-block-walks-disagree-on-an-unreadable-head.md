---
id: three-impl-block-walks-disagree-on-an-unreadable-head
kind: issue
title: Three impl-block walks spell the same keyword-to-head walk and disagree on an unreadable head
status: open
opened: 2026-09-24
priority: P3
cost: D
---

## What

Three censuses each spell the walk "every whole-word `impl` in a
`code_only` view → `test_utils::source::item_body` → `impl_head`", and
each decides for itself what an unreadable block is:

- `crates/pncad-py/src/tests.rs`, the scope walk beside
  `scope_qualifier` (~7340–7390): an unterminated body panics; an
  unparseable head panics ("an `impl` head I cannot read").
- `crates/test-utils/tests/hand_written_impl_census.rs`, `sites_in`
  (~598–650, head via `head_of` ~362): an unterminated head is an
  `Unreadable` verdict only when it names one of that census's traits;
  an unparseable head is skipped (`head_of` answers `None`, and the
  walk `continue`s) — silently, for every trait.
- `crates/topo/tests/certified_enclosure_impl_census.rs`,
  `impl_blocks`: both cases are a collected failure naming
  `file:line` (LANE-4P's fix pass made it fail loud; before that it
  skipped an unparseable head silently).

`test_utils::source` (`crates/test-utils/src/source.rs`) hosts the
pieces (`item_body`, `impl_head`, `ImplHead`) but not the walk, so each
consumer re-spells the loop and its failure policy. `impl_head`'s own
doc says it is shared "because a reader hosted inside one of its
consumers is how the tree got its drift" — the walk around it is the
next instance.

A sibling of the same class: `crates/geom-core/tests/bounds_census.rs`
`where_clause` (~471) finds the first SUBSTRING `where` and answers
`None` if that one is not a whole word, so a head with an identifier
containing `where` ahead of a real `where` clause reads as having no
clause. `test_utils::source::where_at` (added by LANE-4P's fix pass,
used by `collapsed` and the door census) is the whole-word spelling.

## Proposed

One `pub fn impl_blocks(code) -> Vec<Result<ImplBlock, Unreadable>>`
(or equivalent) in `test_utils::source`, with the per-consumer policy
applied by the caller to the `Err` arm, and the three walks moved onto
it; `bounds_census::where_clause` moved onto `where_at`. The
hand-written-impl census's silent skip of an unparseable head is the
one behavioural change, and should be a red where the head could be
one of its traits.

## Found by

LANE-4P's fix pass (PR 3165), review finding S1.
