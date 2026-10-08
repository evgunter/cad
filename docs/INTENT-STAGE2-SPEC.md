# SPEC: INTENT stage 2 — operations and one dependency

Scope: D10's **Operations** paragraph (`docs/DESIGN.md` §D10), as `work/intent/plan.md` stage 2 states it:

- Nodes read and define variables.
- Consuming edges, reading edges, name references and `Measure` refs become reads.
- A `Select` operation defines `Face`/`Edge` variables and owns the N5 ladder.
- The product becomes the world, and A10's sink rule retires.

**Rulings this spec follows.** Ev's rule that the redesign text governs earlier text applies throughout.

- **FORK-2b** (#4220): A10 is rewritten as "the product is the world".
  - The product is every copy a world placement (`PlaceInWorld { body, pose }`, an operation reading one `Body` and defining its copy) defines.
  - Nothing places as a side effect, and an empty world is an empty product.
  - Python and the Rust façade place only when told, with one semantics. The viewer's feature gestures re-point.
  - A second identity placement is allowed.
- **FORK-4** (#4221): DM6 is rewritten as "no edit infers a re-point". An operand slot is written by the one slot door.
- **FORK-5** (#4218): D10 says a `Measure` defines an *observed* variable, read only by an assertion, and a construction reads what was written.
- **FORK-1** (#4222, approved as written): each `Node` variant states a fixed list of named, typed output ports.
  - The kinds are the value kinds, the poses, the *shapes* (`Body`, `Bodies`, `Profile`), which only an operation defines, and the selections (`Face`, `Edge`, `Faces`, `Edges`).
  - A split defines two `Body` ports, a pattern one `Bodies`, and `Transform` takes its operand's kind.
  - An instance defines one `Body` per world placement of its part, its signature recorded on the node when the part is pinned.
  - `Assertion`, `Mate` and `Gauge` define nothing.
- **FORK-1b** (#4222, approved): D10's five poses stay (`Point`, `Direction`, `Axis`, `Plane`, `Frame`), and there is no `AxisInPlane` kind.
  - Both axis datums define an `Axis`, and `Revolve` defines `body: Body` and `axis: Axis`.
  - A pose kind names its symmetry as the mates' `Subgroup` (A11 (1)): `Frame` the trivial group, `Plane` the planar, `Axis` the cylindrical. `Point` and `Direction` name none until a reader needs theirs.
- **FORK-3** (#4222, approved): sets. A selection is a definition, not a node, stating its body once, and the selection kinds are `Face`, `Edge`, `Faces` and `Edges`.
- **The consuming-model holdover audit** (`audit/intent-consuming-holdovers`, hits H1–H14) is applied: §11 maps each hit to the unit that retires it.

D10's last paragraph retires two things here: **A10's sink rule** and **A12's reading edges**. Stage 2 also closes `a-measured-part-is-not-a-product-root` and `a-failed-requirement-refuses-the-whole-product`, and completes VR4's interim exception (a `Measure`'s arithmetic stays in the node "until stage 2 makes `Measure` an operation").

**Baseline.** The grep is at main `9eaf8eab2f`, when stage 1's PR C (#4146, a slot holds a `VarId`) was open and PR D (`Expr` holds no float) was unbuilt, so every count below is measured **before** them. #4146 has since merged (`68ecdb7f3`). Neither one touches an operand field, a name field, the root list or the mint preimage's operand spelling. The counts therefore stand, but the dispatching orchestrator re-greps at the merge of stage 1's D and states the drift.

The state of these mechanisms at the baseline:

- **Operands.** A node's operands are `RecipeNodeId` fields, and `Node::inputs()` (`crates/editor-core/src/node.rs:3375`) lists them as the DAG's edges.
- **Non-edges.** A mate's `SitedFace.at`, `InstantiatePart.gauge` and `Gauge.parent` are not edges. A12's `reading_edges` (`mate/solve.rs:841`) recomputes the mate ones.
- **Measures.** A `Measure`'s `refs[].at` ARE edges (`node.rs:3436`). That is the bug `a-measured-part-is-not-a-product-root` describes.
- **Names.** `StableName`s ride in payloads and are resolved mid-evaluation by `eval/wire.rs`'s three-rung `ladder` (`:2162–2294`). The full diagnostic ladder is `resolve/mod.rs`.
- **Roots.** `Doc::roots` is A10's ordered sink list (`roots.rs`). `VarKind` is `{Length, Angle, Scalar, Count}` and `VarDef` is `{Free, Defined}` (`var.rs:30`, `:79`).

## 0. Ordering: six PRs, each green, none landing half a representation

| PR | Unit (`work/intent/`) | Lands | Representation step it completes | Goldens |
|---|---|---|---|---|
| A | `operations-define-output-variables` | `VarDef::Output { node, port }`, the reference kinds, outputs minted at insert, persistence and the load walk | **an operation defines variables** | re-blessed: vars table and log grow, ids move. Equal up to an id bijection; content keys and geometry unmoved (Q6) |
| B | `operands-are-reads` | every operand field holds a `VarId` read of an output. `inputs()` is derived from reads. Kind-typed operand slots. Delete leaves readers unresolved | **reading is the only dependency** (for operands) | re-blessed: ids move. Roots and geometry unmoved |
| C | `the-product-is-an-explicit-list` (id kept; the unit is "the product is the world") | `PlaceInWorld { body, pose }` and a derived product. `roots.rs` and A10's invariants and maintenance retire, and so do `PlacedUnderTwoRoots`/N4's once-per-product rule, D-2's consumer-ward closure (narrowed) and `InstanceConsumed` | **the product is the world** | re-blessed. A one-time migration check: one placement per body-denoting root, in root order |
| D | `measure-is-an-operation` | one `Measure` is one primitive defining one *observed* scalar. Its arithmetic moves to a `Defined` variable, and `Assertion` reads a scalar variable. A construction slot reading an observed variable refuses | **VR4's exception closes** | re-blessed. Measured bits unmoved |
| E | `select-defines-face-and-edge-variables` | `Face`/`Edge` kinds and `Select`. Fillet/chamfer/shell/face-frame/measure names become reads, and the eval-time ladder moves into `Select` | **a name reference is a read** (outside mates) | re-blessed. Geometry unmoved |
| F | `a-mate-reads-face-variables` | mate sides read `Face` variables. `reading_edges`, the mates-are-not-edges carve-out and A5's minting lift through consumers retire | **A12 retires** | re-blessed. Poses unmoved |

Why this order:

- **A before B.** Readers need something to read. A is the checkpoint where the variable table states every operation's outputs, and every document equals its pre-A self up to an id bijection (Q6). Stage 1 had the same shape: its PR A added `Defined` before PR C made slots read it.
- **B before C.** A world placement is an operation reading one `Body` *variable*, so `Body` variables must exist and be readable first. B is behaviour-preserving: the read graph equals today's edge graph, so A10's sink set computed over reads is today's root set. C's one-time migration reads that root set.
- **C before D, E and F.** While A10 stands, a measure or a mate that becomes an ordinary reader makes its operand a non-sink, and the measured part or mated instance drops out of the product. That is the cut-plate bug, generalised to every assembly. Under the world rule a body appears only if a placement names it, whatever reads it, so C comes first.
- **The audit's refactor and gather changes ride C, not B.** #4220 says D-2's narrowed closure, `InstanceConsumed` and `PlacedUnderTwoRoots`/N4 change "with units B and F". All three are stated over world placements, which do not exist until C. "A remainder read of a cut body crosses when the cut places that body" has no meaning in B, so B keeps the two-way closure over reads, unchanged in behaviour, and C narrows it. A5's minting lift is F's, as #4220 says.
- **D before E.** D is the smaller of the two units that rewrite `measure.rs`. E then converts the refs D leaves as names.
- **E before F.** A mate side's face is a `Select` like any other. F is separate because the mate path resolves differently today: against the product table at the at-rest gate, not at the operand (`assembly.rs:1478`). Its refactor doors (split and inline, AQ8) are their own risk.

Each intermediate state is a whole representation:

- after A, outputs exist and nothing reads them yet, like stage 1 after its A;
- after B, every operand is a read, and names and the product are as today;
- after C, the product is the world, and measures and mates still read by `at` and name;
- after D, measures are single primitives that still name their faces;
- after E, every name except a mate's and a declared pair's (Q2) is a read;
- F leaves only declared pairs, which stage 4 deletes.

**Rejected:**

- **A+B as one PR.** It would be about 1,900 match and construction sites plus every golden with no checkpoint between them. A's bijection check is cheap, and losing it costs more than the extra PR.
- **C first, over node ids.** A placement would read a node id and be retyped in B, breaking the wire twice for no gain.
- **E and F as one PR.** That mixes the eval-time ladder's relocation with the at-rest gate's resolution and the refactor doors, and the reviewers would have to separate them again.

## 1. Final shapes (after F)

**Kinds and definitions** (`var.rs`):

- `VarKind` gains the poses (`Point`, `Direction`, `Axis`, `Plane`, `Frame`) and the shapes, which only an operation defines: `Body`, `Bodies` (an ordered list whose length is a `Count`) and `Profile`. It also gains the selections `Face`, `Edge`, `Faces` and `Edges` (FORK-3).
- A pose kind names its symmetry as the mates' `Subgroup` family: `Frame` → trivial, `Plane` → planar, `Axis` → cylindrical. `Point` and `Direction` name none, since the family has no entry for theirs yet.
- `VarDef` gains `Output { node: RecipeNodeId, port: u8 }` and `Select { body, names }`. A selection is a definition, not a node.
- A reference-kind variable has no free arm. It is never an analysis axis, has no unit and has no distribution.
- It reads as its definition. An unnamed one is shown as "the body of Extrude "base plate"".

**Operations.** Each `Node` variant states an output signature: a fixed list of named, typed ports set by its variant (FORK-1, #4222). The signature is `Node::outputs()`, from which `Doc::outputs(node)` reads.

- `Extrude`, `Boolean` and the rest each define one `Body`.
- `Revolve` defines `body: Body` and `axis: Axis` (FORK-1b). Until the axis line moves onto the node, the `axis` port is the lift of the axis the revolve reads.
- A datum defines its pose: `Plane` a `Plane`, `Point` a `Point`, `Frame` and `FaceFrame` a `Frame`, and both axis datums (`Axis`, `AxisInPlane`) an `Axis`. A `Profile` defines one `Profile`.
- `Transform` defines one port of its operand's kind: `Bodies` over a `Bodies`, `Body` otherwise.
- `Split` defines two, `above` and `below`, and DM3's split-half projection retires.
- `Pattern` defines one `Bodies`. One copy is picked by an index operation.
- `InstantiatePart` defines one `Body` per world placement of its part, a list fixed by the pin.
- `Measure` defines one observed scalar (§D).
- `PlaceInWorld` defines its copy, one `Body`.
- `Assertion`, `Mate` and `Gauge` define nothing.

`InsertNode` mints the outputs and the door returns them in `Applied`. `Doc::output(node, port) -> VarId` is the accessor.

**Operand slots.** Every operand field is `S` (stage 1's slot form): `VarId` stored, and authored as a `Formula` whose lone leaf is a `Var`, a `Name`, or `Output { node, port }`. A bare `RecipeNodeId` is authored sugar for port 0 (Q5).

- The slot's kind is the operand's kind (`Body` for a boolean's `a`, …). A wrong kind refuses at the door as `SlotVarKind`, the same check scalar slots use.
- Lists (`Union.members`, `Loft.profiles`) are `Vec<S>`.
- `Node::inputs()` is retired as a stored-field walk. It becomes `Doc::reads(node) -> Vec<VarId>` and `Doc::upstream(node) -> Vec<RecipeNodeId>`, the defining operations of those reads expanded through definitions. That is the one dependency relation the schedule, the content key, acyclicity, split closure and the viewer tree all read.

**Writing an operand** (DM6 as rewritten by #4221, "no edit infers a re-point"):

- An operand slot is written by the one slot door every slot has (`SetParam`). The formula lowers to a read of the slot's kind (`SlotVarKind` otherwise), the read is live, and the rewritten node passes DM5's distinctness and acyclicity over reads.
- `SetMembers` becomes that door on a list slot.
- The write reports, and never refuses, the names it strands (DM7), and `Rebind` repairs them.
- No door picks a survivor.

**Deletion** (D10: "deleting a variable leaves its readers unresolved, typed, never silently re-pointed"):

- `DeleteNode` of an operation with readers is accepted.
- Each reader of a removed output rides `Applied.maintenance` as `Maintenance::Strand`, which is DM7's report generalised from names to reads.
- The reader refuses at evaluation as `NodeErrorKind::UnresolvedRead { slot, var }`.
- `EditError::DeleteWouldDangle` and the load door's `DanglingInput` retire.
- `cascade_delete_order` stays, as the GUI's delete-with-dependents convenience.

**The product is the world** (A10 as rewritten by #4220; `product.rs`):

- **Representation.** `Node::PlaceInWorld { body: S /* Body */, pose: Placement<S> }` is an operation. It reads one `Body` and defines its copy as one `Body` output, the body at `pose`. The pose is the rigid chain `Transform` holds, identity by default.
  - The product is every copy a `PlaceInWorld` defines, in the placements' document order.
  - No list is stored, and no edit places or unplaces as a side effect.
  - Two placements of one body are two copies. A second identity placement is allowed, and the at-rest gate judges the coincident copies as interference.
- **What retires.** `roots.rs` is deleted (coverage, ancestor-freedom, `on_insert`, `on_set_members`, `on_delete`), along with `Doc::roots`, `DocEdit::SetRoots` and `RootFault`. The gather reads only the placements, so a Measure, Assertion or Mate can never gate it.
- **Empty and stranded.** An empty world is a valid document. A door needing a product refuses `ProductError::EmptyProduct`, naming the unplaced bodies, which replaces `NoBodyRoots`. A placement whose body is deleted is a stranded reader, and the gather refuses naming it.
- **Export** writes the world. It refuses an empty world, naming the unplaced bodies, and a world placement whose body is gone (A11 (2), #3441 narrowed).
- **Names.** `placed_under_two_roots` (`product.rs:1251`), `ProductError::Naming`'s once-per-product rule and N4's gather sentence retire. Each copy is an output with its own id, so a name reaches the product once per copy by construction (audit H9).
- **Who places.** The kernel edit has no default.
  - Python and the Rust façade place only when told (`doc.place(body, pose=…)`), with one semantics between them.
  - The viewer's feature gestures re-point the target's world placement to the result. That is an explicit slot write on the placement's `body` slot (DM6), recorded as one action.

**Measure** (D):

- `Node::Measure { primitive: MeasurePrimitive<S> }` holds one `Distance`, `Angle`, `MinClearance` or `Gap`. Its operands are slots, which read `Face`/`Edge` variables after E.
- It defines one scalar output of the primitive's dimension.
- `MeasureExpr` and `MeasureKind` are deleted: arithmetic over measures is a `Defined` variable (`Expr` already has `Min`/`Max`).
- `Assertion { value: S, bound: S, dir }` reads any scalar variable of the bound's dimension. `AssertionBoundFault::TargetNotMeasure` retires, and the dimension check stays.
- **Observed** (D10, #4218): a measure's output is an observed variable, and so is any definition reading one. Only an assertion reads an observed variable.
  - A construction's slot reading one, directly or through a definition, refuses at the door as `EditError::ConstructionReadsObserved { slot, var }`.
  - The load door refuses it in a new walk, `ObservedRead`.
  - Driven dimensions are deferred, not refused for good (Ev: "no need to support it now").

**Select** (E; FORK-3): a selection variable (`Face`/`Edge`, or the sets `Faces`/`Edges` that #4222 writes) is defined by `VarDef::Select { body: VarId /* Body */, names }`, a definition, not a node. Evaluation runs `eval/wire.rs`'s three-rung ladder (`live` → `Tied` → `Absent`) **in the select and nowhere else**. A failed select poisons its readers with the `ResolveError`, and the post-evaluation diagnostic ladder (`resolve/mod.rs`) diagnoses that select.

- Sets (FORK-3): `Fillet.selection` and `Chamfer.selection` read one `Edges` variable, and `Shell.open` reads one `Faces`, stating the body once. A selection authored at two sites is two variables, and the GUI offers the existing one.
- `Datum::FaceFrame { at, face }` becomes `{ face: S }`, because the body is the select's.
- `Measure`'s operands read selects.
- `Rebind` is addressed by body and name (`Rebind { body, from, to }`, #4222) and rewrites the selections of that body. The edit door's `payload_names` walk shrinks to the selects plus the declared pairs (Q2).

**Mates** (F): `SitedFace { at, name }` becomes a `Face` slot, and `FrameBase::Face` reads that slot.

- `mate::reading_edges` is deleted, and A9's partition runs over `Doc::upstream`.
- A5's minting lift through consumers retires (audit H10): `names::lift`'s consumer walk, `resolve_face` against the product table (`assembly.rs:1478`) and `MintRefusal::Reference`'s `MovedAbove`/`Vanished` arms. The replacement, from #4220:
  - a mate's declaration is minted on the world copies of its members, read through those copies' outputs (Q8);
  - a mate whose members are not both placed mints nothing.
- AQ8's "only a mate EDGE can cross" becomes "only a mate whose two face reads resolve can cross", which is the same predicate respelled.

## 2. PR A — `operations-define-output-variables` (cost M; ~40 files, 1.5–2.5k lines)

1. `VarKind` gains the poses and the shapes per FORK-1 and FORK-1b (`Point`, `Direction`, `Axis`, `Plane`, `Frame`; `Body`, `Bodies`, `Profile`). `VarKind::symmetry` names a pose kind's `Subgroup` family (`Frame` → trivial, `Plane` → planar, `Axis` → cylindrical; `Point` and `Direction` none). The selection kinds are E's. `VarDef::Output { node, port }`, with no authored twin: an output is never declared, only minted by its operation.
2. `Node::outputs()`, exhaustive per variant, so a new variant does not compile without its signature. `Revolve` has two ports, `body` and `axis`, and both axis datums define an `Axis`. An instance defines one `body` in A. The per-world-placement signature is C's, because world placements exist only from C (H11).
3. **Minting.** `InsertNode` draws each output's id after the node's and its steps', as `steps_of_insert` draws `StepId`s (`mint.rs`), under a new `OUTPUT_TAG` with the port as the index, each extending the chain once.
   - Each output is logged `Minted::Var`, ordered as minted (VR1, N1). The log's ordinal is its length plus one, and an insert's preimage holds its slot variables' ids, so **every id after the first output moves**, digests included (Q6, ruled 2026-10-08).
   - `InstantiatePart` defines one `body: Body` in A. The per-world-placement signature recorded on the node when the part is pinned is C's (§4), because world placements exist only from C.
   - `Transform`'s one port is `Bodies` when its operand's port 0 is `Bodies`, and `Body` otherwise. Every other operand refuses `WrongOperand` at evaluation, and B's slot-kind check refuses it at the door.
   - `SetProgram` and `SetMembers` mint nothing: a node's signature is fixed by its variant.
4. **Lifecycle.**
   - An output is not anonymous-GC'd (VR7's "read by something" is about anonymous free and defined variables). It lives exactly as long as its node.
   - `DeleteNode` removes the node's outputs. In A nothing reads them, so there is nothing to strand.
   - A name may be set on an output (VR2).
5. **Persistence.**
   - The vars table carries `Output` rows.
   - A new load walk, `OutputSignature`, refuses an `Output` row whose node is absent, whose port is outside the signature or whose kind disagrees, and also a live node missing one of its outputs.
   - The walk sits after `Vars` in `Walk::ORDER` (`persist/check.rs:250`).
6. **Analysis.** `free_vars` excludes outputs by construction, so axes, MC and stackup are unchanged. A scalar output (a `Measure`'s) is not seeded: `SeedOnDefinedVar` already covers it, renamed `SeedOnNonFreeVar`.
7. **Surfaces.**
   - `pncad` re-exports `Doc::output`.
   - Python gets `Doc.output(node, port=0) -> Var`, `Var.kind` gains the new kinds, and the census and `.pyi` follow.
   - The viewer panel does not list outputs. They show at their node.

**Sites.** `var.rs` (277 lines), `mint.rs` (`MintingEdit::InsertNode` and `steps_of_insert`), `edit.rs` `insert_into` (:5121) and `remove_unread` (:6232), and `persist/check.rs`. Every exhaustive `match VarKind` (grep `VarKind::Count =>`; the tags census in `pncad-py` `tags.rs`) needs the new arms, which the compiler finds.

**Goldens.** The vars table gains one row per node output, and `Minted::Var` entries join the log, in `tests/golden/golden.cad`, the 14 schema-bearing `bool13_goldens` (v6–v19), the four corpus `.pncad` files, `before_extrude_side/plate_param.cad`, `crates/pncad/tests/plate_param.pncad` and `crates/viewer/tests/gallery_ring.pncad`. Ids move. The check is a bijection: replaying each golden's edits, the new document equals the old one up to an id bijection with the outputs removed, and content keys (except the sites that hash ids as payload, which the PR names) and every geometry digest are bit-equal. The comparator is a test helper shown to go red on a mutant.

## 3. PR B — `operands-are-reads` (cost H; ~250 files, 6–9k lines, mostly compile-driven)

**Kernel**

- Operand fields become `S`. The authored-to-stored lowering (`lower_slot`, stage 1 PR C) gains an `Output { node, port }` leaf and the `RecipeNodeId` sugar, and checks the slot's kind.
- **The edit-door operand checks** move from `inputs()` to reads: liveness (`UnresolvedInput`), acyclicity (`check_acyclic`, `edit.rs:4723`), DM5's distinctness (`input_fault`, now over variable ids), and the declared-site check (`check_declared_sides`, `:4681`, now "a declared site is a variable this node reads").
- **The dependency relation.** `Doc::reads` and `Doc::upstream` replace `inputs()` at all 37 call sites (21 files):
  - `eval/schedule.rs` (2)
  - `eval/mod.rs` (2): poison, `MissingInput`, `upstream_keys`, `unplaced_below`
  - `edit.rs` (6)
  - `roots.rs` (3)
  - `refactor.rs` (2)
  - `mate/solve.rs` (2)
  - `persist/check.rs` (2)
  - `spoken.rs` (1)
  - `node.rs` (1)
  - the viewer's `tree.rs` (1) and `display.rs` (1)
  - 16 test calls across 12 files
- **Content key.** `upstream_keys` are still fed by content in read order, so the content key is unchanged in meaning. A re-pointed operand changes the naming key exactly as today (`naming_key`, `eval/mod.rs:5939`).
- **A10 over reads.** `roots.rs` keeps working, its "consumes" now meaning "reads an output of". C deletes it. Roots are byte-equal to before B on every corpus file, which is the PR's check (test 4).
- **Deletion** per §1. `DeleteWouldDangle` (`edit.rs:1266`, 3 sites plus `refactor.rs:1587`) and `DanglingInput` / `ForwardInput` (`persist/check.rs:1629`) retire, and `Maintenance::Strand` gains a read arm. `roots::on_delete`'s splice still runs until C deletes it.
- **Rewiring** (DM6 as #4221 rewrote it). Every operand slot is written by the one slot door (`SetParam`), under the kind, liveness, acyclicity and DM5 checks. Strands are reported as `Maintenance::Strand`, never refused.
  - `SetMembers` (`node.rs:3695`) becomes that door on a list slot. Its edit arm stays as the list spelling, lowered through the same checks.
  - The ruling's row is `an-operand-slot-is-re-pointed-by-the-slot-door` (closed). `recipe/set-members-admits-a-forward-member-the-save-validator-refuses` (filed with #4221) lands on the same door, and B's rewrite of `SetMembers` should close it.
- **Split and inline.** `refactor::remap_node` (`:1972`) re-points reads instead of ids. Its closure check (`:2370`) reads `Doc::upstream`, and a crossing output carries as stage 1's anonymous carry does: one fresh entry, minted on the far side. B keeps D-2's two-way closure unchanged in behaviour. C narrows the consumer-ward half, because the narrowing is stated over world placements.
- **The tube reads a `Frame`** (FORK-1b; `work/intent/tube-spine-reads-an-axis-origin.md`). `Tube` and `HollowTube` read one `frame: Frame` slot in place of `spine` and its three `u_ref` scalars. An `Axis` forgets the origin the tube takes as its centre, so the frame's own door decides `u_ref` (orthonormalized, refusing a degenerate pair), replacing the tube door's exact unit-and-perpendicular check. The PR states that change and rewrites the node's doc paragraph.
- **Not converted** (Q3): `Gauge.parent`, `InstantiatePart.gauge` and the mate sides. Gauges retire in stage 3, and the mate sides are F's.

**Persistence.**

- Operand fields serialize as `VarId`, and `wire::plane_ref` (`persist/wire.rs:404`) reads a plane slot.
- The mint preimage is the authored node, and its operands are now spelled as reads (mint.rs:31–33), so **every node id after the first operand-bearing insert moves**. Re-bless with `M4_PR6_BLESS_GOLDEN=1` and state it in the PR.
- A pre-B file refuses `Unreadable` with the regenerate recourse.

**Façade, Python, viewer.**

- Rust constructors take `impl Into<Formula>`, which `RecipeNodeId` implements (Q5), so the ~1,500 test match and construction sites mostly compile unchanged. The residue is struct literals: `Node::Extrude { profile: p, … }` becomes `profile: p.into()`, done by a sed over the field names listed in §1, with compile-driven residue.
- Python `Node.extrude(profile: NodeId | Var, …)` and the rest of the 23 constructors in `py/doc.rs:2180–3308` accept both forms. The 743 Python call sites are unchanged.
- The viewer's body-seat test (`denotes_body`, `combine.rs`) becomes the slot kind check.

**Sized** (grep `Node::<Variant> {` and helpers, at the baseline):

| Area | Match and construction sites | Helper calls |
|---|---|---|
| editor-core src | 574 | 7 |
| editor-core tests | 1,498 | 858 |
| viewer | 286 | 64 |
| tour | 48 | 32 |
| pncad-py src | 40 | 10 |
| pncad tests | 18 | 7 |

Datum `AxisInPlane` and `FaceFrame` add 29 sites in src, 41 in tests and 15 in the viewer. `demos/wild`, the tools and the benches have 0.

## 4. PR C — `the-product-is-an-explicit-list`: the product is the world (cost M–H; ~130 files, 3–4k lines)

The row keeps its id. Its title is restated as "the product is the world" (FORK-2b, #4220).

**Kernel**

- `Node::PlaceInWorld { body: S, pose: Placement<S> }`, per §1.
  - Its signature is one `Body`, the copy.
  - The pose evaluates as `Transform` does today (`topo::transform_rigid`).
  - Its content key is the body's upstream key plus the pose's slots.
  - Python gets `Node.place_in_world(body, pose=None)` and the façade convenience `doc.place(body, pose=…)`, one semantics in both.
- Delete `Doc::roots` (`doc.rs:816`, `:975`, `:1071`, `:1766`) and its `"roots"` wire field. A pre-C file refuses with the regenerate recourse.
- Delete `roots.rs` and its call sites in `edit.rs`: the backstop at `:5015`, `on_insert` (`:5196`), `on_set_members` (`:5315`), `on_delete` (`:6232`), Promote's slot insert (`:6030`) and `SetRoots` (`:485`, `:5933`).
  - Also delete `RootFault` and its recourse text (`:3238–3260`).
  - `resolve/mod.rs:2279`'s `strict_ancestors` moves to `Doc::upstream`'s closure.
- **The gather** (`product.rs`): `product_in` walks the `PlaceInWorld` nodes in document order (`:1076`).
  - `sources_of`'s non-body skip (`:772–786`) is deleted.
  - The `usable(root)?` that lets a failed check refuse the whole product (`:1080`) now reads only placement outputs. That closes `a-failed-requirement-refuses-the-whole-product`.
  - A placement whose body read is unresolved refuses `ProductError::StrandedPlacement { placement }`.
  - `checks.rs:1166` and `:1268` read the same placements.
- **Errors.**
  - `NoBodyRoots` becomes `EmptyProduct { unplaced: Vec<VarId> }` at `product.rs:200`, `checks.rs:983`, `eval/parts.rs:483` and `assembly.rs:1186`. Its recourse is "place a body in the world".
  - The unplaced bodies are the live `Body` outputs no placement reads, listed in document order.
- **Retired with the gather** (audit H9): `placed_under_two_roots` (`product.rs:1246–1257`), `ProductError::Naming`'s once-per-product refusal in `carry_names` (`:1487`), and N4's gather sentence in `names/README.md`. Each copy is its own output, so its names are qualified by the copy as a pattern copy's are.
- **Instances** (A2 as rewritten by #4220; FORK-1 for the signature, recorded on the node when the part is pinned). `eval/parts.rs:719` takes the part's world: one `Body` output per world placement of the part, each at its world coordinates.
- **Export** (`pncad` `export.rs:63`, `:261`) writes the world. It refuses an empty world, naming the unplaced bodies, and a stranded placement (A11 (2), #3441 narrowed by #4220).
- **Refactor** (`refactor.rs`; audit H3, H7, H8):
  - Split's anchor vote (`:2546`), `UnplaceableRoot` (`:2568`) and `NoMaterial` (`:2865`) read "the cut's world placements". "A cut of unplaced material alone refuses" (A4) stays, and is now the whole rule for a cut with no placement.
  - The part's world is the cut's placements (`:2889`), and the remainder places one copy of the instance (`:3048`). There is no splice "where the first was": order is the placements' document order.
  - **D-2's consumer-ward closure narrows** (#4220): a remainder read of a cut body crosses when the cut places that body, and is re-pointed to the instance's output for that copy. A read of a cut body the cut does not place refuses `SeveredEdge`, because a part delivers only its world. The ancestor-ward half stays. `refactor.rs:21–25`'s module doc, which states the A10 reason, is rewritten.
  - **`InlineError::InstanceConsumed`** (`:1139`, raised at `:3465` via `roots::consumer`) retires: a reader of an instance output re-points to the inlined body.
  - Inline's `UnplaceableFrame` (`:3333`) reads the part's placements.
  - A4's acceptance is as #4220 rewrote it.
- **REFERENCES DM4** (#4220): a deleted pip's transform stays a value, outside the product because nothing places it. `diefillet.rs:563–583` stops deleting its narration body.

**Migration** (a one-time check, not a rule). Regenerating a pre-C corpus file writes one `PlaceInWorld` per body-denoting root of its A10 root list, in root order, with the identity pose.

- There is one exception: a root `InstantiatePart` on the world gauge with no placing mate. Its offset moves into the placement's pose and the instance sits at the empty offset. The pose lives in the placement in stage 2, by Ev's residue 3; see Q9.
- The check is test 6: the regenerated file's product equals the pre-C product, body for body and in order.
- After that nothing preserves membership. A later edit's product is what its placements say.

**Surfaces**

- **pncad.** `document.rs` re-exports (`:352–390`).
- **Python** (82 sites in src, 13 in the `.pyi`, 94 in the tests):
  - `Doc.roots` and `DocEdit.set_roots` go.
  - `Doc.placements()` and `Doc.product()` read the world.
  - `root_fault_tag` goes, and `product_error_tag` gains `empty_product` and `stranded_placement`.
  - Every Python test that relied on an implicit product calls `doc.place(…)` once per product body. The migration check lists those sites.
- **Viewer** (119 sites in src, about 250 in the tests):
  - The root badge (`tree.rs:747`, `:776`) becomes a world badge on placed bodies.
  - `pickindex.rs:1024` and `display.rs:446`, `:514` read placements.
  - The pattern projections kept only to leave each copy a root (`session/op.rs:783`, `session.rs:2776`) go.
  - A feature gesture (combine, fillet, …) re-points the target's world placement to the result in the same action (#4220, residue 1). The 73 `combine_ops` and 38 `creation_ops` rows are restated to read the placements their gestures author.
- **Docs** (wording moved by the ruled change):
  - `docs/guide/assembly.md`: `:322–341` (mates as roots), `:1069` (`outcome.instance in outcome.remainder.roots`) and `:1227` ("a mate is a product root");
  - `crates/editor-core/README.md` and the DESIGN.md companion row.
  - ASSEMBLY A10, A12, A4, A2, A11 (2) and DM4 are already rewritten on main by #4220.
- **Tour** (`gallery.rs`, 27 sites):
  - `the_cut_plate_has_no_product` becomes `the_cut_plate_is_its_product`, placing the cut part. That is the acceptance of `a-measured-part-is-not-a-product-root`: the part appears because it is placed, whatever reads it.
  - The root-count rows (`:215–321`) are restated as placements.
  - The nine bars-and-pins rows that refused `PlacedUnderTwoRoots` now build two copies each.
  - `assembly.rs:483` and `:1284` read the first placement.

## 5. PR D — `measure-is-an-operation` (cost M; ~70 files, 2–3k lines)

- `Node::Measure` holds one primitive (§1). `measure.rs`'s `MeasureExpr` and `MeasureKind` (`:171`) are deleted, and the `Primitive` arms keep their refs as `SitedRef` until E.
- **Migration.** One measure with arithmetic becomes one `Measure` per primitive plus one anonymous `Defined` variable over their outputs.
  - The authored door keeps `Node::measure(expr, refs)` as a **builder** returning the edit list. It is the same edit-list shape split returns, so the 97 test call sites and 220 `SitedRef::new` sites change only where they destructure.
  - The Python `Node.measure` builder does the same, as a recorded batch.
- `Assertion.measure: RecipeNodeId` becomes `value: S` of a scalar kind (`node.rs:2770`, 28 src and 59 test sites). `assertion_bound_fault` (`:3565`) checks the dimension only.
- **Evaluation.** `wire_measure` (`wire.rs:2443`) evaluates one primitive. Its output is **observed** (D10, #4218).
  - A definition reading an observed variable is observed too. It is bound when its measures have run, and only by the assertions that read it, so `Doc::var_env` (`doc.rs:1651`) and every construction's environment are unchanged.
  - The analysis lanes bind an observed definition at the assertion, from the measures' values in that lane.
- **The observed rule.** A construction slot reading an observed variable, directly or through a definition, refuses at the edit door as `ConstructionReadsObserved { slot, var }`. The load door refuses it in a new walk, `ObservedRead`, after `DefinitionRead`.
  - The rule is a reachability test over `Doc::reads` and definitions, so a `Var` read deep inside a formula is caught.
  - Driven dimensions are deferred (Ev, #4218).
- **Content key.** The `Measure` arm (`eval/mod.rs:5743`) hashes `r.at` ids directly as payload. That is replaced by the upstream keys like every other operand, which fixes a key that today moves on an id-only change.

## 6. PR E — `select-defines-face-and-edge-variables` (cost H; ~200 files, 5–8k lines)

- `VarKind::{Face, Edge}` and `Select` per FORK-3. Its evaluation is `named_entity` (`wire.rs:2317`) plus the kind check that `BlendSelectionKind`, `ShellOpenKind`, `FaceFrameKind` and `MeasureSelectionKind` each make today. That is four error families folded into one, `SelectKind { expected, found }`.
- **Converted payloads:**
  - `Fillet`/`Chamfer.selection`: 12 src and 25 test non-empty sites, 25 in the viewer and 1 in the tour.
  - `Shell.open`: 11 src, 12 tests, 16 viewer, 4 tour and 1 py.
  - `Datum::FaceFrame.face`: 16 src, 37 tests and 10 viewer.
  - Measure refs: 20 `SitedRef::` sites in src and 252 in the tests.
- `StableName {` literals (266 in src, 341 in tests, 55 viewer, 9 tour) mostly stay: a select stores one.
- The authored sugar `Formula::select(body, name)`, and a `Vec<StableName>` given to a selection slot, lower at the door to one selection definition: one set variable under #4222's sets, or one per name under singletons. That keeps the 41 + 12 + 20 `Node::fillet/chamfer/shell(` test sites unchanged.
- **Not converted:** `Boolean.declare` and `Union.declare` (`DeclaredPair`, `node.rs:3192`; Q2).
- **The ladder.** `mod ladder` (`wire.rs:2162–2294`) is called only from `Select`'s evaluation. Its other callers (`:1392`, `:2150`, `:2345`, `:2475`) read variables, and the declared-pair callers (`:3281`, `:3382`, `:3919–20`, `:4139–43`) keep calling it until stage 4.
  - `resolve::resolve` and `resolve_with_prior` diagnose a failed select. Their callers are the appearance store (`appearance.rs:465`, `:495`, `:518`), the viewer's `session.rs:977` and `matetool.rs:575`, and `pncad-py` `value.rs:1610`.
- **`Rebind`** (`edit.rs:377`, applied at `:5673`) rewrites selects. `rebind_payload_names` (`node.rs:3744`) shrinks to the declared pairs. Its refusals keep their names.
- **SELECT-DESIGN.** The materializer doctrine stands: a select stores a name, never a query, and `select_where` still returns `Vec<StableName>` for the caller to store. §4's one-type rule ("a GUI selection is the `Vec<StableName>` a recipe stores") is FORK-3's to restate.
- **Viewer.** Picks become select edits. The pick-to-name inversion (`hit.rs`, `pick.rs`) is unchanged.

## 7. PR F — `a-mate-reads-face-variables` (cost H; ~120 files, 3–5k lines)

- `Mate { a, b }` become `Face` slots. `Node::Mate {` has 54 src, 115 test, 19 viewer, 6 tour, 2 pncad and 2 py sites. `SitedFace::` has 5, 2, 1, 1, 1 and 2.
- The member walk (`mate/member.rs:366` `member_of`, `:421` `walk`) starts at the select's `Body` read instead of at `at`.
- `head_face` (`:395`) is the select's name with the qualifiers stripped.
- `MateFault::DanglingHead` becomes the select's `UnresolvedRead`.
- `FaceUnresolved` (`solve.rs:1458`) becomes the select's resolve error.
- **The at-rest gate** (A5's minting lift retires, audit H10, #4220): `assembly.rs:1446–1545`, `resolve_face` (`:1478`), `names::lift` (`names/role.rs:1856`) and `MintRefusal::Reference`'s `MovedAbove`/`Vanished` arms are deleted.
  - A mate's declaration is minted on the world copies of its two members, read through those copies' outputs (Q8).
  - A mate whose members are not both placed mints nothing. Such a mate relates a boolean's operands in the workbench, and that is the boolean's coincidence door's business, not an at-rest fact of the product.
- **A12 retires.**
  - `reading_edges` (`solve.rs:841`) is deleted, along with its re-exports (`mate.rs:88`, `lib.rs:162`, `pncad` `document.rs:422`), its Python door (`py/mate.rs:1161`, `:1290`) and 11 test calls in 6 files.
  - `relative_freedom_components` (`:877`) runs over `Doc::upstream`.
  - `payload_read_sites` (`node.rs:4569`) shrinks to the declared pairs.
- **Refactor.**
  - `SplitError::OperandSeveredFromMate` becomes the ordinary severed-read refusal.
  - `is_mate_edge_end`, `frame_survives` and inline's `MateFrameCrosses` and `MatePairSplits` read the select.
  - `split-and-inline-over-a-mate-read-at-a-union-are-unmeasured` (MSOLVE, P3) gets its rows here, because F rewrites the four sites it names.
- **Content key.** `Mate`'s `a.at` and `b.at` payload hash (`eval/mod.rs:5608–5614`) becomes upstream keys.
- **Docs.** `ASSEMBLY.md` A3's `SitedFace` sentence, A12 (deleted, replaced by a pointer to D10) and AQ8's predicate wording. These retire per D10's last paragraph, so they need no fork.

## 8. What moves

| | A | B | C | D | E | F |
|---|---|---|---|---|---|---|
| Node ids, pinned hex, memo fixtures | — | **all** (the preimage spells reads) | placements and later | measures and later | selects and later | mates and later |
| Var table, mint log | grows (outputs) | — | — | grows (definitions) | grows (selects, if FORK-3 makes them variables) | — |
| Wire | vars rows | operand fields | `roots` goes; `PlaceInWorld` nodes | `Measure`, `Assertion` | payload names | `SitedFace` |
| Content keys | — | ids move, meaning unchanged | — | measure keys (no longer hash `at` ids) | — | mate keys (likewise) |
| Product | — | **byte-equal roots** | **equal, body for body**, at the one-time migration (test 6); afterwards the placements decide | — | — | — |
| Tokens (`ParamSource`) | — | — | — | — | — | — |
| Analysis axes, MC draws, stackup rows | — | — | — | — | — | — |
| `tests/golden/slot_tables.txt` | — | operand slots join the table | — | measure slots | selection slots | mate slots |
| Python `.pyi` and census | `Doc.output`, kinds | operand unions | `place`, `placements`, `product`; `roots`/`set_roots` gone | measure builder | `select` | — |

**f64 geometry does not move in any PR.**

- Every body digest, measured value bit, solved pose and tour frame is bit-equal across each PR. Stage 2 changes how a dependency is *spelled*, never what is computed.
- A tour frame that changes is a bug, not a re-baseline.
- The *intended* product changes are all in C:
  - the cut plate, where today's product is a refusal;
  - the nine bars-and-pins rows that refused `PlacedUnderTwoRoots`, which now build two copies.

  Neither changes a body's bits.

## 9. Test plan (each row names the runtime value that breaks it)

1. **(A) Outputs exist; the document is unmoved up to ids.**
   - After `InsertNode(extrude)`, `Doc::output(n, 0)` is `Some(v)`, `doc.var(v).kind == Body` and `def == Output { node: n, port: 0 }`.
   - `golden.cad` replayed equals its pre-A self up to an id bijection with the outputs removed.
   - *Breaks if* an insert changes anything but ids and outputs, or outputs are minted at port ≠ signature.
2. **(A) Load.**
   - A file with an `Output` row naming port 1 of an extrude refuses `OutputSignature`.
   - A live extrude with its output row removed refuses the same.
   - *Breaks if* the walk checks only the row side.
3. **(B) Kind at the door.** `Boolean { a: <profile output> }` refuses `SlotVarKind { expected: Body }`, and the doc is unchanged. *Breaks if* the lowering skips the kind check, so the boolean evaluates and refuses `WrongOperand` at evaluation instead.
4. **(B) Behaviour-preserving.**
   - For every corpus document and `golden.cad`, `doc.roots()` after B equals the pre-B roots mapped through the id remap, element for element.
   - Every body's f64 digest is bit-equal.
   - *Breaks if* `Doc::upstream` misses an operand family (a sink appears) or double-counts a list member.
5. **(B) Delete leaves a typed reader.**
   - Deleting the extrude under a fillet is accepted, with one `Maintenance::Strand` naming the fillet's `target` slot.
   - Evaluation refuses the fillet `UnresolvedRead { slot: Target }`, and undo restores it bit-equal.
   - *Breaks if* `DeleteWouldDangle` survives (the delete refuses) or the reader is re-pointed (the fillet evaluates).
6. **(C) The one-time migration.** For every corpus `.pncad` and `golden.cad` regenerated at C, the file holds one `PlaceInWorld` per body-denoting pre-C root, in root order, and `product_recorded`'s body digests and order equal pre-C's. *Breaks if* the migration orders by document order instead of root order, places a non-body root, or drops a world-gauge instance's offset instead of moving it into the pose (that digest moves).
7. **(C) The measured part stays.**
   - `cut_plate` (tour) with its web `Measure` and `Assertion`, its cut part placed: `product()` holds one body whose digest equals the cut part's, where pre-C refused `NoBodyRoots`.
   - *Breaks if* any code still derives membership from sinks.
8. **(C) A failing check gates nothing.**
   - A document whose `Assertion` reads a measure naming a face its target no longer draws (a vanished name; after E, a failed select): `product()` is `Ok` with the placed body, and `checks()` reports the assertion failed.
   - *Breaks if* the gather reads a non-placement node's `usable()`.
9. **(C) Nothing places as a side effect.**
   - Two placed extrudes `a`, `b`, then `InsertNode(Boolean{a, b})` through the kernel edit: `product()` is still `[a′, b′]`, and the boolean is unplaced.
   - The same through the viewer's combine gesture: `[c′]`, with `a`'s placement re-pointed to `c` in one recorded action (one undo restores `[a′, b′]`).
   - Python `a.cut(b)` without `doc.place` leaves `[a′, b′]`.
   - *Breaks if* any door keeps `on_insert`'s tip transfer, or the façade and Python disagree.
10. **(C) Empty and stranded.**
    - A document with bodies and no placement: `product()` refuses `EmptyProduct` naming every unplaced body in document order, and export refuses the same.
    - Deleting a placed body: the delete is accepted, and `product()` refuses `StrandedPlacement` naming the placement.
    - *Breaks if* an empty world is read as an error at load, or the stranded placement is silently skipped (the copy vanishes, which #3441 forbids).
11. **(C) Two copies.**
    - Two identity placements of one body: `product()` has two copies with distinct output ids, and each name resolves once per copy.
    - A gallery bars-and-pins row that refused `PlacedUnderTwoRoots` builds.
    - *Breaks if* `placed_under_two_roots` or N4's once-per-product refusal survives.
12. **(C) Split and inline under the world.**
    - A cut whose body is placed: a remainder reader of it is re-pointed to the instance's output for that copy.
    - A cut whose body is not placed: a remainder reader refuses `SeveredEdge`.
    - Inline of an instance read by a boolean succeeds (no `InstanceConsumed`), the boolean reading the inlined body.
    - *Breaks if* the consumer-ward closure is kept whole (the first row refuses) or dropped (the second splits).
13. **(D) Measure arithmetic is a definition.**
    - `Node::measure(distance(a,b) − distance(c,d), …)` builds two `Measure` nodes and one anonymous defined `Length`.
    - The assertion over it gives the same `AssertionVerdict` bits as pre-D on `tolerance` and `mcplate`.
    - The Dual derivative of the margin with respect to a seeded free variable is bit-equal.
    - *Breaks if* the definition is bound before its measures run (an `UnboundVar`), or a primitive's operand order flips (the sign flips).
14. **(D) Observed is read only by an assertion.**
    - An extrude whose depth slot reads a measure's output refuses `ConstructionReadsObserved`, and so does one reading `m + 1 mm` with `m` observed. An assertion reading it is accepted.
    - A hand-written file holding the extrude refuses `ObservedRead` at load.
    - *Breaks if* the test reads only direct reads, missing the definition.
15. **(D) Measure content key.** Re-pointing a measure from one node to another with an equal body changes its naming key and not its content key. *Breaks if* `at` ids are still hashed as payload.
16. **(E) Select owns the ladder.**
    - A fillet whose edge vanished refuses with the *select's* `ResolveError::Vanished` carried as `UnresolvedRead`, and `resolve::resolve` on the select returns the same `Diagnosis` as pre-E's `BlendSelectionResolve`.
    - *Breaks if* a second ladder call survives at the reader (two diagnoses) or the kind check is lost (a face is silently filleted as its boundary).
17. **(E) Rebind.** `Rebind { body, from, to }` changes exactly the selections of `body` naming `from`, and the fillet's slot ids are unchanged. *Breaks if* rebinding mints new selects (the reader's id moves).
18. **(E) One select read twice.** A fillet and a measure reading one `Edge` variable: renaming the minting step's piece breaks both with one diagnosis. *Breaks if* the door dedups selects by name value: two selects authored apart must stay two (VR8's lesson).
19. **(F) Poses unmoved.** Every MSOLVE fixture's `SolvedPoses` is bit-equal pre/post F, and every gallery assembly's product digest is equal. *Breaks if* the member walk starts from a different instance in the copy chain (A12's "whatever the depth").
20. **(F) A9 over reads.** Two instances mated: one component. Unmated with a measure across them: still one component (unchanged today, `a-measure-merges-free-instances-into-one-relative-freedom-component`, Q4). *Breaks if* `reading_edges`' gauge edges `(instance, gauge)` and `(gauge, parent)` are dropped with A12: two instances on one gauge must stay one component.
21. **(F) Split across a mate.** A cut taking one mated instance and not the other refuses the severed-read refusal naming the select. The union-read cases are the MSOLVE row's. *Breaks if* `OperandSeveredFromMate`'s check is lost and the split succeeds, stranding the mate's select.
22. **(F) Minting on placed copies.**
    - A mate between two placed instances mints its declaration on the two copies, read through their placements' outputs, and the at-rest gate passes as pre-F.
    - A mate whose member is a boolean operand (unplaced) mints nothing, and the product is unchanged.
    - *Breaks if* the consumer-walk lift survives (a `MovedAbove` is reported).
23. **(A–F) Python.**
    - `Node.extrude(profile_node, …)` and `Node.extrude(doc.output(profile_node), …)` store the same slot id.
    - The census and the `.pyi` agree, and `Doc.product()` reads the placements.

Loud census rows: `pncad-py` `tags.rs` / `surface_census` / `prose_census`, `display_contract`, the persist `Walk` roster, `tests/golden/slot_tables.txt` and `f6_variants!`.

## 10. Risks

- **B's size.** It is about 2,500 match and construction sites. Compile-driven, it is H on volume, and the `From<RecipeNodeId>` sugar is what keeps it mechanical. The real hand edits are the 37 `inputs()` callers.
- **The schedule** (narrowed by FORK-5). In D, an observed definition is the first variable whose value exists only mid-evaluation. Only assertions read it, so `Doc::var_env` (`doc.rs:1651`) and the construction lanes are untouched. The new machinery is binding observed definitions at the assertion, per lane (Interval, Dual, Sym).
- **Scripts and fixtures that relied on an implicit product.** After C nothing appears unless placed. Every test, demo and guide example that reads `product()` without placing must call `doc.place` (Python, Rust façade) or author a `PlaceInWorld`. The viewer's 73 `combine_ops` and 38 `creation_ops` rows are restated to read the placements their gestures author. This is the largest mechanical part of C, and the migration check (test 6) only covers regenerated files.
- **The pose moving into the placement** (Q9). An instance on the world gauge has its offset moved into its placement's pose. One posed by mates or by a non-world gauge keeps its pose until stage 3. That is two homes for a world pose in the interim, which Ev accepted as transient (#4220, residue 3).
- **E's diagnosis parity.** The four error families folding into one select error must not lose the `Diagnosis` the viewer shows. Test 16 pins it, and `refusal_concision_chains` (12 sites) will move.
- **F and the at-rest gate.** Today the gate resolves a mate's face against the *gathered product's* name table (instance-qualified), not against the member's body. The two tables agree only when the qualifier is carried faithfully through the read. Q8 states how, and tests 19 and 22 check it.
- **Re-blessing four times** (B, D, E, F). Each PR states which pins moved and why, and f64 digests are the guard.
- **Load.** This stage takes the program from about 36 to about 61 against a budget of 45. While stage 1 is open, the six units are filed `parked` (they do not count). At stage 1's close the orchestrator splits stage 2 into its own program, or confirms it fits once stage 1's rows close (`work/README.md` Track size).

## 11. Open questions

### FORKs: where each stands

Each FORK changed ratified text or turned on Ev's preference, and a designer pair weighed each one.

**FORK-1 — The output signature of an operation.** *Ruled* (#4222, approved as written; row `operations-state-their-outputs`). FORK-1b (row `the-pose-kinds-are-one-order`) answers the split it left.

- **The problem.** D10's kind list did not name a profile, a pattern's copies or a split's halves (REFERENCES DM3's `Part` projection), and stage 2 cannot type an operand slot until every operation states what it defines.
- **The converged recommendation**, written into #4222's diff:
  - a fixed list of named, typed ports per variant;
  - the shapes `Body`, `Bodies` and `Profile`;
  - a split defines two ports;
  - one copy of a `Bodies` is picked by index;
  - an instance defines one `Body` per world placement of its part;
  - an assertion or a mate defines none.
- **FORK-1b answers the split left to Ev**: there is no `AxisInPlane` kind. D10's five poses stay, both axis datums define an `Axis`, and the revolve defines `body` and `axis`. A pose kind names its symmetry as the mates' `Subgroup`, one type shared with the fold.
- A builds it.

**FORK-2 — How the explicit product list is kept.** *Superseded by FORK-2b and ruled* (#4220, approved, merging on green; row `the-product-list-is-kept-by-a-default-at-the-edit`).

- **The ruling.** The product is the world: every copy a `PlaceInWorld { body, pose }` defines, in document order. Nothing places as a side effect, and an empty world is an empty product.
- **Residues:**
  1. Python and the Rust façade place only when told, with one semantics, and the viewer's feature gestures re-point.
  2. A second identity placement is allowed.
  3. The pose lives in the placement in stage 2.
- **Text changed by the ruling:** D10's product sentence and its new "construction never reads the world", A10, A12, A4's acceptance, A2, A11 (2) export (#3441 narrowed) and DM4.
- **Text changed with the units:** D-2's closure, `PlacedUnderTwoRoots`/N4 and `InstanceConsumed` change with C; A5's minting lift changes with F.
- §4 is the build.

**FORK-3 — What a selection is.** *Ruled* (#4222, approved: sets; row `a-selection-is-a-definition-of-a-body-s-faces-or-edges`).

- **The problem.** D10 said a `Face`/`Edge` variable is a selection of a `Body` by `StableName`. It did not say whether a selection is a node or a definition, one entity or a set, or shared or distinct, against SELECT-DESIGN §4's one-type rule.
- **The converged recommendation:**
  - a definition, not a node, stating its body once;
  - distinct by authoring, with the GUI offering the existing one;
  - repair addressed by body and name (`Rebind { body, from, to }`).
- **Ev chose sets** (`Faces`/`Edges`). The edges that sharing a variable replaces are dependency-graph edges.
- E builds it.

**FORK-4 — Re-pointing an operand.** *Ruled* (#4221, merged; row `an-operand-slot-is-re-pointed-by-the-slot-door`).

- DM6 now reads "no edit infers a re-point".
- An operand slot is written by the one slot door, under kind, liveness, acyclicity and DM5. Strands are reported, never refused, and no door picks a survivor.
- B builds it, and `SetMembers` becomes that door on a list slot.

**FORK-5 — May a construction read a measured value.** *Ruled* (#4218, merged; row `a-construction-reads-a-measured-value`).

- D10 now says a `Measure` defines an *observed* variable. An observed variable, and any definition reading one, is read only by an assertion, and a construction reads what was written.
- Driven dimensions are deferred.
- D builds the refusal (`ConstructionReadsObserved`, `ObservedRead`).

### Questions with a recommendation (not forks)

1. **Delete with readers.** D10 settles that readers are left unresolved and typed. **Recommendation:** accept the delete and report each stranded read as `Maintenance::Strand` (DM7 generalised). `DeleteWouldDangle` and `DanglingInput` retire in B. `cascade_delete_order` stays as the GUI's "delete with dependents".
2. **Declared pairs** (`Boolean.declare`, `Union.declare`). D10 retires them in stage 4, and converting them to selects only to delete them later is wasted churn. **Recommendation:** leave them as names until stage 4. The eval-time ladder keeps its declared-pair callers until then, and `payload_names` shrinks to them.
3. **Gauges** (`Gauge.parent`, `InstantiatePart.gauge`) are not DAG edges today, and A11 (2)'s gauges retire in stage 3. **Recommendation:** leave them as node ids in stage 2, and keep `reading_edges`' gauge edges inside A9's partition when F deletes the mate half (test 16).
4. **A9 and measures** (`a-measure-merges-free-instances-into-one-relative-freedom-component`). Stage 2 does not change the partition's behaviour. **Recommendation:** leave the row parked and move its trigger to stage 3, where spaces are decided. Its fix (stop at space-free values, as `spaces_with` does) belongs with the per-space frame.
5. **Authored spelling of an operand.** **Recommendation:** a `RecipeNodeId` stays authored sugar for that node's port 0 in Rust and Python. A multi-output node (a split, an instance of a part with several world placements, per FORK-1) needs an explicit `Output { node, port }` and refuses the sugar with `AmbiguousOutput`. This keeps ~2,300 test sites and 743 Python sites unchanged.
6. **Output minting.** *Ruled by the orchestrator (2026-10-08).* Each output is logged as an ordinary `Minted::Var`, ordered as minted, as VR1 and N1 say, so ids move in A and again in B. The recommendation was to keep A's ids byte-equal, but that cannot hold while outputs are logged: an id's ordinal is the log's length plus one, and an insert's preimage holds its slot variables' ids, so one logged output moves every later digest. Keeping outputs out of the ordinal count would change N1's log invariant and VR1's order, which is ratified text, for a cheaper checkpoint B discards anyway. A's check is therefore equality up to an id bijection.
7. **Slot table.** Operand, measure, select and mate slots join `SlotId` and `tests/golden/slot_tables.txt`. **Recommendation:** name them by field (`SlotId::Operand(OperandSlot::Target)`, …), not positionally, so the Python slot words stay readable. `placement-step-slots-are-spelled-three-ways` stays separate (stage 3).
8. **The mate's face across the at-rest gate.** The lift through consumers retires (#4220, audit H10).
   - **Recommendation:** the selection reads the member's `Body` output (the walk's minting instance). The gate mints the declaration on each world copy of that member: the copy is a `PlaceInWorld` output reading the member, and its face is the selection's entity under the placement's pose, recorded by `transform_rigid`'s graft map.
   - So there is one resolution, at the selection, and none against the product table.
   - A member no placement reads has no world copy, and the mate mints nothing.
9. **Where a stage-2 world pose lives** (Ev's residue 3: in the placement).
   - **Recommendation:** C's migration moves a world-gauge instance's offset into its `PlaceInWorld` pose, and the instance sits at the empty offset.
   - An instance posed by placing mates, or by a non-world gauge, keeps that pose until stage 3, which retires gauges and makes placement the bundle of mates, and gets an identity world placement.
   - Moving every gauge chain now would rebuild A11 (2) one stage before it is deleted.

**The consuming-model holdover audit** (`audit/intent-consuming-holdovers`), mapped to this spec:

| Hit | What | Where it goes |
|---|---|---|
| H1 | A10's sink-set invariants | ratified away by #4220; code retires in C (`roots.rs`) |
| H2 | A10's maintenance (tip transfer, orphaned inputs) | ratified away; C deletes `on_insert`, `on_delete`, `on_set_members` with no successor, and the viewer re-points by gesture. Test 6 is a one-time migration check, not a rule |
| H3 | A4's split acceptance ("where the first of them was") | text rewritten by #4220; C's split and inline lose the splice |
| H4 | mates as non-body roots (A10, A12) | A12's root sentence is gone (#4220); F retires reading edges |
| H5 | E3's "sink" wording | filed as `error-design-e3-calls-a-measure-a-sink` (#4266); D answers the mechanism: a measure defines an observed variable and is not a sink |
| H6 | DM4's orphaned-transform delete | rewritten by #4220; `diefillet` stops deleting (C) |
| H7 | D-2's consumer-ward closure | narrowed in C (§4); B keeps it unchanged |
| H8 | `InstanceConsumed` | retires in C |
| H9 | `PlacedUnderTwoRoots`, N4's once-per-product rule | retire in C |
| H10 | A5's minting lift through consumers | retires in F (§7, Q8) |
| H11 | A2's "takes its A10 product" | rewritten by #4220; instance outputs are per world placement (C, FORK-1's signature) |
| H12 | A9's "consuming ∪ reading edges" | rewritten to "operand ∪ reading" by #4220; F rewords to "over reads" when `reading_edges` goes |
| H13 | the guide's roots examples (`assembly.md:322–341`, `:1069`, `:1227`) | C |
| H14 | `DeleteWouldDangle` | retires in B |

**Sequencing note.** #4220 places H7, H8 and H9 "with units B and F". This spec puts them in C, because each is stated over world placements, which exist only from C. The orchestrator confirmed this on 2026-10-07 as a sequencing correction that changes no design. H10 stays in F.

**Inconsistencies found.**

- ASSEMBLY A12 (as #4220 rewrote it) still names `reading_edges` and "operand ∪ reading edges". Both retire in F, and A12 becomes a pointer to D10.
- `Node::Measure`'s content-key arm hashes `at` ids as payload (`eval/mod.rs:5743`), as does `Mate`'s (`:5608`), against `content_key`'s stated rule that upstream identity enters by content. D and F fix both.
- SELECT-DESIGN §4's one-type rule and D10's `Face`/`Edge` variables state the stored form of a selection differently (FORK-3).
- `docs/guide/assembly.md:322–341`, `:1069` and `:1227` still speak of roots. They are A10's, and are reworded in C.
- **The baseline moved.** Stage 1's PR C (#4146) merged after this spec's grep (`68ecdb7f3`). The operand, name and root counts are unaffected (it touched slots, not operands), but line numbers in `edit.rs`, `doc.rs` and `persist/check.rs` have drifted, and the dispatching orchestrator re-greps.
