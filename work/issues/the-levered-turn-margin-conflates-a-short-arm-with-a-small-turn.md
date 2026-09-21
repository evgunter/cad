---
id: the-levered-turn-margin-conflates-a-short-arm-with-a-small-turn
kind: issue
title: fillet_corner_turn's levered margin cannot tell a short leg from a degenerate angle, and its definite arm refuses a real corner as already tangent
status: open
opened: 2026-09-13
priority: P0
cost: H
---


## Finding

`fillet_corner_turn` (`crates/profile/src/sugar.rs`, `arc_fillet_corner`'s
gate (2)) classifies `Margin::levered(perp_dot(dir_in, dir_out), arm)` —
`sin φ · arm`, the sine of the angle between the legs times the shorter
leg's own extent. One number, two independent user situations:

- the **angle** is degenerate (a smooth tangency or a cusp), which is
  what the gate was written for;
- the **leg** is short at a perfectly ordinary angle. A straight leg of
  `500 ε` meeting a radius-2 circle at `asin(0.01)` puts the margin at
  `5 ε`; so does a 64° corner on a `10.5 ε` leg.

The gate cannot tell them apart, and neither can anything downstream —
the payload carries the product, not the factors.

**The in-band arm's sentence was false at the second site, and is
repaired.** BLEND-12's fix pass rewrote `FILLET_TURN_INBAND_RECOURSE`
to name both situations and the leg extent as a lever (PR 2508); the
rows are `fillet_recourse_followability::the_turn_in_band_recourse_names_both_of_its_situations`
and the two reviewer rows it cites.

**The DEFINITE arm is still wrong in the same way, and that is this
item.** At `a = 50 ε` the identical 0.57° corner classifies `Sign::Zero`
and the door refuses `ArcTrimRefusal::AlreadyTangent` → the caller reads
`NoCornerForFillet { CarriersParallel }`, "the incoming ray and the
arrival carrier are parallel at tolerance — if they are meant to run
tangentially, author the tangency". They are not parallel: they cross at
0.57°, and the same carriers with a leg of 1.0 build and validate. The
refusal names a geometric fact that is not true of the input, and its
recourse (`.tangent()`) would declare a tangency that does not exist.

Driven by
`crates/profile/tests/review_fillet_recourse_arm_r1_probes.rs`'s
`the_turn_gate_is_reachable_in_band_with_a_short_leg_and_a_real_angle`,
whose last third asserts the definite arm at `a = 50 ε` and pins only
that it is definite and carries no fillet recourse — it does not assert
the text, because the text is what this item says is wrong.

## Why it is not a wording fix

A recourse can be corrected in prose; a REFUSAL that names the wrong
geometric class cannot. `CarriersParallel` is a classification the
envelope carries, `pncad-py` tags and callers match on. The repair is at
the gate: either the turn is classified on the angle alone and the arm
enters as a separate extent gate (which `fillet_corner_arm` already is,
so the two would need reconciling — its margin IS the arm), or the
levered margin keeps its shape and the refusal carries the factors so the
arm can say which one collapsed.

That is a decision about the predicates' bands and lever arms, which
BLEND-12's spec put explicitly out of scope, and it touches
`crates/profile/src/sugar.rs` — S-BOOL's glob, with no announced seam
for it. Filed here rather than on that program's slate because a lane
may not file into another program's directory (`work/README.md`, and the
standing lane rules): the owner should re-home it.

## Where it sits relative to its neighbours

`work/blend/fillet-inband-recourse-drops-the-tolerance-lever.md` is the
same gate family's OTHER prose defect and is independent of this one.
The reachability correction this item came out of is recorded in
`work/blend/fillet-escalation-site-has-no-producer.md`'s landed state.
