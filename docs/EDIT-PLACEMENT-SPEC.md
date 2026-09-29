# EDIT-PLACEMENT: gauges, and a placement that is parametric

This is the unit slate for `work/edit/placement-is-spelled-three-ways-node-registry-and-rule.md`. It follows Ev's rulings on `[ev]` #3437 and `[ev]` #3441, both from 2026-09-29, which are now A9 and A11 (2)–(5) in `crates/editor-core/ASSEMBLY.md`. Read those two clauses and the row's `RULED` section first: they are the contract. The designer reports behind them are rows 17–19 of `docs/DESIGN-FORK-LOG.md`.

The ruling reaches the data model, the mate solve, evaluation, persistence, export, Python and the viewer, so it ships as three units in order. Each one leaves main green and correct. None of them builds half of a later unit.

| # | Unit | Branch | Review |
|---|---|---|---|
| P1 | The `Placement` type, held by `Node::Transform` | `edit/placement-type` | dual (`docs/DUAL-REVIEW-PROTOCOL.md`) |
| P2 | Gauges; the registry and maintenance go; own space; STEP refusal | `edit/placement-gauges` | dual |
| P3 | Viewer: the group-wide probe and "place where shown" | the viewer owner's | filed on its slate, not EDIT's |

Every premise below was read off `origin/main` by earlier surveys and may have moved. Treat each one as a hypothesis, verify it, and report any correction. This program's specs have each carried at least one false premise.

## P1: the `Placement` type (spec)

**What.** One type, `Placement`, beside `Frame` in `crates/editor-core/src/placement.rs`: an ordered chain of steps, composed left to right.
- `Step::Rigid { translation: [Expr; 3], axis: [Expr; 3], angle: Expr }`: the three components `Node::Transform` holds today. It is proper by construction; its axis is decided at evaluation by the direction door, as `Transform`'s is.
- `Step::Matrix(Frame)`: a literal proper frame, held to A6 (`Frame::admission_fault`) at every door that writes one and at load.

`Node::Transform` holds one `Placement` (`{ input, placement }`), so a body can take a `point_at` frame too. P1 does not touch `Doc::placements`, `InstantiatePart` or maintenance; P2 removes them.

**Premises.**
- `Node::Transform { input, translation: [Expr;3], rotation_axis: [Expr;3], rotation_angle: Expr }` is at `node.rs` ~2134. Its slots are `Translation(Axis3)`, `RotationAxis(Axis3)` and `RotationAngle`.
- It evaluates in lane T through `wire_transform` → `eval::transform_map` (`wire.rs`).
- The mate solve builds the same map at f64 (`mate/member.rs`).
- `Transform` appears in `corpus/tour/die_composed_tour.pncad`. That file is regenerated from source if the wire shape moves.

**Rulings.**
1. **One evaluator.** `Placement::eval::<T>(&ParamEnv<T>, …)` returns the rigid motion.
   - A `Rigid` step goes through the node's existing construction, so a single-`Rigid` `Transform`'s bits do not move.
   - A `Matrix` step goes through the literal frame.
   - `Placement::literal(&Frame)` is the one bit-exact `Frame` → `Placement` door.
2. **Slots.**
   - A `Transform`'s components are addressed per step. A single-step placement keeps today's `SlotId`s addressing the same components. Adding the step index is the unit's call; justify it, and keep the census exhaustive.
   - A `Matrix` step has no expression slots.
   - The parameter-reference and dimension checks run over every `Expr` in the chain, at the edit door and at the load door.
   - Split's and inline's parameter walks cover them.
3. **No silent wrong answer in lane T.** A `Transform` whose chain holds a `Matrix` step is literal in that step and evaluates in every lane. The mate solve still runs at the nominal only. That class is filed as `work/msolve/a-mate-through-a-parametric-placer-is-solved-at-the-nominal-in-box-and-seed-runs.md`, and P1 does not widen it.
4. **Persistence.**
   - `Transform`'s wire shape becomes the chain. An old file refuses, typed (`Unreadable`, with the regenerate recourse); it never loads with a different meaning.
   - `Doc::bit_eq` compares placements through `Expr::bit_eq`. Literal f64 bits round-trip exactly, including `-0.0`.
   - Every golden or corpus file that moves is regenerated from source and named, together with what moved it.
5. **Python.**
   - `Transform` authoring takes a `Placement`, and the existing three-component form stays as sugar for one `Rigid` step.
   - `point_at` and `path_start_frame` give a `Placement` with one `Matrix` step.
   - The census, the tag inventory and the stubs move with it. This is LIB's ground; announce the crossing.

**Rows.** Each is red on `origin/main`, then green:
- Every corpus `Transform`'s evaluated motion is bit-identical before and after.
- A `Transform` over a `point_at` frame evaluates to that frame exactly.
- A two-step chain composes in order.
- A `Matrix` step that is improper or non-finite is refused, typed, at the edit door and at load.
- A parameter drives a `Rigid` step's angle.
- An old file is refused, typed.

**Seams.**
- WIRE: `eval/wire.rs`.
- MSOLVE: `mate/member.rs`, where the f64 map is built.
- LIB: `pncad`, `pncad-py`.
- Viewer readers of `Transform`'s fields.

Announce each crossing.

## P2: gauges (outline; spec'd in full when P1 merges)

The ruled final state, from A11 (2)–(5) and A9:
- **Gauge.** A document node kind holding a `Placement`, relative to the world. Whether gauges nest (inline turns an instance's placement into a gauge under the host's) is decided in P2's spec.
- **Instances.**
  - `InstantiatePart` gains `gauge` (the world by default) and an optional `offset: Placement`.
  - Insert writes an identity offset, so an absent offset arises only by deletion or on purpose.
  - A group's root is its earliest member that has an offset.
  - World pose = gauge frame ∘ offset(root) ∘ the solved relative pose.
- **What goes.**
  - `Doc::placements`, and `ClusterMaintenance` with `reconcile` and `registry_after`.
  - The edit door's maintenance solve, and `LoggedEdit`'s rows.
  - `EditError::MaintenanceRefused` and `MaintenanceUnrecorded`, and `PersistError::MaintenanceFrame`.
  - `apply` keeps the reach only for a mate insert's clocking rider.
- **Edits.**
  - `SetPlacement` writes an instance's offset.
  - `SetGauge`.
  - "Copy x's gauge to y, then mate" is one compound edit that re-gauges y's whole group.
  - Deleting a gauge, a placed member or a placing mate is never refused.
- **Mates.** A mate places only between instances on one gauge. Across gauges it declares, and it is verified at the at-rest gate. A further offset inside one placed group is verified against the solve, never ignored.
- **Unplaced.** A group whose chain does not reach a live gauge:
  - is evaluated in its own frame, with its earliest instance at the origin;
  - is not compared with anything outside it (the gate's cross-instance interference and cross-group measures do not ask);
  - solves and checks internally as usual.
  - STEP export refuses unplaced parts, naming how to place them.
  - How a deleted gauge's references are held (kept, so the refusal can name the cause, or cleared) is P2's call, with the reason.
- **The seven `EditError` placement arms** held on `edit-refusals-short-of-the-shape-guard` are rewritten or removed here.

## P3: viewer (filed, not EDIT's)

- G3's free-move probe is widened to a whole group.
- A group that becomes unplaced is drawn where it was last shown: the probe is seeded from the prior draw and discarded when the group's key changes, undo included.
- "Place where shown" is one edit whose frame comes from the gesture.
- This is filed on the viewer owner's slate when P2 merges, citing this file.
