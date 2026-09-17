---
id: fused-arc-fillet-steps-have-no-per-segment-radius-address
kind: issue
title: A fillet arc's radius never reaches the wall it drew: the radius binds on one step and the segments are credited to another
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
unambiguous. Every fillet shape fails one of those two, and the reason
is the same one in each: **the step a fillet's radius is authored on is
not the step its arc is credited to.**

**A `fillet(r)` is a tip-state BINDER.** `ProgramStep::Fillet` holds
`StepArg::Radius` and emits no segment at all — its `StepSpan` is
empty, and `edit_step_segments`'s attribution table classifies it
`Binds`. The arc it opens is emitted by the ARRIVAL step, which carries
no radius of its own. So there is no single step whose radius and whose
segments can be paired, and the arc's wall carries nothing.

**A FUSED step is the same binder, or an arrival with several
segments.** The count of radius ROLES a fused step holds is not fixed
at two or three: only the `Radius`, `Sweep` and `ArcLen` arc specs
carry a `CarrierRadius`/`CarrierRadius2` role at all, while a `Bulge`,
`Via` or `Center` spec carries a bulge, a through-point or a centre and
no radius. So a fused step over one of those three holds exactly the
fillet's own `StepArg::Radius`, `radius_arg` answers it, and its
spelling enters the content key like any other one-radius step's. What
keeps it out of the ATTACH is measured, not assumed, and it is one of
two things:

- `arc_fillet(spec, r)` authors the incoming arc carrier AND opens a
  fillet off it in one act, and the step is a BINDER exactly as
  `fillet` is: its recorded span is EMPTY, and the incoming arc, the
  fillet arc and the arrival leg are all credited to the ARRIVAL step,
  which holds no radius. Pinned by
  `edit_step_segments::a_one_radius_fused_step_attaches_to_no_edge`,
  whose fixture is `arc_fillet` over a bulge spec.
- `fillet_arc(r, spec)` and `arc_fillet_arc(spec, r, spec2)` are
  ARRIVAL steps: they emit the fillet arc AND the spec's arc, so their
  span is longer than one segment and the "one radius, one segment"
  rule drops them. That arm has **no row**, because the radius-less
  arrival specs are `Via` and `Center` (a bulge in the arrival position
  is refused by the replay lattice — `profile`'s `family::ArrivalSpec`
  is implemented for `Center`, `Via` and `Radius` alone, and the
  document door returns a `Transition` refusal for it, measured), and
  no fixture closing a loop through a `Via` or `Center` arrival off a
  fillet was authorable inside this unit's budget: the attempts refused
  with `NoCornerForFillet` — the fillet solver's two carriers did not
  meet. What would pin it is one closed chain with
  `FilletArc { radius, spec: Via { .. } }` in it, asserting
  `step_radii` answers one pair and `segment_radii` answers none.

A step holding MORE than one radius role — `arc_fillet_arc` over two
`Sweep` specs, say — is dropped a step earlier, by `radius_arg`
answering `None`; `edit_step_segments::a_step_with_several_radii_answers_no_radius`
is that row and only that row. It says nothing about spans and is not
the guard for this shape.

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
(`a_fillets_radius_is_a_program_answer_and_no_edges` and
`a_one_radius_fused_step_attaches_to_no_edge` are the rows that would
be replaced) and `crates/editor-core/tests/seat7_sweep_lowering.rs`
§2b. The missing arrival-step row above is the taker's to write first,
because it is the one shape of this finding nothing pins today.

Citations accurate at `edit/chain-radius-attach`'s head; the stable
halves are `radius_arg`, `StepArg::is_radius`, `LoopProgram::step_radii`,
`ProfileProgram::segment_radii` and `profile::ReplayStructure`.
