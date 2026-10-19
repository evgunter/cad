---
id: tangent-arc-ahead-in-band-builds-a-straddling-arc-at-interval
kind: issue
title: paths: a tangent arc whose target is DECIDED on the departure line still builds an arc off the sub-band across — a ~1e16 carrier at f64, an unbounded enclosure at Interval, silently where no later junction reads it
status: closed
closed: 2026-10-19
branch: paths/arc-onto
pr: 5384
opened: 2026-10-19
priority: P1
cost: M
---


Found while writing `arc_onto`'s degeneracy rows (`crates/profile/tests/arc_onto.rs`).

`tangent_arc_geom` (`crates/profile/src/path.rs`, the free fn both `tangent_arc_to` and `arc_onto` call) decides `path_collinear_target` on `across` and, when it is ZERO and `along` positive, proceeds to `tangent_arc`, which builds a line only where its quarter-tangent `across / (|d| + along)` is EXACTLY zero (`crate::is_exact_zero`). The decision and the build read two different facts. At f64 an axis-aligned or exactly symmetric director makes `across` exactly zero, so the two agree; at `Interval` a director such as `.toward(1, 1)` normalizes to an enclosure, `across` straddles zero, the gate decides it collinear, and `tangent_arc` builds an arc with `rho = |d|² / (2·across)` from the straddling enclosure.

Reproduction (passes at f64, refuses at Interval): `At(0,0), LineTo(2,0), Toward(1,1), TangentArcTo(Point(3,1)), LineTo(0,1), LineTo(Start)` through `profile::replay` at `Interval` refuses at step 4: "whether the path turns at this junction is undecided: margin is invalid (NaN or a refused enclosure)". `arc_onto((3,1)).toward(1,1)` from the same leg end refuses the same way, through the same kernel.

The likely shape of the fix: when the gate DECIDES the target collinear ahead, emit the straight segment the decision names rather than re-reading exact zero. At f64 that would turn today's in-band near-collinear tiny-bulge arcs into lines, so it moves stored bits and wants its own rows.

## Corrected scope (PR 5384's review)

The failure is **silent**, and P1, not just a NaN at the next junction:
- `if let Ok(Sign::Zero) = decide(path_collinear_target…)` treated an escalation as "not collinear" and fell through to the arc.
- A decided `Zero` also fell through to `tangent_arc`, which builds a line only on an EXACT zero.

Where no later junction reads the leg, `validate` returns Ok with the arc at Interval: centre `[−inf, inf]`, radius `[0, inf]`, sweep about ±6e-16. Two such paths:
- `arc_onto(Start.arrives_tangent())` along its own chord (`Supplied`, nothing checked): `At(1,1) Toward(1,1) Line(2) LineTo(-1,3) LineTo(-1,-1) ArcOnto(StartArriving)`, at ε 1e-9 and 1e-12, and with start direction `(3,1)` too.
- A `.tangent()` follower after `tangent_arc_to` or `arc_onto` along the ray.

The reviewer's numbers: over 200 random oblique directors, 196 Interval runs closed silently, and at f64, 132 of 200 stored an arc of radius about 2e16 instead of a line.

## Outcome

Fixed in PR 5384 at the root, in `tangent_arc_geom` (`crates/profile/src/path.rs`):
- The collinear reading is a three-way match. `Zero` emits the straight segment the decision names, so the arc is never built. A definite sign builds the arc. An undecided reading escalates (`PathError::Escalated`) instead of falling through.
- `tangent_arc` no longer re-reads an exact zero: it is called only off the line.
- That one change also fixes the f64 ~2e16-radius case.

Rows, red on the previous head and green after, at ε 1e-9 and 1e-12:
- `arc_onto::the_token_closer_along_its_own_chord_is_a_line_at_both_scalars`;
- `arc_onto::a_tangent_follower_after_a_leg_along_its_chord_reads_a_line` (through `tangent_arc_to` and through `arc_onto`);
- the counterexample search `tangent_arc_on_the_ray::a_target_on_an_oblique_departure_ray_is_a_line_at_both_scalars` (random directors and lengths, `CAD_FUZZ_EFFORT`, at f64 and Interval).

The chord-ahead degeneracy row now uses the oblique director. No golden moved: the profile suite is green at every ε, the stored-form dump included.
