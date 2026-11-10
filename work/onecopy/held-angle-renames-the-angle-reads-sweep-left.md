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

Two `f64` folds into a window from an end, found in review of PR 6626,
stay spelled by hand for a reason each:
- `viewer/src/camera.rs` `wrap_angle` (`[−π, π)`, a floor then a fold of
  `+π` to `−π`): `Period::at_or_above(angle, −π)` is the same rule, but
  `Camera::new` folds any finite yaw, and one past `i32::MAX` turns has
  no `Period` copy and would read NaN. Route it once the constructor
  refuses such a yaw (or `Period` names a far copy).
- `editor-core/src/eval/mod.rs` `ComputingTwist::for_node`'s
  `rem_euclid(τ)` on the twist's advanced angle: an exact `fmod` into
  `[0, τ)` of any magnitude, read only as a turn's angle, so no reader
  takes the window's ends.
  An extent-or-rename of this bullet's class; repoint it with the rest.

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

## Closed by `onecopy/edge-ends-turns`

Measured first: production `reduce_periodic(` is down to nine sites, not
about forty; `boxes`' and `classify`'s had already gone. Each is one of
two questions, and each now asks it through `Period`:
- **A rename into a window from an end**, `Period::at_or_above`:
  `chart_singularity::foot` (now `[from, from + τ)`: a point a hair
  behind `from` whose lift rounds onto `from + τ` reads `from`, the same
  point), `pair_section::within_span`, `chord_join`'s `place`, and
  `shell`'s moved-wall split at period π.
- **A forward run between two angles**, the new
  `Period::forward(from, to)` (`[0, P)`, `at_or_above(to, from) − from`):
  `chord_join::oriented_arc` ×2, `chord_arc`'s middle ×2, and `shell`'s
  half-turn test. A `to` equal to `from` runs nothing; `oriented_arc`'s
  whole turn is `ArcEnd::WholeTurn`, stated by its caller, as before.
  Pinned by `forward_runs_from_its_start`.

Stay, each argued at its site:
- `profile::sugar::swept`, a sketch's own forward turn (already argued);
- `editor-core` `ComputingTwist::for_node`'s `rem_euclid(τ)` and the
  viewer's `wrap_angle`. Both are of any magnitude, and `Period`'s renames
  read NaN past `i32::MAX` turns.

Not touched: the four sites of
`a-turn-renamed-parameter-is-placed-against-a-span-at-four-sites`
(the census lane's unit). None of them is a site above.
