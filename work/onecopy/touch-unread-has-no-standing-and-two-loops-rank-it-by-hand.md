---
id: touch-unread-has-no-standing-and-two-loops-rank-it-by-hand
kind: issue
title: TouchUnread states no standing, so census's germ gather and link::judge each rank its refusals by hand
status: closed
opened: 2026-11-06
priority: P2
cost: M
closed: 2026-11-06
pr: 6553
---


Found by `kept-refusal-loops-the-ladder-row-never-listed`'s sweep
(`onecopy/refusal-standing`).

## What

`link::TouchUnread` (`crates/topo/src/link.rs`) has no standing: no
`refusal()` beside its variants (`ladder`'s module docs). Two loops read
it, and each ranks its refusals by hand.

- **`census::Germ::of_holds`** (the face loop keeping
  `refused.get_or_insert(v)`). Every face is needed and no reading
  answers: the ladder's `First`, as `vtxfac::faces_holding_pierce` climbs.
  `Unreadable` returns at once, and every other variant is kept
  first-of-any-kind, in face order.
- **`link::judge`** (the sector loop keeping `refused.get_or_insert(v)`
  from `Link::pieces`). A `Crossing` answers past the kept refusals. Where
  none does, the ranking is spelled inline: the pieces' first refusal of
  any kind, then a `Degenerate` note (a limit), then an `InBand` note.

## What closing it takes

1. `TouchUnread::refusal`, beside its variants. Under the ruling on
   `a-refusals-standing-has-two-return-types`, it returns
   `Result<Refusal, Aborts>` if a variant refuses the whole query (does
   `Unreadable`, which `of_holds` returns at once?), and `Refusal`
   otherwise. Each variant's class comes from what it means: `InBand` and
   `PieceInBand` in band, `Degenerate` and `ChordScale` limits;
   `Unanalysed` is to be argued.
2. Port both loops: `of_holds` onto `climb_evidence` (`First`), and
   `judge`'s pieces and notes onto one `Evidence`.
3. **Both ports move rankings**, from first-of-any-kind to `Evidence`'s
   order (limit, then in band, then unsettled). That is reachable wherever
   a face or sector kept in band precedes one kept as a limit. Measure the
   reach and pin each change as PR 6536 did, for a ruling.
