---
id: held-angle-renames-the-angle-reads-sweep-left
kind: issue
title: The forward reductions into [0, τ) that the angle-reads sweep left are extents, the turn rule's question, not swept site by site
status: open
opened: 2026-11-03
---


Found by `onecopy/angle-reads`' second sweep pass (shapes: `while … ±= τ`,
`/ τ).round()|floor()|ceil()`, `reduce_periodic_centred(`,
`rem_euclid(` on an angle), at production sites not on its row's list.

## What remains

The forward reductions into `[0, τ)` (`reduce_periodic(`, about forty
sites, among them `chord_join`'s arc sweeps, `boxes`' crest offsets,
`shell`'s split, `classify`'s rows) are an EXTENT, the turn rule's
question (`Period::turn`), not a rename near a reference. Not swept
site by site. Sweep them: each is either an extent that belongs on
`Period::turn`'s reading, or a rename into a window from an end, which
is `Period::above` / `Period::at_or_above`.

Four of them are a rename into a window from an end, `lo + (t −
lo).reduce_periodic(P)`, i.e. `[lo, lo + P)`, found by
`onecopy/half-open-angles`' second sweep pass (shape
`+ (x − y).reduce_periodic(`). Each is `Period::at_or_above(t, lo)`,
except that at `Interval` the floor reads a turn wide across the cut,
and the mark decision does not. They are generic in `T`, so repointing
moves decisions at `Interval` and forms at `Sym`. Each wants its own
test outcomes read:
- `geom-brep/src/chart_singularity.rs` `foot` (`[from, from + τ]`, its
  doc says the upper end is reached by rounding; step-import's ray walk
  reads it);
- `geom-brep/src/pair_section.rs` `within_span` (`lo + (t −
  lo).reduce_periodic(2π)`);
- `topo/src/chord_join.rs`'s `place` (`lo + (t − lo).reduce_periodic(turn)`);
- `topo/src/shell.rs`'s moved-wall arc split (`from + (φ −
  from).reduce_periodic(π)`, period π, then a decided walk).

Blind spot of the shapes: a rename spelled through a local helper or a
multi-line loop; `rim_wedge.rs`'s four floors wait on PR 6443.

## Closed by `onecopy/half-open-angles`

- `geom-brep/src/curve_pair/pose/window.rs` `holds_only`'s `copies` now
  starts from `Period::near(t, t0)`. A copy half a turn or more below
  the start meets the span only through a reach past half a turn, so
  the tie side is moot under that. The reach past half a turn is filed
  on cpair
  (`work/cpair/window-copies-miss-a-turn-below-the-start-past-half-a-turn-of-reach.md`).
- `pose.rs`'s slow-point offset and `pcurve_cache/projected.rs`
  `off_period`: no `Period::separation` (orchestrator ruling). A lattice
  distance is the same on either branch at a tie, so the floor's pick
  never shows in `reduce_periodic_centred(p).abs()`, and a `Sym` form
  should not gain mark decisions. `off_period`'s own doc states the
  argument.
- `mesh/src/walk.rs` `unwrap_tie`: argued in its doc. It is a
  two-reference rule (a rename near `prev`, with its tie zone resolved
  toward `anchor`) that `Period::near` does not state.
- `profile/src/sugar.rs` `swept` and `signed_swept`: argued in their
  docs. They are a sketch's own forward and signed turns, whose floors
  carry the exact-fit structural zero, and are not a carrier's rename.
