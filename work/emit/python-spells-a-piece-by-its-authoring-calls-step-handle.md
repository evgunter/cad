---
id: python-spells-a-piece-by-its-authoring-calls-step-handle
kind: unit
title: Python spells a profile piece by the step handle its authoring call returned
status: closed
opened: 2026-09-25
priority: P1
cost: M
branch: emit/python-step-handles
closed: 2026-09-29
---

## What

The second half of `profile-pieces-are-named-by-minted-step-ids`, split
off it (the coordinator's call on the build, 2026-09-25). The first half
gives every profile step a minted `StepId` and names a piece
`{ step, role }` (`crates/editor-core/src/names/README.md`, "N1, the
profile pieces"). Its Python surface is the raw one:

- `Doc.step_ids(profile)` answers the ids, one int per authored step;
- `Doc.pieces(profile)` answers each canonical segment's piece as opaque
  text;
- `DocEdit.set_program(node, outline, ids)` takes one kept id or `None`
  per new step;
- `band`, `band_pi`, `band_rim` and `meridian_vertex` take a piece's
  text;
- `SplitOutcome.step_map` / `InlineOutcome.step_map` pair the ids across
  a refactor.

So a Python author finds a piece by reading `Doc.pieces` at a canonical
position, which is the positional reading the rule exists to retire.

## Scope

- The authoring calls (`Open.at(...).line_to(...)`, the fillet and fused
  verbs, `circle`, `circle_split`) return a handle for the step they
  author, which the insert door's minted id backs once the program is in
  the document.
- A piece is spelled from that handle and a role (`Leg`, `RunIn`, `Arc`,
  `RunOut`, `Piece(k)`), and `set_program` keeps a step by its handle.
- `Doc.pieces` stays, as the reading for a caller that holds no handle.
- `pncad.pyi`, the binding census and the role-name rows move with it.


## The fork (2026-09-29)

The first lane stopped before writing code. The row says the handle is
"backed by the insert door's minted id", but a loop is a free value
that can be placed several times, and each placement mints different
ids. So what a handle denotes, and how it binds to an id, is an open
question.

The designer pair and its split are in the `[ev]` PR. Both designers
agree on the handle: an authored address (the step's index plus the
program's shape up to that step, values erased), bound to an id by
editor-core per placement.

Three questions remain for Ev:
- whether the author names the loop, or the lookup finds it;
- whether a role is checked by per-verb handle classes or at the
  `piece()` door;
- the form of `set_program`'s keep map.

## Ruled (2026-09-29, #3473)

Ev ruled on the three open points:

- **Loop:** the author states it. The lookup does not search for it
  ("the convenience … probably not worth the extra engineering").
- **Roles:** one per-verb role list lives in editor-core. The doors
  check it, and the Python handle's role accessors are generated from
  it.
- **`set_program`:** it keeps steps through one dict per loop,
  `keep=[{handle: StepId}, …]`, in outline order.

Everything else is as agreed: an `AuthoredStep` is index plus
value-erased prefix shape; `Doc.step(profile, loop, h)` and
`Doc.piece(profile, loop, h.role)`; typed `StepId` and `Piece`;
`step_map` is `dict[StepId, StepId]`; the binding lives in editor-core
(`ProfileProgram::step`, `LoopProgram::shape`). The row is now the build.

## Closed: PR 3481

Built as ruled.

**editor-core** (`src/step_handle.rs`):
- `LoopProgram::shape()`: the value-erased shape of a loop's program.
- `AuthoredStep`: the step's index plus the value-erased shape of the program before it.
- `ProfileProgram::step` and `ProfileProgram::piece`. The author states the loop. They refuse with `OffProgram`, `Unminted`, `StepIds` or `RoleNotDrawn`.
- `keep_grid`: turns the per-loop keep dicts into the stored grid.

**profile** (`structure.rs`):
- One `RoleList` per verb.
- The replay checks every role it draws against that list.

**Python:**
- `.step` on every chain state.
- Typed `StepId`, `Role` and `Piece`.
- Role properties generated from `RoleList::ALL`, with typed stubs.
- `Doc.step` and `Doc.piece`.
- `set_program(keep=[...])`, with `keep` required.
- `step_map` is now a dict.

**Fixed on the way:** the role-list check exposed that fused arrivals emitted by a later binder step were misnamed far from the origin at tight ε. The fix makes the Circle run out structural: the fused verb claims `RunOut` at the emission site. `rides` is now asked only for the Ray case.

**Filed:**
- `work/doctail/a-name-door-admits-a-piece-role-its-steps-verb-never-draws.md`
- `work/paths/run-out-riding-misreads-...` (the Ray arm)
