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

## A pattern's spacing is a positive length; its step is a signed angle within a turn

Measured on main: a spacing or step of zero builds every copy on the master, a negative spacing builds the mirrored pattern, a step of a full turn or more builds (7 rad does), `PlacedUnion` refuses only at the one point (through its disjointness certificate), and the range certificate certifies a driven spacing straight through zero. `Instance(i)` is "i steps from the master", so a spacing driven through zero moves every reference to instance 2 to the other side without a refusal: #3551's end-cap case. No document, demo, corpus model or test authors a negative or zero spacing or step.

The tree's practice splits by what the quantity is. A **length** an operation covers is positive, with its direction stated elsewhere: a radius, a wall, an authored arc's length and sweep (`profile/src/seg.rs`, its turn a structural `side`), and since PR 3912 an extrude's depth. An **angle about a directed axis** is signed by the right-hand rule, nonzero and within a full turn: the revolve's angle (`DegenerateAngle`, `FullRangeAngle`), a stored arc's sweep (`ArcCheck::SweepRange`). Poses (a transform's rotation, a frame's spin) are not sizes and stay unbounded. The #3551 rule is about lengths.

- **Linear.** `spacing` is a positive length, the distance between neighbouring copies; the direction triple the pattern owns is the one home of which way they step. A definitely negative spacing refuses: "the spacing evaluated to −4 m, below zero, and a spacing is a size: which way the copies step is the direction's to say, not a sign's. Recourse: make the spacing evaluate positive (its sign comes from whatever drives it) and point the direction the other way, (−1, 0, 0)", quoting the evaluated direction negated, which builds the same bodies bit for bit. A zero or tolerance-small spacing refuses: every copy would land on the master. What this buys is not consistency: a failure over the half-line below the floor, which a range probe finds for every seed, where a zero-only refusal is a tolerance-wide band a probe steps over; no silent mirror of instance references under a driven pitch; one spelling per placement.
- **Circular.** `step` keeps its sign, right-handed about the datum axis, as the revolve's angle does. A zero or sliver step refuses (the copies coincide); a step at or past a full turn refuses with the recourse "write step ∓ 360°, which places every copy where this does". A positive-only step would buy one spelling per placement set and a probe-visible floor, but full rings (360°/n) only renumber under the sign, nothing drives a step through zero, and it would make the step the one angle about an axis that is not signed. The band a driven angle can cross at zero is shared with the revolve; if it is closed, it is closed for both, by the probe.
- **Both.** The check sits in one constructor of the stepped operands, which the evaluation and the mate solve's derived offset both use (today `eval/wire.rs` `stepped_map` and `mate/member.rs` `pattern_map` each read the slots), and runs only where a step reads the value (index ≥ 1), so a one-copy pattern (`360 deg / blades` at one blade) builds. The near-zero and full-turn decisions reuse the evaluation's existing sign decision (the revolve angle's), not a new angle tolerance. No new field, no file format, python signature or edit arm change: four refusals and their tags.
- **The extrude stays as built.** Its sign had no other home (the sketch normal is the profile's, and flipping it mirrors the sketch), it is a length with no wraparound, and reversing it reopens the measured silent flip under a driven thickness.
- **Revolve.** `work/carve/revolve-angle-is-a-signed-size-beside-a-directed-axis` stays as the tree has it (signed, nonzero, within a turn) and can close against this; what is left there is whether its zero and full-range refusals name a recourse.

Weighed by two designers in two rounds (fork-log row 55); their reports are in the PR.
