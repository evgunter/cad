---
id: one-chord-rule-over-each-boundary-images-speed-bound
kind: unit
title: the chord pass tightens every curved face's boundary by one rule - an image's speed bound per chart axis over the face's lattice step
status: closed
opened: 2026-11-14
priority: P0
cost: H
branch: tesslane/chord-rule
closed: 2026-11-14
pr: 6943
---


Unit 1 of the chart lane (`work/tesslane/plan.md`, The chart lane).

## What it is

A curved face's boundary is chorded finely enough for that face's lattice by one rule: on each half-edge, the chord count is at least the stored image's speed along each chart axis, times the parameter span, over the face's lattice step on that axis. The image's own facts sit with the image.

- **Three facts per image form**, beside `Pcurve::eval` in `geom_brep` (`pcurve_cache.rs`): its value at a parameter, a box over a parameter span, and a speed bound per chart axis over a span. Today `closed_form_span_box` answers `Harmonic` and `FocalSection` only, and the speed bounds live in `mesh::chords` for four forms (`Harmonic`, `IsoLine`, `IsoArc`, `General`) plus the sphere's projected conic (`sphere_uv_speeds`). Owed, each in closed form with its derivation and a row that pins it against the sampled image: `Spiric` on a torus, `FocalSection` on a cone, and `Projected` off the sphere.
- **One tightening door in `mesh::chords`**, replacing the circle arm of `sizing::grid_boundary_step`, `sphere_arm_tighten`, `nurbs_tighten` and `edge_target`'s headroom read. `chords.rs`'s header lists those doors and says nothing enforces the list; the list goes.

## What it is not

No new arm in `trimmed::Lane`, no ring, no routing: the faces that refuse today still refuse, at the same sites. A mesh that moves is re-baselined with its cause named; that nothing moved is never the argument for the change.

## Before it is specced further

Read `crates/mesh/README.md` `[mesh.sizing]`'s rim-chord bullet: it names today's three mechanisms. The rule it states (a rim chords at its face's grid step) is what this unit implements once; the bullet's list is re-worded with the code.

`pcurve_cache.rs` is not this program's ground; the seam is announced on its owner's log at dispatch.

## Unit (2026-11-14, TESSLANE)

**Letter H. Tier: dual review.** It sets the contract between the chord pass and every lane that will read it, on a seam with `geom_brep`, and its content is certified bounds: a speed bound that is wrong in the unsafe direction under-chords a boundary, and believing one takes a derivation and a falsifier, not a reading.

Branch `tesslane/chord-rule`.

**Owed by the unit, beyond the section above:**

- Each new bound arrives with its derivation in the doc beside it and a row that compares it against the sampled image's speed over a spread of poses, red when the bound stops dominating (a bound that only ever grows looser must not pass by being loose: say how tight it is where it can be said). Sketches a designer wrote down from reading, unchecked, for the implementer to derive or discard: a spiric on a torus wall `|u′| ≤ |d|·r / (f_min·(R − r))`, `v′ = 1`; a focal section `|u′| ≤ (1 + |β|)/(1 − |β|)`; a projected image off the sphere, `sup|C′|` over the chart's least lever arm.
- The one door states its rule once, and each retired door's callers read it. Where the one rule asks a different chord count than a door it replaces, the PR says which count is right and why, and re-baselines what moves (D9 replay digests, `docs/tess-budget-data`, tour pins).
- The faces that refuse today still refuse, at the same sites: `reach_probes` stays green unedited. A form the door cannot bound refuses typed where today's doors refuse.
- `sizes_at_headroom` and `edge_target`'s read of `UNPROVED_HEADROOM` are how the chord pass learns a face's target today. The unit leaves one statement of a face's target that both the chord pass and the lane read; it does not change which faces size at the headroom.

## Closed (2026-11-14, PR 6943)

`geom_brep::Pcurve::speed_bound(t0, t1)` is each boundary image's third fact, beside `eval` and `chart_box`: a bound on its speed along each chart axis over a span, or a typed refusal (`NoSpeedBound`). `mesh::chords::tighten` is the one rule that reads it: a boundary chord's chart step is within the face's lattice step on each axis. It replaces three doors, and every lattice is sized at `trimmed::face_target`. No count moved.

The new bounds (spiric wall and cap, focal section, projected on every chart) are outward-rounded and were swept by two reviewers on two heads; no pose with a positive-width span had a speed above its bound. No lane reads them yet: unit 2 is their first reader.

What the rule does not yet do, each in a row: the curved side reads the iso classifier, not the stored image (a dated section on `rectangle-faces-move-to-the-chart-lane`); the trimmed cylinder arm's boundary is chorded to no lattice (`the-trimmed-cylinder-arm-chords-its-boundary-to-no-lattice`); which image a lane reads and what it refuses still has three arms, which units 2 to 4 take; three duplications of a speed or a range inside `geom_brep` are on PCERT.
