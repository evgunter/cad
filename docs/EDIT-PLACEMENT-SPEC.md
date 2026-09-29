# EDIT-PLACEMENT — one parametric `Placement` for the node and the registry

Unit for `work/edit/placement-is-spelled-three-ways-node-registry-and-rule.md`,
on Ev's ruling on `[ev]` #3437: a placement is parametric (A11 rule (2),
`crates/editor-core/ASSEMBLY.md`). Kernel unit, dual review under `docs/DUAL-REVIEW-PROTOCOL.md`
(two Opus reviewers on one frozen head); the implementer runs on Opus. Every premise
below was read off `origin/main` at `0b5cf7743` by a survey. Treat each
one as a hypothesis, verify it before building on it, and report any
correction. This program's recent specs have each carried at least one
false premise.

## What the unit is

One type, `Placement`, beside `Frame` in `crates/editor-core/src/placement.rs`.
`Node::Transform` and `Doc::placements` both hold it, and it evaluates to
a rigid motion at a parameter environment. The unit does not decide
whether `Transform` stops being a node. That question stays open on the
row.

## Premises (read, not yet verified by you)

1. `Node::Transform { input, translation: [Expr;3], rotation_axis: [Expr;3], rotation_angle: Expr }`
   (`node.rs` ~2134). Its slots are `Translation(Axis3)` (Length),
   `RotationAxis(Axis3)` (Scalar) and `RotationAngle` (Angle). It
   evaluates in lane T through `wire_transform` → `eval::transform_map`
   (`wire.rs` ~4067). The mate solve builds the same map at f64
   (`mate/member.rs` ~678).
2. `Doc::placements: BTreeMap<RecipeNodeId, placement::Frame>` (`doc.rs`
   ~748). `Frame { columns: [[f64;3];3], translation: [f64;3] }` is a
   general linear part, deliberately not axis-angle, so that an A6
   improper frame can be represented in order to be refused
   (`placement.rs` ~59–120). The writers are `DocEdit::SetPlacement`
   (door `doc::placement_fault`) and cluster maintenance (`reconcile`,
   `solve.rs` ~1565), which mints a frame by composing onto a solved pose.
   The mint must keep the new gauge's pose exact in bits (`solve.rs` ~1255).
   So a minted row is a general matrix, and it cannot round-trip through
   axis-angle.
3. The registry is consumed at f64 only:
   - `SolvedPoses::placement` (`solve.rs` ~143);
   - `pair_left_factor`, which conjugates by the registry frame (`solve.rs` ~938);
   - `wire_instantiate_part`, which lifts it with `affine::<T>()` (`wire.rs` ~413);
   - refactor's split (copies rows verbatim) and inline (COMPOSES the host frame onto them, `refactor.rs` ~2119);
   - the Python binding;
   - the load door (`persist/check.rs`).
4. No slot addresses a registry row today. `SetParam`/`SetExpression`
   go through `node.expr(slot)`, and `InstantiatePart` has no slots.
5. No committed wire file carries a non-empty registry, a `SetPlacement`
   or a maintenance row, so the registry's type change moves no golden
   or corpus bytes. `Transform` appears 21 times in
   `corpus/tour/die_composed_tour.pncad`. If its field layout moves, that
   file is regenerated from source.

## Rulings (the orchestrator's)

1. **The type.** `Placement { translation: [Expr; 3], rotation: Rotation }`
   with `Rotation::AxisAngle { axis: [Expr; 3], angle: Expr }` (parametric,
   proper by construction) and `Rotation::Columns([[Expr; 3]; 3])`.
   `Columns` is admitted **literal-only**: every entry is an `Expr`
   literal. It carries what only a matrix can: maintenance mints, the
   Python `point_at` / `path_start_frame` / `mirror_across_plane`
   constructors, and the viewer's future free-move release. A6's
   refusals keep their meaning: a `Columns` placement is refused where
   its literal matrix is improper or non-finite. `AxisAngle` is refused
   where its axis evaluates degenerate.
2. **One evaluator.** `Placement::eval::<T>(&ParamEnv<T>, ..)` returns
   the rigid motion. `AxisAngle` goes through `eval::transform_map` (the
   node's existing construction, so the node's bits do not move).
   `Columns` goes through the literal matrix. `Frame` stays as the
   evaluated, concrete value the solver composes. `Placement::literal(&Frame)`
   is the one bit-exact `Frame` → `Placement` door.
3. **Both holders.**
   - `Node::Transform` holds `{ input, placement: Placement }`, and its
     door refuses `Columns` (the node's slots address axis-angle
     components). Its existing `SlotId`s keep addressing the same
     components.
   - `Doc::placements` holds `Placement`, and `ClusterMaintenance` rows
     hold `Option<Placement>`.
4. **A slot address for a cluster placement.** `SetParam` and
   `SetExpression` can address a registry placement's component, keyed
   by an INSTANCE and resolved to its cluster's gauge at the door
   (`gauge_of`), because the key moves under maintenance. Reuse the
   `Translation` / `RotationAxis` / `RotationAngle` roles and dimensions.
   Editing a component of a cluster with no row first writes the
   identity as `AxisAngle` literals. A `Columns` row refuses a
   component edit typed (it is literal-only). The recourse is
   `SetPlacement` with an `AxisAngle` placement. Parameter-reference and
   dimension checks run over registry placements at the edit door and
   the load door, as they do over node slots. Split's and inline's
   parameter walks cover them.
5. **Evaluation at the nominal.** `SolvedPoses` stores each cluster's
   evaluated `Frame`, computed at solve time from the solve's own
   environment, so `SolvedPoses::placement` stays total after the solve.
   `Doc::placement` evaluates at `param_env()` and becomes fallible, with
   a typed fault. A placement that fails to evaluate after a parameter
   edit faults its cluster's instances typed at evaluation. It is never
   treated as identity.
6. **No silent wrong answer in lane T.** Until MSOLVE decides how a
   parametric cluster frame enters the solve under a box or a seed, a
   cluster whose placement is not literal refuses typed in every
   non-nominal lane. It is never evaluated at the nominal and then
   lifted. `eval/mod.rs` ~3132 names that outcome as a silent wrong
   answer. File that follow-up on MSOLVE's slate with the seam named.
   A literal placement behaves exactly as today in every lane.
7. **Maintenance that re-mints a parametric row reports it.** When a
   gauge moves and `reconcile` mints a new row, it writes literals. That
   is A11 as ruled. When the row it replaces was parametric, the edit's
   `Applied.maintenance` carries a typed act naming the cluster and the
   parameters whose drive was dropped, so the loss is loud where it
   happens (DM7's posture). Inline, which composes a host frame onto a
   part's placements, does the same for a parametric row: it evaluates,
   writes literals, and reports.
8. **Persistence.** The registry's wire form, `SetPlacement`'s payload,
   the maintenance rows and `Transform`'s fields change. An old file
   refuses typed (`Unreadable`, with the regenerate recourse). It never
   loads with a different meaning. `Doc::bit_eq` compares placements
   through `Expr::bit_eq`. Literal f64 bits round-trip exactly, including
   the load-bearing `-0.0` (`placement.rs` ~391). Any golden or corpus
   file that moves is regenerated from source and named, with what moved it.
9. **Python.** `DocEdit.set_placement` takes a `Placement` (a `Frame`
   converts through `Placement.literal`). `Doc.placement(s)` answers
   evaluated `Frame`s and the stored `Placement`s. The census, tag
   inventory and stubs move with it. This is LIB's ground; announce the crossing.

## Rows the unit adds (each red on `origin/main`, then green)

- A document parameter drives a cluster's placement: `SetParam` on the
  instance's `Translation(X)` moves the instance at evaluation, and the
  mate solve reads the moved frame.
- A `Transform`'s evaluated motion is bit-identical before and after,
  over every corpus document that has one.
- A gauge move over a parametric cluster re-mints literals and reports
  the dropped drive. The same for inline.
- A non-literal cluster placement refuses typed in lane T. A literal
  one evaluates as before.
- A `Columns` component edit refuses typed. A `Columns` improper matrix
  refuses at both doors.
- An old file refuses typed.

## Seams

- **MSOLVE** (`mate.rs`, `mate/*`): `SolvedPoses`, `pair_left_factor`,
  `reconcile` and `maintain`. Announce the crossing on MSOLVE's log before
  dispatch. Ruling 6's follow-up is theirs.
- **LIB** (`pncad`, `pncad-py`) and **WIRE** (`eval/wire.rs`). Announce these crossings.

## Not in scope

- Whether `Transform` stops being a node.
- `PatternKind::Explicit(Vec<Frame>)`.
- The viewer's free-move release (DI5, unbuilt).
