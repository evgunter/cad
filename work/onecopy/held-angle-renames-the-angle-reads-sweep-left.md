---
id: held-angle-renames-the-angle-reads-sweep-left
kind: issue
title: Held-angle renames and distances round a period that the angle-reads sweep found outside its list
status: open
opened: 2026-11-03
---


Found by `onecopy/angle-reads`' second sweep pass (shapes: `while … ±= τ`,
`/ τ).round()|floor()|ceil()`, `reduce_periodic_centred(`,
`rem_euclid(` on an angle), at production sites not on its row's list.
That unit retired the exact `Period::near` equivalents it found
(`geom-brep/src/pcurve.rs`'s cylinder fit, step-import `entities.rs`'s
chart step, `chart_singularity`'s `principal`, `recognize_curve`'s and
`winds`' steps). These are left, each for the reason given:

- `geom-brep/src/curve_pair/pose/window.rs` `holds_only`'s `copies`:
  `t0 + (t − t0).reduce_periodic_centred(τ)` is `Period::near(t, t0)`
  except at the tie, where it takes the LOWER copy `t0 − π`, and the
  walk then collects copies upward from it, so the tie side decides
  whether `t0 − π` is collected. Decide whether it should be, then
  repoint (an `Interval` `t` straddling the tie reads a turn wide today).
- `geom-brep/src/curve_pair/pose.rs` (the slow-point offset,
  `reduce_periodic_centred(π)`) and `pcurve_cache/projected.rs`
  `off_period` (any period): a distance round a period other than τ.
  `geom::separation` is τ's; a `Period::separation` (`atan2`-free, e.g.
  through `Period::near`) would take both, if one is wanted.
- `mesh/src/walk.rs` `unwrap_tie`: a rename near `prev` whose tie zone
  (a quarter period either side of the half) resolves toward a second
  reference, `anchor` (the wire-band pole junction). A two-reference
  rule `Period::near` does not state.
- `profile/src/sugar.rs` (`reduce_periodic`, `reduce_periodic_centred`
  of a sketch's turn): a sketch's own signed and forward turns, the
  user's angle, not a carrier's.
- The forward reductions into `[0, τ)` (`reduce_periodic(`, about forty
  sites, among them `chord_join`'s arc sweeps, `boxes`' crest offsets,
  `shell`'s split, `classify`'s rows): an EXTENT, the turn rule's
  question (`Period::turn`), not a rename near a reference. Not swept
  site by site here.

Blind spot of the shapes: a rename spelled through a local helper or a
multi-line loop; `rim_wedge.rs`'s four floors wait on PR 6443.
