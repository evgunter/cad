---
id: a-chain-repeats-a-run-of-its-steps-with-each
kind: unit
title: A chain repeats a run of its steps with each k { steps }, drawn N times by the one pen
status: parked
opened: 2026-11-22
priority: P1
cost: H
parent: a-loop-cannot-repeat-inside-a-profile
blocked_on: [a-profiles-loop-list-repeats-a-loop-with-each]
---


Unit 4 of `a-loop-cannot-repeat-inside-a-profile` (Ev, PR 7263): a run
of one chain's steps, drawn N times by the one pen. Examples are a
rack, or a serrated edge.

## The ruled text it implements

`docs/PATHS-DESIGN.md` §2 *Composition*:

> `each k { steps }` inside the one chain, each copy starting where the
> last ended … The steps of a run leave the pen in the lattice state
> they entered in, which the door checks once with the count unknown.

And: "The braces say which steps repeat because those steps need not
read `k`."

## Done when (each a test)

- **Drawn by one pen.** A chain `at(p0)`, `angle(0)`, then
  `each k { line(a), turn(q), line(b), turn(-q) }` with `k = index(N)`,
  then the closing legs. It evaluates to one loop whose segments are the
  copies in index order, each starting where the last ended. The run
  reads nothing, which is the case the braces exist for. Its sweep is
  one body.
- **The door's check, with N unknown.** A run whose verbs leave the pen
  in another lattice state than the one they met is refused at the
  edit door and the load door with a typed `ProgramRefusal`. The
  refusal names the block, the state it entered in and the state it
  left in. One example is a run that ends on a pending `fillet(r)`
  whose arrival is outside the braces. The check walks the block's
  `StepForm`s once through `profile::chain_shape` / `chain_states`
  (`crates/profile/src/path/shape.rs`) and compares the `TipState`
  before and after. It reads no value, as `ProfileProgram::lattice`
  reads none.
- **Names.** Copy 7's flank piece is `Piece { step, role, member: [(k, 7)] }`,
  and its swept wall names it.
- **Written addresses.** A braced step has one written index in its
  loop, counted in written order with the braces flattened. Its
  `SlotId::Profile` and its id are those of the written step, and an
  edit of the written step changes every copy.
- **The reach rule and the stray-read refusal** behave as in unit 3,
  for a run's rows.
- **Census and persistence** as in unit 3. A document with no run is
  byte-identical.

## Check first

1. **The authored address** (`crates/editor-core/src/names/README.md`,
   "The address"; `AuthoredStep { prefix: Vec<StepShape> }` in
   `crates/editor-core/src/step_handle.rs`) is "the step's index in its
   loop and its program's shape up to that step, values erased". A
   block changes the prefix's shape. Decide by reading whether
   `StepShape` needs a block marker so that a step's address inside
   braces differs from the same step written out, and so that a
   reshape adding braces refuses a stale address. If the README's rule
   cannot be kept without changing what it decides, stop and report:
   it is a ratified page, and changing it is an `[ev]` PR.
2. **Pen-relative steps inside a run.** `trace::resolve_chain`
   (`crates/editor-core/src/trace.rs`) replays the prefix to lower a
   step that runs from the pen. The block has to expand before that
   replay, so copy `k`'s pen steps see copies `0..k`. Confirm the
   expansion order. If the prefix replay cannot see an expanded prefix,
   stop and report.
3. **A fillet whose corner spans two copies** (a run ending on a
   `fillet(r)` whose arrival is the next copy's first leg, the last
   copy's arrival being the chain's next step) is not settled by the
   ruled text. Under the check above, such a run either passes, because
   the pending-fillet state is the same at both ends, or refuses. Which
   one it does is the question in the cut. Do not build either reading
   until it is answered. If it is still open when this unit is
   dispatched, the unit lands with such a run refused, typed, and says
   so.

## Ordering with the smooth-continuation build (PR 7251)

Ev approved on PR 7251 that `tangent_arc_to` and `continue_to` go and
that `.tangent()` carries on smoothly. Its build deletes
`ProgramStep::TangentArcTo` and `ProgramStep::ContinueTo` at about 240
sites. The two builds are semantically independent. The block wraps
steps whatever their verbs, and the door's check reads `TipState`,
which PR 7251 says gains no new state. They share
`crates/editor-core/src/program.rs`, the vocabulary census and the
wire, so:
- this unit's fixtures use no `tangent_arc_to` and no `continue_to`,
  so neither build rewrites the other's tests;
- whichever lands second merges main and re-runs the census;
- if PR 7251's build is in flight when this unit is dispatched, this
  unit waits for it to merge rather than racing it on `step_handle.rs`'s
  exhaustive matches.

## Leaves for later

- The ring and `Next`: `a-ring-closes-each-k-from-its-start-point`.
- Nesting, `N = 0`, and a fillet across copies (questions in the cut).
- The viewer and the Python door, on VSEAM and PYDOOR.

## Rows and ground

- With unit 5 it closes the parent row's run half.
- It crosses the same ground as unit 3, plus `crates/profile`'s
  `chain_shape`, which it reads and does not change.
- Review: **dual**. The door's value-free check and the address rule
  are where a wrong answer silently names the wrong copy. Difficulty
  **H**.
