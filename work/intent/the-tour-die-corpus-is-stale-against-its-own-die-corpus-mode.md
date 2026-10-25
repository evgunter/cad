---
id: the-tour-die-corpus-is-stale-against-its-own-die-corpus-mode
kind: issue
title: The committed tour die document is stale against the tour's die-corpus mode, and regenerating it drops the corpus's only AddConstraint and AddValue edits
status: closed
opened: 2026-10-20
refs: [a-placement-names-its-target-space]
closed: 2026-10-22
---


## Finding

`crates/editor-core/tests/corpus/tour/die_composed_tour.pncad` was last
generated before `demos/tour/src/world.rs`'s `place_in_world` moved to
the world pin (INTENT stage 3 C, `56868da313`, `fefb320a43`). Its log
ends in the old placement gesture, `AddConstraint` + three `AddValue`;
`cd demos/tour && cargo run -- die-corpus <file>` now ends it in one
`PinWhereItStands`. So:

- nightly's "the tour die corpus is regenerable byte-for-byte" step
  (`.github/workflows/nightly.yml`, k-lint release-default) is red on
  main (run 39375288176, 2026-10-20);
- regenerating the file drops the registry's only `AddConstraint` and
  `AddValue` edits, and `m4_pr8_corpus::vocabulary_coverage_is_total`
  then fails ("the Band 4 corpus does not cover: edit AddConstraint,
  edit AddValue"): the coverage rests on the stale asset.

`wire/pair-seam-name-order` had to re-spell the file's stored seam
names. It kept the old placement edits and re-spelled the selection
names in place, re-minting the ids after them, rather than regenerating:
regenerating moves the corpus's edit coverage onto placement ground
(D10 hold). The fix is a corpus home for `AddConstraint` and `AddValue`
(or a ruling that the registry need not cover them), then a regenerated
file.

Signed (WIRE lane `wire-seamorder`).

## Skipped by the hygiene batch (2026-10-22)

This is D′1's ground, and D′1 does the fix on its branch
(`intent/dprime1-placement-target-space`, commit `345feb23`). It
re-saves `die_composed_tour.pncad` by the tour's own door, and the
corpus edit roster (`tests/corpus/mod.rs`) drops `AddConstraint`,
`AddValue` and `AddValues` for `SetConstraints`, because every bundle
is written whole. That is the ruling this row asked for, made by the
design. D′1 should close this row when it lands. If it lands without
closing it, check the regenerability step first and then close it.

## Closed (2026-10-22)

D′1 (`a-placement-names-its-target-space`, PR 5592) regenerated the
file from the tour's own `die-corpus` mode (`345feb23`): every bundle
is written whole, so the log holds no `AddConstraint` or `AddValue`,
and the corpus's edit roster names `SetConstraints` in their place
(`corpus::EDIT_KINDS`, with the reason the increments are not listed:
`incremental-constraint-edits-cannot-succeed-on-a-pinned-bundle`).
