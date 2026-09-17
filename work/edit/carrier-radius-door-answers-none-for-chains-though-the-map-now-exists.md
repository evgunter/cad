---
id: carrier-radius-door-answers-none-for-chains-though-the-map-now-exists
kind: issue
title: LoopProgram::carrier_radius answers None for chain loops though the step-to-profile-edge map now exists: the widening waits on the content-key attach
status: review
opened: 2026-09-16
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

**Not built, and why.** A fused step (`fillet_arc`, `arc_fillet`,
`arc_fillet_arc`) carries two or three radii and emits several
segments, and the span does not say which radius drew which. Those
steps answer nothing at either door, which leaves them exactly where
they were — unattached and unkeyed — rather than guessing. Filed as
`fused-arc-fillet-steps-have-no-per-segment-radius-address`.

**One spec premise corrected.** The RED row's spelling pair is
param → literal, not `10 mm` → `0.01 m`: display units never enter the
key by ratified design (D7), so the two unit spellings are one
expression and move nothing.
