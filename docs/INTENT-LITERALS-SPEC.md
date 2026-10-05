# SPEC: `no-dimensioned-literal-in-a-slot` (VR4–VR6, VR9), with `parameters-defined-by-formulas`

Scope: a slot holds a `VarId` (VR4); `Expr` holds no float, only `Var` leaves, exact rationals and `turn` (VR5); an authored `Formula` is lowered at the edit door and mints an anonymous free variable per written quantity (VR6); the façade, Python and persistence follow (VR9). It also closes `equal-literals-lower-to-one-identity-token`, and in passing `range-synthetic-name-mints-a-name` and `python-param-classes-name-a-variable`.

Baseline: main at `332f28e1c3`. INTENT-VARS-1 is merged. Today a slot is an `Expr`, `VarDef` has the one arm `Free`, `ExprKind` has `Literal(Lit{f64, UnitSym})`, `CountLiteral(i64)`, `Var(VarId)` and `Name(VarName)` (authored-only), and `param_source::encode` writes `T_LITERAL` + f64 bits.

## 0. Ordering: one program, five PRs, `Defined` first

VR4 says a slot showing `w * 2` holds an anonymous **defined** variable. In today's tree slots hold compound expressions everywhere: `parse_expr("w * 2")`, `Expr::add(…)`, the corpus's `r − t`. So the slot change cannot land before `VarDef::Defined` exists. The two units therefore merge into one sliced sequence. `parameters-defined-by-formulas` is PR A and closes on its own. The rest close `no-dimensioned-literal-in-a-slot` at PR D.

| PR | Lands | Representation step it completes | Goldens |
|---|---|---|---|
| A | `VarDef::Defined(Expr)`, cycle refusal, topological env, expansion in tokens | definitions | unchanged except files that add a defined var |
| B | `Formula` is the authored type and `Expr` the stored one. Doors take `Formula`. Node and program are generic over the slot type | authored/stored split (VR6's types). The `Name` leaf leaves `Expr`, which retires INTENT-VARS-1's R2 interim | **byte-identical**: the PR's own check |
| C | A stored slot is a `VarId`. Slot-root lowering mints anonymous variables, with a fresh table | VR4 | re-blessed (ids move); f64 geometry must not move |
| D | `Expr` loses `Literal`/`CountLiteral` and gains `Ratio`, `Integer`, `Turn`. Lowering mints inside definitions | VR5 and the rest of VR6 | re-blessed; f64 geometry must not move |

Every PR leaves the tree building and every suite green. Each intermediate state is a whole representation:
- after A, a definition may hold literals, as slots still do;
- after B, both types still carry literals;
- after C, a defined variable's `Expr` may hold a literal leaf, the pre-VR5 `Expr`;
- D removes that leaf everywhere at once.

Rejected: **VR5 before VR4** (a minting lowering while slots still hold `Expr` leaves sharing untyped and C's readers written twice) and **one PR for B–D** (~600 files, no byte-identical checkpoint).

## 1. Final shapes (after D)

**`Expr`** (`expr.rs`) has these leaves:
- `Var(VarId)`, with the cached kind;
- `Ratio(Ratio)`, dimension `Scalar`;
- `Integer(i64)`, dimension `Count`;
- `Turn`, dimension `Angle`.

The operators, the F1 lattice and `MAX_NESTING` are unchanged.

`Ratio { num: i64, den: u64 }` has private fields, is reduced with `gcd(|num|, den) = 1` and `den ≥ 1`, and is bounded by `|num| ≤ 2^53` and `den ≤ 2^53`. Constructors:
- `Ratio::new(num, den)` and `Ratio::from_decimal(&str)` are exact (`0.1` gives 1/10). They refuse a zero denominator or a value out of range with `DimensionError::ConstantOutOfRange { text }`.
- No rational arithmetic exists anywhere. The operators stay tree nodes and nothing folds, so overflow is possible only at construction.

Evaluation:
- `Ratio` evaluates as `T::from_f64(num) / T::from_f64(den)`, exact operands and one operation:
  - **f64:** one correctly-rounded division, the same bits the decimal parse gives today (`round(p/q)` either way), so f64 geometry does not move;
  - **Interval:** outward-rounded, so it encloses the true 1/10 where today's point interval at the double does not;
  - **Dual:** a zero tangent;
  - **Sym:** `Mul(Lit p, Inv(Lit q))`, which the form folds to the exact rational p/q instead of `Rat::of_f64(0.1)`'s dyadic.

  A `den == 1` ratio skips the division.
- `Turn` evaluates to `T::tau()`. That is the tight enclosure at Interval and the node `2·π` at Sym (`sym.rs` `tau`). `turn/4` evaluates in f64 to the bits of `FRAC_PI_2`, because division by 4 is exact.
- `Integer` is today's `CountLiteral`.

Deleted from `Expr`:
- the constructors `literal`, `literal_with_unit`, `written_length`, `written_angle`, `length_in`, `angle_in`, `count` and `named`;
- `literal_value`, `display_unit`, `literal_bits` and `erase_display_units`;
- `Lit` and the leaf `Name`.

Their errors go with them: `LiteralCountIsInteger`, `NonFiniteLiteral` and `DisplayUnitMismatch` move to `Formula`. `EvalError::UnloweredName` is deleted.

**`Formula`** (new, `formula.rs`) is the authored twin. It has the same operator set and dimension checks, and is `Clone`, serde and `Display`. Its leaves:
- `Var(VarId)` and `Name(VarName)`;
- `Quantity(FreeVar)`: a written quantity, which is a value plus its unit, or a count, plus an optional distribution;
- `Fresh(u16)`: an index into the edit's fresh table (Q2);
- `Ratio`, `Integer` and `Turn`.

Constructors:
- `Formula::length_in / angle_in / written_length / written_angle` replace their `Expr` namesakes.
- `Formula::scalar(f64)` writes a `Scalar` quantity with unit `ONE`.
- `Formula::count(i64)`.
- `Formula::ratio(p, q)` and `Formula::turn()` are the constants.
- `Formula::var` and `Formula::named`.
- `From<VarId>`, `From<VarName>`, `From<WrittenLength>` and `From<WrittenAngle>`.
- `with_distribution` is available on a `Quantity` leaf.

The text door is `parse_formula(src, scope) -> Formula`, replacing `parse_expr`. The grammar is unchanged except for two points:
- A bare number is a constant: an integer is `Integer` and a decimal is `Ratio`.
- `turn` is a keyword. A unit-suffixed number is a `Quantity`.

`unparse(&Expr, names)` writes:
- `Ratio` as a decimal when `den = 2^a·5^b`, and as `p/q` otherwise;
- `Turn` as `turn`;
- an anonymous `Var` as its value in its unit (`5 mm`), with `#<16 hex>` kept for diagnostics.

`unparse` (and `Formula`'s `Display`) round-trips through the parser up to the identity of anonymous variables.

**Slots** (VR4). `Node<P, S = VarId>` is generic over the slot type, and so are `LoopProgram<S>`, `ProgramStep<S>`, `ArcSpec<S>`, `Placement<S>`, `PatternKind<S>`, `PartSelect<S>`, `Datum<S>` and `TubeWindow<S>`.
- The authored form is `S = Formula` and the stored form is `S = VarId`. `ProfilePayload` gains `type Authored` so that the payload follows `S`.
- `row_readers!` and `expr_table!` take `S`.
- One `try_map_slots(&self, f) -> Result<Node<P, S2>, E>` is the only conversion. Lowering is `try_map_slots(lower_slot)`, and re-authoring is `map_slots(Formula::var)`.
- A stored node therefore cannot hold a formula. That is a type fact, where today it is a door post-condition (`NameLeafWritten`).
- `MeasureExpr`'s `Value` leaf and an assertion's `bound` become `S` too (Q5). After D, `VarDef::Defined` is the only stored home of an `Expr`.

**Slot-root lowering** (`lower_slot`, one function used by every door):

| Lowered formula | Slot holds |
|---|---|
| a lone `Var`/`Name` | that id: this is how two slots share one variable |
| a lone `Quantity`, or a lone number (Q1) | a fresh anonymous `Free` variable, with the quantity's value, unit and distribution |
| a lone `Fresh(i)` | fresh entry `i` |
| anything else (`w * 2`, `turn/4`, `w + 5 mm`) | a fresh anonymous `Defined(expr)` variable |

Inside a definition, every `Quantity` leaf mints its own anonymous free variable and becomes a `Var` leaf. Two typed `5 mm` therefore become two variables (VR6, VR8).

Minting order is deterministic: rows order, then depth-first within a formula, with the fresh table first. Each mint uses `MintingEdit::DeclareVar`'s preimage, which is the kind only. The node id is minted from the lowered node (INTENT-VARS-1 §1).

**`VarDef`** (`var.rs`) becomes `Free(FreeVar) | Defined(Expr)`, and the authored twin is `VarDecl { Free(FreeVar), Defined(Formula) }`. `Var::kind` is the expression's dimension. `DeclareVar { name, def: VarDecl }` and `DefineVar { var, def: VarDecl }` (free ↔ defined, same identity, VR7).

**Edits**:
- **Slot writes.** `SetParam` and `SetStructuralParam` take `formula: Formula`. They stay two arms, for spec D3's divide. Every slot-writing arm carries `fresh: Vec<VarDecl>` (Q2): `InsertNode`, `SetParam`, `SetStructuralParam`, `SetProgram`, `SetOffset` and `DefineVar`.
- **`SetExpression`** becomes `{ var: VarRef, path, formula }` and edits a defined variable's definition (Q8). The slot-addressed `ExprPath` stays as a diagnosis address: node, slot, then a path in the slot variable's expansion.
- **The anonymous GC** (`Maintenance::AnonymousVarRemoved`) cascades. An anonymous variable is live if a slot reads it, or if the definition of a live variable does.

**Tokens** (`param_source.rs`): `lower(scope, slot_var)` expands through definitions to free ids (VR8).
- A `Free` variable writes `T_VAR` + id.
- A `Defined` variable writes its expression, expanded recursively.
- `T_RATIO` (num, den), `T_INTEGER` and `T_TURN` are new. `T_LITERAL` and `T_COUNT_LITERAL` go to a retired-tags row in the census.
- `feed_content_key` writes the same expansion.
- The expansion is bounded. `DefineVar` and lowering refuse `DefinitionTooLarge { nodes }` past 4096 expanded nodes, so a token cannot grow exponentially through a diamond of definitions.

## 2. PR A — `parameters-defined-by-formulas` (cost M; ~60 files, 3–4k lines)

1. `VarDef::Defined(Expr)` and `VarDecl`. The definition is lowered by the existing door; `DocEdit::exprs_mut` already lists `DefineVar`.
2. Cycles:
   - The door refuses `DefinitionCycle { var, through: Vec<SpokenVar> }` on both `DeclareVar` and `DefineVar`.
   - Load gets a new walk, `DefinitionCycle`, which runs after the kind walk.
   - Load also checks `kind == expr.dim()` (`DefinedVarKind`).
3. Environments:
   - `Doc::var_env` and `analysis::var_env_over` bind the free variables, then evaluate the defined ones in topological order (a `Doc::definition_order()`, memoized on the doc), at the lane scalar.
   - A failing definition refuses at each reader with `EvalError::DefinitionRefused { var, source }`, wrapped with the slot address by `NodeErrorKind::Expr`.
4. Analysis reads free variables only:
   - axes and MC draws are already `free_vars`;
   - a seed on a defined variable refuses `SeedOnDefinedVar`;
   - stackup's entry set is `free_vars`;
   - a defined variable's sensitivity is the pushforward, with no entry of its own.
5. `SetVarValue`, `SetVarUnit` and `SetVarDistribution` on a defined variable refuse `NotAFreeVar`.
6. Invalidation: `DocDiff::vars` closes over definitions. A node reading `h := 2w` is dirty when `w` changes (`resolve/mod.rs`'s diff consumer).
7. Tokens: a `Var` leaf of a defined variable expands, so a slot `h` and a slot `2*w` lower equal.
8. Lifecycle cascade (above). `var_readers` includes definitions.
9. Surfaces:
   - `pncad` re-exports.
   - Python `Doc.define_var(var, Expr | VarDecl)`, and `VarDecl.defined(...)` in `pncad.pyi` and the census.
   - Viewer panel rows for a defined variable show its formula (via `unparse`), read-only; editing goes through `SetVarText`'s existing text door, as `DefineVar`.

## 3. PR B — authored `Formula`, stored `Expr` (cost H, mechanical; ~350 files, ~6k lines)

There is no behaviour change and no snapshot wire change. The log's authored leaves serialize as `WireFormula`, whose variant names equal today's `WireExpr` ones. `golden.cad`, `bool13_goldens/*` and the corpus `.pncad` files must stay **byte-identical**. If one moves, the PR is wrong.

- `Formula` is introduced with the leaves `Var`, `Name`, `Literal` and `CountLiteral`. `Ratio`, `Integer`, `Turn`, `Quantity` and `Fresh` arrive in C/D.
- `Expr` loses `Name`.
- The slot genericity of §1 lands with `S ∈ {Formula, Expr}`.
- `parse_expr` → `parse_formula`.
- The doors lower `Formula → Expr` (names resolved, nothing minted).
- `NameLeafWritten` and load walk 4 (`NamedReaderInSnapshot`) are deleted. The type now carries them.

Mechanical rewrites, with the sed script in the PR body:
- `Expr::named(` → `Formula::named(`, `Expr::count(` → `Formula::count(` and `parse_expr(` → `parse_formula(`;
- `Expr::{literal,literal_with_unit,written_*,length_in,angle_in}(` → `Formula::…`;
- the test helpers (`test_support::{len,ang,scl,len2}`, the viewer's `len_mm`, the tour's local `len/scl/ang/pe/param`) return `Formula`. That moves ~3,000 call sites without touching them.
- The arithmetic constructors at authored sites become `Formula::{add,…}`. This is compile-driven: a type error marks every site.

Hand edits are the readers of stored slots:
- `.expr(slot)`: 53 sites in 23 files;
- `literal_value` and `display_unit`: 64 sites, mostly the viewer's `props.rs`, `tree.rs`, `session.rs` and `sketch.rs`;
- `literal_bits`: 20 sites in 7 files.

Python:
- The `Expr` class becomes `Formula`. Stored definitions read back as `Expr`, read-only.
- ~1,040 `Expr.<ctor>(` sites in `crates/pncad-py/tests/**`, plus ~170 in `docs/GUIDE.md` and `docs/guide/*.md`, are renamed by sed.
- `ParamName`/`DocParam`/`DocParamValue` are renamed to `VarName`/`FreeVar`/`FreeValue` in the same census pass, which closes `python-param-classes-name-a-variable`.

Review: orchestrator's read. The byte-identical goldens are the check.

## 4. PR C — a slot holds a `VarId` (cost H; ~220 files, 6–8k lines)

**Kernel**
- `S = VarId` for stored nodes.
- `lower_slot` and the fresh table.
- `eval_slots` becomes an environment lookup: `env[var]`, where a free or defined variable is already bound by PR A.
- Readers move to `Doc` accessors:
  - `Doc::slot(node, slot) -> Option<VarId>`;
  - `Doc::slot_value(node, slot) -> Option<(f64, UnitDef)>`, for a free slot variable; this replaces `literal_value` + `display_unit`;
  - `Doc::slot_expansion(node, slot) -> Expr`.
- `Node::authored(&self) -> Node<P, Formula>`, by `map_slots(Formula::var)`, is what every re-authoring caller uses.

**Range.** `range.rs` closes `range-synthetic-name-mints-a-name`.
- `RangeField::Slot` resolves to the slot's variable. A free variable is widened in place: no declare, no name.
- A defined one refuses `SlotIsDefined`, replacing `SlotIsNotALiteral`.

**Split and inline** (`refactor.rs`). INTENT-VARS-1 §8 Q2 refused an anonymous variable crossing a cut. After this PR every node reads anonymous variables, so that refusal would refuse every split, and it retires.
- An anonymous variable crosses as a fresh-table entry of the first carried edit that reads it, with its definition bit-equal, distribution included.
- Later carried readers read the minted id.
- A named variable crosses as today.
- A variable read on both sides of the cut still refuses, now for anonymous variables as well (`UncutVarReference`).

**Persistence** (`persist/check.rs`). Slot fields serialize as `VarId`. The load walks:
- **Slot-reads-minted.** A slot's id must be logged as `Minted::Var`, otherwise `SlotReadsUnmintedVar { node, slot }`.
- **Slot kind (VR4).** A live slot variable's kind must equal `slot.dimension()`, otherwise `SlotVarKind`. The structural divide also refuses: a Count variable in a continuous slot, or the reverse.
- **Anonymous ⇒ read.** Walk 6 is re-read as live-through-a-reader (a slot, or the definition of a live variable).
- The existing walks are re-pointed at definitions.

**Façade, Python, viewer**
- Every slot argument takes `impl Into<Formula>` in Rust, so a `VarId`, a `Formula` or a written quantity is accepted (VR9).
- Python takes `Var | Formula | WrittenLength | WrittenAngle | Length | Angle | float | int` (Q9).
- `Doc.slot(node, slot) -> Var`.
- **Viewer value gestures and sketch drags** write `SetVarValue` on the slot's variable (Q6). This keeps identity, so a toleranced dimension keeps its distribution. Typing text into a slot writes `SetParam` (re-lower).
- **Viewer program edits** (`SetProgram`) re-author from `Node::authored()`, so unchanged arguments keep their ids.
- The panel lists named variables. An anonymous one shows at its slot.

**What it closes.** Two separately typed lone `5 mm` slots now mint two variables, so their tokens differ and `field_source_evidence` answers `None`. That is half of `equal-literals-lower-to-one-identity-token`. The half inside formulas closes in D.

## 5. PR D — `Expr` holds no float (cost M–H; ~160 files, 3–5k lines)

- `Expr`'s leaf set becomes §1's.
- `Formula` gains `Quantity`, `Ratio`, `Integer` and `Turn`, and loses `Literal`/`CountLiteral`. `Formula::{length_in,…}` now build `Quantity`.
- Lowering mints inside definitions.
- `WireExpr` becomes `Var | Ratio{num,den} | Integer | Turn | ops`. A snapshot holding `Literal` refuses `Unreadable` + `REGENERATE_RECOURSE` with no code: the variant is simply gone.
- `WireFormula` gets `Quantity { value, dim, unit, distribution }` and `Fresh`. A non-reduced or out-of-range ratio refuses at rebuild as `PersistError::Dimension` (`ConstantOutOfRange` / `RatioNotReduced`).
- `param_source.rs` gets the new tags.
- `sym.rs`: the `Lit` atom stays. It is what the kernel's own `from_f64` constants are, and no document value reaches it any more (that is the issue's second arm).
- Parser and `unparse` per §1. Python gains `Formula.ratio`, `Formula.turn` and `Formula.count`.
- The doc rows move with the change:
  - `crates/verbs/README.md` VS-Q4: "literals as `f64` bits" becomes "constants as exact rationals and `turn`";
  - `expr.rs` and `parse.rs` module docs;
  - DESIGN.md's companion row: VARIABLES-DESIGN "unbuilt" → "built (stage 1)".

  This is wording moved by the code. It changes no decision.

Closes `no-dimensioned-literal-in-a-slot` and `equal-literals-lower-to-one-identity-token`.

## 6. Literal sites, sized (grep at `332f28e1c3`)

| Where | Hits / files | Strategy |
|---|---|---|
| editor-core tests: helper and constructor sites (`len/ang/scl/len2`, `Expr::literal*`, `Expr::count`, `parse_expr`) | ~2,800 / 268 | B: the helper body returns `Formula`, plus sed. Compile-driven residue. |
| `Expr::count(` | 284 / 96 | B: sed to `Formula::count(` |
| `Expr::named(` | 202 / 105 | B: sed |
| `parse_expr(` | 177 / 39 | B: sed to `parse_formula(` |
| stored-slot readers (`.expr(slot)`, `literal_value`, `display_unit`, `literal_bits`) | ~117 / ~40 | C: hand-migrate to `Doc::slot*` |
| `next_mint` fixture (predicts the next node id by inserting an `xy_frame`) | 8 | C: delete. A predicted id is wrong once lowering mints variables first. Rows read the id the insert returns. |
| production authored literals (`program.rs` `len_lit/ang_lit/scalar_lit`, `from_recorded`; `eval/class.rs`, `eval/wire/stepped.rs` recourse proposals; `mint.rs`; `mate/member.rs`; `eval/measure.rs`) | ~30 / 12 | B: these are proposals and authored programs, so they become `Formula` |
| viewer | 460 / 54 | B (helpers, `author.rs`/`combine.rs`/`op.rs` `Expr` fields become `Formula`); C (readers, gestures) |
| `demos/tour` (`plate`, `chain`, `diefillet`, `bracket`, `teapot`, `heatsink`, `assembly`, `impeller`, `mcplate`, `tolerance`, `tests/teapot_document.rs`) | 150 / 12 | B: local helpers return `Formula`. C: where two equal literals must coincide, author one variable (below). |
| `demos/wild` | 0 expression sites | nothing to migrate; clippy it anyway |
| `pncad` (`tests/all.rs`, re-exports, `guide.rs` doctests) | 42 / 1, plus re-exports | B |
| `pncad-py` src, `.pyi` | 48 / 8 src; 92 `Expr` mentions, 35 signatures | B (rename), C (slot unions), D (constants) |
| Python tests, GUIDE.md and `docs/guide/*.md` | ~1,040, plus ~170 | B: sed `Expr.` → `Formula.`. C: `Formula.written_length(w)` → `w` where cosmetic (optional). |
| `tools/tess-meter` | 2 / 1 | B |
| Goldens: `tests/golden/golden.cad`, 19 `bool13_goldens/*.cad`, 4 `.pncad`, `before_extrude_side/plate_param.cad`, `wire_rv_bytes`, `perf12_census_goldens` | 25 files, plus 2 rows | B: untouched. C and D: re-bless (`M4_PR6_BLESS_GOLDEN=1`), saying in the PR what moved. |

Sweep pattern: `Expr::(literal|…|count)\(|(?<![.\w])(len2?|ang|scl|len_mm|count)\(|parse_expr\(`.

Blind spots:
- a literal built through a local helper of another name (the 60 `fn …() -> Expr` helpers);
- a literal reached through `Expr` deserialization.

The second pass greps `-> Expr\b` and `serde_json::from_str::<.*Expr` and takes each site. After D, `Expr` has no literal constructor, so the compiler closes both gaps.

**Equal literals that must coincide.** C will turn some passing builds into refusals. `topo::boolean::join`'s cylinder pair reads `field_source_evidence` and refuses `IntersectingCylinderAxes { evidence: None }` where two equal-radius cylinders were typed separately. These are found at runtime (CI), not by grep. The candidates are the `hollow_tube_*` corpus, `crosslap`, `tube` and `twopeg` in the tour. The fix is the D10 one: author one variable read by both slots. It is never a kernel change.

## 7. What moves

- **Node ids and content keys.** Every insert with a written quantity mints variables first, so the chain extends and every node id moves in C. Every pinned hex, memo fixture and golden is re-blessed. The id-free corpus digest moves only where the `vars` table enters it, and the PR states which.
- **Tokens.** Lone literals become `T_VAR` in C; constants become `T_RATIO`/`T_TURN` in D.
- **Analysis axes.** Every anonymous continuous free variable is an axis (VR8):
  - **ParamBox** gains degenerate (`FIXED`) axes only. `varying()` is unchanged, so the drive's splits are unchanged.
  - **MC** draws only `varying()`, in declaration order, so the sheets stay bit-equal (test 9).
  - **Stackup** gains one dual pass and one report row per anonymous continuous variable (Q4). Its cost grows with the number of written dimensions, outside profiles; profile arguments resolve at f64 under `ProfileLift::Pinned`.
  - **Sym lane.** Every former literal outside a profile binds as `nominal + param_over(id, [0,0])`, a symbol, where it was a `Lit` constant. A0's constant fold and the coefficient ring see more indeterminates. Pins on `sym_9_retry_interval`, the `m10_*_interval` probes (22 files hold `Sym<`) and `sym11_*` are expected to move (Q3).
- **Interval pins.** In D, non-dyadic constants enclose their true value instead of the nearest double: one-ulp widenings, and they are correct.
- **f64 geometry does not move** in C or D. Literal value bits become free-variable value bits, and `Ratio` evaluation bits equal the decimal parse's. A tour frame that changes is a bug, not a re-baseline.

## 8. Test plan (each assertion names the runtime value that breaks it)

1. **(A) Cycle.** `DefineVar w := h + 1 mm` where `h := 2w` refuses `DefinitionCycle` naming `[w, h]`, and the doc is unchanged. *Breaks if* the door misses an indirect read (the doc then holds `vars[w].def = Defined`). A hand-edited file holding the cycle refuses at load.
2. **(A) Pushforward.** With `h := 2w` and a slot reading `h`, the Dual seed on `w` gives ∂depth/∂w `== 2.0` exactly. A seed on `h` refuses `SeedOnDefinedVar`. *Breaks if* the defined variable is bound as free (derivative 0) or seeded.
3. **(A) Invalidation.** `SetVarValue w` re-runs the reader of `h`: recompute count 1 for that node and 0 elsewhere. *Breaks if* `DocDiff` does not close over definitions (count 0, a stale memo).
4. **(B) Pure refactor.** The bytes of `golden.cad`, every `bool13` golden and every corpus `.pncad` are equal before and after B. *Breaks if* any wire spelling or lowering moved.
5. **(C) Two typed values are two variables.** Two fillets each authored `Formula::length_in(5.0, Mm)`:
   - their slot ids differ;
   - their tokens differ in the `T_VAR` id bytes;
   - `field_source_evidence == None`.

   The same fillets authored with one `VarId` give equal tokens and `Declared`. *Breaks if* lowering dedups quantities by value.
6. **(C) Shape is structure.** Slots `w*2`, `w*2` and `h` (with `h := w*2`) lower to one token; `w*3` does not. *Breaks if* tokens encode the anonymous defined variable's id instead of its expansion.
7. **(C) Lifecycle.**
   - `SetParam` replacing a slot `w + 5 mm` with `w` reports two `AnonymousVarRemoved` (the defined variable, then its free variable), and the mint log still holds both ids.
   - Loading a file where an anonymous variable is read only by an unread anonymous definition refuses `AnonymousVarUnread`.
8. **(C) Identity survives a gesture.** A distribution set on a sketch argument's anonymous variable survives a drag (`SetVarValue`) and a `SetProgram` that re-authors via `Node::authored()`: `doc.free(v).distribution` stays `Some`, bit-equal. *Breaks if* either path re-lowers a quantity.
9. **(C) MC is unmoved.** On `mcplate`, the MC draw matrix is bit-equal before and after C. *Breaks if* anonymous fixed variables enter the draw order.
10. **(C) Split carries anonymity.**
    - A cut node whose two slots share one anonymous variable with a distribution lands in the part with one anonymous variable, read twice, with its distribution bit-equal.
    - The part loads.
    - A variable read on both sides of the cut refuses `UncutVarReference`.
11. **(C) Range names nothing.** A certified range on a literal slot leaves `doc.var_names()` unchanged and widens the slot's own id. A defined slot refuses `SlotIsDefined`.
12. **(C, D) f64 geometry.** The f64 body digest of every corpus document (`tests/corpus/*`) is bit-equal across C and across D. *Breaks if* a value's bits move: a unit conversion re-done, or `Ratio` evaluated as `from_f64(p as f64 / q as f64)` from a non-exact path.
13. **(D) Constants.**
    - `Ratio::new(2,4)` is `bit_eq` to `Ratio::new(1,2)` and lowers to one token.
    - `parse_formula("0.1")` is `Ratio(1/10)`.
    - The f64 eval bits equal `0.1f64.to_bits()`.
    - The Interval eval has `lo < hi` and contains 1/10. *Breaks if* evaluated as a point.
    - `Ratio::new(1 << 54, 1)` refuses `ConstantOutOfRange`.
    - `turn/4` evaluates to the bits of `FRAC_PI_2`.
    - In the Sym tier, `turn/4 − turn/4` from two slots decides Zero by theorem, and `90 deg − 90 deg` does not.
14. **(D) Load.**
    - A snapshot with `{"Literal":…}` refuses `Unreadable`, naming `Literal`.
    - A ratio `{num:2,den:4}` refuses `PersistError::Dimension`.
    - A slot of an unminted id refuses `SlotReadsUnmintedVar`.
    - A Length slot holding a Count variable refuses `SlotVarKind`.
15. **(B–D) Python.**
    - `Node.extrude(p, w)` with `w` a `WrittenLength` mints one anonymous variable (`len(doc.vars())` grows by 1).
    - Passing one `Var` to two slots gives equal `Doc.slot` handles.
    - The census and the `.pyi` agree.

Loud census rows: `ParamSource` `ALPHABET`, `pncad-py` `tags.rs`/`surface_census`/`prose_census`, `display_contract`, `f6_variants!`, the persist `Walk` roster.

## 9. Risks

- **Size.** C is the largest unit this program has run. The node genericity is a precondition, which is why it lands in B under a byte-identical check.
- **Sym reach** (Q3). The tier may prove less on every document with literal dimensions. This is measured in C, re-blessed, and the residue is filed on `work/sym/` beside `sym-tier-reach-depends-on-symbol-order`.
- **Stackup cost** (Q4). It is linear in the number of written dimensions. Measure on `tolerance`, `mcplate` and `chaintol` in C.
- **Runtime-only refusals** (§6, equal literals that must coincide) are not found by the compiler. Run the full editor-core and tour suites before marking C ready.
- **Re-minting.** Any caller that re-authors from a stored node by value, not through `Node::authored()`, silently drops a variable's identity and distribution. Sweep for `.clone()` of a stored node into an authored door. The type split makes most such sites fail to compile; the residue is `serde` round trips.
- **Token expansion.** It is bounded by `DefinitionTooLarge`. A real document near 4096 nodes would refuse. Pick the bound from the corpus maximum ×16, stated in the PR.

## 10. Open questions (each with a recommendation)

1. **Is a lone bare number in a slot a value or a constant?** VR5/VR6 class bare numbers as constants and written quantities as values, but they never say what a slot holding only `3` or `0.5` is. D10 says "typing a value mints a free variable", and calls constants "the shape of a formula".
   - **Recommendation:** a lone number at a slot root is a typed value. It mints a free `Count` or `Scalar` (unit `ONE`) variable.
   - A number inside an operator tree is a constant. `turn/4` alone is a formula, so it is a constant.
   - Datum direction components and bulges therefore become fixed axes until stage 3's `Direction`.
2. **How one edit mints a variable it reads twice.** This is needed by split/inline carry and by Python sharing in one call.
   - **Recommendation:** a `fresh: Vec<VarDecl>` table on every slot-writing edit, read by `Formula::Fresh(i)`. A written quantity is sugar for a fresh entry read once. An entry the edit does not read refuses `FreshUnread`.
   - The alternative is anonymous `DeclareVar` with the GC deferred to a `Recording` transaction. It splits the GC rule into two, and I recommend against it.
3. **Sym lane: does a fixed (zero-width) axis bind as a symbol?** VR8 and the merged code say yes, for every free continuous variable. Pinned by INTENT-VARS-1 test 6: `w − v` at equal values is not Zero. Binding fixed axes as constants would be sound for the box question and would keep today's reach. It would also make the tier unusable as the stage-4 structural rung, which D10 names.
   - **Recommendation:** keep symbols, measure, re-bless, file the reach loss.
4. **Stackup's entry set** (today every continuous free variable, fixed ones included). **Recommendation:** keep it (VR8): a toleranced anonymous dimension is what a stackup is for. Add a caller-chosen subset only if C's measurement demands it.
5. **`MeasureExpr` and the assertion bound.** VR4's "formulas have one home: definitions" is unattainable for the measure arithmetic until stage 2 makes `Measure` an operation. VARIABLES-DESIGN is inconsistent with the code here.
   - **Recommendation:** in C, the `Value` leaves and `bound` become slots (`S`). The operator skeleton over measure primitives stays until stage 2.
6. **What does a viewer value gesture write?**
   - **Recommendation:** `SetVarValue` on the slot's variable. Dragging a slot that reads a named or shared variable moves every reader, which is what sharing means.
   - Typed text re-lowers.
   - Detaching and the offer are `typing-a-value-mints-or-offers-a-variable`'s.
7. **The rational bound.** **Recommendation:** `|num|, den ≤ 2^53`, so both operands embed exactly in f64 and interval evaluation is one rounded division. `Integer` keeps i64.
8. **`SetExpression`.** **Recommendation:** re-address it to a defined variable (`{ var, path, formula }`); "a sub-expression of a slot" means nothing once the slot holds an id.
9. **Python bare `Length`/`float`/`int` in a slot.** **Recommendation:** accept them as written quantities in the canonical row, `ONE`, or a count. This is today's `Expr.literal` behaviour, so it adds no new D6 hole.

**Inconsistencies found.**
- VR4 vs `MeasureExpr` (Q5).
- VR5/VR6 are silent on a lone number (Q1).
- INTENT-VARS-1 §8 Q2's refusal of an anonymous variable crossing a cut becomes a refusal of every split after C. It was an interim ruling and C supersedes it.
- `range.rs` mints a name, against VR2. Closed in C.
- `crates/verbs/README.md` VS-Q4 states the literal-bits encoding. It is reworded in D.

## 11. Orchestrator rulings (binding; they override the sections above)

- **The merge and the slicing are accepted.** A closes `parameters-defined-by-formulas`, D closes `no-dimensioned-literal-in-a-slot` and `equal-literals-lower-to-one-identity-token`, and each PR is green on its own. B's byte-identical goldens are its gate.
- **Q1–Q9:** each recommendation is accepted.
  - Q1 and Q5 are written into VARIABLES-DESIGN, at VR6 and VR4 respectively.
  - Q2's fresh table replaces INTENT-VARS-1 §8 Q2's cut refusal at C.
  - Q3's reach loss is measured in C and filed on `work/sym/`.
  - Q4's stackup cost is measured in C on `tolerance`, `mcplate` and `chaintol`, and stated in the PR body.
- **Equal values typed apart stop counting as shared** (§9). When C makes two separately typed equal radii lose `Declared` evidence, a fixture or demo that relied on that evidence shares one variable. That is D10's way to say it, and it is never a kernel change. Each such site is listed in the PR body. A tour demo that now has to declare what it once got by coincidence is a usage finding (`memories/demo-purpose.md`). It is filed on `work/intent/` for stage 4's coincidence door, which glues on Zero.
- **Review posture:**
  - A, C and D draw a concurrent dual (H, STRUCTURAL).
  - B is mechanical and gated by byte-identical goldens, so it gets the orchestrator's read plus one single review.
- **Disk:** each implementer and reviewer runs in its own cloud session.
