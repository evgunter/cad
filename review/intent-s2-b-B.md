# Review B — PR #4342 "INTENT stage 2 PR B: operands are reads" @ `08e37ed675`

Base `84e4d2c348` (merge-base with main). Probes: `review/intent-s2-b-B-probes.rs` (drop into `crates/editor-core/tests/`, add `mod review_b_probes;` to `all.rs`) and `review/intent-s2-b-B-geom.rs` (the id-free body digest, run on both trees).

## Verdict: NOT-MERGEABLE-AS-IS

Two MAJORs, both executed, both small to fix:
1. The slot door admits an assertion re-pointed across dimensions, which the snapshot save door refuses but a logged save and load accept.
2. A read's port never reaches the content key, so the memo (D4) serves the other half's body after a port re-point. This is DR-59's stale-memo class, minted afresh.

Everything else the PR claims held under execution:
- the roots/bijection one-shot;
- geometry unmoved, 29/29 corpus documents id-free bit-equal base vs head;
- five door mutants all caught;
- strands and replay.

## MAJOR

**M1. `SetParam` at an assertion's `measure` skips the bound-dimension check the insert door makes.**
- Where: `edit.rs` `set_operand` (:5483) → `write_reads` (:5533) runs only `check_node_inputs` + `check_acyclic`. `insert_into` also runs `check_node_slots` (:5294), whose `assertion_bound_fault` is the AssertionDimension rule. `SlotKind::Measured` (`operand.rs`) admits any scalar output.
- Demonstrated by execution, probe `p1_assertion_repointed_across_dimensions`:
  - an Assertion over a Length measure with a Length bound, re-pointed by `SetParam{slot: Operand(Measure)}` at an Angle measure, is **accepted**;
  - the insert door refuses the same node `AssertionDimension{measured: Angle, bound: Length}`;
  - `save(snapshot)` then refuses `Snapshot(AssertionBound)`;
  - `save(empty, log)` **succeeds**, and `load` gives back the bad document bit-equal;
  - evaluation refuses `AssertionDimension`.
- This is the shape `set-members-admits-a-forward-member-the-save-validator-refuses` describes (a door admits what the validator refuses), and this PR closes that issue. C1 ("each check enforced") is falsified for the DM5-adjacent payload checks.
- Confidence: sure.

**M2. The content key ignores which port a read reads: stale memo across a split-port re-point.**
- Where: `eval/mod.rs` :5954 feeds `upstream_keys` from `Doc::upstream_of` (`doc.rs`:1542), which maps each read to its operation and dedups by node. The `Boolean`/`Union` key arms feed no operand. A read of `split.above` and one of `split.below` therefore key alike.
- Demonstrated by execution, probe `p9_memo_across_a_port_repoint`:
  - `Boolean{a: Output(split,0), b: far block}` is evaluated, then re-pointed via `SetParam` to `Output(split,1)`;
  - both content keys equal `68397654924711493483964612400278585551`;
  - the two bodies differ;
  - `evaluate(doc1, prior=ev0)` returns the **above** half's vertices, not fresh's;
  - the naming key is equally blind (`upstream_naming` pairs `(input id, key)`).
- Why it matters: silent wrong geometry, the one thing fail-loud forbids. Ports exist only from this PR, so this is new reachable surface. `Part{SplitHalf}` is safe (it feeds its select). Revolve's two ports differ in kind, so no seat can swap them; the split is the live case.
- DR-59 (`docs/DUAL-REVIEW-LOG.md`) fixed the same class by making every key arm name its fields; that discipline does not cover a read's port.
- Confidence: sure.

## MINOR

**m1. The bijection comparator cannot see a port (C3 overclaimed).**
- Where: `tests/wire/up_to_ids.rs` `reads_as_inputs_by` (:95) rewrites each output id to its node id, dropping the port.
- Execution, probe P5: two documents identical but for `Boolean.a = split.above` vs `split.below` give `same_up_to_ids → Ok(())`, though the raw JSON differs.
- So "cannot pass two documents that differ in a read" is false. It does not invalidate this PR's one-shot:
  - none of the five files holds a split (grep: 0 `"Split"`, 0 `SplitHalf`);
  - a revolve's body and axis ports differ in kind, so no admitted read can confuse them.
- Confidence: sure.

**m2. Ruling (7), "DM5 over the operations read", contradicts the spec and refuses a FORK-1b use.**
- Spec §3 says "DM5's distinctness (`input_fault`, now over variable ids)". The PR instead rewrote ratified REFERENCES DM5 to "over the operations read". That is a design choice, not a re-word forced by the code.
- Execution, probe P10: `Pattern{input: rev, Circular{axis: Output(rev,1)}}`, a pattern of a revolve's body about the revolve's own `axis` port (the port FORK-1b added to be read), refuses `DuplicateInput`. The same pattern about the datum the revolve reads is accepted.
- Execution, probe P3: `Union[split.0, split.1]` refuses `DuplicateInput`, while `Union[Part(Above), Part(Below)]` of the same split is admitted. One fact spelled two ways gets two answers (Ev 2026-10-03: "many ways to say these things").
- Needs the orchestrator, and Ev if DM5's text stays.
- Confidence: likely.

**m3. `Operand::Node` documents a selection rule the code doesn't implement, and neither matches Q5.**
- `operand.rs`:25 says "its one output, or the one output of the seat's kind".
- `Doc::read_of_node` (`doc.rs`:1574) returns port 0 unless another port shares port 0's kind, and ignores the seat.
- Q5 says a multi-output node refuses the sugar `AmbiguousOutput`.
- Execution, probe P2: `Revolve{axis: Node(rev)}` refuses `SlotVarKind{found: Body, expected: Is(Axis)}`, where the doc promises the axis port; `Output(rev,1)` is accepted.
- Confidence: sure.

**m4. Ruling (8) is still partly positional.**
- `check_declared_sides` (`edit.rs`:5458) holds a name in reach when `minter < at || at reads minter` (less the carrier and its descendants).
- So a name minted by any earlier node the carrier does not read passes. The PR body says "decided by the read relation".
- It is an improvement on base (purely `minter < at`), but document order still decides (Ev: no "declaring by position").
- Confidence: likely.

**m5. Ruling (6): a pre-B file meets two refusal families, and the spec said `Unreadable`.**
- Execution, probe P6 (base files loaded on head):
  - snapshots (`die_tool`, `gallery_ring`, `plate_param`) refuse `Snapshot(OperandUnminted)`;
  - logs and the tube file (`die_composed_tour`, `golden.cad`) refuse `Unreadable`.
- All five carry the true `REGENERATE_RECOURSE` (checked in the Display text).
- `unreadable_by_this_build.rs`'s header calls `Unreadable` "the one refusal that stays"; B adds a second, and `OperandUnminted`'s text hedges between "never minted" and "written before an operand was a read".
- Neither `OperandUnminted` nor `ReadCycle` has a committed file-level row: both appear only in `display_contract`'s built values. `ReadCycle` is reachable (probe P8, a union member doctored to its own transform's output → `Snapshot(ReadCycle)`).
- Confidence: sure.

## NOTE

- **n1.** `Maintenance::StrandedRead` is a sibling variant, not the "read arm" of `Strand` that spec §1/§3 describe. The orchestrator accepted this (C3, log); recorded, not a defect. Confidence: sure.
- **n2.** `name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time` fails on head (10.5s) and on base (12.0s, `CAD_FUZZ_SEED=0x5335ff905aff4ce5`) in this container. Environmental, not this PR. Confidence: sure.
- **n3.** A stranded placer behaves (probe P7): a `Transform` whose input is deleted reports `StrandedRead`, saves, loads, saves with its log, and re-points at a body. Its reader's poison names the root cause (`UnresolvedRead` at the transform). Confidence: sure.

## The implementer's rulings

1. **A read at a scalar slot is that variable's formula.** Right in effect. It adds a second spelling of one write, and the edit log keeps whichever was authored (`SlotValue::Read` vs `Formula`). Under-specified rather than wrong. Likely.
2. **A formula at an operand refuses `SlotDimensionMismatch` with `expected: SlotKind`.** Right. Sure.
3. **`SlotId` on the read refusals; `FreshUnread`.** Right. `FreshUnread{index: 0}` reports 0 whatever the table's length. Likely.
4. **The profile-program slot word becomes `program`.** Right: `profile` is the operand's word now. Not executed. Unsure.
5. **`slot_var_kind`'s `expected`/`found` swap.** Right. The old meaning (expected = the variable's declared kind) was the reverse of every other `expected`/`found` in the census. The swap is silent (same field types), which is tolerable in an unreleased kernel; the Python row was restated. Likely.
6. **A pre-B file refuses `OperandUnminted` with the recourse.** Acceptable but under-specified (m5). Stage 1 and unit A routed "older shape" through `Unreadable` plus the read walks; B splits the one fact across both. Likely.
7. **DM5 over the operations read.** Wrong against the spec's letter and FORK-1b's intent (m2, P3/P10). Likely.
8. **`DeclaredNameNotUpstream` by the read relation.** An improvement, but mis-described: a positional disjunct remains (m4). Likely.
9. **The viewer's seats read through `Doc::read_of_node`.** Right, and inherits m3's port-0 rule. `denotes_body` hand-rolls `kind == Body` (S5). Likely.

## Style

- **S1 — three hand-kept lists of operand fields (Q1; sure).** `node.rs` `operand_rows` (:3410), `operand_rows_mut` (:3557) and `try_map_slots` (:4349). Only the first names every field with `_`; `_mut` matches with `..` and `Node::Datum(_)`. So the doc's "a reference added to a node does not compile until it is stated" is false for the writable twin. `set_operand` tests presence through one list and writes through the other: a drift is a silent no-op write. Look also at `Node::outputs` and `OperandSlot::kind`.
- **S2 — two refusals for "this operand reads nothing live" (Q1; likely).** `lower_operand` (`edit.rs`:1180–1225): a dead `Node`/`Output` id refuses `UnresolvedInput`; a dead `Var`, an unknown `Name` or a port past the signature refuses `OperandUnresolved`.
- **S3 — `write_reads` is a second, narrower list of the insert door's node checks (Q1/Q4; sure, as a class).** M1 is one instance. Sweep `insert_into`'s list (:5997– 6085: payload refs, declared sides, gauge ref, alignment, placement frame, `check_node_slots`) against `write_reads`/`SetMembers`, and record per check why it is or isn't owed. The `DM6` doc says the write passes "the checks the insert door … make of a node's reads", which nothing enforces.
- **S4 — the fix mints a fresh instance of the defect class it inherits (§1 stance; sure).** DR-59's lesson was "every key arm names its fields". B moved operands out of fields into reads and the port fell through (M2). Look also at `naming_key`'s `upstream_naming`, the viewer's `ancestry`, and `relative_freedom_components` over `upstream`.
- **S5 — `crates/viewer/src/combine.rs`:989 `denotes_body` compares `var.kind() == Body` by hand (Q1; likely).** Spec §3 says the seat "becomes the slot kind check"; `SlotKind::admits` is that check.
- **S6 — `crates/pncad-py/src/tests.rs`:7845 still uses `DeleteWouldDangle` (Q4; sure).** That arm is retired; it appears as the census-swap example.
- **S7 — `intent_s2_b_reads.rs` claims a row that doesn't exist (Q2; sure).** In `the_slot_door_re_points_an_operand_and_reports_what_it_strands`, the comment "a transform of `a`, and `a`'s profile re-pointed at nothing that reads it is fine" describes no assertion that follows. The row's name promises a strand, but it asserts only "strands nothing"; `Took::Reach` is pinned in `dm7_delete_strands.rs` instead.
- **S8 — many ways to say one thing (Ev's 2026-10-03 concern; likely).** One read can be spelled four ways (`Operand::{Node, Output, Var, Name}`), and a slot value two (`SlotValue::{Formula, Read}`). A list member can be written two ways (`SetMembers` and `SetParam{Member(i)}`), and a split half two (`Part{SplitHalf}` vs a port). P3 shows two of these disagree. Each is spec'd or ruled; the sum is the shape the transcript set out to remove.
- **S9 — the declared-name rule is now two rules (Q1; unsure).** The load door asks only "not self-minted" (`persist/check.rs`:1943); the edit door asks the reach rule (`edit.rs`:5458). The comment's reason (a re-point may strand, and that must load) is sound, but the module's "one rule asked by both doors" framing no longer holds.
- **S10 — `Doc::reads` (`doc.rs`) has no caller anywhere in the workspace (Q7; likely).**
- **S11 — `work/intent/part-split-half-retires.md` is filed with no unit or trigger (Q6; sure).** It says only "a later unit than B", so it is disclosed but unscheduled. Its own premise (it moves roots) points at C.
- **S12 — Q8: I did not read `edit.rs` (7691 lines) end to end (unsure).** I read `operand.rs` whole, and these regions of `edit.rs` in full: `lower_operand`, `set_operand`, `write_reads`, `stranded_by_repoint`, `check_declared_sides`, `check_node_slots`, the `SetParam`/`SetMembers`/`DeleteNode` arms and `insert_into`'s check list.

## Claims exercised

- **C1 — exercised, partly falsified (M1).** Kind, DM5, acyclicity and reach strands are each bitten by a mutant (below). Liveness is read, not mutated.
- **C2 — exercised.**
  - The delete row (strand, `UnresolvedRead`, undo bit-equal) passes in the slow set.
  - P4: a re-point that strands (`Strand{Reach}`) replays bit-exact through save+load with its log, and round-trips to the start.
  - P7: a stranded placer behaves.
  - Re-kinding: kinds are fixed (VR3), and the transform shape change refuses (row green).
- **C3 — exercised.**
  - Re-ran the one-shot: `PRE_B_TREE=<base worktree>`, all five "equal up to ids".
  - Comparator mutant P5 passes a port re-point (m1). The PR's own union mutant is caught.
- **C4 — exercised.**
  - An id-free body digest (sorted vertex bits per body, by node position) over all 29 `corpus::documents()`, run on the base and head trees: **identical**. This covers seat7's five sweep docs, `die_tool`, `plate_param`, `die_composed_tour` and the four tube docs, whose geometry is unmoved despite FORK-1b's frame.
  - `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked` passes on head with its pre-PR constant.
- **C5 — exercised** (above): P2, P3, P6, P10.
- **C6 — exercised in part.**
  - `cargo nextest -p pncad-py`: 134/134.
  - P6 (recourse text) and P8 (`ReadCycle` reachable).
  - The Python wheel `unittest` was not run.
- **C7 — five mutants, all caught:**
  - kind check off in `check_read`: 4 rows red;
  - DM5 off in `write_reads`: 1 row red (`the_slot_door_re_points…`, the only one);
  - reach strands off: 2 red;
  - load-door operand kind off: 4 red;
  - `StrandedRead` off: 2 red.
- **Slow set:** `cargo nextest run -p editor-core --profile default` (private target, probes excluded) gives 2914 run, 2912 passed, 2 failed:
  - `every_suite_file_is_aggregated`: my own probe files, then removed;
  - the timing row: red on base too (n2).
- **Not exercised:** viewer, tour, clippy/fmt, the Python wheel, ε = 1e-6/1e-12 runs.
