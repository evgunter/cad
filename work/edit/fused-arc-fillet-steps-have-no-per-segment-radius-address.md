---
id: fused-arc-fillet-steps-have-no-per-segment-radius-address
kind: issue
title: A fillet arc's radius never reaches the wall it drew: the radius and the segments sit on different steps, and a fused step holds several
status: open
opened: 2026-09-17
---

Disclosed by the `edit/chain-radius-attach` unit, which built the
per-edge radius door and left this shape outside it deliberately.

## The finding

`ProfileProgram::segment_radii` and `LoopProgram::step_radii`
(`crates/editor-core/src/program.rs`) answer a radius per profile edge
by reading a step's OWN radius arguments and the replay's record of
which segments that step emitted. A step that holds exactly one
radius-bearing argument AND emitted exactly one segment is
unambiguous. Every fillet shape fails one of those two, in one of two
ways.

**The radius and the segments are on different steps.**
`ProgramStep::Fillet(r)` is a tip-state BINDER: it holds
`StepArg::Radius` and emits no segment at all (`StepSpan` is empty for
it — `edit_step_segments`'s attribution table classifies it `Binds`).
The arc it opens is emitted by the ARRIVAL step, which carries no
radius of its own. So there is no single step whose radius and whose
segments can be paired, and the arc's wall carries nothing.

**A fused step holds several radii.** `ProgramStep::FilletArc` and
`ProgramStep::ArcFillet` each hold two (`StepArg::Radius` plus a
spec's `CarrierRadius` or `CarrierRadius2`), `ProgramStep::ArcFilletArc`
holds three, and each emits several segments. The record says WHICH
segments the step emitted and nothing about which of them each radius
drew, so `radius_arg` answers `None` and the step contributes no pair.

The consequence is not a wrong answer, it is a missing one, at the
attach and in the SAFE direction: the walls swept from those arcs
carry no parameter identity, so a boolean germ over a filleted
corner's wall reads `None` where a plain `arc_to`'s reads `Declared`.
The SPELLINGS of the one-radius shapes do enter the content key —
`step_radii` is program-side and cannot see a span — which is the
conservative side of the inclusion `eval::content_key`'s feed states,
and costs a memo hit rather than a stale token.

## What a taker does

Give the replay record a way to say which segment a fillet's radius
drew — which segment of an arrival's span is the fillet arc, and which
of a fused step's span each of its radii produced — and then widen the
rule from "one radius, one segment" to that attribution. The natural
home is beside `ReplayStructure::steps` in
`crates/profile/src/structure.rs`, which already records
`FilletDecision`s and per-segment `SegmentShape`s (the latter in
CANONICAL indices, which is why it is not usable here as it stands);
`Core::fillet_arcs` already pairs each emitted fillet arc with the
radius asked for, which is close to the missing fact but is not in the
record. Reading the shape back off the geometry is not an option: DM8
rules that this map reads the records the evaluation produced and
never re-derives them.

Both halves move together, as they did here: a radius that reaches a
wall must have reached the key first.

The rows a taker needs are the ones `segment_radii` already has, one
loop form over: `crates/editor-core/tests/edit_step_segments.rs` §5
(`a_step_with_several_radii_answers_no_radius` is the row that would
be replaced) and `crates/editor-core/tests/seat7_sweep_lowering.rs`
§2b.

Citations accurate at `edit/chain-radius-attach`'s head; the stable
halves are `radius_arg`, `LoopProgram::step_radii`,
`ProfileProgram::segment_radii` and `profile::ReplayStructure`.
