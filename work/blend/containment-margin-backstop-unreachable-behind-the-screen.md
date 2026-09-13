---
id: containment-margin-backstop-unreachable-behind-the-screen
kind: issue
title: fillet: the ring-clearance CONTAINMENT margins were filed as unreachable behind predicate 2's screen; the dual falsified both halves and the rows landed on PR 2215
status: closed
opened: 2026-09-08
pr: 2215
closed: 2026-09-08
---


Filed by BLEND-6's Phase 2 while writing the two refusing rows its spec
asks for (`docs/BLEND-6-SPEC.md` §4, "a REFUSING row per form ... each
asserting `RingClearance` with a negative reading"), on the claim that
neither row could assert that. **Both halves of that claim are false**,
and this file records the corrected measurements rather than the ones it
was opened with. Closed on the same PR that opened it.

## What was claimed, and what is true

**Claim 1 — "the containment margins are bit-identical to predicate 2's
screen on every coaxial pair." FALSE.** The two compute the same REAL by
different associations, and the doubles differ. Measured on this PR's
head with the screen disabled locally (`face_clearance` short-circuited
to `Ok`), reading the value each predicate actually delivers:

| pair | screen's double | exact backstop's double | apart |
| --- | --- | --- | --- |
| ladder, flat top narrowed to `0.55`, dome rim at `r = 0.1` | `-0.041607978309961546` (`0x…1bd0`) | `-0.04160797830996166` (`0x…1be0`) | **16 ulps** |
| annulus, dome grown to `0.92`, top rim at `r = 0.1` | `-0.020000000000000018` (`0x…1480`) | `-0.020000000000000018` (`0x…1480`) | 0 |

The screen's `gap - setback_here - setback_there` associates as
`(rr - a) - (si - a)` where the closed form associates as `rr - si`; on
the ladder pair those round differently. The original file's own table
compounded the error by putting DECIMAL LITERALS in its "derived"
column: `-0.02` is 5 ulps from the screen's double and `-0.05` is 10
(only row 1 was quoted at full precision, and that row is the one that
matched). The rows now assert `to_bits()` equality against a derived
expression and say whose value they are checking
(`ring_clearance_forms::a_ladder_boundary_nested_inside_its_trim_circle_refuses`,
`…::a_hostless_annulus_ring_in_the_excised_strip_refuses`).

**Claim 2 — "the exact backstop has no reachable fixture." FALSE, and
the construction this file proposed is not the one that works.** It
named "a pip cut into the top of a pole-touching revolve"; that route is
BLOCKED, but a different one is open:

- **The boolean route is blocked.** The split-join refuses every
  off-axis pierce of a cylinder cap with `Join(SectionLoopMixed)` except
  at azimuth 0 and π, and at those two the ring's closest approach to
  the rim lands ON a sample of predicate 2's lattice, so the screen is
  exact and answers first. Measured across seven azimuths and rowed as
  `review_ring_clearance_r2_probes::the_boolean_route_to_the_exact_containment_backstop_is_blocked`
  and `review_ring_clearance_r1_probes::r1_diag_cylinder_pierces`.
- **The EXTRUDE route is open.** A bored cylinder — one extrude of a
  profile whose outer loop is the unit circle with its vertices at
  `11.25°` and whose inner loop is a circle of radius `a` centred at
  `(d, 0)` — misaligns the two rims' sample lattices by construction.
  The bore's nearest point to the outer rim sits at azimuth 0, which is
  on no sample of `11.25° + k·22.5°`, so the sampled gap is strictly
  larger than the true one and `fillet3_ring_clearance` takes the
  decision. It reads `-0.01` at the front door for BOTH new relations,
  with the carving sides carving tier-3 valid:
  `review_ring_clearance_r1_probes::r1_a_bored_cylinders_off_axis_ring_reaches_the_annulus_backstop_at_the_front_door`
  and `…_the_ladder_backstop_at_the_front_door`. The fixture is homed as
  `test_support::bored_cylinder`.

## Landed

Everything this file asked for, on PR 2215:

- both relations rowed at the exact backstop, on the bored cylinder;
- the two-tolerance trio built at `fillet3_ring_clearance` through the
  public door on that fixture — definite `-0.01`, exactly zero, in-band
  escalation with the ring recourse
  (`m5_pr12_refusals::trio_hostless_annulus_ring_containment`). The
  coaxial trio it replaces is kept beside it as
  `…::trio_coaxial_ring_containment_is_answered_by_the_screen`, labelled
  for what it pins: predicate 2's answer, not this unit's form;
- the ladder outer-cycle `Curve3::Circle` arm's `max` witnessed on BOTH
  halves — its containment half by the bored cylinder's ladder row, its
  external half by
  `review_ring_clearance_r1_probes::r1_a_convex_corner_arc_of_a_mixed_outer_cycle_takes_the_external_term`
  (a dimpled plate whose rounded-rectangle corner arcs enclose nothing).
  A mutant metering that arm on containment alone reds that row and only
  that row;
- the centre-distance term of `circle_margins` witnessed: deleting it
  reds six rows including both bored-cylinder rows and the new trio;
- a swap of the ring walk's two relations reds eleven.

## What remains true of the original finding

The screen still ANSWERS FIRST on every coaxial pair, because there its
sampled gap is the true one, and the exact forms and the screen agree to
within the association above. That is the soundness argument for the
ordering — sampling never reads a gap smaller than the true one — and it
is now stated at `CircleMargins` rather than as a claim of bit equality.
Nothing about it is a gap: the backstop decides exactly where the screen
cannot, and both halves are rowed.
