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

## A second instance, measured, and the census this row asked for

Added 2026-09-14 by TOPO's `D107` lane, which hit this class head-on and
is filing its evidence here rather than opening a duplicate
(`work/README.md`: one file per item).

**The instance.** `crates/topo/src/review_d18.rs`'s two hammer rows
floored with `census.require_nonzero_among(&LINK_OPS, 4)` over six
operators. `kemr` was one of the six and its count was **0 in both rows
from the day the floor was written** — the file's own module doc said
so, and the row was green throughout. That is this class's exact shape:
the aggregate rode the other five, and the floor's slack (4 of 6) was
argued in a comment as headroom against legitimate change, which is a
real argument that also happens to hide a class that was already
silent.

Not hypothetical, and not an inference from the arithmetic. With the
fixture and enumeration D107 added, `kemr` reaches 2 mutation phases on
the deterministic row and ~45 on the sampling one. Reverting either half
alone drops it back to 0 — and `require_nonzero_among(&LINK_OPS, 5)`
**still passes**, because the other five are untouched. The aggregate
cannot see a class go silent even when five sixths of the surface is
intact.

**The remedy that worked**, both halves of the "static witness" this row
asks for:

- a **named** floor beside the aggregate — `census.require("kemr", 1,
  …)` — so the class that needs a fixture to exist at all cannot go
  silent under the slack;
- on the row that is deterministic end to end, the **exact per-category
  tally** asserted from a table (`SPENT_GRAFT_EXPOSURE`) rather than
  written in a comment. That one also caught a stale figure: the comment
  beside the old floor said `mev_line 60` where the measurement was 52.
  Nothing had held it, so nobody had noticed.

**The partial census this row said did not exist.** Two patterns, both
over `crates/`, `tools/` and `demos/`:

| pattern | hits | disposition |
| --- | --- | --- |
| `require_nonzero_among` — the aggregate-over-classes floor | 3, all `crates/topo/src/review_d18.rs` | fixed by D107 (named floor + exact tally) |
| `easured on an intact tree` — a per-category measurement carried only in prose beside a floor | 1 `review_d18.rs` (fixed), 4 `crates/sweep/tests/review_d2_adv_probes.rs` | not-this-unit: those four sit beside `require_each` floors that are already per-class, so the prose figure argues the floor's LEVEL rather than standing in for a claim; they carry the drift hazard but not the vacuity one |

**What the patterns cannot match**, which is the part that keeps this a
partial census: both are spellings, not shapes. A hand-rolled
`assert!(reached >= n)` over several per-class counters is the same
defect and matches neither; so is any aggregate floor that does not go
through `test_utils::vacuity`. The `.require(` census is noisy for the
same reason — `crates/viewer/src/combine.rs`'s eight hits are a seat
lookup, a different API with the same name. A shape-level census still
wants writing.
