# SPEC: INTENT stage 2 — operations and one dependency

Scope: D10's **Operations** paragraph (`docs/DESIGN.md` §D10), as `work/intent/plan.md` stage 2 states it:

- Nodes read and define variables.
- Consuming edges, reading edges, name references and `Measure` refs become reads.
- A `Select` operation defines `Face`/`Edge` variables and owns the N5 ladder.
- The product becomes an explicit list, and A10's sink rule retires.

D10's last paragraph retires two things here: **A10's sink rule** and **A12's reading edges**. Stage 2 also closes `a-measured-part-is-not-a-product-root` and `a-failed-requirement-refuses-the-whole-product`, and completes VR4's interim exception (a `Measure`'s arithmetic stays in the node "until stage 2 makes `Measure` an operation").

**Baseline.** Main is at `9eaf8eab2f`. Stage 1's PR C (#4146, a slot holds a `VarId`) is open and PR D (`Expr` holds no float) is unbuilt, so every count below is measured **before** them. Neither one touches an operand field, a name field, the root list or the mint preimage's operand spelling. The counts therefore stand, but the dispatching orchestrator re-greps at the merge of stage 1's D and states the drift.

The state of these mechanisms at the baseline:

- **Operands.** A node's operands are `RecipeNodeId` fields, and `Node::inputs()` (`crates/editor-core/src/node.rs:3375`) lists them as the DAG's edges.
- **Non-edges.** A mate's `SitedFace.at`, `InstantiatePart.gauge` and `Gauge.parent` are not edges. A12's `reading_edges` (`mate/solve.rs:841`) recomputes the mate ones.
- **Measures.** A `Measure`'s `refs[].at` ARE edges (`node.rs:3436`). That is the bug `a-measured-part-is-not-a-product-root` describes.
- **Names.** `StableName`s ride in payloads and are resolved mid-evaluation by `eval/wire.rs`'s three-rung `ladder` (`:2162–2294`). The full diagnostic ladder is `resolve/mod.rs`.
- **Roots.** `Doc::roots` is A10's ordered sink list (`roots.rs`). `VarKind` is `{Length, Angle, Scalar, Count}` and `VarDef` is `{Free, Defined}` (`var.rs:30`, `:79`).

## 0. Ordering: six PRs, each green, none landing half a representation

| PR | Unit (`work/intent/`) | Lands | Representation step it completes | Goldens |
|---|---|---|---|---|
| A | `operations-define-output-variables` | `VarDef::Output { node, port }`, the reference kinds, outputs minted at insert, persistence and the load walk | **an operation defines variables** | vars table grows. Node ids unmoved (Q6) |
| B | `operands-are-reads` | every operand field holds a `VarId` read of an output. `inputs()` is derived from reads. Kind-typed operand slots. Delete leaves readers unresolved | **reading is the only dependency** (for operands) | re-blessed: ids move. Roots and geometry unmoved |
| C | `the-product-is-an-explicit-list` | `Doc::product: Vec<VarId>` of `Body` variables. A10's invariants and sink maintenance retire | **the product is explicit** | re-blessed. Product membership is unmoved on every corpus file |
| D | `measure-is-an-operation` | one `Measure` is one primitive defining one scalar output. Its arithmetic moves to a `Defined` variable, and `Assertion` reads a scalar variable | **VR4's exception closes** | re-blessed. Measured bits unmoved |
| E | `select-defines-face-and-edge-variables` | `Face`/`Edge` kinds and `Select`. Fillet/chamfer/shell/face-frame/measure names become reads, and the eval-time ladder moves into `Select` | **a name reference is a read** (outside mates) | re-blessed. Geometry unmoved |
| F | `a-mate-reads-face-variables` | mate sides read `Face` variables. `reading_edges` and the mates-are-not-edges carve-out retire | **A12 retires** | re-blessed. Poses unmoved |

Why this order:

- **A before B.** Readers need something to read. A is the checkpoint where the variable table states every operation's outputs, and no id moves (Q6). Stage 1 had the same shape: its PR A added `Defined` before PR C made slots read it.
- **B before C.** The product is a list of `Body` *variables* (D10), so they must exist and be read first. B is behaviour-preserving: the read graph equals today's edge graph, so A10's sink set computed over reads is today's root set, and C's migration can be checked against it.
- **C before D, E and F.** Once a measure or a mate is an ordinary reader, its operand stops being a sink. Under A10 the measured part or mated instance would drop out of the product: the cut-plate bug, generalised to every assembly. Only an explicit product makes those reads safe, so C comes first.
- **D before E.** D is the smaller of the two units that rewrite `measure.rs`. E then converts the refs D leaves as names.
- **E before F.** A mate side's face is a `Select` like any other. F is separate because the mate path resolves differently today: against the product table at the at-rest gate, not at the operand (`assembly.rs:1478`). Its refactor doors (split and inline, AQ8) are their own risk.

Each intermediate state is a whole representation:

- after A, outputs exist and nothing reads them yet, like stage 1 after its A;
- after B, every operand is a read, and names and the product are as today;
- after C, the product is explicit and measures and mates still read by `at` and name;
- after D, measures are single primitives that still name their faces;
- after E, every name except a mate's and a declared pair's (Q2) is a read;
- F leaves only declared pairs, which stage 4 deletes.

**Rejected:**

- **A+B as one PR.** It would be about 1,900 match and construction sites plus every golden with no checkpoint between them. A's no-id-moves golden check is cheap, and losing it costs more than the extra PR.
- **C first, over node ids.** The product would be retyped twice and the wire broken twice for no gain, since C is small once B lands.
- **E and F as one PR.** That mixes the eval-time ladder's relocation with the at-rest gate's resolution and the refactor doors, and the reviewers would have to separate them again.

## 1. Final shapes (after F)

**Kinds and definitions** (`var.rs`):

- `VarKind` gains the reference kinds `Body`, `Face` and `Edge`, and whatever further output kinds FORK-1 settles: a profile, a datum before stage 3, and a multi-body value.
- `VarDef` gains `Output { node: RecipeNodeId, port: u8 }` and the `Select` definition, whose home FORK-3 settles.
- A reference-kind variable has no free arm. It is never an analysis axis, has no unit and has no distribution.
- It reads as its definition. An unnamed one is shown as "the body of Extrude "base plate"".

**Operations.** Each `Node` variant states an output signature, `Node::outputs() -> &'static [(port, VarKind)]` (FORK-1 decides the table):

- `Extrude`, `Revolve`, `Boolean` and the rest each define one `Body`.
- `Measure` defines one scalar.
- `Assertion` and `Mate` define nothing in stage 2. A mate's placement is stage 3's.

`InsertNode` mints the outputs and the door returns them in `Applied`. `Doc::output(node, port) -> VarId` is the accessor.

**Operand slots.** Every operand field is `S` (stage 1's slot form): `VarId` stored, and authored as a `Formula` whose lone leaf is a `Var`, a `Name`, or `Output { node, port }`. A bare `RecipeNodeId` is authored sugar for port 0 (Q5).

- The slot's kind is the operand's kind (`Body` for a boolean's `a`, …). A wrong kind refuses at the door as `SlotVarKind`, the same check scalar slots use.
- Lists (`Union.members`, `Loft.profiles`) are `Vec<S>`.
- `Node::inputs()` is retired as a stored-field walk. It becomes `Doc::reads(node) -> Vec<VarId>` and `Doc::upstream(node) -> Vec<RecipeNodeId>`, the defining operations of those reads expanded through definitions. That is the one dependency relation the schedule, the content key, acyclicity, split closure and the viewer tree all read.

**Deletion** (D10: "deleting a variable leaves its readers unresolved, typed, never silently re-pointed"):

- `DeleteNode` of an operation with readers is accepted.
- Each reader of a removed output rides `Applied.maintenance` as `Maintenance::Strand`, which is DM7's report generalised from names to reads.
- The reader refuses at evaluation as `NodeErrorKind::UnresolvedRead { slot, var }`.
- `EditError::DeleteWouldDangle` and the load door's `DanglingInput` retire.
- `cascade_delete_order` stays, as the GUI's delete-with-dependents convenience.

**The product** (`doc.rs`, `product.rs`): `Doc::product: Vec<VarId>`, ordered, duplicate-free, each a live `Body` variable.

- `roots.rs` is deleted: coverage, ancestor-freedom, `on_insert`, `on_set_members` and `on_delete` all go.
- The list's maintenance under edits is FORK-2's.
- `DocEdit::SetRoots` becomes `SetProduct { bodies }`. `RootFault` becomes `ProductFault { NotLive, Duplicate, NotABody }`.
- The gather reads only the listed variables. A Measure, Assertion or Mate is never in the list, so none can gate it.
- `ProductError::NoBodyRoots` becomes `EmptyProduct`.
- `placed_under_two_roots` stays: two listed bodies can still share an instance below them.

**Measure** (D):

- `Node::Measure { primitive: MeasurePrimitive<S> }` holds one `Distance`, `Angle`, `MinClearance` or `Gap`. Its operands are slots, which read `Face`/`Edge` variables after E.
- It defines one scalar output of the primitive's dimension.
- `MeasureExpr` and `MeasureKind` are deleted: arithmetic over measures is a `Defined` variable (`Expr` already has `Min`/`Max`).
- `Assertion { value: S, bound: S, dir }` reads any scalar variable of the bound's dimension. `AssertionBoundFault::TargetNotMeasure` retires, and the dimension check stays.
- Whether a *geometric* slot may read a measured value is FORK-5's.

**Select** (E): a `Face` or `Edge` variable is defined by `Select { body: VarId /* Body */, name: StableName }`. Evaluation runs `eval/wire.rs`'s three-rung ladder (`live` → `Tied` → `Absent`) **in the select and nowhere else**. A failed select poisons its readers with the `ResolveError`, and the post-evaluation diagnostic ladder (`resolve/mod.rs`) diagnoses that select.

- `Fillet.selection` and `Chamfer.selection` become `Vec<S>` of `Edge`, and `Shell.open` becomes `Vec<S>` of `Face`, if FORK-3 rules one entity per variable.
- `Datum::FaceFrame { at, face }` becomes `{ face: S }`, because the body is the select's.
- `Measure`'s operands read selects.
- `Rebind { from, to }` becomes an edit of the select's `name`. The edit door's `payload_names` walk shrinks to the selects plus the declared pairs (Q2).

**Mates** (F): `SitedFace { at, name }` becomes a `Face` slot, and `FrameBase::Face` reads that slot.

- `mate::reading_edges` is deleted, and A9's partition runs over `Doc::upstream`.
- The at-rest gate's `resolve_face` against the product table (`assembly.rs:1478`) is replaced by the select's resolution against the member's `Body`, with the instance qualifier carried by the read (Q8).
- AQ8's "only a mate EDGE can cross" becomes "only a mate whose two face reads resolve can cross", which is the same predicate respelled.

## 2. PR A — `operations-define-output-variables` (cost M; ~40 files, 1.5–2.5k lines)

1. `VarKind::{Body, …}` per FORK-1. `VarDef::Output { node, port }`, with no authored twin: an output is never declared, only minted by its operation.
2. `Node::outputs()`, exhaustive per variant, so a new variant does not compile without its signature.
3. **Minting.** `InsertNode` draws each output's id from the insert's own chain step, as `steps_of_insert` draws `StepId`s (`mint.rs`), under a new `OUTPUT_TAG` with the port as the index. The chain extends once per insert as today, so **no node id moves** (Q6).
   - Each output is logged `Minted::Var`.
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

**Goldens.** The vars table gains one row per node output, and `Minted::Var` entries join the log, in `tests/golden/golden.cad`, the 14 schema-bearing `bool13_goldens` (v6–v19), the four corpus `.pncad` files, `before_extrude_side/plate_param.cad`, `crates/pncad/tests/plate_param.pncad` and `crates/viewer/tests/gallery_ring.pncad`. Node ids, content keys and `order` are byte-equal.

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
- **Deletion** per §1. `DeleteWouldDangle` (`edit.rs:1266`, 3 sites plus `refactor.rs:1587`) and `DanglingInput` / `ForwardInput` (`persist/check.rs:1629`) retire, and `Maintenance::Strand` gains a read arm. `roots::on_delete`'s splice still runs, over the orphaned readers' operands.
- **Rewiring.** `SetMembers` (`node.rs:3695`) becomes a slot write on a list slot. Whether every operand slot is writable by `SetParam` is FORK-4. Until it is ruled, B keeps DM6: only list slots are rewritable.
- **Split and inline.** `refactor::remap_node` (`:1972`) re-points reads instead of ids. Its closure check (`:2370`) reads `Doc::upstream`, and a crossing output carries as stage 1's anonymous carry does: one fresh entry, minted on the far side.
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

## 4. PR C — `the-product-is-an-explicit-list` (cost M; ~110 files, 2–3k lines)

- `Doc::roots` becomes `Doc::product` (`doc.rs:816`, `:975`, `:1071`, `:1766`). The wire field `"roots"` becomes `"product"`, and a pre-C file refuses with the regenerate recourse.
- Delete `roots.rs` and its four call sites in `edit.rs`: the backstop at `:5015`, and `on_insert`, `on_set_members` and `on_delete`. Promote's slot insert (`:6030`) goes too.
- Maintenance is FORK-2's.
- **The gather** (`product.rs`): `product_in` walks the list (`:1076`).
  - `sources_of`'s non-body skip (`:772–786`) is deleted, because the list is typed.
  - The `usable(root)?` that lets a failed check refuse the whole product (`:1080`) now reads only `Body` variables. That closes `a-failed-requirement-refuses-the-whole-product`.
  - `checks.rs:1166` and `:1268` read the same list.
- `NoBodyRoots` becomes `EmptyProduct`, at `product.rs:200`, `checks.rs:983`, `eval/parts.rs:483` and `assembly.rs:1186`, with recourse text "add a body to the product".
- **Refactor** (`refactor.rs`, 22 sites):
  - Split's anchor vote (`:2546`), `UnplaceableRoot` (`:2568`) and `NoMaterial` (`:2865`) read "the cut's listed bodies".
  - The part's product is the cut's listed bodies in list order (`:2889`).
  - The remainder lists the instance's `Body` output where the first cut body was (`:3048`).
  - Inline splices the part's product at the instance's slot (`:3615`).
  - A4's acceptance sentence is reworded with the change, which is wording moved by code (CLAUDE.md).
- `eval/parts.rs:719` (an instance is its part's product) is unchanged in meaning.
- `assembly.rs:1512`'s name walk stops at listed bodies.
- `resolve/mod.rs:2279`'s `strict_ancestors` moves to `Doc::upstream`'s closure.
- **Surfaces.**
  - `pncad` `document.rs` (7) and `export.rs` (2).
  - Python: `Doc.roots` becomes `Doc.product`, and `DocEdit.set_roots` becomes `set_product`. `root_fault_tag` becomes `product_fault_tag`. That is 82 sites in src, 13 in the `.pyi` and 94 in the tests.
  - The viewer's root badge (`tree.rs:747`, `:776`), `pickindex.rs:1024`, `display.rs:446` and `:514` (`instances_by_root`), the projections kept only to leave each copy a root (`session/op.rs:783`, `session.rs:2776`) and `pane/create.rs:59`. That is 119 sites in src and about 250 in the tests.
- **Docs.** `docs/guide/assembly.md` (the normative example at `:322–341` lists mates as roots, and `:1227` says "a mate is a product root"). Also `ASSEMBLY.md` A10, A12's "an ordinary non-body root" sentence and A4's acceptance, and DESIGN.md's companion row. A10's replacement text is FORK-2's ratified answer, so this part of C waits on it.
- **Tour.** `gallery.rs` (27 sites):
  - `the_cut_plate_has_no_product` becomes `the_cut_plate_is_its_product`. That is the acceptance of `a-measured-part-is-not-a-product-root`, and it needs only C: the measured part is listed whatever reads it.
  - The root-count rows (`:215–321`) are restated as product lists.
  - `diefillet.rs:563–583` stops deleting its narration body, which is simply not listed.
  - `assembly.rs:483` and `:1284` read `product().first()`.

## 5. PR D — `measure-is-an-operation` (cost M; ~70 files, 2–3k lines)

- `Node::Measure` holds one primitive (§1). `measure.rs`'s `MeasureExpr` and `MeasureKind` (`:171`) are deleted, and the `Primitive` arms keep their refs as `SitedRef` until E.
- **Migration.** One measure with arithmetic becomes one `Measure` per primitive plus one anonymous `Defined` variable over their outputs.
  - The authored door keeps `Node::measure(expr, refs)` as a **builder** returning the edit list. It is the same edit-list shape split returns, so the 97 test call sites and 220 `SitedRef::new` sites change only where they destructure.
  - The Python `Node.measure` builder does the same, as a recorded batch.
- `Assertion.measure: RecipeNodeId` becomes `value: S` of a scalar kind (`node.rs:2770`, 28 src and 59 test sites). `assertion_bound_fault` (`:3565`) checks the dimension only.
- **Evaluation.** `wire_measure` (`wire.rs:2443`) evaluates one primitive. A definition reading an output variable is bound when its operation has run: `Doc::var_env` binds free and pure definitions as today, and an output-reading definition is scheduled as a dependency of its readers (`eval/schedule.rs`). If FORK-5 rules that geometric slots may not read outputs, this binding serves only assertions and other measures.
- **Content key.** The `Measure` arm (`eval/mod.rs:5743`) hashes `r.at` ids directly as payload. That is replaced by the upstream keys like every other operand, which fixes a key that today moves on an id-only change.

## 6. PR E — `select-defines-face-and-edge-variables` (cost H; ~200 files, 5–8k lines)

- `VarKind::{Face, Edge}` and `Select` per FORK-3. Its evaluation is `named_entity` (`wire.rs:2317`) plus the kind check that `BlendSelectionKind`, `ShellOpenKind`, `FaceFrameKind` and `MeasureSelectionKind` each make today. That is four error families folded into one, `SelectKind { expected, found }`.
- **Converted payloads:**
  - `Fillet`/`Chamfer.selection`: 12 src and 25 test non-empty sites, 25 in the viewer and 1 in the tour.
  - `Shell.open`: 11 src, 12 tests, 16 viewer, 4 tour and 1 py.
  - `Datum::FaceFrame.face`: 16 src, 37 tests and 10 viewer.
  - Measure refs: 20 `SitedRef::` sites in src and 252 in the tests.
- `StableName {` literals (266 in src, 341 in tests, 55 viewer, 9 tour) mostly stay: a select stores one.
- The authored sugar `Formula::select(body, name)`, and a `Vec<StableName>` given to a selection slot, lower to one select per name at the door. That keeps the 41 + 12 + 20 `Node::fillet/chamfer/shell(` test sites unchanged.
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
- **The at-rest gate.** `assembly.rs:1386–1387` and `:1478` (`resolve_face` against the product table) read the select's resolved entity carried to the product. `MintRefusal::Reference` keeps its two arms (Q8).
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
| Node ids, pinned hex, memo fixtures | — | **all** (the preimage spells reads) | — | measures and later | selects and later | mates and later |
| Var table, mint log | grows (outputs) | — | — | grows (definitions) | grows (selects, if FORK-3 makes them variables) | — |
| Wire | vars rows | operand fields | `roots` → `product` | `Measure`, `Assertion` | payload names | `SitedFace` |
| Content keys | — | ids move, meaning unchanged | — | measure keys (no longer hash `at` ids) | — | mate keys (likewise) |
| Root/product membership | — | **byte-equal roots** | **equal sets** after migration (test 6) | — | — | — |
| Tokens (`ParamSource`) | — | — | — | — | — | — |
| Analysis axes, MC draws, stackup rows | — | — | — | — | — | — |
| `tests/golden/slot_tables.txt` | — | operand slots join the table | — | measure slots | selection slots | mate slots |
| Python `.pyi` and census | `Doc.output`, kinds | operand unions | `product` | measure builder | `select` | — |

**f64 geometry does not move in any PR.**

- Every body digest, measured value bit, solved pose and tour frame is bit-equal across each PR. Stage 2 changes how a dependency is *spelled*, never what is computed.
- A tour frame that changes is a bug, not a re-baseline.
- The one *intended* product change is the cut plate (C), where today's product is a refusal.

## 9. Test plan (each row names the runtime value that breaks it)

1. **(A) Outputs exist, no id moves.**
   - After `InsertNode(extrude)`, `Doc::output(n, 0)` is `Some(v)`, `doc.var(v).kind == Body` and `def == Output { node: n, port: 0 }`.
   - The node ids of `golden.cad` replayed are equal before and after A.
   - *Breaks if* outputs are drawn by extending the chain (every later id moves) or are minted at port ≠ signature.
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
6. **(C) Migration preserves the product.** For every corpus `.pncad` regenerated at C, `product_recorded`'s body digests and order equal pre-C's. *Breaks if* the migration orders by document order instead of root order, or lists a non-body.
7. **(C) The measured part stays.**
   - `cut_plate` (tour) with its web `Measure` and `Assertion`: `product()` holds one body whose digest equals the cut part's, where pre-C refused `NoBodyRoots`.
   - *Breaks if* any code still derives membership from sinks.
8. **(C) A failing check gates nothing.** A document whose `Assertion` reads a measure naming a face its target no longer draws (a vanished name; after E, a failed select): `product()` is `Ok` with the body, and `checks()` reports the assertion failed. *Breaks if* the gather reads an unlisted node's `usable()`.
9. **(C) Typed list.** `SetProduct { bodies: [measure output] }` refuses `ProductFault::NotABody`. A duplicate refuses `Duplicate`.
10. **(D) Measure arithmetic is a definition.**
    - `Node::measure(distance(a,b) − distance(c,d), …)` builds two `Measure` nodes and one anonymous defined `Length`.
    - The assertion over it gives the same `AssertionVerdict` bits as pre-D on `tolerance` and `mcplate`.
    - The Dual derivative of the margin with respect to a seeded free variable is bit-equal.
    - *Breaks if* the definition is bound before its measures run (an `UnboundVar`), or a primitive's operand order flips (the sign flips).
11. **(D) Measure content key.** Re-pointing a measure from one node to another with an equal body changes its naming key and not its content key. *Breaks if* `at` ids are still hashed as payload.
12. **(E) Select owns the ladder.**
    - A fillet whose edge vanished refuses with the *select's* `ResolveError::Vanished` carried as `UnresolvedRead`, and `resolve::resolve` on the select returns the same `Diagnosis` as pre-E's `BlendSelectionResolve`.
    - *Breaks if* a second ladder call survives at the reader (two diagnoses) or the kind check is lost (a face is silently filleted as its boundary).
13. **(E) Rebind.** `Rebind { from, to }` changes exactly the selects naming `from`, and the fillet's slot ids are unchanged. *Breaks if* rebinding mints new selects (the reader's id moves).
14. **(E) One select read twice.** A fillet and a measure reading one `Edge` variable: renaming the minting step's piece breaks both with one diagnosis. *Breaks if* the door dedups selects by name value: two selects authored apart must stay two (VR8's lesson).
15. **(F) Poses unmoved.** Every MSOLVE fixture's `SolvedPoses` is bit-equal pre/post F, and every gallery assembly's product digest is equal. *Breaks if* the member walk starts from a different instance in the copy chain (A12's "whatever the depth").
16. **(F) A9 over reads.** Two instances mated: one component. Unmated with a measure across them: still one component (unchanged today, `a-measure-merges-free-instances-into-one-relative-freedom-component`, Q4). *Breaks if* `reading_edges`' gauge edges `(instance, gauge)` and `(gauge, parent)` are dropped with A12: two instances on one gauge must stay one component.
17. **(F) Split across a mate.** A cut taking one mated instance and not the other refuses the severed-read refusal naming the select. The union-read cases are the MSOLVE row's. *Breaks if* `OperandSeveredFromMate`'s check is lost and the split succeeds, stranding the mate's select.
18. **(A–F) Python.**
    - `Node.extrude(profile_node, …)` and `Node.extrude(doc.output(profile_node), …)` store the same slot id.
    - The census and the `.pyi` agree, and `Doc.product` round-trips.

Loud census rows: `pncad-py` `tags.rs` / `surface_census` / `prose_census`, `display_contract`, the persist `Walk` roster, `tests/golden/slot_tables.txt` and `f6_variants!`.

## 10. Risks

- **B's size.** It is about 2,500 match and construction sites. Compile-driven, it is H on volume, and the `From<RecipeNodeId>` sugar is what keeps it mechanical. The real hand edits are the 37 `inputs()` callers.
- **The schedule.** In D, a definition reading an operation's output is the first variable whose value exists only mid-evaluation. `Doc::var_env` is built before evaluation (`doc.rs:1651`), and the analysis lanes (Interval, Dual, Sym) bind it the same way. Binding an output-reading definition per lane, at schedule time, is new machinery. FORK-5 bounds how far it reaches.
- **The product's maintenance (FORK-2) is UX.** Whatever the ruling, every viewer flow that today relies on "the tip replaces its operands" (the combine ops: 73 rows in `combine_ops`, 38 in `creation_ops`) needs a row stating what the product holds afterwards.
- **E's diagnosis parity.** The four error families folding into one select error must not lose the `Diagnosis` the viewer shows. Test 12 pins it, and `refusal_concision_chains` (12 sites) will move.
- **F and the at-rest gate.** Today the gate resolves a mate's face against the *gathered product's* name table (instance-qualified), not against the member's body. The two tables agree only when the qualifier is carried faithfully through the read. Q8 states how, and test 15 checks it.
- **Re-blessing four times** (B, D, E, F). Each PR states which pins moved and why, and f64 digests are the guard.
- **Load.** This stage takes the program from about 36 to about 61 against a budget of 45. While stage 1 is open, the six units are filed `parked` (they do not count). At stage 1's close the orchestrator splits stage 2 into its own program, or confirms it fits once stage 1's rows close (`work/README.md` Track size).

## 11. Open questions

### FORKs (each changes ratified text or turns on Ev's preference; a designer pair weighs each)

**FORK-1 — The output signature of an operation.** D10 lists the variable kinds as the scalars, the discrete kinds, the geometric values (`Point` … `Frame`) and the references (`Face`, `Edge`, `Body`), and says a node "defines one or more" variables. Today's operations produce values the list does not name:

- a profile (a planar region with step identity);
- a datum frame, plane or axis, which stage 3 makes a geometric variable, but which stage 2's operand reads must already type;
- a pattern's `Instances` (a count of bodies fixed only by a variable, so not a fixed set of ports);
- a split's two halves, read today through REFERENCES DM3's `Part` projection node (ratified).

Stage 2 cannot type an operand slot until every operation states what it defines, and the answer may add kinds to D10's list or change DM3. *Blocks A.*

**FORK-2 — How the explicit product list is kept.** A10's maintenance rules make the list follow the graph:

- a new sink appends;
- an insert consuming roots takes the first one's place;
- a delete restores the orphaned inputs.

Every viewer flow and the guide's assembly walk rely on it. Once nothing consumes anything (D10), "the tip replaces its operands" has no graph meaning, so no rule falls out of the structure. Some edits must still add to, replace in, or leave alone the list: inserting a body, inserting a boolean over two listed bodies, deleting a listed body, split and inline, Promote. Python's `Doc.roots` and the viewer's root badge must show the result. A10's text is ratified, and what replaces its maintenance clause is a choice about how a person and a script experience the document. *Blocks C's docs; C's code can land behind the ruling only if the ruling is in hand at dispatch.*

**FORK-3 — What a selection is in the document.** D10 says a `Face` or `Edge` variable "is a selection of a `Body` variable by `StableName`, and the N5 resolution ladder lives there". It does not say:

- whether a selection is a recipe node (a tree row, memoized and evaluated like an operation) or a variable definition beside `Defined` (living in the variable table, shown at its reader);
- whether a fillet's many edges are many variables or one set-valued variable;
- whether a selection authored at two sites is shared or distinct.

SELECT-DESIGN (ratified) rules that a recipe stores `Vec<StableName>` and that the GUI's selection is the same value (§4). It also rules no live query in a recipe, which a stored name still satisfies. Each answer moves the tree, the panel, `Rebind`'s address and how many variables a typical part holds. *Blocks E.*

**FORK-4 — Re-pointing an operand.** REFERENCES DM6 (ratified) rules that "no edit rewires a live node's inputs, and none is planned": every graph change is an insert, a delete or `SetMembers` on a list. Once an operand is a slot holding a variable, stage 1's slot edits (`SetParam`) could re-point it like any other slot. The stage-1 GUI's offer of an existing variable (`typing-a-value-mints-or-offers-a-variable`) is exactly that gesture for scalars. Whether operand slots join the edit vocabulary, stay outside it, or are writable only under a rule (same kind, no cycle, names re-checked) is a decision DM6 made the other way for reasons (the die chain, D-2's closure rule) that may or may not survive D10. *B keeps DM6 until ruled.*

**FORK-5 — May a geometric slot read a measured value.** After D, a `Measure` defines a scalar variable, and D10 lets any slot read any variable of its type. So an extrude's depth could be defined as a distance measured on another body: a driven dimension.

- It is expressible and acyclic, because the schedule orders it.
- It makes a value exist only mid-evaluation, which every analysis lane must then bind per lane.
- It makes a measured face's name resolution part of the geometry's dependency, not just of a check.
- It is the shape behind several existing requests (a hole sized to a mating part).

Whether stage 2 admits it, refuses it at the door with a typed error, or admits it only into definitions read by assertions is Ev's call. D10 is silent on it. *Blocks D's schedule work.*

### Questions with a recommendation (not forks)

1. **Delete with readers.** D10 settles that readers are left unresolved and typed. **Recommendation:** accept the delete and report each stranded read as `Maintenance::Strand` (DM7 generalised). `DeleteWouldDangle` and `DanglingInput` retire in B. `cascade_delete_order` stays as the GUI's "delete with dependents".
2. **Declared pairs** (`Boolean.declare`, `Union.declare`). D10 retires them in stage 4, and converting them to selects only to delete them later is wasted churn. **Recommendation:** leave them as names until stage 4. The eval-time ladder keeps its declared-pair callers until then, and `payload_names` shrinks to them.
3. **Gauges** (`Gauge.parent`, `InstantiatePart.gauge`) are not DAG edges today, and A11 (2)'s gauges retire in stage 3. **Recommendation:** leave them as node ids in stage 2, and keep `reading_edges`' gauge edges inside A9's partition when F deletes the mate half (test 16).
4. **A9 and measures** (`a-measure-merges-free-instances-into-one-relative-freedom-component`). Stage 2 does not change the partition's behaviour. **Recommendation:** leave the row parked and move its trigger to stage 3, where spaces are decided. Its fix (stop at space-free values, as `spaces_with` does) belongs with the per-space frame.
5. **Authored spelling of an operand.** **Recommendation:** a `RecipeNodeId` stays authored sugar for that node's port 0 in Rust and Python. A multi-output node (per FORK-1) needs an explicit `Output { node, port }` and refuses the sugar with `AmbiguousOutput`. This keeps ~2,300 test sites and 743 Python sites unchanged.
6. **Output minting.** **Recommendation:** draw output ids from the insert's own chain step under an `OUTPUT_TAG`, as `steps_of_insert` draws step ids, so A moves no node id and stays a cheap checkpoint. B then moves every id once, because the preimage spells reads.
7. **Slot table.** Operand, measure, select and mate slots join `SlotId` and `tests/golden/slot_tables.txt`. **Recommendation:** name them by field (`SlotId::Operand(OperandSlot::Target)`, …), not positionally, so the Python slot words stay readable. `placement-step-slots-are-spelled-three-ways` stays separate (stage 3).
8. **The mate's face across the at-rest gate.** **Recommendation:** the select reads the member's `Body` output (the walk's minting instance). The gate maps the resolved entity into the product table through the instance's graft map, as `transform_rigid` already records. It no longer resolves the qualified name a second time. That is one resolution, at the select.

**Inconsistencies found.**

- ASSEMBLY A12 says reading edges are not consuming "because a consuming operand would take the mated bodies out of A10's root set". After B every operand is a read, so the stated reason is A10's alone, which is why C precedes F.
- `Node::Measure`'s content-key arm hashes `at` ids as payload (`eval/mod.rs:5743`), as does `Mate`'s (`:5608`), against `content_key`'s stated rule that upstream identity enters by content. D and F fix both.
- SELECT-DESIGN §4's one-type rule and D10's `Face`/`Edge` variables state the stored form of a selection differently (FORK-3).
- `docs/guide/assembly.md:1227` ("a mate is a product root") is A10's, and is reworded in C.
