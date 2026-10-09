---
id: a-steep-ellipse-travel-margin-ties-band-apart-sites-and-falls-back-to-the-chord
kind: issue
title: On a steep ellipse the join's travel margin can tie two band-apart sites and fall back to the chord order the margin replaced
status: closed
opened: 2026-10-04
priority: P3
cost: E
refs: [a-pocket-crossing-a-side-face-refuses-at-ring-rehoming-on-a-curved-face, join-ranks-conic-facing-germs-by-chord]
closed: 2026-10-09
branch: join/three-small-join-rows
pr: 4396
---


Found by the fix-pass review on PR 4008 (MINOR 1, inspection, likely).
Unmeasured: no committed battery reaches it.

## What

`crates/topo/src/boolean/join.rs` `nearer_along` ranks one germ's
candidates within a half-turn by `bool_join_arc_travel`, the margin
`turned_past` computes: a candidate's signed distance from the plane
through the conic's axis and the incumbent's site. On a steep ellipse
that distance is not the distance between the two sites. Between the
ellipse's vertices it can be about 5× smaller than the gap between two
sites at aspect k = 10. Two sites a few bands apart can then decide
`Zero`, and `nearer_along` falls through to `nearer`, which is the
chord order, the order that is not monotone on that ellipse.

So the claim that the margin "resolves two sites as finely as the
distance between them does" holds on a circle, not on a steep ellipse.
The defect is at band scale only.

## Done when

A pose with two sites a few bands apart on a k ≥ 6 ellipse is a row.
It either builds sound, or `nearer_along` refuses typed rather than
falling back to the chord. One way to get there: scale the margin by
the conic's local radius so it reads arc length, or make an in-band
travel tie escalate.

## Built

`join.rs` `turned_past` reads the arc: `turn(p − site) / |turn(t̂)|`.
Here `turn_of` is the one spelling of `axis·((p − c) × v)`, which
`rotational_sense` reads too, and `t̂` is the conic's unit tangent at
the site (the germ direction recorded there, carried on `Turn`). That
is the plane distance over `sin ψ`, the arc to first order near the
site, `s (1 + ½ s κ cot ψ)`, and at the half-turn. The axis's length
cancels in the ratio.

The sites are vertices, certified only within ε of their carriers. So
`travel` decides the margin against `off_conic_slack`, `2ε cot ψ`:
- a definite sign must clear it (`bool_join_arc_clear`);
- a tie stands only where the slack is itself within the band
  (`bool_join_arc_slack`);
- anything else escalates.

The slack is zero on a circle and `ε (k² − 1)/k` at an ellipse's
flank. `nearer_along` (`bool_join_arc_travel`) and `germ_arm`
(`bool_join_arc_ahead`) both read through `travel`. Two records at one
site still take the chord.

The rows:
- `join.rs` `travel_rows`, on synthetic germs.
  - `the_partner_nearer_along_the_walk_is_taken_where_the_chord_says_the_other`
    poses two partners on the lower flank of a `k = 10` (24 bands
    apart) and a `k = 60` (100 bands) ellipse, where the chord from the
    germ prefers the far one, and asserts the near one is taken from
    either side.
  - `two_sites_within_the_slack_of_a_steep_flank_escalate` asserts 24
    bands at `k = 60` escalate at `bool_join_arc_clear`, and that a
    circle orders the same gap.
  - Mutants: main's plane-distance margin, the order flipped, the
    incumbent always kept, and a forced tie falling back to the chord
    each turn the first row red.
- `crates/sweep/tests/band_apart_partners_on_a_steep_ellipse.rs`,
  through the public booleans: a cylinder leaned to `k = 10` (notch and
  finger 24 bands wide) and `k = 60` (12 bands) through a block whose
  top face has a notch and a finger on the ellipse's flank, every op in
  both orders.

## Measured

At ε 1e-9, 1e-6 and 1e-12 (the pose is sized in bands, so the readings
are the same in bands at each):

- `k = 10`: on main every run escalated at `bool_join_arc_travel`,
  measured with 12-band gaps at −4.78 bands. With the row's 24-band
  gaps the plane distance computes to 9.5 bands, still inside the
  escalation band; that was not measured on main.
  Now the travel clears the slack of 9.9 bands, and every run stops at
  the next reading, `bool_join_nearest`. That is the order between two
  pairs on the notch's two parallel walls, whose chords differ by a few
  bands. The evidence is added to
  `a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale`.
- `k = 60`: the old margin tied `Zero` (0.8 bands) and fell back to the
  chord, and every run then refused at the certifier's span check,
  `IntervalNotForward`. That check meters the finger's `12ε` section
  edge at the ellipse's minor semi-axis, `12ε·√2/60`. Filed as
  `work/issues/an-ellipse-span-is-metered-at-its-minor-axis-and-refuses-a-flank-edge-k-times-longer-than-the-band.md`.
  Now 24 bands of arc do not clear the slack of 60, and every run
  escalates at `bool_join_arc_clear`.

The row asserts every run builds `SOUND` or stops at those doors.
