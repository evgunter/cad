---
id: carrier-radius-door-answers-none-for-chains-though-the-map-now-exists
kind: issue
title: LoopProgram::carrier_radius answers None for chain loops though the step-to-profile-edge map now exists: the widening waits on the content-key attach
status: closed
opened: 2026-09-16
closed: 2026-09-17
pr: 2804
branch: edit/chain-radius-attach
---

Disclosed by the DM8 unit
(`authored-step-to-canonical-segment-map-has-no-home`, PR 2759) and
filed at the disclosure rather than left in the PR body.

## The finding

`LoopProgram::carrier_radius` (`crates/editor-core/src/program.rs`)
answers `Some(radius)` for the complete-loop carrier forms and `None`
for every CHAIN loop. Until DM8 its doc gave the missing step→segment
map as the reason: a chain's arc steps carry their own radii
(`StepArg::CarrierRadius` and the arrival spec's twin), each addressing
one segment, and pairing those with the walls they swept needed a map
nobody had. That map now exists —
`ProfileProgram::profile_edges_of` — so the stated reason is gone
and the `None` stands on the OTHER reason the same doc gives.

## Why it was not widened with the map

The memo's guard on this channel is scoped at the ATTACH, not at the
key. `eval::content_key` writes a carrier radius's spelling whenever
any migrated verb declares a profile edge's radius into a field
(`param_source::operand_flow_bearing`), which is already true, so the
key cannot notice that chain radii are UN-attached. Widen the door to
answer per segment and the stale-token class reopens silently for
exactly those loops: a chain radius re-spelled value-preservingly
would be attached to a wall while its key still says the value alone.

So the two changes are not separable in the safe direction — chain
radii enter the content key in the SAME change that attaches them —
and only the second is safe on its own. That is the obligation the
door's doc states and this row schedules.

## What a taker does

1. Widen the door (or add its per-segment sibling) to answer a chain
   loop's per-step radii, addressed through
   `ProfileProgram::profile_edges_of`.
2. In the same change, carry the chain radii into `eval::content_key`,
   with the feed's sentence updated to match.
3. A row that re-spells a chain radius value-preservingly and asserts
   the content key MOVES — the stale-token shape the attach guard
   exists for.

Citations accurate at PR 2759's head; the stable halves are
`LoopProgram::carrier_radius`, `ProfileProgram::profile_edges_of`,
`eval::content_key` and `param_source::operand_flow_bearing`.

## Spec (2026-09-17, EDIT orchestrator) — middle tier, branch `edit/chain-radius-attach`

**Premises, verified against the tree.** `LoopProgram::carrier_radius`
(`program.rs` ~994) answers the loop's `Radius` slot for the two
carrier forms and `None` for a chain, with the attach obligation in
its doc. `param_source::profile_radius_tokens` (~577) maps each
canonical loop through `carrier_radius` — so a chain loop's walls get
no token today — and the per-edge attach stamps each loop's token on
the walls swept from it. `eval::content_key` (`eval/mod.rs` ~4000)
feeds `CARRIER_RADIUS` + the expression for each carrier loop under
`operand_flow_bearing(ProfileEdge(Radius))`, and its comment says
chain radii enter the key in the same change that attaches them.
`ProfileProgram::profile_edges_of(structure, naming, loop_, step)`
answers a step's `ProfileEdgeRef`s in the program's numbering (DM8).

**What lands, as ONE change — the two halves are not separable in the
safe direction.**
1. **The per-segment door.** `ProfileProgram::segment_radii(&self,
   structure, naming, loop_) -> Result<Vec<(ProfileEdgeRef, &Expr)>,
   StepSegmentsError>` (or the name that says it): for a chain loop,
   every arc step's radius expression (`StepArg::CarrierRadius` and the
   arrival spec's twin — say which `StepArg`s carry a radius by reading
   `StepArg::dimension`) paired with the profile edges that step swept,
   through `profile_edges_of`; for a carrier form, the loop's one radius
   on every edge (so one door answers both shapes and `carrier_radius`
   stays as the per-loop question). Straight steps answer nothing.
2. **The attach.** `profile_radius_tokens` becomes per EDGE: a chain's
   arc walls carry their own step's radius token; the per-edge attach
   stamps by profile edge, not by loop. Carrier loops attach exactly
   what they attach today (measure: no golden moves for a carrier
   document).
3. **The key.** `content_key` feeds each chain radius's spelling (tag +
   `feed_content_key`) beside the carrier feed, in program-step order,
   with the feed's comment rewritten: the obligation is discharged, and
   the sentence that says why the guard cannot see the difference
   stays true of a THIRD per-edge scalar someone adds later.
4. `carrier_radius`'s doc loses the obligation paragraph and points at
   the per-segment door.

**Rows (red first where marked).**
- RED first: re-spell a chain arc's radius value-preservingly
  (`10 mm` → `0.01 m`, same bits) and assert the sweep's wall carries
  the new spelling's token AND the content key moves — the stale-token
  shape the attach guard exists for. Today the key does not move.
- A carrier-form document's key and tokens are bit-identical before
  and after (the widening changes nothing for carriers).
- A chain with two arcs of different radii: each wall carries its own
  step's token, addressed through the published `ProfileEdgeRef`.
- A straight-only chain: no radius token, no feed, key unchanged.
- The door's refusals ride `StepSegmentsError` (a loop this program
  does not have; a record that does not describe it) — one row.

**Mutants, each named with the rows it reds:** widen the door and
attach without feeding the key (the RED-first row reds on the key
half); feed the key without attaching (the token half reds); feed
chain radii in canonical rather than program order (a row over a
reversed loop reds — write it); map every wall to the loop's FIRST arc
radius (the two-arc row reds).

**Territory.** `crates/editor-core/src/program.rs`,
`src/param_source.rs` (EDIT); `crates/editor-core/src/eval/mod.rs`
(**WIRE's** — `eval::content_key`; cross by announcement in the PR
body: the feed's own comment schedules this change); corpus goldens
that carry a chain profile with arcs (the content-key/name-digest
goldens may move — say which and why, and re-baseline with the reason
stated); `crates/editor-core/tests/*` (TCOST/TINT). Middle tier: one
opus style review with a correctness arm, then a fix pass.

## Built (2026-09-17, `edit/chain-radius-attach`)

All four halves landed as one change.

1. `LoopProgram::step_radii` — the PROGRAM-side question: each step's
   own radius expression, a carrier form's one at step 0 and a chain's
   per radius-bearing step. `ProfileProgram::segment_radii` — the
   RECORD-side one: those expressions paired with the profile edges
   their steps swept, through `profile_edges_of`, in program indices.
2. The attach is per EDGE end to end. `SweptOut::walls` keeps the
   record's second index (`Vec<Vec<Option<FaceKey>>>` — the revolve's
   `None` positions are no longer flattened away), `attach_swept` takes
   a token per canonical segment, and `profile_radius_tokens` lowers
   the per-edge expressions the profile's value now carries.
3. `content_key` feeds `step_radii`, so a chain arc's spelling moves
   the profile's key. The feed's comment states the invariant the guard
   now rests on: the key feeds the program's answer, the attach stamps
   the record-filtered subset of it, and a spelling cannot be attached
   without having been keyed.
4. `carrier_radius` keeps its per-loop question and loses the
   obligation paragraph; DM8's consumers line names the built door.

**Not built, and why.** No fillet arc's radius reaches its wall,
because the step a fillet's radius is authored on is never the step
its arc is credited to. A `fillet(r)` binder holds the radius and
emits no segment — the arc is the ARRIVAL step's, and that step holds
no radius. `arc_fillet` is the same binder with an incoming carrier
fused into it: measured, its span is EMPTY too. `fillet_arc` and
`arc_fillet_arc` are arrival steps emitting the fillet arc AND the
spec's, so their span is longer than one segment. Every one of them
answers no edge, which leaves those walls exactly where they were
rather than guessing; the one-radius shapes' spellings do enter the
key, which is the conservative side. Pinned by
`a_fillets_radius_is_a_program_answer_and_no_edges` and
`a_one_radius_fused_step_attaches_to_no_edge`, filed as
`fused-arc-fillet-steps-have-no-per-segment-radius-address`.

**The radius COUNT is not what keeps a fused step out.** Only the
`Radius`, `Sweep` and `ArcLen` arc specs carry a `CarrierRadius` role;
a `Bulge`, `Via` or `Center` spec carries none, so a fused step over
one of those holds exactly the fillet's one radius and passes
`radius_arg`. `a_step_with_several_radii_answers_no_radius` is the row
for the two-and-three-radius shapes and is not cited for the span.

**One spec premise corrected.** The RED row's spelling pair is
param → literal, not `10 mm` → `0.01 m`: display units never enter the
key by ratified design (D7), so the two unit spellings are one
expression and move nothing.

**One deviation stands, argued rather than scheduled.**
`ProfileValue::edge_radii` is computed in `wire_profile` from the
node's own inputs, carries no `Serialize`, and the replay record stays
`pub(crate)` on `ProfilePre` — so the value gains an answer, not a
witness, and WIRE's PP1/PP2 question about where a structure witness
lives is untouched.

## Fix pass (2026-09-17)

The style review's findings, built. `StepArg::is_radius` replaces the
hand-written three-variant roster in `radius_arg` with an exhaustive
match; `segment_radii`'s loop-shape test is `step_radii`'s own match;
`attach_swept` refuses a walls/tokens shape mismatch loudly instead of
attaching nothing; `edge_radii`'s `unreachable!` says why the state
cannot occur and its lookup matches the whole `ProfileEdgeRef`. The
review lane's probes are adopted as rows, including a chain
canonicalization that both reverses and ROTATES (no committed fixture
had a non-zero `start`), a mechanical inclusion row over the whole
corpus, and a revolve whose first leg lies ON the axis, which pins
that the attach addresses by position across the `None` its record
holds there. The loft's per-loop-per-segment walls are filed as
`work/issues/loft-walls-carry-no-per-edge-radius-address.md`.

## Closed (2026-09-17, EDIT orchestrator)

Built and merged as PR #2804 (middle tier: one opus style review with
a correctness arm, then the union fix pass). The radius attach is per
EDGE end to end: `LoopProgram::step_radii` answers each step's own
radius expression in program order, `ProfileProgram::segment_radii`
pairs them with the edges their steps swept through `profile_edges_of`,
`SweptOut::walls` carries one wall per edge, `attach_swept` stamps
`walls[i][j]` from `tokens[i][j]`, and `eval::content_key` feeds
`step_radii` — so a chain arc's spelling moves the profile's key and
reaches its wall, and carrier documents write byte-identical keys
(measured by a corpus dump at both heads: 2 of 355 per-node keys
move, both `declared_tangency`'s, whose fillet radius now enters the
key on the conservative side). One deviation ruled to stand:
`ProfileValue::edge_radii` carries the per-edge ANSWER (computed in
`wire_profile` from the node's own inputs, not serialised), while the
replay record stays on `ProfilePre` — WIRE's PP1/PP2 row is untouched.
The review (0 MAJOR, 2 MINOR, 3 NOTE) found the fused-step account
wrong (a `Bulge`/`Via`/`Center` spec carries no radius role, so a
one-radius fused step enters the key) and `attach_swept` quiet on a
shape mismatch; the fix pass corrected the account by measurement —
`arc_fillet` is a binder like `fillet`, its span empty, so it attaches
to no edge for the binder reason — and made the mismatch
`unreachable!` with its why; a `None` wall row (a revolve chain's leg
on the axis) and rotated-chain rows were authorable and are in. The
hand-written radius roster became `StepArg::is_radius`, an exhaustive
match. Filed: `fused-arc-fillet-steps-have-no-per-segment-radius-address`
(the one-radius fused ARRIVAL arm over `Via`/`Center` has no row — the
fixture that would pin it is named) and
`work/issues/loft-walls-carry-no-per-edge-radius-address` (the loft's
walls are per loop per segment with no flow row today). Territory
crossed by announcement: WIRE's `eval/{mod,wire,anchor}.rs` and
`verbs/src/flow.rs`, TCOST/TINT suites, the DM8 consumers line.
