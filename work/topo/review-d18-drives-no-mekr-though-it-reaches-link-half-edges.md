---
id: review-d18-drives-no-mekr-though-it-reaches-link-half-edges
kind: issue
title: review_d18's hammer drives no mekr, though mekr reaches link_half_edges at twelve splices
status: open
opened: 2026-09-14
---


Found by D107's lane while making the module doc's coverage sentence
true, and filed rather than fixed: it is the same defect the sentence
had for `kemr`, on a different operator, and closing it needs a
different fixture and a four-variant site enumeration.

## What

`crates/topo/src/review_d18.rs`'s module doc said the sweep *"CALLS
every operator that reaches `link_half_edges`"*. D107 narrowed that to
the driven set, because it is false of **`mekr`**:

| operator | reaches `link_half_edges` | driven by the hammer |
| --- | --- | --- |
| `kef`, `kev` (`euler_kill.rs`) | yes, 4 splices each | yes |
| `kemr` (`euler_ring.rs`) | yes, 2 splices | yes (since D107) |
| `mev_line`, `mef_chord` (`euler.rs`) | yes, 16 splices across four executors | yes |
| `split_edge` (`split.rs`) | yes, 4 splices | yes |
| **`mekr` (`euler_ring.rs`)** | **yes, 12 splices across `mekr_cycles`, `mekr_empty_ring`, `mekr_empty_target`, `mekr_both_empty`** | **no** |
| `mfkrh_plug` (`euler_kill.rs`) | no — mints a face surface, adds a face, touches `face`/`loop`/`shell` records only | yes, as evidence, excluded from `LINK_OPS` |

Derived with `grep -n "link_half_edges" crates/topo/src/` and mapping
each call site to its enclosing `fn` — the sweep's blind spot is that it
sees calls, not reachability through a private helper, so an operator
that reached the splices only through a callee would not appear.

So twelve of the row-4 arms' call sites are attacked by nothing in this
file. That is the same shape as S161 and it deserves the same answer:
either an input witness or the written finding that there is none.

## Why D107 did not take it

`mekr`'s mutation phase needs a face carrying a **ring** — the opposite
of what D107's `ops_ring_bridge` leaves behind, since the bridge
consumes the ring. `ops_holed_box` presents one, but reaching it needs
a target/ring pair in two loops of ONE face, which the hammer's
`halves × take(4)` enumeration cannot be relied on to draw, and the
empty-loop variants need a body with an `Empty` loop that no `ops_*`
fixture carries after construction. That is a unit, not a line.

## What it would cost

An O(n²) pair enumeration for `MekrSite::Cycles`, loop-keyed
enumerations for the other three variants, `ops_holed_box` (and a body
with a live `Empty` loop) added to `FIXTURES`, and the exposure table in
`review_d18.rs` re-derived. Fence: `crates/topo/src/review_d18.rs`,
`crates/topo/src/fixtures.rs` — both this program's, Track P's
review-and-fixture sub-lane.
