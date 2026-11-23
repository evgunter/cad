---
id: a-ring-closes-each-k-from-its-start-point
kind: unit
title: A ring is each k from P[k] { steps, to Next }, closing on P[0] read twice
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: a-loop-cannot-repeat-inside-a-profile
blocked_on: [a-chain-repeats-a-run-of-its-steps-with-each]
---


Unit 5 of `a-loop-cannot-repeat-inside-a-profile` (Ev, PR 7263). A
ring is a loop that is nothing but the copies of a run: a gear, a
spline section, a polygon.

## The ruled text it implements

`docs/PATHS-DESIGN.md` §2 *Composition*:

> A ring, a loop that is nothing but the copies of a run, is
> `at(P[0]); each k from P[k] { steps …, to Next }` with `Next` meaning
> `P[(k + 1) mod N]`: at the last copy that is the value the chain
> starts from, so it lowers to `Start` and the ring closes with nothing
> compared.

## Done when (each a test)

- **It closes by construction.** A 24-tooth ring, with `P[k]` the
  tooth's start at angle `scalar(k)·turn/24` and pen-relative steps
  inside the tooth, ending `to Next`. It evaluates to one closed loop.
  The last copy's last leg targets `Start` in the lowered program, so
  the kernel receives the `Start` target and no point near it. A test
  reads the lowered steps and asserts it.
- **Intermediate joins are exact.** Copy `k`'s last leg ends on
  `P[k+1]`, which is copy `k+1`'s written start, read once. The
  lowered program shows the same resolved value at both places.
- **`Next` is only legal in a ring.** `Next` outside a block with
  `from`, or before the block's last step, refuses at the door, typed.
- **The ring's `P` is its own written start point reading `k`.** It is
  not a named `Point2`: SELFREF's U1
  (`work/selfref/a-profile-point-argument-holds-one-point2-variable.md`)
  has not landed. When it lands, `P` may become a named point, and that
  is SELFREF's follow-up, not this unit's.
- **The 60-tooth outline** of about 8 pieces per tooth evaluates within
  the PR gate's budget, or its test goes in the slow set. Validation's
  pair pass is quadratic
  (`work/profile/validation-pair-pass-has-no-pruning.md`); this unit
  records the measured time in its PR.

## Check first

`N = 0` for a ring. The designers have it refusing at evaluation,
naming the index, but the ruled text does not say. It is a question in
the cut, and this unit does not decide it. Also check that `from` with
the block's first copy at `k = 0` lowers `at(P[0])` to the same bits as
`P[0]` read inside the block. Both are reads of one written expression
at one member env. If the lowering resolves them in different
environments, stop and report.

## Rows and ground

- Closes the parent row together with the VSEAM and PYDOOR halves.
- Same ground as unit 4.
- Review: **single FULL**. It is a narrow lowering rule on top of unit
  4, and the exactness assertions are the whole review. Difficulty **M**.
