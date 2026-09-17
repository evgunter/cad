---
id: fused-arc-fillet-steps-have-no-per-segment-radius-address
kind: issue
title: A fused arc/fillet step's radii have no per-segment address, so neither the per-edge door nor the content key carries them
status: open
opened: 2026-09-17
---

Disclosed by the `edit/chain-radius-attach` unit, which built the
per-edge radius door and left this shape outside it deliberately.

## The finding

`ProfileProgram::segment_radii` and `LoopProgram::step_radii`
(`crates/editor-core/src/program.rs`) answer a radius per profile edge
by reading the step's OWN radius arguments and the replay's record of
which segments that step emitted. A step that holds exactly one
radius-bearing argument is unambiguous; a FUSED step is not.

`ProgramStep::FilletArc` and `ProgramStep::ArcFillet` each hold two
radius arguments (`StepArg::Radius` plus a spec's `CarrierRadius` or
`CarrierRadius2`), and `ProgramStep::ArcFilletArc` holds three. Each
also emits several segments. The replay record
(`profile::StepSpan`) says WHICH segments the step emitted and nothing
about which of them each of its radii drew, so `radius_arg` answers
`None` for all three and the whole step contributes no pair.

The consequence is not a wrong answer, it is a missing one, at both
ends and in the SAFE direction:

- the walls swept from those arcs carry no parameter identity, so a
  boolean germ over a fused fillet's wall reads `None` where a plain
  `arc_to`'s reads `Declared`;
- and their spellings do not enter the profile's content key, which is
  consistent with the first point and is what keeps the stale-token
  class closed (`eval::content_key`'s feed states the inclusion the
  guard rests on).

## What a taker does

Give the replay record a way to say which segment of a fused step's
span is which — the fillet arc, the trimmed leg, the spec arcs — and
then widen `radius_arg` from "exactly one radius" to a per-segment
attribution. The natural home is beside `ReplayStructure::steps` in
`crates/profile/src/structure.rs`, which already records
`FilletDecision`s and per-segment `SegmentShape`s (the latter in
CANONICAL indices, which is why it is not usable here as it stands).
Reading the shape back off the geometry is not an option: DM8 rules
that this map reads the records the evaluation produced and never
re-derives them.

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
