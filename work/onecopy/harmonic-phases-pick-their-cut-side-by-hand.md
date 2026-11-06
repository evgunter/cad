---
id: harmonic-phases-pick-their-cut-side-by-hand
kind: issue
title: Three harmonic phases read clear of atan2's cut by a hand-written two-formula choice, one of them logged
status: open
opened: 2026-11-03
---


Found by `onecopy/angle-reads`' `atan2` sweep (its gate lists these as
class `harmonic-cut`, `scripts/gates/atan2-sites.tsv`).

## The finding

The phase of a harmonic `a cos θ + b sin θ` is read clear of `atan2`'s
cut by choosing, by hand, between `atan2(b, a)` and
`atan2(−b, −a) + π`, which is `geom::azimuth`'s `stable_azimuth` spelled
again:
- `geom-brep/src/curve_face/plane_crossing.rs` (`conic_plane_candidates`,
  the phase `phi`), decided LOGGED under `plane_crossing_phase_frame` at
  the caller's band, though either outcome names one angle;
- `geom-brep/src/curve_face/circle_roots.rs` `first_harmonic_root_directions`
  (`base`, `opposite`) with `HarmonicRoot::from`, which then picks
  between the two by `select_le_zero` and renames each by
  `reduce_periodic_centred`;
- `geom-brep/src/props/curved.rs`'s torus chart lift (`sin_v`, `cos_v`,
  "any branch serves").

Each is `geom::Azimuth::of(b, a).near(reference)` for a reference the
site holds (the span's middle in `plane_crossing`, `t` in `HarmonicRoot::from`).

## Fix

Repoint the three at `Azimuth::near`, which decides its cut side outside
the log at `Band::least()` (F31). `plane_crossing`'s logged row
`plane_crossing_phase_frame` goes, which moves K-stat populations: measure
and re-baseline. Lower the gate's table as each goes.

## Since PR 6491 (2026-11-06)

The second site is retired: `first_harmonic_root_directions` and `HarmonicRoot` went with the sphere door's landings, so `circle_roots.rs` holds no hand phase now. Two sites remain:
- `plane_crossing.rs`'s `conic_plane_candidates` phase, now read by the region landings too (`span_plane_meet`) as well as the lane;
- `props/curved.rs`'s torus chart lift.
