---
id: a-mate-placed-seat-is-unproven-at-the-doors-budget
kind: issue
title: a mate-placed seat replays at Sym but stays unproven within the door's budget: the fold's frames hold component symbols renormalised at every hop
status: parked
opened: 2026-10-18
priority: P0
cost: H
design: true
refs: [placed-carriers-compare-through-their-frames, mates-declare-no-contact, a-pose-is-moved-by-lengths-and-angles]
blocked_on: [transform-retires-into-a-placement]
---

## What

D10 says a mate-placed face is structural. The door does not prove one.
Stage 4 H (PR 5300) built the replay that would read it: each bundle
folds at `Sym`, and the kernel's carrier verdict runs on the two copies'
faces (`crates/editor-core/src/coincide/replay.rs` `prove_row`, the
`CensusAtRest` arm, with parts resolved at `Sym`). That replay is behind
`coincide::REPLAYS_PLACED_COPIES`, which is off. Switched on, a mated
seat is `Numeric`, not proven, at a cost the door cannot pay for nothing
(below). Pinned off and unproven by
`mate6_gather_mints::a_mated_record_cites_the_census_row_the_lint_reports`.
It is visible as the tour bench's three `unproven-coincidence` findings
(`demos/tour/src/assembly.rs` `bench`). The same holds for contacts
between patterned copies, which stage 4 §9 also expected to shrink.

## The cost the fix must also beat

Measured with the switch on, locally (debug), against main:

| viewer row | main | switch on |
|---|---|---|
| `tree_badges::a_union_over_a_chained_copy_points_at_the_refused_placement` | 0.28 s | 4.3 s |
| `story_assembly::the_windmill_story` | 1.07 s | 19.1 s |

About 4 s is resolving the parts at `Sym`. About 15 s more on the
windmill is the carrier verdicts on its seats, whose forms freeze past
the budget and pay the retry ladder. Turning the switch on is part of
this row's fix, and it must not cost that.

## Measured (H lane, 2026-10-18)

The carrier verdict needs the copy's face normal to equal the target's as
a polynomial identity. Cap normals built the way an extrude builds them
(`normalize(u × v)` over a `Datum::Frame`'s six component symbols):

- every factor of the fold proves at the door's 256 terms in isolation:
  a pose's frame (`mate/solve.rs` `frame_of`, the decided shortest turn)
  and the value's turn `R(d, v − atan2(…))`;
- their product does not. `F_to·F_onᵀ` applied to the copy's normal is
  unproven at 256 and proven only from 1024 terms. The spin step
  `R(n, θ)·n = n` is proven only at 16384 terms with a hand-written
  Rodrigues on the unit axis. Through `Mat3::rotation_about`, which
  renormalises an axis that is already unit, it is unproven even at 16384;
- applied as a chain of factors rather than one composed map, the same
  identities collapse step by step (`F_onᵀ·n` is exactly `z`), but only
  from 1024 terms with these frames. The door's own measure is 0.9 s at
  256 terms against 172 s at 1024 (`coincide/replay.rs` `BUDGET`).

With unit-axis frames over three free symbols (degree-1 normals) the
chain proves at 256 terms. So the cost is the frames' component symbols
and the renormalisation at every hop. It is not the fold's structure.

## First step

Re-measure after D, with the switch on, on mate6's stand and on the
tour bench: does a mated seat prove at the door's 256 terms once frames
are `Slide`/`Spin` chains, and at what cost? The identity probe is
`intent_s4_h_placed::measure_a_mated_seat_at_sym` (`#[ignore]`d; its
run command is in its doc). If a seat still does not prove, run the
part-variable fork (route 0) through designers (orchestrator,
2026-10-18), then weigh the other routes.

## Routes (a design question, weigh before building)

0. **A part's variables as constants of the assembly.** This is a fork
   between D10's "every variable a symbol" and AQ4's "instantiation
   takes no arguments". H binds each document version's variables as
   its own symbols. R1 of H's review measured the other reading: with a
   part's variables bound as numbers, mate6's mated seat proves at 256
   terms and the lint reports nothing. The cost is that it would also
   cut tolerance stack-ups across parts, which is the main thing
   symbols buy, so it is not free.

1. Frames from slides and spins, with no component slots (FORK-MOVE).
   Stage 3 D (`transform-retires-into-a-placement`) retires `Datum`
   entirely, so every construction's pose becomes a `Slide`/`Spin` chain
   over the world read. Normals then become trigonometric polynomials
   with no `sqrt` chain. This is the hypothesis the first step tests.
2. Place a copy factor by factor at `Sym` (prototyped on the H branch and
   dropped: lane-dependent arithmetic, with no proof at budget under
   today's frames). This is likely still needed after route 1, because a
   composed map hides each step's collapse.
3. Stop renormalising a vector that is unit by construction on the fold's
   path (`Affine3::rotation_about_axis` over a `UnitVec3` in
   `mate/value.rs` `settle`; the chart's `UnitVec3::new` on rotated unit
   vectors).
4. A tier capability for a `sqrt` whose argument is identically one after
   rational cancellation (SYM's ground).

`mates-declare-no-contact` (stage 4 I) waits on this. Retiring
`ContactClass` before the door proves a mated seat would turn every
resting mate into a finding.

## Other sites on the fold's path that build a map in a form the tier reads poorly (H's sweep, 2026-10-18)

These were not measured. They sit on multi-mate bundles; the measured case was one plane mate and its values:

- `mate/coset.rs` `ball_pair`: `chord.orthonormal_basis()` (min/max/select atoms) in the antipode arm.
- `mate/coset.rs` `candidate_rotation`, `clocking_about`, `ball_on_line`, `ball_pair`: `Mat3::rotation_about`, which renormalises a unit axis, over a `solve_atan2` angle.
- `mate/coset.rs` `clocking_about`, `ball_on_line`, `ball_pair`: a representative inverted by `Affine3::inverse` (the adjugate over the determinant) where the map is rigid by construction.

## Owed when `coincide::REPLAYS_PLACED_COPIES` turns on (H's review, both reviewers)

The switched-off code in `coincide/replay.rs` was reviewed but has no row. Both reviewers' mutants survived on it:

- **A positive row:** an at-rest row the carrier verdict proves `Structural`. R2 measured one constructible today: two copies of one part at their identity pins.
- **The relation guard:** a verdict of another relation proves nothing (`carrier_verdict`'s `relation == wanted`).
- **`at_rest_unread`:** a copy the replay did not build says `ThroughAPart`/`Refused`, not `NotReplayed`. A replay that decides the pair otherwise needs a reason of its own, not "not two faces".
- **Cancellation:** the census arm stops canceled.
- **The band:** a band that fails reports as a band failure, not as no verdict (the arm's `Option<Option<…>>`).
- **The recourse:** a mated seat that is only over the budget is `Undischarged::Otherwise`, so its recourse reads "build the two cells as one construction". The user already built it as one construction, so the recourse must name the budget, not ask for a construction.
