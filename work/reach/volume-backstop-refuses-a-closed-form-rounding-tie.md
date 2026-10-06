---
id: volume-backstop-refuses-a-closed-form-rounding-tie
kind: issue
title: The volume backstop refuses ResultVolumeImplausible on a two-ulp tie between closed-form volumes, calling a correct body a kernel defect
status: closed
closed: 2026-10-02
opened: 2026-10-01
priority: P2
cost: M
pr: 3844
branch: reach/door-backstop
---


Found on PR 3657 (`reach/cosurface-continuation`) after `origin/main`
was merged in with CLEAVE's #3716.

## Repro

`crates/sweep/tests/reach_continuation.rs`
`declared_rounded_continuations_inside_a_wall_build_subtract_and_intersect`,
the flush-top pose: A is the rounded 6 × 4 × 1 plate (corner fillets
r = 0.5), B the same outline 0.5 thick on z 0.5..1, every finding
declared. `intersect_with(A, B)` builds B itself (10 faces, tier 3 and
3′ clean), and the backstop refuses it:

    ResultVolumeImplausible { which: "vol(A ∩ B) ≤ vol(B)",
        got: "11.892699081698725", bound: "11.892699081698723" }

The oracle, `(24 − (4 − π)/4)/2`, is `11.892699081698725` in `f64`. The
result and the operand are the same solid, measured through two face
orders, and their closed-form flux sums round two ulps apart. The flush
bottom pose's intersect, the same solid again, happens to round equal
and builds.

## Cause

`boolean/ops.rs` `bound_holds`, arm 1, decides the margin against the
exact (bit-hairline) band: at `f64` any nonzero negative is a certain
violation. The margin's ends come from `MassProperties::enclosure`,
whose `volume_pad` is `0.0` for a closed-form face (`props.rs`
`fold_runs`, "`0.0` for closed-form faces"). So the floating-point
rounding of the closed-form flux sum is treated as exact. The
`volume_backstop` docs say the closed-form corpus's flux sums are exact
for dyadic fixtures; a fixture carrying π is not dyadic. Any boolean
whose result is congruent to an operand that it is bounded by, with a
curved closed-form face, can refuse this way, and the text ends with
the kernel-defect ending.

## What the taker owes

A rounding allowance the closed-form margin carries soundly (for example
a pad from the flux terms' magnitudes), so that a tie at rounding scale
reads `Zero` while a wrong-component result still refuses. The row's
flush-top intersect then flips to building at the oracle.

## Resolution (`reach/door-backstop`)

Arm 1 no longer reads an `f64` closed-form sum as exact. When the
walk's sums call a violation, the backstop re-derives both sides in
interval arithmetic (`PastTarget::interval_volume`):

- every closed-form face is re-derived at the interval scalar over its
  stored geometry (`QuadLane`'s `closed_form`, `quad_lane::closed_form`);
- every quadrature face contributes the enclosure its lane returned,
  not the midpoint and half-width rounded from it;
- the fold sums in interval arithmetic.

A violation the interval margin does not certify is the rounding's and
stays open. No pad and no constant is involved. The flush-top intersect
builds at the oracle `(24 − (4 − π)/4)/2`.

It is pinned by its own rows:

- `sweep/tests/reach_continuation.rs`, the flush-top intersect;
- `boolean::ops::tests::volume_backstop_passes_a_closed_form_rounding_tie`,
  a non-dyadic prism started at two corners, whose sums round 3e-16 m³
  apart.

Both go red with the re-derivation skipped.

The class's general form, a tie at a tight bound, has a second source
that rounding does not explain: a declared coincidence the door settles
inside the band. It moves a correct result past the bound by up to the
band over the glued face. That source still refuses (the safe
direction). It is filed with its measurements as
`a-settled-declared-coincidence-crosses-a-tight-volume-bound`, after a
volume allowance for it was shown unsound in PR 3844's dual review.

## Closed (2026-10-02, PR 3844)

When the walk's f64 sums call a bound violated, the backstop now
re-derives both sides in interval arithmetic. Each closed-form face is
lifted at the interval scalar over its own stored geometry, quadrature
faces use their returned enclosure, and the fold is done in interval
arithmetic. A violation the interval margin does not certify goes to
the open arm. The flush-top 2-ulp tie builds. A non-dyadic prism tie
and a curved rod tie are pinned, and both go red when the
re-derivation is skipped or the per-face lift is collapsed.

The declared-pair allowance the PR first added was removed on the dual
review (DR-46, MAJOR from both reviewers). It was a volume, so it
forgave a defect of that size anywhere in the body. The
settled-coincidence residue it was meant for refuses again. That is
pinned per ε and filed as
`a-settled-declared-coincidence-crosses-a-tight-volume-bound`.
