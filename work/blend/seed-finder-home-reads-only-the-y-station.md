---
id: seed-finder-home-reads-only-the-y-station
kind: issue
title: the homed seed finder reads center.y, so every z-poled fixture rolls its own scan
status: open
opened: 2026-09-08
---


## The shape

`sweep::test_support::arcs_at` (`crates/sweep/src/test_support.rs:243`)
is the tree's one seed finder for "the circle at radius `r`, station
`y`". Its station read is `center.y` and nothing else
(`crates/sweep/src/test_support.rs:253`), pinned by
`crates/sweep/tests/review_blend4_r4_probes.rs:251`.

That is right for every fixture `revolved_about_y` mints — the axis is
+y by construction — and it is the reason four suites that pole their
bodies along **z** cannot ask the question here and each roll a scan of
their own:

- `crates/sweep/tests/m5_pr5_tilted_cut.rs:153` — `(center.z - 0.5).abs()
  < 1e-12`, then `(radius - 0.5).abs() < 1e-12`. The tightest window of
  the four, and nothing states why it is tighter than the home's.
- `crates/sweep/tests/m5_pr9_boss_union.rs:85` — `(center.z - 1.0)` and
  the radius, both `1e-9`.
- `crates/sweep/tests/review_m5_pr9_boss_probe.rs:77` — the same pair
  against a computed `seam_z`, `1e-9`.
- `crates/sweep/tests/m5_s13_pips.rs:171` — `(center.z - 1.0)` and the
  radius against `slack()` (`:47`, `max(1e3·eps, 1e-9)`), which is the
  only one of the four whose window is eps-scaled on purpose.

None of the four is wrong; each is a fixture selecting among radii and
stations it stated analytically. The defect shape is the one this
program already ruled on for the y-poled half: a caller copying any of
them copies a number nobody chose, and the copies drift — the four
windows here already disagree by three orders of magnitude.

## Two shapes a fix could take

Neither is decided here.

1. **An axis on the home.** `arcs_at(body, r, station, axis)` reading
   `center.dot(axis)` (or an `Axis` enum over the three coordinate
   poles). Cheap, and it keeps ONE scan; but every existing caller grows
   an argument that is `+y` at all but four sites, and a "station" that
   is not a coordinate needs the axis to say what it is measured from.
2. **A station-along-axis door.** Name the rim by its carrier circle's
   own frame — the circle stores a `center` and an `axis` — and compare
   the station along THAT, so the question becomes "which latitude of
   this body's own axis of revolution", with no coordinate premise at
   all. Closer to what the callers mean and it composes with a posed
   body; it needs the body's axis from somewhere, which for a fixture is
   another thing to state.

The real answer may be neither: a rim with a NAME needs no station
(`no-public-rim-arc-selector`, and the names vocabulary
`rim-seed-finders-disagree-on-at-this-radius` points at).
