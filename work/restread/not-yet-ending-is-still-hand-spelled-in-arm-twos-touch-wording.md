---
id: not-yet-ending-is-still-hand-spelled-in-arm-twos-touch-wording
kind: issue
title: The not-yet ending is still hand-spelled in arm 2's touch wording and the census's other refusal sentences
status: open
opened: 2026-10-26
priority: P4
cost: E
refs: [tier-3-prime-reads-no-side-between-coincident-edges-on-one-line]
---



Found by PR 5529's second verifier.

## What

PR 5529 gave the refusal ending "There is no way through yet for this
shape" one home: `geom_core`'s `not_yet_ending!()`, from which
`NOT_YET_ENDING` is defined. Only `TouchUnread::sentence` and
`census::sentence_on_line` end through it. The same ending is still
spelled by hand in production code:

- `census.rs` `Undecided::what` (`:6253`–`:6332`), ten arms, among
  them the touch arms (TouchUnreadable, TouchDegenerate,
  TouchPieceInBand, TouchInBand, TouchUnanalysed) that are arm 2's
  parallel wording of `TouchUnread::sentence`;
- `census.rs` `FLUSH_UNREAD` (`:10408`);
- `boolean/refusal_routes.rs:2157`;
- `validate.rs:12340`, `:12351`.

A sweep for the literal on one line misses wrapped literals and the
"for a designed …" endings, which is how these were left.

The same PR's `census::tests::wedge_unread_is_every_reason_wedge_gives`
reads only the body of `fn wedge`. A `WedgeRefusal::Unread(_, X)`
built elsewhere (`line_wedges`, `strictly_inside`, a helper) leaves it
green while `WEDGE_UNREAD` goes stale.

## Owed

End every such sentence through `not_yet_ending!()`, with the rendered
bytes unchanged. Have arm 2's touch arms render through
`TouchUnread::sentence` rather than keeping their own wording. Widen the
`WEDGE_UNREAD` pin to every `WedgeRefusal::Unread(` construction in the
file.
