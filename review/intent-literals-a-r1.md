# Review r1 — PR #4065 INTENT-LITERALS PR A: definitions

Frozen head `e8336050a2`, base `b553731da5`. Reviewer lane r1 (correctness + style).
I read: `docs/prompts/reviewer-style-lane.md`, CLAUDE.md, `docs/INTENT-LITERALS-SPEC.md` (§2, §8 A-rows, §11),
VARIABLES-DESIGN VR3/VR7/VR8, the PR body (not its comments). No other `review/*` branch or reviewer output was
read or glimpsed.

## Verdict: APPROVE-WITH-FIXES

The kernel half holds against every claim I could exercise. Seven probes I wrote and the PR's 2732 rows all
pass. One MINOR is in the viewer's text door: typed constant arithmetic silently turns a toleranced free
parameter into a defined constant, which goes against C8. The rest are NOTEs and style.

## Execution record

- `cargo nextest run -p editor-core --profile default --no-fail-fast` (slow set) at the frozen head, with my
  probe module temporarily added: **2738 run, 2737 passed (4 slow), 108 skipped, 1 failed**. The one failure
  was my own over-strict probe assertion (see C2), fixed and re-run green. That is the PR's 2732 plus my 6, and
  it includes `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked` and its siblings.
- Python: built `pncad-py` (debug, `extension-module`), staged it, and ran `python3 -m unittest test_variables`:
  **7/7 OK**, including the 3 `TestADefinedVariable` rows.
- Viewer: a probe row (below) under `cargo nextest -p viewer --profile default`.
- I did not run the workspace, the tour or clippy. I relied on the PR's stated runs for those.
- Probe files were removed afterwards. The tree is clean at the head.

## Findings

### MINOR-1 — the viewer's text door turns a toleranced parameter into a defined constant and drops its distribution, silently (C8)
`crates/viewer/src/session.rs` `set_param_text` (the `constant_count` / `free` match → `DocEdit::DefineVar`).

- **Demonstrated by execution.** Probe `r1_constant_expression_typed_over_a_toleranced_param`: a free
  `base_r = 50 mm` with `Normal{σ=1e-4}`, given the text `50 mm + 1 mm`, `-(51 mm)` or `2.0 * 25.5 mm`:
  - each commits `DefineVar { Defined(<constant expr>) }` with `refusal=None`;
  - afterwards `doc.free(base_r)` is `None`, so the distribution is gone;
  - the parameter has left the analysis axes, MC laws and stackup entries (they read `free_vars`);
  - nothing tells the person.
- **Before this PR** all three texts refused `ParamNotANumber`.
- **The door is inconsistent with itself.** A constant **count** expression is treated as a value (`constant_count`).
  A constant **continuous** expression becomes a definition that reads no variable.
- A definition that reads nothing gives a variable that can never move again from analysis. That is not what
  a person typing `50 mm + 1 mm` into a value field meant.
- It is one undo step, so nothing is lost for good. I file it MINOR, not MAJOR, for that reason.
- Confidence: **sure** on the behaviour, **likely** on the severity.

### NOTE-1 — no way back from a definition in the GUI (C8)
`crates/viewer/src/pane/properties.rs` (the `defined_rows` arm draws `widgets::message`, read-only).

- The PR says "a number typed over a defined one makes it free again" (`set_param_text`'s `if defined` branch).
- But the panel draws no field for a defined variable, so that branch is reachable only from a hand-built
  `SessionOp`, as the filed issue says of its siblings.
- A person who types `rim * 2.0` can only undo. They cannot edit the formula or free the variable from the panel.
- The filed `viewer-value-doors-read-a-defined-variable-as-absent` covers the absent-reads. It does not name
  this missing affordance.
- Confidence: **sure** (inspection).

### NOTE-2 — the definition-order memo's five hand clears are unobservable (C3, C9)
`edit.rs` (5 × `new.definition_order = DefinitionOrderMemo::default()`) and `doc.rs` `DefinitionOrderMemo`.

- **Mutant (all five clears deleted):** survived `intent_*`, `m10_4*`, refactor/split rows and my probes (245/246
  passed; the one red was my own probe, unrelated).
- **By inspection it is equivalent today.**
  - `new` is a clone, and `Clone` empties the memo.
  - No arm computes `definition_order` on `new` before its own var-table write.
  - A memo left stale by a removal only carries extra ids, which every reader skips via `vars.get(..)?`.
- So C3 holds, but by the clone rule and not by the clears. Nothing pins the clears.
- Confidence: **likely** that it is equivalent; **sure** that the mutant survives.

### NOTE-3 — the expansion bound covers definitions, not slots (C4)
`edit.rs` `check_definition`; `param_source::encode`.

- A slot's token is its own node count times up to 4096 per defined read.
- Spec §1 has lowering refuse past 4096 too. In PR A no door checks a slot's expansion.
- It is linear in authored size, so this is not an exponential hole. It is still a bound that is weaker than
  the spec states, and it is not disclosed.
- Confidence: **likely**.

### NOTE-4 — the bound is exact at both doors (C4, positive)
- **Demonstrated by execution.** Probe `r1_bound_edge`: `Neg(v11)`, 4096 nodes:
  - declares and round-trips `save` → `load`;
  - one more `Neg` refuses `DefinitionTooLarge { nodes: 4097 }`.
- The door and the load walk agree, and `expanded_size` counts what `encode` writes.
- Confidence: **sure**.

### NOTE-5 — one extra K-stats check per definition per environment build (C2)
`doc.rs` `bind_definitions` → `expr::eval` → `refuse_non_finite`.

- Every `var_env*` / `seed_env` now runs one `k_stats::check_unlogged("expr_non_finite")` per definition.
- At Sym my probe saw `symbolic_zero` go from 1 to 2. That is not a defect: it stays out of the verdict log.
- But K-stats sample counts now grow with the number of definitions times the number of environment builds,
  and edit doors build `var_env::<f64>` several times per edit.
- Nothing moves for a document with no definition.
- Confidence: **unsure** whether any register pins these counts.

## Claims

| Claim | Exercised | Result |
|---|---|---|
| C1 nothing moved without definitions | full editor-core slow set incl. the corpus fence | **holds** (sure). Tour, MC and stackup goldens not re-run by me beyond editor-core. |
| C2 every lane scalar | f64 (`r1_count_definition`, PR rows); Interval (PR row); Dual: `r1_pushforward_matches_finite_difference`, `h := w·s`, `k := h + w`, stackup gives ∂k/∂w = 2.7 and ∂k/∂s = 0.3 exactly, finite difference agrees to 1e-9, two entries (free only); Sym: `r1_sym_tier_binds_definition_over_symbols`, `h := w + w` minus `2·w` decides **Zero with `numeric: 0`** (a theorem); a refused definition: `w/z` at `z = 0` refuses `PayloadExpr{…, DefinitionRefused{h, NonFiniteResult}}` with the slot address | **holds** (sure) |
| C3 no stale result | `r1_chain_invalidation`: `w→h→g`, a node reads `g`. `SetVarValue w` gives 1 recompute (the unrelated reader reused) and `diff.vars == [w,h,g]`; redefining `h` gives 1 recompute and `diff.vars == [h,g]`. Memo: inspection + mutant (NOTE-2) | **holds** (sure; memo: likely) |
| C4 cycles and bound | PR rows (door incl. the redefinition that closes a cycle, load); NOTE-4 edge | **holds**, with NOTE-3 |
| C5 tokens and keys | inspection of `encode`: ids only, no name read; PR token and key rows | **holds** (likely; I wrote no new token probe) |
| C6 analysis | pushforward vs finite difference (C2 row); seed refusal (PR row); MC by inspection (`mc.rs` degenerate `ParamBox` → `var_env_over` → `bind_definitions`) | **holds**; MC with a definition **not executed** |
| C7 lifecycle | PR rows (cascade order `[h, w]`; delete leaves a typed refusal); inspection of `unread_anonymous_vars` and `RenameVar` | **holds** (likely) |
| C8 surfaces | Python 7/7; viewer probe | Python **holds**; viewer: **MINOR-1**, NOTE-1 |
| C9 rows can go red | one unlisted mutant (NOTE-2) survived as equivalent; the PR's listed mutants not re-run | **partly** |

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q7. **Q8 only partly**: I read the changed regions of `edit.rs` and
`doc.rs` in full, not either file end to end. **Q6**: the one deviation (Python `DocEdit.define_var` instead of
`Doc.define_var`) is an improvement, and the sweep's residue is filed. Nothing unscheduled found.

### Duplicates and parallel roles (Q1)

1. **`edit.rs` and `persist/check.rs` (`DefinitionCycle`, `DefinitionTooLarge`, `DefinitionVarKind`):**
   - The `Display` sentences are written twice, verbatim: `EditError` and `SnapshotError` each hand-render the
     ` → ` cycle loop and the "past the bound of" sentence.
   - The two arms are one fact at two doors, rendered twice. A wording change has to be made twice.
   - **likely**
2. **`edit.rs` `check_definition` vs `check.rs` `first_definition_cycle`:**
   - The bound scan is written twice, in two orders. The door reports the first offender in `definition_order`,
     load in `var_order`.
   - For a redefinition-from-below the two doors can name different variables for one document.
   - **likely**
3. **Graph walks over definitions, each its own fixpoint loop:**
   - `doc.rs`: `reached_through_definitions` (definers, upward), `definition_cycle` (DFS), `definition_order`
     (Kahn by repeated linear scans), `unread_anonymous_vars` (rounds);
   - `refactor.rs` `node_var_reads` (reads, downward);
   - `diff.rs` (closure pass).
   - Six hand-rolled traversals of one graph, and no shared adjacency. A class, not an instance.
   - **likely**
4. **`var.rs` `VarDecl` and `VarDef`:**
   - Two enums with identical arms and payloads, and `kind()` written twice.
   - Spec-sanctioned until PR B gives `VarDecl` a `Formula`, so it is temporary — but until then it is a
     parallel pair. `VarDecl::stored()` clones a full `Expr` on every door call.
   - **sure** (that it is a twin), **unsure** (that it matters)
5. **`pncad-py/src/edit_payload.rs`, the `DefinitionUnknownVarName` arm:**
   - Here the payload's `param` is the name **read**. In every sibling definition arm it is the variable
     **defined**, so one field has two meanings across one family.
   - **likely**

### Invariants held by hand and stale prose (Q2, Q4, Q7)

6. **`doc.rs` `DefinitionOrderMemo`:**
   - A `PartialEq` that is always true and a `Clone` that empties it hold the memo's validity by convention.
   - Five hand clears in `edit.rs` do too, and NOTE-2 shows nothing tests them.
   - A memo whose invalidation rests on comments at five write sites is the shape Q2 names. I would have
     computed the order where it is used, or keyed it on the table.
   - **likely**
7. **`doc.rs` `free_vars` doc ("the ONE iteration base every lane reads (the evaluation environment, …)"):**
   - The evaluation environment now also iterates `definition_order()`, so the sentence no longer covers every
     lane's base.
   - The doc rotted while the code stayed right.
   - **likely**
8. **`viewer/src/pane/properties.rs` `param_showing` ("A parameter is never driven, so there is no text to show"):**
   - False since this PR. A parameter can now be defined.
   - It survives only because defined rows never reach `param_showing`.
   - **likely**
9. **`viewer/src/session.rs` `set_param_text`:**
   - `let Some(unit) = unit else { unreachable!(…) }` restates, as a panic, a fact the match three lines up
     already decided (`(Some(value), Some(unit), _)`).
   - The tuple loses it. A type would carry it.
   - **likely**
10. **`doc.rs` `expanded_size`:**
    - It finds a `Var` leaf with `matches!(kind, Var)` and then calls `var_reads` into a fresh `Vec` to recover
      the id it just matched on.
    - That is a roundabout read of one field, and it allocates per node.
    - **sure**
11. **`viewer/src/session.rs` `reads_a_variable`:**
    - A local helper that asks two collectors and keeps neither.
    - `Expr` already answers "reads" questions (`reads`, `var_reads`, `named_reads`), so this is a third spelling
      at a consumer.
    - **unsure**

### Can the rows fail (Q3)

12. **`intent_literals_a_definitions.rs` `an_input_edit_reruns_the_reader_of_its_definition`:**
    - The PR body says it plainly: the recompute count cannot see a missing diff closure, because the
      content-keyed memo already catches it.
    - The diff assertion is the real guard. Fine, but the row's title promises the weaker half.
    - **likely**
13. **No row runs MC or the Sym tier over a document that holds a definition.**
    - My probes covered Sym. MC is still only inspected.
    - A regression that skipped `bind_definitions` in `var_env_over` would be caught by the interval row, but not
      by anything at MC.
    - **likely**

### Dispatch premise (§1)

14. **"the symbolic tier" in C2:** the PR adds no Sym row (`intent_literals_a_definitions.rs` imports no `Sym`).
    The claim was unexercised in the PR, and my probe supplied the evidence. **sure**

## What I could not exercise

- MC draws and stackup sizes on a document with a definition: inspection only.
- Tour and workspace suites, clippy and fmt: the PR's own runs.
- The PR's listed mutants: not re-run.
- C5: no new token probe of mine. I relied on the PR's token and key rows plus inspection.
