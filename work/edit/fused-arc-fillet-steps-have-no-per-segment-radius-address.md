---
id: fused-arc-fillet-steps-have-no-per-segment-radius-address
kind: issue
title: A fillet arc's radius never reaches the wall it drew: the radius binds on one step and the segments are credited to another
status: closed
closed: 2026-09-20
pr: 2892
branch: edit/radius-emission-record
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

## Ruled and spec'd (2026-09-19, EDIT orchestrator) — kernel unit, v6 dual, block EDIT-B2 slot 1, branch `edit/radius-emission-record`

**Ruling: the replay record says which segment each radius drew, and
DM8's map reads it.** `ReplayStructure` gains `radii: Vec<RadiusEmission
{ step, role: RadiusRole { Fillet, Carrier, Carrier2 }, segment }>`,
recorded at the moment each arc's bulge is set (a carrier arc at the
current step; a fillet arc at the BINDER step whose `radius` it is,
the pending fillet carrying that index) and reported by `into_record`
as what the pass emitted, exactly as `steps` is. `segment_radii`
answers every emission through the one permutation `profile_edges_of`
computes (a per-segment door factored out of it); the "one radius,
one segment" rule and `radius_arg`'s single-answer role in the attach
retire; `step_radii` yields every radius role of every step so
"attached ⊆ keyed" holds by construction and the moved corpus keys are
measured and listed. DM8's clause sentence is re-worded to the
records it now reads (a description the code moved). The seams are
PATHS's (`crates/profile`, the record and its emission sites — not
persisted) and BLEND's (`path/arc_fillet.rs`), announced with lines.
The full spec — premises, rows (the arrival-step row this row names
first), mutants, territory, verification — is `docs/EDIT-RADIUS-SPEC.md`
on main, deleted at merge and ledgered.

## Built (2026-09-19)

`ReplayStructure` gains `radii: Vec<RadiusEmission { step, role:
RadiusRole { Fillet, Carrier, Carrier2 }, segment }>`, recorded at the
moment each arc's bulge is set: a fillet arc at the step that BOUND it
(the pending fillet's chain-side meta carries that index), a carrier
arc at the step whose spec authored it — which for a fused verb's
incoming and arrival specs is that verb's own step, not the step being
lowered, since a `Radius` or `Via` arrival binds its anchor and its
director on later steps. An arc no radius argument drew records
nothing. `into_record` reports what THIS pass emitted, and
`replay_guided` compares the emissions it reproduced against the
record the way it already compared the spans (`Decision::RadiusEmission`,
`DecisionValue::Emission`).

`ProfileProgram::segment_radii` reads that record through the one
permutation `profile_edges_of` checks (both now go through a factored
`checked_records`), so a fillet arc's wall carries its radius and a
fused step's two or three radii each reach their own wall. The "one
radius, one segment" rule and `radius_arg` are retired;
`LoopProgram::step_radii` yields every radius ARGUMENT of every step,
so "attached ⊆ keyed" holds by construction. Two emission-shaped
refusals at the map: `SpanOffTheLoop` for a segment the loop does not
have, `RadiusNotAnArgument` (new arm) for a radius argument the step
it names does not hold.

Rows: the arrival-step row the finding named is authored and green —
a closed chain with `FilletArc { radius, spec: Via { .., Start } }`,
which the filed row could not close inside its budget; the four §5
rows it named are inverted; three profile-side rows pin the emission
record and two the guided comparison. Corpus keys: the dump at both
heads is byte-identical (355 node keys) — the feed widens only for a
step holding more than one radius argument, and the corpus authors
none, so the strict side of the inclusion row is authored rather than
found.

Not done here: nothing from the spec. `crates/profile/src/path/arc_fillet.rs`
(BLEND's) turned out not to need touching — every emission site is in
`path.rs` and `path/family.rs`.

### Fix pass (2026-09-19)

The v6 dual's two blinded reviews were both APPROVE-WITH-FIXES and
converged on five findings; the union was built under the EDIT
orchestrator's rulings, with both review branches merged
authorship-preserving and their probes folded into the suites they
belong to.

**The door is one walk, and both arms are checked.** `segment_radii`'s
CARRIER arm walked `0..segments` off the canonical record and read
neither the recorded span nor the emission list, so a record the
per-step door refused was answered here. Both arms now go through
`CheckedRecords::edges_of_step`, and a carrier record carrying
emissions refuses `CarrierRecordsEmissions` — a carrier form emits
none. An emission crediting a segment off the loop draws its own arm,
`EmissionOffTheLoop`, rather than a span refusal with a range the
record never carried; `RadiusNotAnArgument` no longer renders "carrier
radius radius"; both sentences are pinned whole in the F6 census.

**The exact-fit close joins the list.** `family::resolve_arc_close`'s
exact-fit arm routes through `record_fillet_arc`, so its arc is
re-read at the close like every other fillet arc; measured, the
`bulge * 2.0` mutant that passed the whole suite before is now
refused by the door. That closes
`work/paths/exact-fit-close-fillet-arc-is-never-re-read-for-its-stored-tangency.md`
(the duplicate `radius-r1` filed is folded into it and deleted).

**One address spelling.** `record_fillet_arc` is `record_radius` plus
the tangency re-read's own list; `PendingMeta::carrier_address` is
gone; `pending` and its meta are ONE `Option<(Pending, PendingMeta)>`,
so `take_pending` has one refusal instead of two and the window where
a fallible `current_step()?` left half of it written cannot exist.

**The vocabularies and the fence.** `ArcData::carries_radius` and
`spec_slots` are held to one answer, mode for mode and position for
position, by one row; the coverage corpus gained a `Radius`-arrival
fused chain so the guided fence reproduces all three roles, anchored
on `RadiusRole::ALL`; `fillet_arc(r, Radius)` and a `Via` close's
binder address get rows of their own. §5's seven-statement preamble
is `fixture::wall_row`.

Re-baselined: `fillet_stored_tangency`'s three corpus dump goldens, at
the three eps rows — the new corpus chain is an addition to the dump,
not an edit of anything in it. And
`generic_replay::no_corpus_row_escalates_at_interval`'s relay pin gains
the new row at `eps = 1e-12`: the path door relays
`carrier_circles_external` for it, because a fillet arc is tangent to
its carriers by construction and an `Interval` enclosure of the centre
separation straddles the classifier's edge — the same fact row 1 is
already pinned for on the internal side. Not an escalation; the
escalating set stays EMPTY. Hosted CI caught it, which a local run
could not: the lane did not build `--features interval` first.

## Closed (2026-09-20, EDIT orchestrator)

Built and merged as PR #2892 (kernel unit, v6 dual, block EDIT-B2 slot
1; sample #223, ordinals 4806/4807). The replay record now says which
segment each radius drew: `ReplayStructure.radii` is a decision like
`steps` beside it, a fillet arc credited to the step that bound its
radius and a carrier arc to the step whose spec authored it, and DM8's
map reads that record through the one checked permutation both of its
doors now share. The finding's own shape — a `FilletArc { Via }`
arrival in a closed chain — is a green row, as are a fused step's
two and three radii each on its own wall. Two spec premises fell to
the implementer before the build (a fused verb's carrier arc is
emitted when the arrival resolves, so "the current step" names a
step holding no radius — every role addresses the fused verb's own
step; two `Sweep` specs are unrepresentable, the three-radii chain is
`arc_fillet_arc(Sweep, r, Radius)`), and none to the dual: both
blinded reviewers were APPROVE-WITH-FIXES with no MAJOR, converging on
the guided fence's blind spot for `Carrier2`, the exact-fit close's
fillet arc that nothing re-read (both measured the PR's stated reason
for not filing it false), the refusal sentence's doubled word, and two
false sentences in the PR body; one reviewer alone found the carrier
arm of `segment_radii` skipping the span check the chain arm goes
through — the factoring minting the class it closed, one arm out. The
fix pass built the union: one checked walk for both arms, the
exact-fit arm through `record_fillet_arc` (closing the row filed for
it on PATHS's slate), one address spelling and one `Option` for the
pending fillet, the two mode vocabularies pinned against each other,
a `Radius`-arrival chain in the coverage corpus, and the §5 preamble
given one home. The spec is deleted and ledgered; the sample row is
in `docs/MODEL-AB-LOG.md`; the block record's slot-1 line is on
`edit/b2-block`.
