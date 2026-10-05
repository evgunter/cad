# Review: INTENT-LITERALS PR B (#4072, "authored Formula, stored Expr")

Single reviewer. Frozen head `75a2594dcb`, base `fcfb3e0204` (`git merge-base 75a2594dcb origin/main`).
Read: `docs/prompts/reviewer-style-lane.md`, `CLAUDE.md`, `docs/INTENT-LITERALS-SPEC.md` (§1, §3, §11),
`docs/VARIABLES-DESIGN.md` VR6, and the PR description (not its comments).
The probes are in `review/intent-literals-b-probes.rs`. Drop the file into `crates/editor-core/tests/` and add `mod` to `all.rs`.
At the base, use the `s/Formula/Expr/` spelling.

## Verdict: NOT-MERGEABLE-AS-IS

The refactor is careful and the bulk claims hold: every suite is green, no golden moved, and a stored `Expr` cannot hold
a name. One MAJOR blocks it. An authored node the door could always refuse now **panics**. It reaches
`unreachable!` in `lower_node`, from Rust and from Python (`PanicException`). The base refused it typed.
The fix is small: pre-check what `try_map_slots` maps, or drop the pre-check and map the fault to an address.
Two refusal-text changes contradict C1 and are not covered by the PR's rulings (MINOR).

## Findings

### MAJOR-1 — A refusable insert panics: count-spelling nodes holding an unheld name (C3, C1, C5)
`crates/editor-core/src/edit.rs:716-733` (`lower_node`), `crates/editor-core/src/node.rs:2863-2870` (`rule_rows!`),
`node.rs:4283-4294` (`try_map_slots` maps `count` unconditionally), `node.rs:3298-3303` (the documented edge).

`lower_node` pre-checks each formula in `node.rows()` plus the payload carriers. It then calls `try_map_slots`, which
maps every `S` field, and treats any fault there as unreachable. But `rows()` deliberately omits the count of a
placement-rule node whose rule takes none: `Pattern` with `PatternKind::Explicit`, or `PlacedUnion` with a list and
`count: Some(..)`. `Node::exprs`' own docs call this "its edge". So a count that is an unheld name passes the
pre-check and then fails inside `try_map_slots`.

**Execution.** Probes `p1`/`p2`: `InsertNode(Pattern { count: Formula::named("nope", Count), kind: Explicit([IDENTITY]) })`
and the `PlacedUnion` twin.
- Head: both panic at `edit.rs:731`, "entered unreachable code: every slot of the node lowered one by one above, yet
  no variable is named nope did not".
- Base `fcfb3e0204`, same probes: both refuse typed. P1 gives `PlacementRuleMismatch { shape: ListedOnPattern }`.
  P2 gives `ListedWithCount`.
- From Python, at a staged cdylib built from the head: `a.parse_formula("n")` on a doc declaring `n`, then
  `b.insert(Node.pattern(cube, f, PatternKind.explicit([...])))` on a doc without `n`. It raises
  `pyo3_runtime.PanicException`, a `BaseException`, so `except PncadError` does not catch it.
- By inspection (not executed): the same panic is reachable by replaying an authored edit log holding such an insert.
  The log carries `WireFormula` and replays through the same door.

Under the base, the name stayed and the door refused `PlacementRuleMismatch`, in fail-loud style. This PR turns a typed
refusal into a crash. The removed `NameLeafWritten` post-condition was the "refuse rather than store" backstop.
Its replacement is an `unreachable!` with a premise the node's own docs contradict.

Probe `p5` mapped each of 328 corpus nodes with `try_map_slots` and compared against `slots()`. It found no other
mismatch, so this edge is the only instance on well-formed nodes. The same pre-check-then-`unreachable!` shape is also
at `SetProgram` (`loops_refusal`, `edit.rs:771`) and `SetOffset` (`edit.rs:~5884`). Their rows do cover their maps
today, but nothing ties the two lists together. Confidence: **sure** (executed at both commits).

### MINOR-1 — `DefineVar` changed its refusal order; ruling 1 does not list it (C1, C4)
`edit.rs:5454-5461`. `lower_decl` now runs before the `VarKindFixed` check. The base ran `def.stored()` and then
`VarKindFixed`, and refused the name later in `check_definition`.

Probe `p3`: declare `n: Count`, then `DefineVar n := named("nope", Length)`.
- Base: `VarKindFixed { kind: Count, offered: Length }`.
- Head: `DefinitionUnknownVarName { name: "nope" }`.

Ruling 1 names only `InsertNode`/`SetProgram`/`SetOffset`, and the PR says the other arms' order is unchanged.
`DeclareVar` similarly moves the name refusal ahead of `VarIdCollides`; that collision is unreachable in practice.
Either list the arms in ruling 1 or restore the order. Confidence: **sure** (executed at both commits).

### MINOR-2 — Ruling 2 is unsound: a refused insert's spoken id moves (C1, C3, C4)
`edit.rs:4997-5005`. The refusal speaks `Mint::insert(authored)`. Under the base, held names were lowered first, and
the id was drawn from that partly-lowered node. "Lowering preserves the mint, since a Var leaf maps to a Var leaf" is
true only when *every* name lowers. That is never the case on this path, which exists only for a refusal.

Probe `p4`: a pattern whose spacing is `w + nope`, with `w` held.
- Head: authored by name it speaks `Pattern 10735d413b9c`; authored by id it speaks `Pattern 15bf7f84613b`.
- Base: both speak `15bf7f84613b`.

So refusal text moved for any node that holds a held name and an unheld one. No golden pins it. Accepted inserts are
unaffected, and the corpus fence and golden hashes agree. Confidence: **sure**.

### MINOR-3 — Python-visible docstrings still name the retired `Expr` class (C6)
These `///` comments are the runtime `__doc__`, and the perl rename never ran over `crates/pncad-py/src/**`.
- `crates/pncad-py/src/py/expr.rs:166-253`: `Formula.literal.__doc__` says "`Expr::literal`" and
  "`Expr.literal(25 * mm)` reads back". Checked at runtime.
- `py/doc.rs:4051` (`declare_var`) and `:4077` (`define_var`): "An `Expr` (or `VarDecl.defined`) declares a DEFINED
  variable". The binding rejects a Python `Expr` (`DeclArg::Defined(Formula)`, `doc.rs:3471-3477`), and the `.pyi`
  says `Formula`. So stub and `__doc__` disagree. Checked at runtime: `"An `Expr`" in DocEdit.declare_var.__doc__` is true.
- `py/doc.rs:1832, 2361, 2518, 2968, 3042, 3282`: `Expr.count(3)`, `Expr.literal(0 * rad)`, and "`Expr` from
  `Doc.parse_formula`".
- `docs/guide/north-star-audit.md:232,236,362,556,563`: present-tense `DocParam`, `ParamName` and `Doc.parse_expr`.
  The PR touched this file.

Nothing in the census reads docstrings, so no row went red. Confidence: **sure**.

### NOTE-1 — Ruling 5 (`DatumDistance` keeps a stored `Expr`) is sound, and closes a door it never opened (C4)
`crates/pncad-py/src/py/select.rs:707-716`. At the base, a name in the comparand was never lowered: Python's
`select_where` passes the predicate straight through, and evaluation refused `UnloweredName`. Refusing at construction
is strictly earlier. The comparand is a query value, not a slot, so keeping it stored is right.

But Python still has no way to state a variable comparand at all. `Formula` has no by-id constructor, and
`datum_distance` lowers in an empty scope. Yet SELECT-DESIGN §5 calls a named-parameter rule "the whole point".
Not new; worth filing beside G1. Confidence: **likely**.

### NOTE-2 — Ruling 1's order change on `InsertNode`/`SetOffset` is defensible (C4)
Lowering is a precondition of every later check. The two-simultaneous-fault order is not pinned anywhere, so moving it
is an improvement in clarity. No objection beyond MINOR-1's scope gap. Confidence: **likely**.

### NOTE-3 — A snapshot carrying a name loses its address (C2, C5)
The rows now accept `Unreadable` with "unknown variant `Name`" (`intent_vars_3_readers.rs:630`,
`intent_literals_a_definitions.rs:815`). The old `NamedReaderInSnapshot` named the node; the serde message names only
a JSON position. The rule is sound and the type carries it, but the diagnostic got poorer. Acceptable; noted.
Confidence: **sure**.

## Style

- **S1 (Q1) Five hand-written re-authoring wrappers and five lowering helpers.**
  - `Node::authored` (`node.rs:4352`), `Placement::authored` (`placement.rs:688`), `Alignment::authored`
    (`mate.rs:487`), `LoopProgram::authored` (`program.rs:1341`) and `MeasureExpr::authored` (`measure.rs:577`) are
    each `let Ok(x) = self.try_map_slots(Formula::from)`.
  - `test_support::{stored, stored_expr, stored_program, stored_loop, stored_placement}` (`test_support.rs:81-130`)
    are each `try_map_slots(Expr::try_from).expect`.
  - Every slot-bearing type also hand-writes its own `try_map_slots`: Datum, TubeWindow, PatternKind, PartSelect,
    Placement, Alignment, ProgramTarget, ArcData, Step, LoopProgram and ProfileProgram.

  A trait over "a value generic in its slot form" would give each one home. As written, the next slot-bearing type
  adds three more copies. **likely**
- **S2 (Q1/Q2) Two lists of "the slots a node holds" that must agree, held by an `unreachable!` message.**
  The lists are `rows()` and `try_map_slots`. MAJOR-1 is the instance, and `SetProgram`/`SetOffset` repeat the shape.
  The pre-check exists only to name the address. Mapping the fault together with its address in one walk would remove
  the second list. This is the PR replacing a checked post-condition with an asserted one, exactly the
  "fix mints a fresh instance" trap. **sure**
- **S3 (Q1) The name fault is mapped to Python's vocabulary twice.** `tags::name_fault_tag` (`tags.rs:2323`) maps
  `Kind` to `"var_kind_mismatch"`. `name_fault_err` (`py/expr.rs:399-408`) maps `Kind` through
  `eval_err(EvalError::VarKindMismatch)` instead, and never asks `name_fault_tag` for that arm. Both say the same word
  today. Also, `Unheld` is raised as Python `EvalError` variant `unlowered_name`, though the kernel's
  `EvalError::UnloweredName` is deleted. The Python class now has an arm with no kernel twin. **likely**
- **S4 (Q4) `VarReadFault::Name` survives with one producer.** It is built only by `edit.rs:684` (`name_fault`), as a
  funnel into `read_refusal`. `doc.rs:var_read_faults` can no longer emit it, and `persist/check.rs:455` keeps it as an
  `unreachable!` arm. A lowering fault dressed as a read fault keeps the old conflation the PR set out to retire.
  **likely**
- **S5 (Q7) `Formula::named_reads` fakes its walk** (`formula.rs:166-175`). It reuses `try_map_leaves` and builds a
  throwaway `Expr::var(VarId(0), dim)` per leaf to satisfy the signature. A plain leaf visitor would say what it means.
  **likely**
- **S6 (Q2/Q5) A sed rewrote history into anachronism.**
  - `crates/pncad-py/tests/test_binding_census.py:2083-2113, 3296-3300` narrate LIB-B-EXPR-READ: "closing it moved
    `Formula`, `parse_formula`". It now says "no door takes an `Formula` INTO a document", which is false today.
  - "an `Formula`" grammar appears 21 times: `pncad.pyi:2740,3228`, `docs/GUIDE.md:143`, `ty_fixtures/illegal.py` ×6,
    `test_north_star.py` ×4, and others.

  **sure**
- **S7 (Q5) Rust docs cite a constructor that is no longer public.** `pncad-py/src/tests.rs:1681`,
  `tags.rs:2253`, `errors.rs:267` and `py/expr.rs:166,182` say "`Expr::literal`'s OWN error type" or "`Expr::count`".
  Those are `pub(crate)` now; the door is `Formula::literal`. Same class as MINOR-3, Rust-side. **sure**
- **S8 (Q7) `ExprTree<L>` exposes `display_unit`/`literal_value` on both forms, and the operator constructors are
  `pub` on `Expr`.** External code can still build a stored `Expr` by `Expr::add(Expr::var(..), ..)` with any
  `VarId`. That is unchanged from the base, but the PR's thesis is "the stored form is what the door writes", and the
  public `Expr` constructors outlive it. **unsure**
- **S9 (Q3) The new rows can go red, but none pins MAJOR-1's class (C7).**
  - `test_a_named_comparand_refuses_at_the_predicate` goes red if the lowering is skipped or the tag moves.
  - `expression_evaluation_tags_are_stable` goes red on a tag rename.
  - The deleted `NameLeafWritten` row had no replacement that exercises the door's own lowering on every slot-bearing
    field. A row enumerating every `S` field reachable by `try_map_slots`, each authored as an unheld name, would have
    caught MAJOR-1.

  **sure**
- **S10 (Q8) Whole-file read, partial.** I read `formula.rs` whole. Of `node.rs` (4934 lines), I read the header, the
  slot-table macros, all of `try_map_slots`, and `authored`, not end to end. `node.rs:3298`'s "Its edge" paragraph is
  the only text in the tree that predicted MAJOR-1. It was written for `exprs()`, and no lowering code reads it.
  **sure**

## Claims exercised

| Claim | How | Result |
|---|---|---|
| C1 behaviour/bytes | `git diff base..head` touches no `.cad`/`.pncad` file and removes no ≥16-hex constant; full workspace suite incl. goldens and the corpus fence (tour frames not run: `demos/tour` is its own root) | **holds for accepted edits**; refusal text moved in MINOR-1/-2, crash in MAJOR-1 |
| C2 no stored `Name` | `StoredLeaf` is uninhabited (`expr.rs`), `WireExpr` has no `Name` (`wire.rs`, `deny_unknown_fields`); snapshot rows refuse `Unreadable`; split/inline carry stored nodes and re-author by id (`refactor.rs:337,378`); viewer/Python `Expr::try_from` sites read | **holds** (type fact) |
| C3 lowering complete, mint, replay | Doors read: insert, SetParam/Structural/Expression, SetProgram, SetOffset, Declare/DefineVar; MintingEdit hashes JSON whose `WireFormula`/`WireExpr` spellings agree; probe p5 over 328 corpus nodes | **holds except MAJOR-1** (and MINOR-2's spoken id) |
| C4 rulings | 1: MINOR-1 gap; 2: unsound (MINOR-2); 3: sound; 4: tag move consistent with base `Doc::authored`'s kind arm; 5: sound (NOTE-1); 6: fine | partial |
| C5 deleted guards | Name in snapshot slot/definition → wire refusal (rows present); `NameLeafWritten`'s role → `unreachable!` that fires (MAJOR-1) | **hole at MAJOR-1** |
| C6 Python renames | grep of stub, tests, src, docs; runtime `__doc__` | stale docstrings (MINOR-3, S6, S7); stub and census agree |
| C7 rows can go red | read the three new rows | yes for their own faults; none covers MAJOR-1 (S9) |

## Runs (head `75a2594dcb`, private `CARGO_TARGET_DIR`)

| Run | Result |
|---|---|
| `cargo nextest run --workspace --profile default --no-fail-fast` (slow set included) | 12006 run, 12006 passed, 345 skipped |
| `crates/pncad-py/run-python-tests.sh` (`ty` stub check included) | 932 tests, OK |
| `cargo test --workspace --doc` | 172 passed, 0 failed |
| probes p1–p4 at head / at base `fcfb3e0204` | head: 4 failures (2 panics, 2 moved refusals); base: 4/4 pass |
| probe p5 (`try_map_slots` vs `slots()`, 328 corpus nodes) | 0 mismatches |
| Python repro of p1 (staged head cdylib) | `pyo3_runtime.PanicException` |

Not run: the ε 1e-6 / 1e-12 rows and `demos/tour`'s own root. The default-profile workspace run covers the editor-core
and viewer sets at the default ε. The full suite ran via the harness's background runner, because it takes ~31 min,
which is longer than a foreground call allows. Each probe and Python run was in the foreground.
