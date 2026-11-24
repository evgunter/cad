---
id: the-windows-level-jet-is-read-exact-where-its-rounding-grows-with-the-aspect
kind: issue
title: The window's and the coplanar subdivision's level jet is read exact where its rounding as a length grows with the aspect, which the admission rule does not cover
status: parked
opened: 2026-11-21
priority: P1
cost: M
blocked_on: [an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre]
---


Found by the review of PR 7221 (`oneround/rounded-pairs`, M4). The ruling
on it was to keep the jets exact, make the citation true, and schedule
the debt here.

## The site

`geom_brep::curve_pair::pose::Level::jet_at` reads `b`'s level along
`a`, `F = X²/A² + Y²/B² − 1`, and its first three derivatives locally at
a parameter. Two readers take that jet as exact (`Rounded::exact`):

- `pose::window`'s `local_taylor`, used by `Window::read`;
- `pose`'s coplanar subdivision, whose touch arm PR 6565 re-posed so
  that an exact jet places the touch (the `jet` closure beside
  `certified_subdivision_touching`'s caller).

The height's jet beside it is a short division-free chain, which the
admission rule (`geom_core::tolerance::admissible`) covers. The level's
jet is not. It divides by `A²` and `B²`, so it is dimensionless, and its
rounding read as a length (through `L·|F|`, `L` the larger semi-axis)
grows with `b`'s aspect `L/s`. The rule bounds a short chain on in-box
coordinates against the headroom; it says nothing about such a quotient.

## What was measured

Under PR 7221, with a running first-order bound on each read (the point
charged `DIRECTION_UNITS·u` at its reach, then the height's and the
level's chains), over geom-brep's 206 `curve_pair`/`pose` tests, about
133k window reads per ε at 1e-9 and 1e-12. The worst readings came from
`curve_pair::order_rows::the_needle_far_from_the_origin_reads_as_at_it`,
and were the same at both ε, to the last digit. The ratios are taken
against the band each read was decided at, so that test's reads used a
band that `CAD_TOLERANCE_EPS` does not move:

- the height's rounding reached 0.77 zero bands;
- the level's rounding times `L` reached 1.5 zero bands.

About 2.1k (1e-9) and 2.5k (1e-12) reads passed a tenth of a zero
band. They fell in the needle,
`conics_tangent_at_a_shared_end_read_one_answer_in_either_order` and
`thin_ellipses_tangent_at_a_vertex_meet_once_on_the_local_distance`,
plus `meet_reads_one_answer_in_either_order_over_random_near_parallel_pairs`
at 1e-9 and `the_leaning_rims_tangent_at_a_shared_end_meet_once_in_either_order`
at 1e-12. One fixture's worst case is not a bound.

## Whether a definite verdict is wrong

Not known. 1.5 zero bands is inside the 9 bands of headroom at K = 10,
so on the measured poses a definite reading past the escalation edge
keeps its sign. But the level's rounding grows with the aspect and with
the carriers' distance from the origin, while the band can shrink to
ε_min. So an admitted (ε, K) with a thin, far `b` (a small semi-minor
axis, its centre far from the origin) may put the loss past the
headroom. The window then reads a definite off, recedes or on from a
level that its rounding moves across the band. The coplanar subdivision
reads its clear and monotone tests the same way.

No probe has been built. Finding out needs that pose, read against an
exact oracle of the level (dyadic or `Interval`) at the finest admitted
ε. Hence P1 until someone measures it.

## Owed

Re-pose the level as a length: a distance form, or the level read as
`L·F` about the piece with its division taken by a pose decided first.
Then the admission rule covers it as it covers the height, under
`crates/geom-core/README.md`, "Float rounding" ("re-posing, never
charging"). Charging the jet instead was considered and ruled out for
PR 7221: it changes the arm PR 6565 re-posed for an exact jet. If the
probe shows no definite verdict can go wrong at any admitted (ε, K),
drop this row to P2.

P1 by `work/README.md` Priority: a possibly wrong definite verdict,
unmeasured.

## Parked on the curve-pair fix unit (2026-11-23)

`oneround/conic-pair-readers` (`an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre`) is asked to re-pose the level from the stored point: the suspects lane showed the level's form `F = X²/A² + Y²/B² − 1` loses about u·a metres whatever its centre. This row is re-read against what that unit lands: what it asks may be met, or changed, by the re-posing.

## Moved to CONIC (2026-11-23)

Moved from ONEROUND by `git mv` when that track was split on Ev's answer to PR 7523 (`work/conic/log.md`); the id and the body above are unchanged.
