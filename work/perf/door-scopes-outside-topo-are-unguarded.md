---
id: door-scopes-outside-topo-are-unguarded
kind: issue
title: what holds a surgery scope closed, and the three populations nothing holds
status: open
opened: 2026-09-10
---

PERF-4 moved D1's whole-body tier-1 sweep from every Euler operator to
every public door. Three mechanisms keep that honest, and this item is
the part of the surface none of them reaches.

**What fires.**

1. **The borrow.** A scope opened with `Body::begin_surgery` hands back
   a `Surgery` guard that owns the `&mut Body`; the only ways to
   release that borrow are `sweep_and_close`, `close_already_checked`
   and `Drop`, and all three decrement. A guarded scope cannot be left
   open, in any crate. Every scope in the tree is guarded except two.
2. **A lexical read.**
   `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`,
   over `source_walk::MutationDoor::surgery_posture`, reds on a door
   that opens a scope and closes nothing.
3. **A runtime read.** `Body::open_surgery_scopes` is public and
   returns 0 in a build with debug assertions off. The two guardless
   sites are held by it, as a `debug_assert` at the next phase boundary
   (`boolean::finish::setopfinish`, `splitting::finish::split_finish`)
   over a body the pipeline owns, plus
   `surgery::tests::every_door_returns_with_its_scopes_closed`, which
   drives a split and a boolean and asserts the result bodies came back
   at depth 0. Deleting either surviving `leave_surgery_and_sweep`
   reds that row — verified by mutation, both sites, 2026-09-11.

**The residue, in three parts.**

**(a) The `Err`-arm close of the guardless pair is undetected.**
`BooleanReduction::leave_join_surgery(false)` — the two
`leave_surgery` calls on the join's refusal path — can be deleted with
nothing red (verified by mutation, 2026-09-11). It is also harmless on
today's paths, and the two facts have the same cause: on that path each
operand body is either replaced by a pristine clone (the REST lane,
`boolean::ops`) or dropped with the reduction, so no surviving body
carries the leaked depth. A future path that kept a refused operand
would inherit a silent hole, and nothing would say so.

**(b) The lexical read's population and its blind spots.** It walks
`pub fn` items taking `&mut self` / `&mut Body` in `crates/topo/src`
only (`source_walk::crate_sources`; that function's own docs carry the
whole inherited blind-spot list). **Eight scopes are outside it** —
`sweep::extrude`, `sweep::loft::assemble`,
`sweep::revolve::{build_lamina, build_wire, build_partial}`,
`sweep::blend::surgery::blend_surgery`,
`step_import::assemble::build_one_solid`, and
`topo`'s own non-`&mut Body` pipeline phases, which are private
functions the walk does not treat as doors. All of them are guarded, so
(1) covers the leak; what is uncovered is the OTHER obligation — that a
composite door in another crate opens a scope at all, or that a
non-sweeping close is really backed by a debug assertion. A new
composite door in `sweep`, `step-import` or `editor-core` that simply
pays the per-operator sweep, or that closes without sweeping over a
typed gate, has no guard with an opinion.

Within its population the read is lexical and says so: two opens
against one close reads as closed, and a close on one branch only reads
as closed on every branch.

**(c) A DELETED SWEEP is not a leaked scope.** Every mechanism above
watches the DEPTH. A close that decrements but no longer sweeps —
`sweep_and_close` turned into `close_already_checked`, or a
`leave_surgery_and_sweep` turned into `leave_surgery` — leaves the
depth exactly right and the check gone. What catches that is a
corruption row per door class, and there are three
(`surgery::tests::{a_corruption_planted_mid_sequence_is_caught_when_the_door_closes,
an_operator_called_directly_still_sweeps, only_the_outermost_close_sweeps}`)
plus `shell10_r2_probes`'s. No door outside those has one.

**What would close this.** For (b): a shared door walk the other
crates' suites can mount — `source_walk::mutation_doors` is
`pub(crate)` and its floor assertion is `topo/src`-specific, so this is
a move into `test_utils::source` with a per-crate root, not a
re-export. For (a) and (c): a per-door corruption row is the only
instrument, which is a row per door rather than a guard, and the right
question is which doors earn one.

Not PERF-4's: the unit's fence is TOPO territory and SEAT/BLEND ground
in `sweep` (`docs/PERF-4-SPEC.md` §5), and a walk that reads four
crates is a `test-utils` change with its own evidence.
