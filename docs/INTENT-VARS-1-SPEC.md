# SPEC: `variables-replace-the-parameter-table` (VR1–VR3, VR7, VR8)

Scope: minted `VarId` identity, a separate unique name, `Expr` reading `Var(VarId)`, the variable edit doors, persistence and load checks, and id-keyed tokens, symbols and seeds. **Literals stay** (`ExprKind::Literal`/`CountLiteral` and every `Expr::literal*`/`length_in` constructor). `ParamSource` keeps lowering a literal by its bits until `no-dimensioned-literal-in-a-slot`.

## 0. Two recommendations up front

**R1 — Leave `Defined` to `parameters-defined-by-formulas`.** This unit ships `VarDef` as an exhaustive enum with one arm, `Free`, plus a `DefineVar` door. The next unit adds `Defined(Expr)` as a second arm, and every reader's `match` fails E0004 until it handles it. Why not now:
- Derived variables need more than an arm: a cycle refusal at both doors, environments built in topological order, `ParamSource` expansion through definitions (VR8's "expanded to free ids"), and the free-only seed/axis/draw sets across `analysis.rs`, `stackup.rs`, `mc.rs` and `drive.rs`.
- This unit is already cost H.
- With no `Defined` arm, VR8's expansion is the identity, so there is nothing to half-build.

**R2 — Do not introduce `Formula` now. Use an authored `Name` leaf in `Expr` that the edit door lowers, so "a stored document holds no name" is an invariant checked at two doors.**
- A real `Formula` type would need an authored form of `Node<P>`: about 25 arms with slot exprs, plus the walkers `rows_mut` and `payload_exprs_mut` in `crates/editor-core/src/node.rs`. VR4 (next unit) reshapes exactly those slots into `VarId`, and `InsertNode` will need an authored-slot node form then anyway. Designing it now would design VR4 early and throw it away.
- The `Name` leaf makes the test migration a rename. `Expr::param(ParamName::from_static("w"), D)` becomes `Expr::named(VarName::from_static("w"), D)`. That matters because most of the 199 `Expr::param(` sites (101 files) build expressions with no document in scope: fixture fns like `fn p(n) -> Expr`, closures defined before the `Recorder`.
- `parse_expr(src, &BTreeMap<VarName, Dimension>)` keeps its signature (163 call sites) and emits `Name` leaves. The next unit moves the `Name` leaf into `Formula` and deletes it from `Expr`, which turns the invariant into a type fact.

## 1. Final shapes

**Identity and name** (new `crates/editor-core/src/var.rs`; spoken forms in `spoken.rs`)
- `pub struct VarId(pub u64)`, derived like `RecipeNodeId` (`node.rs`).
  - `Display` is the 12-hex tag through `spoken.rs::write_tag`; `FullId` gives the machine form.
  - Minted only from the mint chain; never reused.
- `VarName` is today's `ParamName` (`doc.rs`) renamed, with the same admissibility door: `new` / `from_static` / `try_from = "String"`, with `parse::param_name_fault` renamed `var_name_fault`. It is unique per document, and it is not a `Label`.
- `VarKind { Length, Angle, Scalar, Count }`, with `dimension()` and `From<Dimension>`. Later stages add non-scalar arms.
- `SpokenVar { id, name: Option<VarName> }` renders as `w`, or as `variable 3fa9c1d2a0b1`. Refusals carry it, the way they carry `SpokenNode`.
- `VarRef { Id(VarId), Name(VarName) }` is the authored address in edits, with `From<VarName>` and `From<VarId>`. The door resolves it.

**The variable** (replaces `DocParam`, `DocParamValue` and `Doc.params`)
- `FreeVar` is today's `DocParam`, renamed with arms, fields and methods unchanged: `continuous`, `written_length`, `with_value`, `with_display_unit`, `with_distribution`, `first_non_finite`, `bit_eq`.
- `FreeValue` is today's `DocParamValue`.
- `pub enum VarDef { Free(FreeVar) }` and `pub struct Var { kind: VarKind, def: VarDef }`.
  - Invariant: `def`'s kind equals `kind`.
  - `kind` is fixed at minting.
- `Doc` fields:
  - `vars: BTreeMap<VarId, Var>`, through a `persist::strict::vars` map.
  - `var_names: BTreeMap<VarId, VarName>`, beside the variable like `labels`: `default` and `skip_serializing_if = "BTreeMap::is_empty"`. A rename touches nothing else.
  - `params` is deleted.
- New `Doc` methods:
  - `vars()`, `var(VarId)`, `var_name(VarId)`, `var_named(&str) -> Option<VarId>` (a linear scan; documents are small), `spoken_var(VarId)`.
  - `var_scope() -> BTreeMap<VarName, Dimension>`, the parser's scope.
  - `var_env::<T>()`, which replaces `param_env`.
  - `var_readers(VarId)`, one walk over `rows()`, `payload_exprs` and program rows.
  - `bit_eq` compares `vars` and `var_names`.

**Mint** (`mint.rs`)
- `Minted::Var(VarId)` is a new arm and serializes as `{"var": n}`.
- `VAR_TAG = b"mint/var\0"`.
- New `MintingEdit::DeclareVar { name, def }` arm; its preimage has the display unit erased (D6). The name is included because it is part of the edit's statement; pin it with a fixture.
- `Mint::declare(&mut self, …) -> Result<VarId, VarIdCollides>`, and `has_var`.
- `InsertNode`'s preimage is the node **after** lowering, so a node authored by name and the same node authored by id mint one node id.

**Expr** (`expr.rs`, `persist/wire.rs`)
- `ExprKind::Param(ParamName)` is replaced by:
  - `Var(VarId)`, the stored reader, whose leaf `dim` caches the kind;
  - `Name(VarName)`, authored only.
- Constructors: `Expr::var(VarId, Dimension)` and `Expr::named(VarName, Dimension)`. `Expr::param` is deleted.
- `param_refs` splits into `var_reads(&mut Vec<(VarId, Dimension)>)` and `named_reads(...)`; the same applies in `measure.rs::MeasureExpr`.
- New `lower_names(&mut self, scope)` and `remap_vars(&BTreeMap<VarId, VarId>)`.
- `VarEnv<T> { bindings: BTreeMap<VarId, ParamValue<T>> }` replaces `ParamEnv`.
- `EvalError::UnknownParam` and `EvalError::ParamDimensionMismatch` become:
  - `UnresolvedVar { var }`;
  - `VarKindMismatch { var, bound, read }`;
  - `UnloweredName { name }`, a typed refusal rather than a panic. It is unreachable from a stored document.
- `unparse(expr, names: &impl Fn(VarId) -> Option<&VarName>)`, plus a `Doc::unparse` convenience. A `Name` leaf writes its text. An unnamed or unresolved `Var` writes `#<16 hex>`, which the parser does not read (open question 1).
- `WireExpr` gets `Var { var, dim }` and `Name { name, dim }`. `WireExpr::Param` is deleted.

**Edits** (`edit.rs`; `EditRecord` gains `minted_var: Option<VarId>`; `Recording::declare(name, def) -> Result<VarId, EditError>`)

| Edit | Semantics | Refusals |
|---|---|---|
| `DeclareVar { name: VarName, def: VarDef }` | Mints the id and records the name. Structural when the kind is `Count`. | `VarNameTaken { name, holder }`, `VarIdCollides`, and today's def checks from `write_doc_param`: `NonFiniteVar`, `InvalidDistribution`, `ContinuousVarCannotBeCount`, `VarUnitMismatch` |
| `DefineVar { var: VarRef, def }` | Replaces the definition and keeps the identity (the free↔defined door; free only here). | `UnknownVar { var, door }`, `VarKindFixed { var, kind, offered }`, def checks |
| `SetVarValue` / `SetVarUnit` / `SetVarDistribution { var: VarRef, … }` | Today's carry-forward doors, unchanged (`CarryForwardDoor` stays). | Today's, renamed from `DocParam*` to `Var*` |
| `RenameVar { var: VarRef, name: Option<VarName> }` | Writes only `var_names`. Not structural, no `DocDiff` entry, nothing recomputes. | `UnknownVar`, `VarNameTaken`, `VarNameUnchanged` (`SetLabel`'s twin), `AnonymousVarUnread` (clearing the name of a variable with no reader) |
| `DeleteVar { var: VarRef }` | Removes the variable from `vars` and `var_names`. The mint log keeps the id and readers are untouched (unresolved, typed). Structural if it had readers. | `UnknownVar`, `DeleteAnonymousVar` (open question 3) |

**Every arm that carries an `Expr`** goes through one exhaustive `DocEdit::exprs_mut()`, so a new arm has to say which of its exprs lower. The arms: `InsertNode` (all `rows_mut` plus `payload_exprs_mut`), `SetParam`, `SetStructuralParam`, `SetExpression`, `SetProgram`, `SetOffset`, and the rest the match forces.
- Each such edit first lowers `Name` leaves against `var_names`.
- It then checks every `Var` leaf **it writes**. That leaf must be live and kind-equal, otherwise it refuses with `SlotUnknownVarName`, `SlotVarKind` or `SlotUnresolvedVar`. Payload twins: `PayloadUnknownVarName`, `PayloadVarKind`, `PayloadUnresolvedVar`.
- Pre-existing unresolved readers elsewhere in the document are not re-checked.
- Last, a post-pass removes every **anonymous** variable this edit left unread, reported as `Maintenance::AnonymousVarRemoved { var }` (VR7).
- Because the kind is fixed, `write_doc_param`'s "re-validate every slot after a (re)declaration" pass is deleted.

**The edit log stores edits as authored.** Names and `VarRef::Name` can appear in the log and resolve again on replay, in order, so replay is deterministic. This is VR6's future shape for logs too. Snapshots never hold names outside `var_names`.

**Persistence and load door** (`persist/check.rs`; `Walk` gets new roster rows). There is no version: the old format refuses `Unreadable` with `REGENERATE_RECOURSE` because `Doc`'s `deny_unknown_fields` names `params`. The checks, in order:
1. The per-variable walks: today's `DocParam` float, distribution and notation walks re-keyed by `VarId`, plus `kind == def.kind()`.
2. Every `vars` key is logged as `Minted::Var`, otherwise `VarNotMinted`.
3. Every `var_names` key is live. No name is held twice (`VarNameTwice { name, a, b }`).
4. No snapshot expression holds a `Name` leaf (`NamedReaderInSnapshot { node }`).
5. Every `Var` leaf is minted (`ReaderOfUnmintedVar`). A live one matches its kind (`SlotVarKind` / `PayloadVarKind`). A dead one is legal and unresolved.
6. Every unnamed variable has at least one reader (`AnonymousVarUnread`).
7. Log replay, as today.

The same validator runs on save (`validate_document`).

**Tokens, symbols, analysis**
- `param_source.rs::encode`: the parameter leaf becomes a new tag `T_VAR` followed by the id as 8 big-endian bytes. Use a fresh tag value and add it to the census `ALPHABET`. The length-prefixed name encoding and `T_PARAM` are deleted; `invert` is unchanged.
- `geom-core/src/sym.rs`: `ParamSymbol::of(&str)` is replaced by `ParamSymbol::new(u64)`, and the editor passes `VarId.0`.
- `analysis.rs`: `axis_named(&ParamName)` becomes `axis_of(VarId)`.
- Re-key by `VarId` (refusals carry `SpokenVar`):
  - `ParamBox::axes` and `AnalyzedParams` (`analysis.rs`);
  - `EvalOptions::seed` (`eval/mod.rs`, `eval/wire.rs`);
  - `stackup::Sensitivity::param`;
  - `mc.rs` draws;
  - `drive.rs` depths;
  - `clearance.rs::monotone_in`;
  - `report.rs::by`;
  - `range::RangeField::Param` / `DerivedRange::{axis, pinned}`;
  - `diff.rs::DocDiff::params`, which becomes `vars` (definition changes only; names excluded);
  - `program.rs::references(&ParamName)`, which becomes `reads(VarId)`;
  - `refactor.rs::node_param_refs`, which becomes `node_var_reads`.

**Split and inline** (`refactor.rs`)
- `split` declares each variable that cut nodes read in the part document, then carries those nodes with `remap_vars` from the part's minted ids. A variable read on both sides still refuses (`UncutVarReference`).
- `inline` keeps today's rule: merge by name when the definition is `bit_eq`, otherwise `VarNameConflict`. Carried readers are remapped.
- Anonymous variables are carried by declaring them under a fresh name and clearing the name once the readers land (open question 2).

**Façade, Python, viewer**
- `pncad`: re-exports in `document.rs` and `prelude.rs`; `docs/GUIDE.md` is doctested through `crates/pncad/src/guide.rs`.
- `pncad-py`:
  - New `Var` class (full-hex repr).
  - `Document.vars() -> dict[Var, VarDecl]`, `Document.var(name) -> Var | None`.
  - `DocEdit.declare_var / define_var / set_var_value / set_var_unit / set_var_distribution / rename_var / delete_var`, each accepting `Var | str`.
  - The `set_doc_param*` statics are deleted. `bind_*_param` keep taking names, which are authored.
  - Error tags in `tags.rs` are renamed, along with `pncad.pyi` and the surface and prose census.
- `viewer`: the ops in `session/op.rs` (`SetParam`, `SetParamUnit`, `SetParamText`, `Begin/Preview/CommitParamGesture`) are keyed by `VarId`. `CreateParam` becomes `DeclareVar`. New `RenameVar` and `DeleteVar` ops are tested headless. Params-panel rows in `props.rs` are keyed by `VarId` and show `SpokenVar`. `session.rs::param_dims` becomes `doc.var_scope()`.

## 2. What is deleted

- `ParamName` (becomes `VarName`), `DocParam` / `DocParamValue` (become `FreeVar` / `FreeValue`).
- `Doc.params`, `Doc::params()`, `Doc::param_env`, `ParamRefFault`, `Doc::param_ref_fault`, which becomes `var_read_fault`.
- `ExprKind::Param`, `Expr::param`, `param_refs`, `ParamEnv`, `EvalError::UnknownParam` / `ParamDimensionMismatch`.
- `DocEdit::SetDocParam` and create-or-replace altogether. `SetDocParamValue` / `SetDocParamUnit` / `SetDocParamDistribution` become `SetVar*`.
- `write_doc_param`'s slot re-validation pass, `UNDECLARED_PARAM_RECOURSE`, `WireExpr::Param`, `persist::strict::params`, the `T_PARAM` encoding, `ParamSymbol::of`.
- `range::synthetic_name` moves to `VarId` keys. The fresh-name helper can stay internal until the next unit can mint an anonymous variable in one slot edit.

## 3. Migration: three sequential PRs, each green

**PR 1 — pure renames, no behaviour change (about 200 files, mechanical).** `ParamName→VarName`, `ParamNameFault/Reason→VarNameFault/Reason`, `DocParam→FreeVar`, `DocParamValue→FreeValue`. Put the sed script in the PR body. Pin bytes do not move: serde tags are variant names, and the field and type names involved are not on the wire. Verify with the golden.

**PR 2 — the table keyed by minted id; readers still read names.**
- New: `VarId`, `Minted::Var`, `Var`/`VarDef`/`VarKind`, `vars` plus `var_names`, the `DeclareVar` / `DefineVar` / `SetVar*` doors, and the persistence and load walks 1–3.
- `ExprKind::Param(VarName)` stays and resolves through `var_names` at environment build (`var_env` stays name-keyed internally).
- `ParamSymbol::new`, and analysis, seeds, stackup and MC keyed by `VarId`.
- `RenameVar` and `DeleteVar` are absent, so name→id is a function and the intermediate state is coherent.

Rewrite rules:
- `DocEdit::SetDocParam { name, value }` → `DocEdit::DeclareVar { name, def: VarDef::Free(value) }`. A site that re-sets an existing name refuses `VarNameTaken` at runtime, which is the loud backstop: fix it to `DefineVar { var: name.into(), … }` or `SetVarValue`.
- `SetDocParam{Value,Unit,Distribution} { name, x }` → `SetVar{Value,Unit,Distribution} { var: name.into(), x }`.
- `doc.params().get(&n)` → `doc.var_named(n.as_str()).and_then(|v| doc.free(v))`. Add a `Doc::free_named(&str) -> Option<&FreeVar>` read helper.
- Analysis lookups `.get(&ParamName::from_static("w"))` → `.get(&doc.var_named("w").unwrap())`.
- `ParamSymbol::of("b")` → `ParamSymbol::new(<file-local constant>)`, 49 sites in 29 files.
- Test `Recorder` (`tests/fixture/mod.rs`) gains `declare(name, def) -> VarId`.
- Re-bless `tests/golden/golden.cad` (`M4_PR6_BLESS_GOLDEN=1`): node ids move in every document that declares a variable, because the declare extends the chain.

**PR 3 — readers read ids.**
- `ExprKind::Var` and `ExprKind::Name`, door lowering, `VarEnv` by id, `T_VAR` tokens.
- `RenameVar`, `DeleteVar`, anonymous garbage collection, load walks 4–6.
- `unparse` with names, split/inline remap, Python `rename_var` / `delete_var`, viewer rename and delete ops.

Rewrite rules:
- `Expr::param(` → `Expr::named(` (a rename).
- Hand-built `ParamEnv { bindings: [(name, v)] }` with `eval` → `VarEnv` keyed by `VarId(n)` with `Expr::var(VarId(n), D)`. This is about 41 files and needs per-file edits, all in pure-expression tests.
- `unparse(e)` → `doc.unparse(e)` (25 sites).
- `EditError` / `SnapshotError` variant renames, through a mapping table in the PR body (`SlotUnknownDocParam→SlotUnknownVarName` and so on). They ripple into the `pncad-py` tags, the `display_contract` rows and the `f6_variants!` weld rosters.
- Re-bless the golden again: readers' bytes and node-insert preimages change.
- Geometry must not move. A frame that changes in `demos/tour` is a bug, not a re-baseline.

Each PR also migrates `demos/tour` (`chain.rs`, `plate.rs`, `impeller.rs`, `mcplate.rs`, `tolerance.rs`) and `docs/GUIDE.md`. Run `(cd demos/tour && cargo clippy --all-targets -- -D warnings)` and the same in `demos/wild`.

**Sweep** (discipline §5), with a hit list in the PR body:
- `ParamName|VarName` used as a map key or hash input;
- `as_str()` or `format!` of a variable name feeding `KeyHasher`, `Sym` or `ParamSource`;
- `BTreeMap<VarName`.

Blind spot: names reaching keys through `Display`. Second pass: grep `KeyHasher` / `feed_` call sites for anything rendered.

## 4. Test plan (each assertion names the runtime value that would break it)

1. **A rename moves nothing that identifies.** Fillet radius reads `w`; evaluate; `RenameVar w→v`; evaluate again. Assert:
   - the face `ParamSource` tokens are bytewise equal;
   - every node is a memo hit (recompute count 0);
   - `DocDiff` is empty;
   - `EditRecord.structural == false`;
   - the `ParamSymbol` is unchanged;
   - `doc.unparse(slot) == "v"`.
2. **Delete leaves readers unresolved.** After `DeleteVar w`:
   - the slot still holds `Var(old)`;
   - evaluation refuses at each reader with `UnresolvedVar { var: old }`, and a non-reader still builds;
   - re-declaring `w` mints `new != old`, and the readers stay on `old`;
   - save/load round-trips the unresolved document;
   - a hand-edited reader of a never-minted id refuses `ReaderOfUnmintedVar`.
3. **Uniqueness.**
   - `DeclareVar` with a taken name refuses with the holder's id.
   - `RenameVar` onto a taken name refuses; a no-op rename refuses `VarNameUnchanged`.
   - Loading two ids under one name refuses `VarNameTwice`.
4. **The mint never reuses.**
   - Declare, delete, then declare the identical statement: different ids, and the log holds both as `Minted::Var`.
   - `Mint::logged([Minted::Node(RecipeNodeId(x))])`, where `x` is the bits the declare would draw, refuses `VarIdCollides` with chain and log untouched.
5. **Equal values are not one variable.** `w = v = 5 mm`. Fillets reading `w` and `v` lower to distinct tokens, and `topo::field_source_evidence` is not `Declared`. Two fillets both reading `w` lower equal and are `Declared`.
6. **The symbol is keyed by id.** In the symbolic tier, `w − w` decides Zero and `w − v` at equal values does not. The symbol is the same before and after a rename.
7. **Lowering.**
   - The same node authored by name and by id mints one node id and stores `bit_eq` nodes.
   - An unknown name refuses `SlotUnknownVarName`, with the slot address.
   - A name read at the wrong dimension refuses `SlotVarKind`.
   - A log holding authored names replays to identical ids.
8. **Kind is fixed.** `SetVarValue(Count)` on a Length variable refuses. `DefineVar` with a different kind refuses `VarKindFixed`.
9. **Anonymous lifecycle.**
   - Clearing the name of an unread variable refuses.
   - With a single reader, a `SetParam` that replaces the reader removes the variable, reports `AnonymousVarRemoved`, and the mint log keeps the id.
   - Loading an unread anonymous variable refuses.
10. **Persistence.**
    - Round-trip with `bit_eq` after load, covering the `var` mint arm, names, an anonymous variable and an unresolved reader.
    - A mutated fresh save carrying `"params"` refuses `Unreadable`, naming `params`.
    - A snapshot carrying a `Name` leaf refuses `NamedReaderInSnapshot`.
11. **Analysis.**
    - Stackup sensitivities, MC draws and `ParamBox` axes are keyed by the same `VarId` across a rename, with equal values.
    - A seed on an unresolved variable refuses typed.
12. **Split and inline.**
    - Carried readers point at the part's own minted ids, and the part loads.
    - A variable read across the cut refuses.
    - Inline with a same-name variable that is not `bit_eq` refuses `VarNameConflict`.
13. **Python and viewer.**
    - A `Var` handle survives `rename_var`, and `delete_var` surfaces the unresolved refusal by tag.
    - The viewer's params-panel row identity (keyed by `VarId`) survives the `RenameVar` op.

## 5. Risks

- **The invariant is not a type.** `Name` in a stored document is guarded by the door post-condition and load walk 4, plus `UnloweredName` at evaluation. It holds for one unit only; the next unit's `Formula` makes it a type fact.
- **Ids and pins move.** Every document that declares a variable gets new node ids (chain extension, and reader bytes in insert preimages). Re-bless what moves and say so; there are no external files (`persist/mod.rs` "No schema version").
- **Stale memo.** `DocDiff` and incremental invalidation (`resolve/mod.rs` around line 2528) are re-keyed by id. A missed definition change serves a stale memo; tests 1 and 11 guard it.
- **Error renames ripple** into census and gate rosters (`pncad-py` `tags.rs`, `surface_census.rs`, `display_contract`, `f6_variants!`). They fail loudly; budget for them.
- **Split/inline semantics change** from identity-by-name to explicit remapping.
- **Runtime-only catch in PR 2.** A `DeclareVar` that should have been a `DefineVar` compiles and only fails at runtime. Run the full editor-core suite before marking the PR ready.

## 6. Open questions

1. How should an unnamed or unresolved reader be spelled in text? Recommendation: `unparse` writes `#<16 hex>` and the parser refuses it, since in this unit anonymous variables arise only by clearing a name. Decide a readable tag in the next unit, where the GUI needs it.
2. How does split/inline carry an anonymous variable? Recommendation: declare it under a fresh name, then clear the name once the readers land. Alternatively refuse `AnonymousVarCrossesCut` until VR6.
3. `DeleteVar` on an anonymous variable: refuse (its lifecycle belongs to its readers), or allow? Recommendation: refuse.
4. Inline's merge-by-name-when-bit-equal infers identity from equal values, which is the thing D10 forbids. This unit keeps it. Should a later stage make it a refusal? That is an Ev question if it changes.
5. Should the declare preimage include the name? Recommendation: yes, pinned by a fixture.
6. Reuse `Dimension` instead of a new `VarKind`? Recommendation: `VarKind`, because stage 2 adds non-scalar kinds.

## 7. Size

Cost H overall. Rough diff sizes:
- PR 1: about 200 files, about 2.5k lines, mechanical.
- PR 2: about 170 files, 5–6k lines.
- PR 3: about 140 files, 4–5k lines.

The design pair's PRs (2 and 3) each draw the dual review (`work/intent/plan.md`, "Review posture").

### Critical files
- /home/user/cad/crates/editor-core/src/doc.rs
- /home/user/cad/crates/editor-core/src/edit.rs
- /home/user/cad/crates/editor-core/src/expr.rs
- /home/user/cad/crates/editor-core/src/mint.rs
- /home/user/cad/crates/editor-core/src/persist/check.rs
- /home/user/cad/crates/editor-core/src/param_source.rs
- /home/user/cad/crates/editor-core/src/refactor.rs
- /home/user/cad/crates/geom-core/src/sym.rs

## 8. Orchestrator rulings (binding; they override the sections above)

- **R1 accepted**: `VarDef` ships with the one arm `Free`; `Defined` is
  `parameters-defined-by-formulas`.
- **R2 accepted**: the authored `Name` leaf in `Expr`, lowered at the
  edit door; it retires in `no-dimensioned-literal-in-a-slot` (VR6's
  `Formula`).
- **The three-PR split is accepted**, each PR green on its own: PR 1
  (renames), PR 2 (the id-keyed table), PR 3 (readers read ids).
- **Q1**: `#<16 hex>` in `unparse`, refused by the parser.
- **Q2**: an anonymous variable crossing a split/inline cut REFUSES
  (`AnonymousVarCrossesCut`). Declaring it under a fresh name would
  have the kernel mint a name (VR2 forbids it).
- **Q3**: `DeleteVar` on an anonymous variable refuses.
- **Q4**: inline's merge-by-name-when-`bit_eq` stays in this unit;
  it is filed as `inline-merges-variables-by-equal-value`.
- **Q5 overruled**: the name is NOT in `DeclareVar`'s mint preimage
  (VR2, and the node-label precedent: a label is in neither the id's
  mint nor any content key). Two declares of one definition still mint
  distinct ids because the chain extends.
- **Q6**: a new `VarKind`.

