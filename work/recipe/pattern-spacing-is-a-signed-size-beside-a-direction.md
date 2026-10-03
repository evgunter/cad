---
id: pattern-spacing-is-a-signed-size-beside-a-direction
kind: issue
title: A linear pattern's signed spacing and a circular pattern's signed step state the direction a second time (a named follow-on of Ev's #3551 rule)
status: open
opened: 2026-10-01
priority: P2
cost: M
design: true
needs_ev: true
---


A named follow-on of Ev's ruling on #3551 (2026-10-01), which adopted the rule "**a size an operation covers is positive; its direction has one home**" and applied it to the extrude first (`work/recipe/extrude-distance-is-a-depth-and-a-side`).

A linear pattern's spacing is signed beside its direction vector, and a circular pattern's step is signed beside its axis. The #3551 designers rated this "unsure" (not measured). Weigh it under the rule before building: whether each becomes a positive size with the direction in its vector or axis, and what each refusal's recourse says (Ev asked that the extrude's refusal show how to write the other direction; the same applies here). Record the answer here.

Filed by the AUTHOR orchestrator on Ev's ruling. Ground: `Node::Pattern` (`crates/editor-core/src/node.rs`, EDIT).

## A pattern's sizes are positive, and its direction is the one it already owns

Measured on main: a spacing or step of zero builds every copy on the master, a negative one builds the mirrored pattern, `PlacedUnion` refuses only at the one point (through its disjointness certificate), and the range certificate certifies a driven spacing or step straight through zero. `Instance(i)` is "i steps from the master", so a driven sign flip moves every reference to instance 2 to the other side without a refusal: #3551's end-cap case again. No document, demo, corpus model or test authors a negative or zero spacing or step.

- **Linear.** `spacing` is a positive length, the distance between neighbouring copies; the direction triple the pattern owns is the one home of which way they step. A definitely negative spacing refuses: "the spacing evaluated to −4 m, below zero, and a spacing is a size: which way the copies step is the direction's to say, not a sign's. Recourse: make the spacing evaluate positive (its sign comes from whatever drives it) and point the direction the other way, (−1, 0, 0)", quoting the evaluated direction negated, which builds the same bodies. A zero or tolerance-small spacing refuses on its own grounds: every copy would land on the master.
- **Circular.** `step` is an angle in the open interval (0°, 360°), turning right-handed about the datum axis; the axis is the one home of the turning sense. An angle wraps (−30° places as 330° does, 400° as 40°), so within that interval each placement set has one spelling, and a step driven through 0° or 360° refuses rather than collapsing and coming back. A negative step's recourse is "write 360° − |step|" (the same placements and instance indices, up to rounding; a driven arc the other way is `360 deg − p`), then reversing the axis. A step of 0°, or of 360° or more, refuses.
- **Both.** The check sits in one constructor of the stepped operands, which the evaluation and the mate solve's derived offset both use (today `eval/wire.rs` `stepped_map` and `mate/member.rs` `pattern_map` each read the slots), and runs only where a step reads the value (index ≥ 1), so a one-copy pattern (`360 deg / blades` at one blade) builds. The near-zero and near-360° decision reuses the evaluation's existing sign decision (the revolve angle's `Sign`), not a new angle tolerance.
- **No new field.** The extrude needed a structural `side` because a plane has two sides and a depth has no other spelling. A rotation is periodic, so a structural sense beside a capped step would state every placement set twice (30° against the axis is 330° with it), and beside an uncapped step leaves the wrap through 360° open. No datum axis on main has more than one consumer, and a revolve cannot take a pattern's axis (`AxisInPlane` against `Datum::Axis`), so the local recourse never needs to touch the axis. No file format, python signature or edit arm changes: four refusals and their tags.
- **Revolve.** `work/carve/revolve-angle-is-a-signed-size-beside-a-directed-axis` shares the convention (right-handed, the sense in the axis) and differs where it must: 360° is a full revolution there, and a revolved solid is not periodic, so 360° − |angle| is not its recourse.

Weighed by two designers (fork-log row 55); their reports are in the PR.
