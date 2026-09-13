---
id: anti-vacuity-floor-cannot-go-red-on-degradation
kind: issue
title: The GUI0-R1 walk's anti-vacuity floor survives deleting either refusal arm: a guard 16x looser than it reads
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the S-TCOST orchestrator out of the style review of
PR 2433. That PR routed the row onto the fuzz harness and **did not
create this**; the floor is pre-existing and the PR correctly left it
alone. Routed here rather than kept, by `work/tint/plan.md` §The fence
with S-TCOST: nothing below is a second of cpu or wall, and *"a row
justified by a claim that cannot fail ... is this program's"*.

## The row

`crates/viewer/tests/review_gui0_r1.rs`,
`the_camera_contract_survives_random_operation_walks`. It walks a camera
through random operations and asserts the contract holds, with an
anti-vacuity floor so a generator that stopped producing refusals cannot
pass the row vacuously:

```
assert!(refusals > walks / 4, "the walk barely produced refusals ...")
```

## Defect 1 — the units do not match, and the message says so

`refusals` accrues **per step**, inside `for step in 0..steps` with
`steps = 16`. The floor is a quarter of **`walks`**. The row's own
failure message then prints `walks * steps` as the denominator — so the
message and the assertion disagree about what population the count is
drawn from, and the message is the one that is right.

The floor therefore admits one refusal per four walks of sixteen steps:
**16x looser than reading `refusals > walks / 4` beside a per-step
counter suggests.**

## Defect 2 — it cannot go red when the guarantee degrades

Two of eight operation arms refuse by construction (`Dolly` with a
factor drawn from a negative range always refuses; half of arm 0's yaws
are NaN), so p is roughly 3/16 per step and the expected refusal count
over `walks * steps` is about 3 x `walks`, against a floor of
`walks / 4`. The margin is ~12x.

So, from the review and consistent with that arithmetic:

- delete the `NonPositiveDolly` check entirely and roughly 48 NaN
  refusals remain against a floor of 12 — **green**;
- delete the finite check instead and roughly 96 remain — **green**.

Either deletion removes a whole refusal class from the camera's
contract, and the guard written to notice exactly that stays green. This
is `docs/prompts/reviewer-style-lane.md`'s Q3 in its second shape: *every
"never silent" or "certified enclosure" claim needs a row that goes red
when the guarantee DEGRADES, not merely when it is violated at one
chosen fixture.*

## Defect 3 — the row is the mixed shape the memory warns against

`memories/test-suite-cost.md`: *"The trap is mixing the first and third
in one test. A property test with an anti-vacuity floor bolted on is
both at once, and its sample count then feeds two obligations of which
only one is safe to cut. Make the floor's witness static, or split the
test."*

This row is that mix. PR 2433 argued the mix is safe **because the floor
is a fraction of the count rather than an absolute K**, so raising
EFFORT raises both sides together. That argument is sound as far as it
goes and is not what this row disputes — scaling is safe. What it does
not address is that the floor is too weak to be a guard at any count,
which is a property of the constant and not of the dial. The memory's
two prescribed remedies (a static witness, or a split) were not taken.

## What this asks for

Per-class evidence rather than an aggregate count: a floor that is red
when **any one refusal class** stops firing, not one satisfied by the
survivors of the others. That is the static-witness remedy — assert each
class was reached — and it also fixes defects 1 and 2 at once, since a
per-class assertion has no denominator to get wrong.

## The class

**An anti-vacuity floor stated as an aggregate over several independent
classes.** Any one class can go silent and the aggregate survives on the
others; the floor reads as a guard over all of them and guards only
their sum. Where to look: every `#[test]` carrying a "the generator has
drifted" / "barely produced" / "too few" style assertion over a counter
that several distinct arms increment. Neither this row nor the review
swept for it — the census does not exist yet.
