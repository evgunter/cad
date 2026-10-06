# Review r1 — PR #4146 "INTENT-LITERALS PR C: a slot holds a VarId"

Head reviewed: `4b06b7e9595a4d2f4be8b5fd74142c483a9c2a89` (frozen); base
`35e4ac1829c60187ab0978ec550a2d5ca8b32d26`. Private `CARGO_TARGET_DIR`; every
run below is `cargo nextest … --profile default` unless marked `ci`. I read no
other `review/*` branch, no reviewer output and none of the PR's comments.

## Verdict: APPROVE-WITH-FIXES

One MAJOR: the `SetExpression` door silently severs a slot from an anonymous
variable it reads through a definition, dropping that variable's tolerance.
Everything else I could exercise holds:
- geometry did not move, Interval included;
- lifecycle, split carry, mint determinism and P1 hold;
- the VR8 enforcement points read `is_axis`.

The other findings are test strength and documentation that promises more than
the code does.

## Findings

### MAJOR

**M1. `SetExpression` re-mints every anonymous variable in the slot it edits, untouched subtrees included, so shared reads and distributions are lost silently.**
- Where: `crates/editor-core/src/edit.rs` (`write_edit`, the `DocEdit::SetExpression` arm, ~l.5874–5898), via `Doc::slot_expansion` → `Doc::written` (`doc.rs` ~l.1673–1711).
- Claims: C3 and C5 (and spec §9 "Re-minting").
- What happens: the arm rebuilds the whole slot from its *written* expansion, in which every anonymous variable is replaced by its literal value. It then re-lowers the whole formula. The comment says "so every variable the rest of it reads is read again by id", which holds only for named variables.
- Demonstrated by execution (probe `r1_set_expression_keeps_shared_anonymous`, red):
  1. Point `a` is written `x = 0.5 m`, giving anonymous `v`. `SetVarDistribution(v, Normal σ=1e-3)`.
  2. Point `b` is written `x = v·2` with `Formula::var(v)`, which Python reaches as `doc.slot(a, "x") * 2`. `b.x` is an anonymous `Defined(v·2)`.
  3. `SetExpression { b, x, path [1] } := 3` produces `b.x = Defined(Literal(0.5)·Literal(3))` plus one `AnonymousVarRemoved`.
- Result: `b` no longer reads `v`. Its stackup, MC and Sym lanes stop seeing `a`'s tolerance, and no refusal or notice says so.
- Same mechanism: a slot whose definition reads its own edit's fresh entry loses that entry's identity and spread on any sub-path edit.
- Fix direction (not a design): expand only along `path`, or keep anonymous reads outside the replaced subtree as `Var` leaves.
- No PR row covers `SetExpression` over a slot whose expansion reaches an anonymous variable.
- Confidence: **sure**.

### MINOR

**m1. VR8's Sym row cannot go red under the bug it names.**
- Where: `crates/editor-core/tests/intent_literals_c_slots.rs`, `sym_difference` (~l.699) and `an_untoleranced_variable_is_a_constant_in_the_symbolic_lane` (~l.750).
- Claim: C9 (and C6's guard).
- The row decides `x − y` at `Sym<f64>`. Two width-zero symbols at equal nominals have an f64 value channel of exactly `0.0`, so `decide` answers `Zero` whether the lane bound a constant or a symbol. The `SymCounts` it collects are discarded (`let (decided, _)`).
- Demonstrated by two surviving mutants:
  - `is_axis` → every continuous variable (VR8 reverted): this row stays green. The `toleranced`/`stackup` rows and `m10_4_stackup_interval::an_untoleranced_parameter_is_no_axis…` go red.
  - Removing only `var_env_over`'s new constant arm (`analysis.rs` ~l.1075): **all 83 `intent_*` rows stay green** (`ci`).
- I could not finish `sym_9_retry_interval` / `m10_sym_profile_interval` under that mutant inside 590 s. That fits the cost blow-up being the only symptom, but no fast row guards VR8's "binds its exact nominal".
- Confidence: **sure**.

**m2. `Node::written` promises that re-inserting written nodes "is the document, ids included"; it is not, in general.**
- Where: `crates/editor-core/src/node.rs` `Node::written` doc (~l.4355–4360); `test_support::as_written` (~l.95–99).
- Claims: C3 and C4.
- Demonstrated (probe `r1_written_reinsert_is_not_the_document`, red): a point whose x and y read one fresh entry carrying `Normal σ=1e-3`, re-inserted from `written`, mints a different node id. x and y become two variables, and the distribution is `None`.
- The same happens for any anonymous variable value-edited after its insert, since the mint read the old value under ruling 1.
- `diefillet::corpus_text` relies on the promise and is correct only because the die has none of these. Nothing guards that precondition, and its own id assert covers profile step ids only.
- Confidence: **sure**.

**m3. The `mint.rs` module doc contradicts ruling 1.**
- Where: `crates/editor-core/src/mint.rs` l.14–18. It says each anonymous variable extends the chain "by the variable's kind … neither the name nor the value is in the preimage".
- `MintingEdit::DeclareAnonymous { kind, held }` puts the value in the preimage, which is the whole point of the ruling.
- The rewritten test doc `the_display_unit_is_not_part_of_what_an_insert_hashes` repeats "the variable's preimage (its kind alone)".
- Claim: C4. Inspection. Confidence: **sure**.

### NOTE

**n1. The spec and the PR body name a refusal that does not exist.**
- Spec §8 row 14 and the PR body name `SlotReadsUnmintedVar`.
- The code refuses `SnapshotError::ReaderOfUnmintedVar` (`persist/check.rs` `read_refusal`), and the row asserts that.
- Inspection. Confidence: **sure**.

**n2. Row 7's load half does not exercise what its doc says.**
- Its doc says "an anonymous variable read only by the definition of an anonymous variable nothing reads", but the fixture's only anonymous variable is the unread definition itself (`intent_literals_c_slots.rs` ~l.236–265).
- The through-definition case *is* guarded, by PR A's rows. A liveness mutant that ignores definers (`doc.rs` `unread_anonymous_vars`) reddens three `intent_literals_a_*` rows, C's `an_entry_reads_the_entries_before_it`, and my chain probes.
- C5. Execution. Confidence: **sure**.

**n3. The "structural divide included" load row tests only one direction.**
- `the_load_door_reads_every_slots_variable` doc: "the structural divide included". It tests a Count variable in a Length slot, never a continuous variable in a count slot.
- C2. Inspection. Confidence: **sure**.

**n4. `RangeField::Slot` now widens a shared variable for every reader.**
- On a slot reading a *named* or shared variable, it widens that variable for **every** reader. Before, it refused `SlotIsNotALiteral`.
- This is spec-consistent (§4 "Range"), but the certified answer is reported as the slot's (`CertifiedRange::field`) while it varies every reader.
- C7. Inspection. Confidence: **likely** that this deserves a sentence in `range.rs`.

## What I exercised (execution unless noted)

- **PR rows** (default profile): `intent_literals_c_slots`, `intent_literals_b_door`, `m10_4_stackup*`, `intent_vars_3_readers`, `docm9_range`: 77/77 pass. The C1 pins (`m10_p_fence`, `edit_placement_corpus_bits`, `perf2…`, `lib_g16…`, `unreadable_by_this_build`, `seat4/6/7/8`, `m4_pr3/pr4`, `msolve14`, `emit_union_rim`, `m4_pr4_resolve`, golden rows): 112/112 pass.
- **C1, Interval:** I added an **id-masked Interval digest** (every corpus point's `lo`/`hi` bits, nodes sorted by content, per document) to `m10_p_fence` on base and on head. All 29 per-document words and the total are **identical**, so the re-blessed Interval fence hides no value change. The f64 id-masked row is unchanged in the diff and green. The `edit_placement_corpus_bits` table has the same 21 `die_composed_tour` rows as base.
- **C4:**
  - equal values in one edit, fresh or written, mint three distinct ids;
  - sibling inserts differing in a value mint two nodes, and the same value mints one (D9);
  - mutant `Held::Value(0)` reddens `asm_parent_held_names::sibling_versions_mint_two_node_ids…`, `switch_slots::every_node_shapes_mint_is_pinned`, the B door sweep and two `meta_minted_ids` rows. Guarded.
- **C5:**
  - a chain `v ← d ← slot` saves and loads;
  - rewriting the slot reports `[d, v]` removed and the log keeps both;
  - a slot doctored off the chain refuses `AnonymousVarUnread`;
  - an anonymous variable read by another node survives its first reader's rewrite and retires when that node is deleted.
- **C2:**
  - an Angle or Count variable written into a Length slot refuses, either as `SlotDimensionMismatch` (written at its own kind) or as a kind fault at the address;
  - `lower_value` / `Lowering::slot` is the one walk;
  - `finish`'s `FreshUnread` reads the same `unread_anonymous_vars` as the GC and the load walk, which is one home. Inspection.
- **C7:**
  - an anonymous variable read by a cut node and, through an anonymous definition, by a kept node refuses `UncutVarReference`;
  - a frame, profile and extrude cut, where the extrude's distance is `v·4` and `v` has a spread, lands with one variable read through the carried definition, bit-equal, and the part loads.
- **C6** (inspection, plus the m1 mutants):
  - `analyzed_box`, `var_env_over`, stackup's `continuous_params` and MC's `varying()` all go through `is_axis`;
  - `range::derive` sets the seed as a distribution, so the widened variable becomes an axis and binds as a symbol;
  - every other variable is cleared to a constant.
  - The Sym `DriveMemo` keys forms by node id, and a constant and a parameter are distinct nodes, so I found no path where a constant-decided Zero is served over a widened variable. Confidence: **likely**.
  - The content-key memo feeds Sym value bits only (E12), so a constant and a zero-width symbol key equal. I found no caller that crosses `is_axis` status within one memo. Confidence: **unsure**.
- **C3:** inspected the viewer path editor. `carry_unmoved` keeps unmoved arguments' variables, and `Lit` equality is unit-blind, so a notation change does not re-mint.

**Not exercised:**
- **C8 Python:** no wheel built. I read `test_slot_variables.py`, which does not cover `int`/`Angle`/`WrittenAngle` slot args.
- **Viewer:** the viewer test suite was not run.
- **Tour:** no tour frames run. There are no committed frames to diff, so the claim rests on the PR's own run.
- **Full suites:** neither `--workspace` nor the full `_interval` slow set was run. I claim green only for the sets named above.
- **Disk:** fine; nothing cleared.

## Style

Questions exercised: Q1 (copies), Q2 (comments doing work), Q3 (can it go red, by mutants), Q4 (premises cited), Q5 (module docs), Q6 (unscheduled). **Q8 was not exercised**: I did not read `edit.rs` end to end.

- **S1.** `edit.rs` ~l.5880, the `SetExpression` comment "every variable the rest of it reads is read again by id". This is M1's defect stated as a guarantee. It is Q4's sub-case 2 (the code drifted from an intended invariant), so the sentence should stay and the code be fixed. Class: any re-authoring through `Doc::written` rather than `Node::authored`. Where else to look: `viewer/src/tree.rs` ~l.610, `product.rs` ~l.1256, `spoken.rs` ~l.676, `session.rs` ~l.361 (this last one is safe: it compares and keeps `reader`). **sure**
- **S2.** There are two re-authoring doors with near-identical names and different identity semantics: `Node::authored` keeps ids and `Node::written` re-mints. `authored_with` serves both, and so does split's `VarCarry::author`. Each doc defends its own, and nothing at a call site says which one a caller needs. Q1 and Q7: a type, or `#[must_use]` wording that says "re-mints", would do the work the prose does. **likely**
- **S3.** `mint.rs` l.14–18 and the test doc ~l.620 are stale against `DeclareAnonymous` (m3). The comment predates ruling 1 and was only half-updated. **sure**
- **S4.** `MintLogOrder` is checked twice: `persist/check.rs` `first_var_fault` (~l.497) and the structural walk (~l.1533), with a comment reconciling them ("the structural walk asks it again"). That is Q2's "two spellings of one rule" shape, disclosed in prose. **likely**
- **S5.** `stackup.rs` ~l.1078: `fn continuous_params` now filters `is_axis`, so its name says "continuous" and it answers "toleranced". **sure**
- **S6.** `analysis.rs`, `axis_tail_mass` / `axis_std_deviation` / `axis_box_mass` docs and `AnalyzedParam`'s `distribution: None` arm. "An axis built with no distribution is FIXED" describes a state `analyzed_box` no longer produces, because it filters `is_axis` first. The `None` arms and their docs are now reachable only through a hand-built box. Q5. **likely**
- **S7.** VR8's heading, "Coincidence tokens and the analysis lanes read ids", now sits over a body whose new half says untoleranced variables are *not* read by id in the analysis lanes; they are constants. The VR8 and §11 text is faithful to the rule as relayed, but the heading undersells the change. **unsure**
- **S8.** `intent_literals_c_slots.rs` `sym_difference` decides at `Sym<f64>`, where constant and symbol are indistinguishable (m1). Class: any "is a symbol / is a constant" row that reads only a `Sign`. Where else to look: `a_toleranced_variable_is_an_axis_and_a_symbol…` asserts "x − y at equal nominals is not [Zero]", which also needs the Interval or counts lane to mean it. **sure**
- **S9.** The PR body and spec §8 name `SlotReadsUnmintedVar`, but the code reuses `ReaderOfUnmintedVar` (n1). The second spelling of one refusal lives only in prose. **sure**
- **S10.** `Node::written` / `as_written` / `diefillet::corpus_text` (m2). The rebuild-equals-document precondition is a measurement-shaped claim with no guard. Q6: it owes a mechanical check, or a stated precondition at the claim site. **sure**
- **S11.** `range.rs` module doc: "a slot is boxed through the free variable it reads, exactly as a variable named directly is". That is true, and it makes `RangeField::Slot` and `RangeField::Param` the same query for a free slot (n4), leaving two field spellings for one widening. Q1. **unsure**
- **S12.** `lower_value`'s error path (`edit.rs` ~l.961) re-lowers every addressed row against the partially-minted `new` to find the address. It is a second walk whose only job is diagnosis. It is correct because the door aborts, but "one walk" in the PR body holds only for success. Q7. **unsure**

## Claims

| Claim | Status |
|---|---|
| C1 | Exercised; holds, including the id-masked Interval. Tour and MC sheet not run. |
| C2 | Exercised in part; holds for the doors probed. |
| C3 | **Falsified at `SetExpression`** (M1). Holds for sketch drag, `SetProgram`, the path editor and split. |
| C4 | Exercised; holds. Doc overclaims (m2, m3). |
| C5 | Exercised; holds at the GC, but `SetExpression` drives an unwanted retirement (M1). |
| C6 | Inspected; holds, but its Sym guard is weak (m1). |
| C7 | Exercised; holds. |
| C8 | Mostly not exercised. |
| C9 | Exercised by mutants; one row can't go red (m1), and one row's doc overstates (n2). |
