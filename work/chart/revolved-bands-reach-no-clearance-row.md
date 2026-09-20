---
id: revolved-bands-reach-no-clearance-row
kind: issue
title: No revolve replays at Interval over an epsilon box, so a revolved band never reaches window_of
status: open
opened: 2026-09-14
refs: [clearance-window-tightening-needs-chart-boundary]
---



## What

TRIM-3 PR-2 tightens a clearance carrier window to the face's chart
boundary for planes and cylinders. **Every cylinder its suite reaches
is extruded**, because no revolve on this tree replays at the interval
scalar over an ε-scaled analysis box: both M10-5 R1 revolve fixtures —
the y-axis quarter annulus and its z-axis control — refuse at the
SELECTION door (`node did not build in this leaf's replay, so it has no
faces to measure a clearance between`), so `ClearanceReport::refused`
mints its counts before `window_of` runs and no face of a revolved body
is ever windowed, described, cut or dropped.

Measured, both reviewer arms of PR-2's v6 dual independently: the two
R1 rows print `windows (0, 0)`; R1 already carried the skip as a
printed note and PR-2 tried to turn the z-axis control into an
assertion and had to put it back
(`m10_5_r1_probes_interval::a_partial_revolve_about_z_is_the_control_for_the_hulled_band`).

## What this is NOT

This file replaces `negative-revolve-band-has-no-e2e-row`, filed by
PR-2 and **refuted before it merged**. That file claimed a negative
azimuth band is minted by exactly one head constructor — a
negative-angle revolve — and therefore that the `[0, τ] ∩ hull` mutant
on the cylinder root rule had no reachable e2e row. Both halves are
false. The walk pins a loop's branch from its first half-edge's
PRINCIPAL azimuth in `(-π, π]`, so any arc of a shared carrier whose
first half-edge starts past `π` goes negative: a
`LoopProgram::CircleSplit { n: 4, phase: -π/4 }` extruded peg gives
wall 2 the band `[-π/2, 0]`, measured from `window_of`. That row now
exists
(`trim_3_windows_interval::a_negative_band_is_not_intersected_with_the_canonical_turn`)
and the rule is decided directly by `clearance::root_rule`'s unit rows.

## What is still uncovered

The bands a revolve mints and an extrude cannot: a partial revolve's
cylinder at an arbitrary axis (including the sign-hulled `u_ref` case),
and every cone, sphere and torus window, which only a revolve produces
and which `work/trim/clearance-window-cone-sphere-torus.md` is about
describing at all. As long as no revolved body replays at `Interval`,
those carriers have no clearance coverage of any kind — tightened or
loose — and a reader should not read PR-2's green suite as saying
anything about them.

## Fix shape

Not this program's: the replay failure is issue 1191's class (no node
builds at `Interval` over a box wider than about `ε/64`, and the
revolve does not build over one that narrow either). The clearance-side
work is one row per carrier the day a revolve replays, and it is
cheaper than it looks because the fixtures already exist in
`m10_5_r1_probes_interval`.

## Home

TRIM filed it from the clearance seam; the replay failure's ground is
the evaluation lane's, and the rows that would land here are this
program's.
