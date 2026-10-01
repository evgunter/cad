---
id: a-clocking-rider-is-levered-unreduced
kind: issue
title: A coincidence's clocking rider is levered as authored: a full turn refuses as contradictory, and a huge one quotes an infinite deviation
status: open
opened: 2026-10-01
priority: P3
cost: E
---


## Finding

Found by MSOLVE-11's sweep of levered predicates (PR 3680), which made
the ARM a finite length by construction (`mate::coset::Arm`) and named
what that sweep could not reach: the NUMBER a levered margin multiplies
the arm by.

`crates/editor-core/src/mate/solve.rs`, `mate_coset`, the
`FrameCoincidence` arm: the rider's margin is
`Lever::Roll { radians: theta, arm }.margin()` = `theta · arm`
(`Margin::levered`), with `theta` the authored `Alignment::clocking`
unreduced. Every other levered number in the solve is a sine, a cosine
or a rotation residual of unit witnesses, bounded by 4; `theta` is
bounded only by `Alignment::is_finite` (the insert door's admission).
Two consequences, both read off the code, neither constructed:

1. A clocking of `2π` (or any whole turn) on a coincidence is the same
   pose as `0`, and the rider is decided non-zero:
   `MateFault::Contradictory { predicate: "mate_clocking_redundant" }`
   on a mate that contradicts nothing.
2. A finite `theta` large enough that `theta · arm` overflows decides on
   an infinite margin (maximally definite, so still `Contradictory`,
   which is the right verdict for a non-zero roll), and the sentence
   quotes "a deviation of inf m".

The coaxial rider (`MatePrimitive::Coaxial` with clocking) spins the
target by `theta` through `Mat3::rotation_about`, which reduces it
implicitly; only the coincidence's rider levers it.

## What would close it

Decide the rider on the roll reduced to `(-π, π]` (the angle the pose
actually differs by), at the one site that forms `Lever::Roll`, so the
quoted deviation is the deviation of the pose; and a row that pins a
whole-turn rider as redundant.
