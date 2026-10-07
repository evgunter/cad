# Review — PR #4243, INTENT: inline carries a part's variables as new ids

Frozen head `b7d95c5399`, base `bca03b14ee` (merge-base with `origin/main`).
Reviewer lane: correctness claims C1–C7, plus the style lane
(`docs/prompts/reviewer-style-lane.md`). I did not read the PR's comments.

## Verdict: APPROVE-WITH-FIXES

The code implements FORK-6 as ruled. Inline never merges a variable by value
or by name. Split's closure is right on every shape I built, including chains,
tied free variables and anonymous variables read by a moving definition. The
remainder's `DeleteVar`s replay and save/load bit-exact, and the round trip
holds on every cut I tried.

There are no MAJORs. The fixes are about evidence and wording:
- no row pins the closure's fixed point, and a mutant that breaks chains survives the whole suite;
- the round-trip comparator gives a false red on a valid shape;
- one refusal message misdescribes a deleted variable.

## Findings

**MINOR-1. No row covers chains or tied free variables; a closure mutant survives.**
- Location: `refactor.rs:2493` (`moving_vars`, the reverse tie edge). Claims C2 and C7.
- Demonstrated by execution. I deleted `ties.entry(read).or_default().push(var);`, so ties run one way only.
- Every PR row passes under that mutant: the 119 tests in `intent_vars_3_readers`, `intent_literals_a_definitions`, `p2_split`, `asm4_split_inline`, `place_mate_frame_offset` and `refactor*`.
- Only my probes go red:
  - `w` cut-only, `k = w + 1 mm`, `m = k + k`: `m` stays in the remainder reading the deleted `k`, so it dangles.
  - `k = w + u` with `u` a free variable nothing else reads: split refuses.
- The PR's closure rows have one level each: `an_unread_definition_moves_with_what_it_reads_and_comes_back` and `a_definition_reading_both_sides_refuses`. Nothing pins "followed to a fixed point" or a free variable pulled in through a tie.
- The code at head is correct on both shapes: my probes are green there, including the round trip, replay and save/load.
- Confidence: sure.

**MINOR-2. `same_up_to_ids` gives a false mismatch when a named definition reads an anonymous variable.**
- Location: `tests/fixture/round_trip.rs:212` (`var_ids` maps named ids only) and `:280`. Claim C4.
- Demonstrated by execution. The edit door accepts `k := Formula::var(anon, Length) + 1 mm`, where `anon` is the cut extrude's anonymous depth.
- Split and inline handle it correctly. I checked directly:
  - the inlined `k` reads exactly the carried extrude's anonymous id;
  - the variable count is 38 in both documents;
  - replay, save/load and no-`Strand` all hold.
- `same_up_to_ids` still fails. The anonymous id inside `k`'s rendered definition is not in `var_ids`, so it "reads as itself".
- The failure is loud, never a false pass: it does not mask a difference. But C4's "for every cut split admits" cannot be checked by this comparator on that shape.
- The reverse direction holds. My probe `rv_comparator_catches_differences` confirms the comparator rejects two documents that differ only in a variable's value (1 ulp), distribution, unit, name or definition.
- Confidence: sure.

**MINOR-3. The `DefinitionStraddlesCut` message presents a deleted variable as one "which stays".**
- Location: the Display arm at `refactor.rs:934`; ruling 2 at `:2510-2517`. Claim C5.2.
- Demonstrated by execution. Setup: `k = w + z`, `w` cut-only, `z` deleted. The refusal reads:
  > `...ties it both to w, which moves with the cut, and to #89dbd35508e1848a, which stays...`
- That names a hex id the document does not hold as if it were a live variable on the kept side.
- Refusing is right, and consistent with `UnresolvedVarCrossesCut` for node readers: the part could not hold an unresolved read either. The ruling is sound.
- The wording is the problem. Today's `UnresolvedVarCrossesCut` already has words for this case ("which this document no longer holds"). This message and its recourse ("define k over variables of one side") do not tell the user that `z` is gone.
- Confidence: likely.

**NOTE-1. The order of the remainder's `DeleteVar`s is not observable.**
- Location: `refactor.rs:3292`. Claim C3.
- Demonstrated by execution. I mutated the loop to forward definition order, so a read is deleted before its reader. Every row and every probe stayed green.
- `DeleteVar` accepts any order (`edit.rs:6167-6190`: readers left unresolved, VR7). So "a reader before what it reads" is harmless but unguarded, and the brief's "an order the edit door accepts" holds for every order.
- Undo is trivially satisfied, because split is pure and undo means keeping `doc` (`refactor.rs:14`).
- Confidence: sure.

**NOTE-2. The A4 *Split* sentence is narrower than the closure.**
- Location: `ASSEMBLY.md:245`. Claim C5 (text fidelity).
- Demonstrated by inspection plus a probe. The sentence says "one every reader of which is cut, or which no node reads and whose definition reads only what moves".
- A free `u` read only by an unread `k = w + u` moves (probe `rv_tied_free_moves_with_group`). But `u`'s reader is a definition, not a cut node, and `u` has no definition. Read literally, neither clause covers it.
- The module doc ("A variable's side is the union of its readers'") and VR9's one-liner are faithful.
- A4 *Inline* says nothing about variables, but the ruling did not ask it to.
- Confidence: likely.

**NOTE-3. Python carries only `var` for `DefinitionStraddlesCut`.**
- Location: `pncad-py/src/py/refactor.rs:335`. Claim C6.
- `moving` and `staying` reach Python only through the message text. `UncutVarReference` puts both of its nodes on fields.
- The tag `definition_straddles_cut` is in `tags.rs`, `TAG_INVENTORY` and `MEMBERS_BOUND_AS`. The display and concision contracts each carry a sample.
- No Python row raises the refusal end to end.
- Confidence: sure (inspection).

**NOTE-4. The PR-body mutation claims reproduce.** Claim C7.
- **Mutant M2: no remainder `DeleteVar`.** Red: `a_cut_only_named_variable_moves_into_the_part`, `an_unread_definition_..._comes_back`, `a_name_redeclared_after_the_split_refuses_the_inline`, `p2_split::r1_every_shape_...`, `p2_split::r1_a_cut_holding_nested_gauges_...` and `place_mate_frame_offset::the_offset_and_its_parameter_cross_split_and_inline`. This is as the PR says.
- **Mutant M4: inline merges a same-named, bit-equal free variable.** Red: `an_equal_valued_name_the_host_holds_refuses`, `inline_refuses_a_definition_the_host_already_holds` (it refuses at `w`), and `a_name_redeclared_after_the_split_refuses_the_inline`.
- The two flipped rows go red under that plausible bug, as claimed.
- Confidence: sure.

**NOTE-5. One timing row fails, and it fails on base too.**
- Row: `name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time`. It took about 13 s and failed, both in the full head run and alone.
- I built base `bca03b14ee` in a separate worktree and target directory: it fails there the same way (13.1 s).
- This PR does not cause it; it is the bounded-time failure the brief expected in this container.
- Confidence: sure.

## Claims

- **C1 (never merge by value): holds.**
  - By inspection: `inline` at `refactor.rs:3727-3737` has no value path left, and `VarCarry::agrees` is deleted. `var_named(` in `crates/editor-core/src` is only the inline guard, the declare/rename uniqueness doors and lookups. `product.rs:1257`'s `bit_eq` compares selections, not variables.
  - By execution: the carried free variable keeps value, unit and distribution bit-equal (the PR row plus `rv_chain` with a toleranced `w`). Carried definitions read the carried ids, named and anonymous (`rv_named_def_reads_anonymous`).
- **C2 (closure): holds in code, under-tested (MINOR-1).** Exercised by execution:
  - a cut-only variable moves;
  - a read on both sides refuses;
  - a mixed group refuses and names the reading member (`k`, `w`, `j`);
  - an unread free variable stays;
  - a chain `k → m` moves;
  - a tied free `u` moves;
  - an anonymous variable read only by a moving definition (through a `DefineVar` fresh table) crosses, leaving no orphan in the remainder (an orphan check over remainder variables) and a variable count of 38 after the round trip;
  - a group reading only a deleted variable stays.
- **C3 (remainder `DeleteVar`s): holds.**
  - By execution: `apply`-replaying `remainder_edits` on `doc` gives `remainder` bit-exact (`bit_eq`); remainder and part save/load bit-exact; no `Maintenance::Strand`. Checked on four shapes.
  - The order is unguarded but harmless (NOTE-1).
- **C4 (round trip): holds on every cut I built.**
  - Shapes: named, anonymous, defined, toleranced, chained, tied-free.
  - The comparator falsely fails one valid shape (MINOR-2), and catches all five kinds of difference.
- **C5 (rulings): sound.**
  1. A literal is not a read: `Expr` keeps `Literal`, and VR5 has not landed. Inspection.
  2. A deleted read counts as staying: sound, but the message is wrong (MINOR-3).
  3. The refusal names the reading member: execution.
  4. A sibling arm, not a wider `kept_node`: sound. `UncutVarReference` carries nodes and the tied variable has none, though FORK-6's log text says "refuse `UncutVarReference`" (style S4).
- **C6 (surfaces): holds.**
  - Execution: `VarNameConflict` → `RenameVar` the host's `d` → inline succeeds, with two distinct variables (`rv_rename_recourse`).
  - Inspection: the tag, both censuses and both contracts.
  - Not done: I did not build the wheel or run Python.
- **C7 (rows can go red): mostly.**
  - Re-ran four mutants: M1 survives (MINOR-1); M2 and M4 reproduce the PR's claims; M3 survives (NOTE-1).
  - Every new row I checked can go red under at least one of these mutants.

Not exercised: Python unittest and wheel; workspace-wide and ε-variant runs; the viewer.

**Runs (private `CARGO_TARGET_DIR`).** `editor-core --profile default`, slow set included, at head plus my probes: 2846 run, 2844 passed, 106 skipped. The 2 failures:
- my probe `rv_named_def_reads_anonymous`, which fails by design (MINOR-2);
- the timing row, which fails on base too (NOTE-5).

The probes were not committed. Their patch is summarised above and was kept locally at `/tmp/claude-0/rv-probes.diff`.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q7, and Q8 (partly: I read the variable sections of `refactor.rs` end to end, about 2280–2530 and 2930–3300, not all 4305 lines). Q6 does not apply: the PR discloses no deviation.

- **S1 (Q1). `moving_vars` builds its own definer graph.**
  - Location: `refactor.rs:2489-2496`, beside `doc.rs:1136` (`var_definers`, `definition_edges`) and `node_var_reads` at `refactor.rs:2285`, a third hand-rolled transitive walk over `definition_reads`.
  - "Who reads this variable through definitions" now has three or more spellings across `doc.rs` and `refactor.rs`. A class worth sweeping: `reached_through_definitions`, `VarCarry::unheld`, `node_var_reads`, `moving_vars`.
  - Confidence: likely.
- **S2 (Q2/Q5). The closure is stated four times in prose.**
  - Places: the module doc (`refactor.rs:70-87`), `moving_vars`'s doc, A4 *Split* and VR9, plus the PR body.
  - They already differ (NOTE-2). The `moving_vars` doc paragraph is longer than the rule it defends ("one no node reaches, directly or through definitions, sits in a group…").
  - Confidence: likely.
- **S3 (Q7). `moving_vars` is quadratic in groups.**
  - Location: `refactor.rs:2515`. Each group re-filters the whole `order`, so every unread free variable (its own group) costs a full pass.
  - Harmless at today's table sizes. It is not how I would write it: one pass over `order`, accumulating per group, is the natural shape.
  - Confidence: unsure.
- **S4 (Q1). Two refusals for one fact, "a variable read on both sides".**
  - Variants: `UncutVarReference` for node readers and `DefinitionStraddlesCut` for definition readers. The log's ruling text names only the first.
  - The split is defensible (different payloads), but it is a second spelling. A third reader kind would mean a third variant.
  - Confidence: unsure.
- **S5 (Q3). The comparator renders through `Debug` and substitutes ids textually.**
  - Location: `round_trip.rs:280`, `renamed`.
  - It is loud on unmapped ids (MINOR-2). But an id the map does not hold "reads as itself", so a coincidental equal raw id in `b` would pass.
  - Not reachable with digest-minted ids in practice.
  - Confidence: unsure.
- **S6 (Q4). The deleted comment premise.**
  - "the remainder keeps its table either way" is gone. I ran `git grep` at the head for "unread twin" and "keeps its table" over `crates/` and `docs/`. The only hits are the historical fork-log row 84, which is process data, and an unrelated `resolve/pick.rs:861`.
  - The A4 acceptance wording now says "minted ids", and the comparator module doc follows it.
  - Confidence: sure (no finding).
- **S7 (Q3). The `an_unread_free_variable_stays` premise.**
  - `spare` is declared before `w` and is never tied, so this row cannot go red under M1 (one-directional ties).
  - It guards only "groups with no outside reads move", as the PR says.
  - Confidence: sure.
- **S8 (Q7). The Python mapping drops the two named variables** of the new refusal (NOTE-3), unlike its sibling, which keeps both nodes.
  - Confidence: likely.
