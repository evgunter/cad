---
id: a-tangent-tip-is-its-own-state-and-line-to-is-checked-from-it
kind: unit
title: Directed gains its inherited flavour by type, and line_to gains the checked row after .tangent() and .cusp() (continue_to's kernel), at every door
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point
---


Ruled on PR 7251 (Ev, 2026-11-21). The text this implements is
`docs/PATHS-DESIGN.md` as PR 7251 leaves it (read at `7c8acfd225`):

- §2's `Directed` bullet, which gives two flavours "distinguished by
  type";
- §2 Binders, `.tangent()`;
- §2 Legs, where `line_to` is the one self-aiming leg that also departs
  a Directed tip;
- §3's row "`line_to(target)` after `.tangent()` / `.cusp()`" (TIER 0)
  and the TIER 1 note "the Point rows only";
- §6, "Representation", where the angle marker carries the
  authored-vs-inherited flavour.

## What is true when it is done

- **The flavour is a type.** The angle marker `HasAng`
  (`crates/profile/src/path.rs`, beside `NoAng`, under the sealed
  `AngMarker`) carries authored or inherited. `.tangent()` and
  `.cusp()` (`tangent_kernel`, `cusp_kernel`) return the inherited
  flavour. `.angle`, `.toward` and `.turn` (`turn_kernel`) return the
  authored one, and so does every `Angle`/`DirectedPlain` state. The
  `DynTip` enum and `TipState` (`crates/profile/src/path/program.rs`)
  gain the inherited Directed state. A test asserts that `Verb::states`
  is the same on the authored and the inherited state for every verb
  except `LineTo`.
- **`line_to` after an inherited direction is checked.** A row on
  `LineTo` at the inherited state takes a `Point`, `Start` or
  `StartArriving` target and runs `continue_to`'s kernels,
  `continue_to_point_kernel` and `continue_to_start_kernel` in
  `path.rs`, moved under the new row's name:
  - It checks `lands_on_ray` (banded, no lever) and refuses
    `PathError::ContinuationTargetOffRay` past ε_input.
  - A target behind the departure refuses `NonpositiveLeg`.
  - The emitted vertex is the authored target bit for bit.
  - `Start` refuses `SeamTurnWithinTolerance` when the seam is
    undeclared and zero-turn. `Start.arrives_tangent()` closes, with
    the D-shape's constructed joints `[0, 3]`.
- **`continue_to` is the same kernel.** `verb ContinueTo` stays and
  calls the kernels above, so for every target
  `continue_to(t)` ≡ `.tangent().line_to(t)`. A differential test
  asserts it, bit for bit, on vertices, carriers, constructed joints
  and refusal kinds. The test is deleted by
  `continue-to-and-the-bare-leg-end-rows-retire`, when one side goes.
- **The authored-director exclusion is unrepresentable.**
  `.angle(θ).line_to(p)` does not compile. At the wire,
  `Step::LineTo` after an authored director refuses
  `ReplayErrorKind::Transition` naming the state.
- **`.cusp().line_to(p)` is admitted.**
  - After a straight leg it refuses `CuspDoublesBackOverLeg`.
  - After an arc it builds a declared cusp, and it closes onto
    `Start`.
  - A row of each.
- **The doors:**
  - **editor-core.** `ProgramStep::Tangent` followed by
    `ProgramStep::LineTo(Point | Start | StartArriving)` resolves and
    replays. `ProfileProgram::lattice` admits it through
    `Verb::states`; nothing in `editor-core` enumerates the verb.
    `LineTo` with a trace target (`OnTrace`, `AtX`, `AtY`) after
    `Tangent` lowers to `Line(len)` along the inherited heading
    (`trace.rs`, the `ProgramStep::LineTo | ProgramStep::ContinueTo`
    arm). After an authored director the same step stays legal: its
    target supplies extent only.
  - **pncad's `Chain`** (`crates/pncad/src/trace.rs`) gains
    `tangent()`. Python's `Chain` (`pncad.pyi`, `py/path.rs`) gains it
    too.
  - **The typed Python door.** The inherited tip can take
    `line_to(point | Start | Start.arrives_tangent())`, and the
    authored one cannot. `surface_census.rs`'s `TipState` map covers
    the new state.
  - **The viewer.** `crates/viewer/src/sketch.rs`'s state description
    (the `TipState` match near "a leg end with a direction bound over
    it") covers the new state.
- **Tests that enumerate states** cover the new one:
  `crates/profile/tests/chain_shape_random.rs`
  (`(TipState::DirectedIncoming, Some(Step::Tangent))`),
  `crates/profile/tests/arc_onto.rs` and `test_support.rs`.

## Left for later units

- `End`: `arc-to-gains-the-end-mode`.
- `Radius` at a Directed tip:
  `the-fused-verbs-take-end-and-radius-at-every-directed-tip`.
- Deleting `continue_to`, the bare leg-end rows, `Step::ContinueTo`, the
  Python `Chain.continue_to` and the recourse re-words:
  `continue-to-and-the-bare-leg-end-rows-retire`.

This unit deletes nothing.

## Check first

1. **That respelling is bit-identical.** The designers read the kernels
   and ran nothing. Before building, push a throwaway probe to hosted
   CI with these differentials:
   - `line(len)` against `.tangent().line(len)` at a leg end, after a
     line and after an arc (`straight_continuation_kernel` against
     `tangent_kernel` and `line_kernel` in `path.rs`);
   - bare `fillet(r)` against `.tangent().fillet(r)` (`ray_extend` in
     `path/family.rs`, documented as ".tangent().fillet(r)'s exact
     emissions");
   - `fillet_arc` likewise.

   Compare vertices, carriers, constructed joints and the lowered
   segments. If any pair differs, stop and report to the orchestrator.
   The migration in the retiring unit assumes these are one act, and a
   difference is a design question, not something to repair here.
2. **The type ripple.** `HasAng` is named outside `crates/profile/src`
   at 8 sites: `crates/profile/tests/path_program.rs`,
   `path_property.rs`, `guided_replay.rs`, and
   `crates/pncad-py/src/py/path.rs`, whose `Directed` enum holds
   `Path<HasPos<Plain>, HasAng>` and `Path<HasPos<WithIncoming>, HasAng>`.
   Inside `crates/profile/src` it is named at 42 sites. Every
   heading-taking row in `transition_table!` (`line`, `fillet`,
   `fillet_arc`, `arc_to`'s Directed row, `tangent_arc_to`, the fused
   Directed rows, the NURBS legs) must become generic over the flavour.
   If the `transition_table!` macro cannot carry a second generic on
   the angle marker without restructuring its row grammar, stop and
   report the shape before restructuring it.

## Questions this unit needs answered (not decided here)

- Is the Python inherited tip its own class (say `PathTangent`), or an
  arm of `PathDirected`? With an arm, `.angle(θ).line_to(p)` is refused
  at run time instead of by the stub's types. The door's convention is
  one class per lattice state, but `PathDirected` already covers two
  `TipState`s. That makes it PYDOOR's call, asked before the Python half
  is written.
- Does `Chain` gain `cusp()` beside `tangent()`? The document layer has
  `ProgramStep::Cusp`.

## Rows

- Carries part of the parent.
- Retires nothing on other slates. Its census row changes BIND's
  ground (`crates/pncad-py/src/surface_census.rs`; the per-verb blind
  spot the designers found there, a missing bound row never flagged, is
  BIND's to file if it is not already).

## Ground crossed

- PYDOOR and BIND: `crates/pncad/src/trace.rs`, `crates/pncad-py/*`.
  Announce on their logs before landing.
- DOCTAIL and SELFREF: `crates/editor-core/src/program.rs`, only if a
  change is needed there.
- TRACE and SELFREF: `crates/editor-core/src/trace.rs`.
- CHROME, DOORS and VGEOM: `crates/viewer/src/sketch.rs`.

No schema change: `ProgramStep` gains no variant.

## Review

Tier **single (FULL)**: a type-level flavour on the public typed surface
and a new checked row at three doors. A reviewer has to run the
differentials and the refusals, and a read will not do. Difficulty **L**.
