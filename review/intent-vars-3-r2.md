# Review r2 — PR #4027 "INTENT-VARS-1 PR 3: readers read ids"

Frozen head `b07891634f`; base `5f768f6b8c` (merge-base with `origin/main`). Diff
reviewed: 8 commits, 233 files. I read the PR body, not its comments, and no
other `review/*` branch or reviewer output. Nothing was glimpsed by accident.

**Verdict: APPROVE-WITH-FIXES.** The core mechanism holds under execution:
- one lowering path, with the insert minted from the lowered node;
- id-keyed tokens, mint preimages and content keys;
- the delete and anonymous lifecycle;
- load walks 4–6;
- split and inline remap.

I found no MAJOR. The fixes are:
- two places where a name goes stale or a `#hex` leaks into user-visible text;
- a split refusal that calls a legal document a kernel defect;
- two surviving mutants on stated behaviour (the inline remap is not caught by any editor-core test).

## Runs (private `CARGO_TARGET_DIR`, foreground)

- **editor-core, `--profile default` (slow set included):** 2703 / 2703 pass. That is the PR's 2698 plus my 5 probes, at the head.
- **Mutant runs, editor-core `--profile ci`, 2647–2654 tests each:**
  - the inline remap mutant survives;
  - the split-order mutant survives;
  - the `Speaker::formula` mutant is killed, by `pattern_spacing_and_step::a_driven_step_past_a_turn_says_what_it_evaluated_to`.
- **pncad-py:** `run-python-tests.sh` ran 927 tests, OK. `test_variables.py` is included.
- **viewer, `--profile default`:** a subset (`panel_edits|gesture_table|story_parametric|refusal_concision_edits|props|valid_range`) ran 71 / 71.

The probes are committed beside this report: `review/intent-vars-3-r2/r2_vars3_probes.rs` (p1–p6) and `pattern_step_display_probe.diff`.

## Findings

### MINOR-1 — memoized refusal text reaches users through plain `Display` as `#<16 hex>` (C2)

`crates/editor-core/src/mc.rs:385` (`monte_carlo`, `NominalDoesNotBuild.cause`), `crates/editor-core/src/drive.rs:1358`, `crates/editor-core/src/stackup.rs:648` (`no_measure`) and `crates/editor-core/src/eval/mod.rs:3239` each bake `e.kind.to_string()` into a `String`, with no document in hand.

Since this PR, `FullRangeStep`, `NegativeSpacing` and the `PlacedTwice` index hold readers as `#<16 hex>`. Only `Speaker::formula` names them again. On base the same text said `th`.

- **Execution** (probe in `pattern_step_display_probe.diff`): a pattern stepping by `th` = 760°.
  - `e.kind` displays as "the pattern step #3da0c3a822ef260e, which evaluated to 13.26… rad … Recourse: make it evaluate within one turn; #3da0c3a822ef260e - 720 deg does".
  - `e.spoken(doc)` says `th`, and says `phi` after a rename. That path is correct.
- **Consequence:** the recourse offers a formula the parser refuses by ruling (§8 Q1).
- **Class:** every `NodeErrorKind` `Display` without a `Speaker` is this shape. The sweep should cover each `kind.to_string()` and `format!("{}", e.kind)` outside tests.
- Confidence: **sure**, for the text at those four sites.

### MINOR-2 — an `AnalyzedBox` taken before a rename speaks the old name after it (C2)

`crates/editor-core/src/analysis.rs:193-215` keeps `spoken` per axis and leaves it out of `PartialEq` on purpose ("a rename … must move no equality"). The lanes speak their refusals from the box rather than the document:
- `mc.rs:673` (`laws_of`);
- `stackup.rs:1882` (`Unavailable { param }`, which reaches `render` as `[{u}]`, while that same row's header uses `doc.spoken_var`);
- `report.rs:194`.

Probe `p6`, by execution:
- declare `w` with a band;
- take `analyzed_box(doc)`;
- `RenameVar w→v`;
- the box compares equal to `analyzed_box(renamed)`;
- `monte_carlo(&renamed, &box, …)` refuses with "parameter **w** carries a band".

Before PR 3 there was no rename, so this exposure is new. One stackup render can say both names. Confidence: **sure**.

### MINOR-3 — split of a document whose cut reads a deleted variable is called a kernel defect (C6/C7)

`crates/editor-core/src/refactor.rs:2740-2745`: "A reader of a variable the parent no longer holds stays unresolved, and the part's insert door refuses it." The `continue` at `doc.var(id)` leaves the parent's dead id in the carried node. The part's door then refuses it, wrapped as `SplitError::PartEdit`.

Probe `p5`, by execution: "split: a part-side edit refused: variable e2382e0f27e1 is not a variable of this document (read by Extrude …, slot distance). There is no way through: this is a kernel defect; report it".

The document is legal (VR7/D10, and it loads). The refusal points at the kernel, not at the reader, and is untyped at the split door.
- Whether to carry the unresolved reader or refuse is a design call.
- The kernel-defect recourse is wrong either way.
- Confidence: **sure**.

### MINOR-4 — inline's reader remap is untested: removing it survives the whole editor-core suite (C7, C9)

`crates/editor-core/src/refactor.rs:3435`: I replaced `remap_node_vars(node, &var_map);` with a no-op. Result: `--profile ci` 2647 / 2647 green, and the filtered `inline|split|asm4|intent_vars|refactor` set 179 / 179 green.

No test inlines a part whose nodes read a variable and succeeds:
- `intent_vars_3_readers::split_and_inline_carry_readers_by_id` exercises only the refusal half of inline;
- `asm4_split_inline` exercises only the conflict half.

My probe `p1` passes on the head and goes red on the mutant. It covers:
- a merge-by-name (`bit_eq`) case and a fresh declare;
- carried readers reading the host's ids;
- the host save/load round trip.

The PR body lists §4 row 12 as covered. Its "carried readers are remapped" clause for inline is not covered. Confidence: **sure**.

### MINOR-5 — "split declares in the parent's declaration order" is unguarded (C5, C9)

The mutant at `refactor.rs:2747-2752` iterates `cut_refs.keys()` (id order) instead of `doc.var_order()`. It survives `--profile ci`; the only red was my unrelated p6.

This order decides:
- the part's minted var ids;
- the part's `var_order`, which every lane iterates.

Inline's "in the PART's declaration order" (`refactor.rs:3384`) has the same shape. That mutant was not run (**likely** surviving). Confidence: **sure** for split.

### NOTE-1 — the GUI re-baseline moved row order as well as ids (C1)

The commit `b0789163` re-baseline moved 8 `demos/renders-gui` cells. I diffed them pixel-wise: every changed pixel is in the right-hand panel.

On `datum-extrude-profile-56d118a1`:
- the tree's node ids move, as expected;
- the params panel reorders `depth, height, width` → `width, depth, height`, from name order to declaration order.

The body discloses the declaration-order iteration. It does not list these cells under "Re-baselined pins (all ids-only)". No geometry moved: no tour frame is in the diff. Confidence: **sure**.

### NOTE-2 — C1 corroborated on the two re-blessed documents

`tests/golden/golden.cad` and `crates/pncad/tests/plate_param.pncad` were compared after normalisation:
- ids above 2³² and hex digests were masked;
- `{"Param":{name,dim}}` was mapped to `{"Var":{dim}}`.

The normalised node multisets, mint-log arm counts and every other snapshot field are identical. I did not re-take the id-free corpus digest. Confidence: **sure** (on structure).

### NOTE-3 — C4 and C5 corroborated by execution

| Probe | What it showed |
|---|---|
| `p2` | An authored log (declare `w`, insert reading the name `w`, `RenameVar w→v`, re-declare `w`, then `SetParam` with `w + v`) replays bit-equal. The slot reads `[new w, old w]`. A snapshot saved with the remaining authored log loads to the same final document. |
| `p3` | An anonymous variable read twice survives losing one reader. `DeleteNode` of the second reports `AnonymousVarRemoved`. `var_order` empties, and a re-declare mints a new id. |
| `p4` | An id-authored reader of a dead id, and one of a never-minted id, each refuse `SlotUnresolvedVar` at the door. |

Confidence: **sure**.

### NOTE-4 — C3 by inspection

- `mint.rs:120-146`: `MintingEdit` holds the node after lowering, and `DeclareVar` is kind only. `EDIT_TAG` extends the chain only for minting edits, so `RenameVar` reaches no chain.
- `stackup.rs:1380+`: `serialize`, the input to `content_key`, writes `.full()` ids only.
- My own sweep, shaped differently from the PR's: `write_str`/`feed`/`update` beside `name`/`label`/`format!`, `{:?}` beside key/hash/mint, and `serde_json::to_vec` in mint. It found no name reaching a key.

Confidence: **likely**. This was inspection only, and the `Display` blind spot is MINOR-1's (text, not keys).

## Style

Questions I exercised: Q1, Q2, Q3, Q4, Q5 (partly), Q6 and Q7. I did **not** do Q8: I read no touched file end to end; `edit.rs` is over 6000 lines and I read about 900 of them.

- **S1 (Q1) — three walkers over a node's mutable expressions.**
  - The three: `edit.rs:719 node_exprs_mut` (branches on Measure/Assertion), `refactor.rs:2213 remap_node_vars` (`rows_mut` plus `payload_exprs_mut`), and the read twin `doc.rs node_exprs` (rows plus payload).
  - `node_exprs_mut` is right only while `rows()` is empty for Measure/Assertion and `payload_exprs` is `None` elsewhere. That invariant is held by two separate matches.
  - One more walker of this family would sit in `persist/check.rs`.
  - **likely**
- **S2 (Q7) — identity carried through text.**
  - Memoized refusals store a `String` with ids encoded as `#<16 hex>`.
  - `Speaker::formula` (`spoken.rs:567`) scans for `#` and 16 hex digits and parses them back to `VarId`.
  - A structured payload (a formula with id leaves) would make MINOR-1 unrepresentable.
  - **sure**, as taste.
- **S3 (Q1) — two spellings of one variable's tag.** `VarId`'s `Display` is 12 hex ("variable 3da0c3a822ef"). `unparse` and the memo text are 16 hex (`#3da0c3a822ef260e`). One refusal can show both. This is spec-driven (§8 Q1), but readers will not match them by eye. **likely**
- **S4 (Q1) — half-migrated vocabulary: "param" and "var" both name the variable.**
  - Rust: `SeedError::UnknownParam { param: SpokenVar }` (`analysis.rs`), `stackup::Unavailable { param }`, and in the viewer `SessionOp::SetParam`, `Refusal::NoSuchParam(VarId)` and `Selection::Param`.
  - Python: `ParamName`, `DocParam` and `DocParamValue` sit beside `Var` and `rename_var`.
  - **likely**
- **S5 (Q2/Q1) — the lowering rule is spelled twice.**
  - `check_reads` (`edit.rs:781-805`) re-derives why a name did not lower by calling `lowering_scope` and `expr_dim_of` again.
  - `Expr::lower_names`'s predicate decided the same thing and threw the reason away.
  - The comment "A name that did not lower is one no variable holds, or one held by a variable of another kind" is the only thing keeping the two in step.
  - **likely**
- **S6 (Q7) — whole-document work on every edit.**
  - `lowered` (`edit.rs:746`) clones every edit, `ReWitnessBulk` payloads included, just to look for names.
  - The `door` post-pass (`edit.rs:4632`) runs `unread_vars()`, a walk of every expression of every node, on every arm, including arms that cannot detach a reader.
  - **unsure** that this matters at document scale.
- **S7 (Q4) — a comment relies on an unchecked premise.** `refactor.rs:2740-2745` relies on "the part's insert door refuses it". What the user then sees is MINOR-3's "kernel defect" sentence. The comment's premise holds, but the outcome it implies (a sensible refusal) does not. **sure**
- **S8 (Q5) — `EvalError::UnresolvedVar` never gets a name.** Its `Display` (`expr.rs:1487`) says "variable {12-hex} has no binding … it was deleted, or never declared here". No path names the variable again when a document is at hand. For a deleted variable the name is gone, so a tag is all there is, but the sentence does not say which reader to repoint. **unsure**
- **S9 (Q3) — the rename row can pass when there is only one row.** `panel_edits::a_rename_keeps_the_parameter_row_…` asserts "the row at the same place is the same variable". With one parameter row that cannot go red under a re-sort. I did not check `common::parametric_plate`'s variable count. **unsure**
- **S10 (Q6) — the order-of-walks choice is undocumented as new.** `first_slot_read_fault` and `first_named_reader` (`persist/check.rs:564-592`) pick "first" in `BTreeMap` id order rather than document order. This mirrors the pre-existing `first_slot_fault`, so it is not new, but the new walks inherit an order an author cannot predict. **unsure**

## Claims exercised

| Claim | Status | How |
|---|---|---|
| C1 | Exercised | NOTE-1 and NOTE-2: masked structural comparison and a pixel diff of the GUI cells. Not re-run: the tour frames and MC sheets (none are in the diff), and the ε = 1e-6 / 1e-12 rows. |
| C2 | Exercised | MINOR-1 and MINOR-2 by execution. The existing row-1 test re-runs green. |
| C3 | Exercised | NOTE-4, by inspection. |
| C4 | Exercised | `p2`, `p4` and the row-7 test. |
| C5 | Exercised | `p3` and the rows 2, 4 and 9 tests. |
| C6 | Exercised | The load rows, the `p2` load and `p5`. |
| C7 | Exercised | `p1`, `p5` and two mutants. |
| C8 | Partly exercised | Python 927 and the viewer subset of 71. Not run: the full viewer suite (`--features app`, the GPU rows). |
| C9 | Exercised | Three mutants: one killed, two surviving (MINOR-4 and MINOR-5). |

Not exercised: rendering the tour, the viewer's GPU tests, and the ε rows.
