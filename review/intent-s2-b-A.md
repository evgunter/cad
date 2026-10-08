# Review A — PR #4342 "INTENT stage 2 PR B: operands are reads", head `08e37ed675`

Base `84e4d2c348` (merge base with `origin/main` at review time). Private
`CARGO_TARGET_DIR`; merge base checked out beside it for the one-shot and
for base-vs-head executions. Probe file (not pushed) is reproduced at the
end of this report's branch as `review/review_probe_a.rs`.

## Verdict: APPROVE-WITH-FIXES

The representation step lands whole: every operand is a typed read, one
door writes it, deletes strand and never refuse, the one-shot holds
against the merge base, and geometry is unmoved where I executed it.
Nothing I found breaks a ruling. What needs fixing is around the edges:
a test that cannot go red, a check blind to half the operand families, a
new doc comment that promises what the code does not do, and tracker
rows the unit closes but leaves standing.

## Findings

**MINOR-1. `Doc::upstream` returns dead node ids for a measure, against its own contract.**
`crates/editor-core/src/doc.rs:1523` says "a read this document does not
resolve contributes nothing", but `upstream_of` (`doc.rs:1542`) chains
`Node::measure_sites()` unfiltered. Execution (probe P4): delete the extrude a
`Measure` is sited at (accepted now; before B this refused
`DeleteWouldDangle`), and `doc.upstream(measure)` still lists the deleted id;
evaluation refuses `MissingInput { input: <dead id> }`, not the typed
`UnresolvedRead` the spec's deletion rule names. The dead id also flows into
`roots::walk_strict_ancestors`, `cascade_delete_order` and
`relative_freedom_components` (`mate/solve.rs:886` inserts it as an adjacency
vertex). The stranded file saves and loads. Q3 defers the measure's `at` to D,
so this is a B-sized gap, not a ruling violation; the contract sentence is
false today. sure on the behaviour, unsure whether any caller misbehaves on
the dead id.

**MINOR-2. The unit's own check is blind to the list families it names.**
`every_re_blessed_document_is_the_pre_b_one_up_to_ids`
(`tests/intent_s2_b_reads.rs:601`) compares five files that hold no `Union`,
`Loft`, `Pattern`, `Transform`, `Split`, `Part`, `Fillet`, `Sweep`, `Mate` or
`InstantiatePart` (counted per file; `die_composed_tour.pncad` is an edit log
with zero nodes). Spec §9 row 4 says it "breaks if `Doc::upstream` misses an
operand family or double-counts a list member". Mutant: `upstream_of`
filtering out `OperandSlot::Member(_)` — the one-shot stays green (only
`a_forward_member_saves_loads_and_cascades` and the re-point row go red).
Mutant dropping `OperandSlot::Profile` goes red, through the load door's
`RootFault::Uncovered`, so the row bites only for families the five files
carry. The claim "roots byte-equal on every corpus file" is true of the
files and vacuous for the list families. sure.

**MINOR-3. Test 5's "undo" assertion cannot fail.**
`tests/intent_s2_b_reads.rs:192-201` asserts `doc.bit_eq(&before)` where
`doc` was never handed to a mutating door: it checks that `apply` is pure,
not that undo restores the stranded reader. The viewer's undo is a stored
snapshot (`viewer/src/history.rs:315`), so the property holds by
construction, but the row named for it exercises nothing. sure.

**MINOR-4. `Operand::Node` and `read_of_node` promise seat-kind resolution the door does not do.**
`operand.rs:22` ("its one output, or the one output of the seat's kind") and
`doc.rs:1570`. Execution (probe P1): `Operand::Node(revolve)` at a circular
pattern's `axis` seat refuses `SlotVarKind { found: Body, expected: Is(Axis) }`;
`Operand::output(revolve, 1)` is accepted. The behaviour is Q5's (port 0, a
multi-port node spells its port) and the PR body says so; the two doc comments
in new code say otherwise. Also `read_of_node` reads "ambiguous" as "another
output of the first's kind", narrower than Q5's "multi-output node refuses the
sugar"; a revolve named alone is silently its body. sure.

**MINOR-5. Rows the unit closes are left standing.**
`work/doors/a-member-set-after-its-union-points-forward-so-save-and-cascade-delete-break.md`
(open, P2) is the same defect as the closed
`recipe/set-members-admits-a-forward-member…`, and its three items
(`ForwardInput`, cascade order, split delete order) all dissolve here.
`work/recipe/no-docedit-splices-a-deleted-node.md` (deferred) rests on "no
DocEdit rewires a live node's inputs", which `SetParam` at an operand
falsifies. sure.

**NOTE-1. `slot_var_kind.expected` at a nested formula leaf is the leaf's read dimension, not the slot's kind.**
`edit.rs:1439-1445` builds `expected: Is(VarKind::from(referenced))` from the
formula fault; at a slot's root that equals the slot's kind, inside `sin(w)`
at a length slot it is `Angle`. The PR body and `work/intent/log.md:487`
state "the slot's kind". Under-specified, not wrong. likely.

**NOTE-2. Façade asymmetry on list members.** Rust accepts
`SetParam { slot: Operand(Member(i)) }` (PR row); Python's `member`/`section`
words have no reading (`slot_word.rs:19-36`, `tags.rs:560,568`). One door,
two reaches. likely.

**NOTE-3. `Doc::reads` (`doc.rs:1506`) has no caller** in src, tests, demos
or the viewer; the spec names it as the accessor beside `upstream`. sure.

**NOTE-4. Stale prose the retirements leave.** REFERENCES DM5 still says the
rule is "called by `InsertNode`, by `SetMembers` … and by the load validator"
(`REFERENCES.md:311`); `write_reads` makes `SetParam` at an operand a fourth
caller. `pncad-py/src/tests.rs:7845` still explains itself by
`DeleteWouldDangle`. sure.

**NOTE-5. Against Ev's 2026-10-03 transcript.** An operand has four authored
spellings (`Node`, `Output`, `Var`, `Name`) and a read at a scalar slot equals
`Formula::var` (ruling 1): two ways to write one slot, both sanctioned by Q5
and the one-door ruling, so recorded, not flagged. `Member(i)`/`Section(i)`
are positional addresses, inherent to a list; nothing places or declares by
position. No constraint falls back to an assertion; no new ceremony. likely.

## Implementer rulings

1. Read at a scalar slot = formula of its one variable: **right**; the PR row
   shows the two saves byte-equal; `VarIsAnOutput` still fires for an output
   (D's business).
2. Formula at an operand refuses `SlotDimensionMismatch { expected: SlotKind }`:
   **right**; one arm, wording checked ("reads a profile, not a length
   expression").
3. `OperandUnresolved`/`AmbiguousOutput`/`DefinesNothing` carry a `SlotId`;
   read with a fresh table refuses `FreshUnread { index: 0 }`: **right**
   (P3, PR row). `index: 0` is a slight fiction when the table has three
   entries; harmless.
4. Python program word becomes `program`: **right**. The old `profile` word had
   no reading, so no script could have relied on it; a `set_param(profile_node,
   "profile", …)` now refuses `unknown_slot`, not silently re-points.
5. `slot_var_kind` expected/found swap: **defensible, under-specified**. New
   reading matches `slot_dimension_mismatch`'s convention (expected = what the
   slot takes) and the old one was the reverse (`bind_count_param` with a
   length var: was expected="length"/found="count", now the other way, test
   diff at `test_document.py:2043`). It is a meaning change under an unchanged
   tag with no deprecation; the stub should state the pair once (NOTE-1).
6. Pre-B file refuses `OperandUnminted` with the regenerate recourse rather
   than upgrading: **right**, and consistent with unit A (`PRE_OUTPUTS` in
   `bool13_r1_probes.rs` refuses `OutputSignature` typed with the same
   recourse) and stage 1 (only the current shape loads). Execution (P5e):
   base `die_tool.pncad` and `plate_param.pncad` refuse `OperandUnminted`
   with the recourse; base `golden.cad` refuses `Unreadable` first (unknown
   field `spine`), so a pre-B file with a tube gets the other wording. Fine.
7. DM5 over operations read: **right** (P3: both halves of one split in one
   boolean → `DuplicateInput`).
8. `DeclaredNameNotUpstream` by the read relation: **right by inspection**
   (`check_declared_sides` through `defined_by`); not exercised.
9. Viewer seats through `Doc::read_of_node`: **right**; see MINOR-4 for what
   the sugar resolves to.

## Claims

- **C1 One door.** Exercised. Kind (PR row; P1, P2 a profile on a
  `Datum::Plane` refuses `Plane` vs `Frame` at the door; the base reached
  evaluation first, `wire.rs:1205` on the base), liveness (P7 re-point at a stranded id → `OperandUnresolved`),
  acyclicity (P3: a union reading itself by node and by output id →
  `WouldCycle`; through `SetMembers` too), DM5 (P3 by id; both halves). No
  other write path: `set_list_input` is only reached from `SetMembers`,
  `operand_rows_mut` only from `set_operand`; `remap_node` rewrites reads
  under split/inline (`refactor.rs`). Mutant 1 (kind check off) caught by 3
  rows.
- **C2 Strands.** Exercised: delete under a fillet (PR row), under an
  assertion and a profile under an extrude (P4, P7): `StrandedRead` rows,
  `UnresolvedRead` at eval, save/load bit-equal, repair by the slot door
  then evaluates. "Re-kinding what a slot reads" is not reachable: no door
  re-kinds an output (a transform's kind is fixed, a measure's dimension has
  no edit). Undo: by inspection (snapshot history); replay: golden.cad's log.
  See MINOR-1 for the measure site.
- **C3 Roots and the comparator.** One-shot re-run against the merge base:
  five files "equal up to ids, reads as inputs". Comparator mutants by
  execution (P6): a renamed variable, a moved value, a name moved to another
  variable, an added node — each named at its path; plus the PR's re-pointed
  member. Coverage gap: MINOR-2.
- **C4 Pins.** Three by execution on both trees: `m10_p_fence::…_with_ids_masked`
  prints the same id-free digest `(556aeaf5b2dc3e4a, 33618bf1bd5dd21e)` on
  base and head; `asm2b row5` (volume bits `2.0`, solid count) and
  `msolve14 a3` pass on both, and the only constants that differ between the
  trees are the names/ids digests. Not executed: the other 20 rows of the
  table (inspection of the diff: ids and the `value` wire word only).
- **C5** above. **C6** Load refusals reachable and readable (P5a–d:
  `SlotVarKind`, `OperandUnminted` with recourse, `ReadCycle`, a node id at an
  operand). Tags/`.pyi`/census by inspection plus the pncad-py Rust census
  run below; the Python wheel suite not re-run (PR reports 951/951).
- **C7** Mutants 1–3 above; the test plan's rows 3, 4, 5 run green.

## Test runs

- `cargo nextest run -p editor-core --profile default` (slow set) on the
  frozen head, `--no-fail-fast`: **2913 / 2913 passed, 109 skipped** with one
  row set aside: `name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time`
  fails on this box (13.5 s against its bound) and fails identically on the
  merge base's tree (13.9 s), so it is this environment's, not the PR's.
- `cargo nextest run -p pncad-py` (tag inventory and census rows): 134 / 134.
- Stage-2 A and B rows: 21 / 21. One-shot vs merge base: 5 / 5 files equal.
- Not run: the Python wheel suite, viewer, tour.

## Style

- `operand.rs:109-117` `OperandSlot::Plane` admits only `VarKind::Frame`
  while a `VarKind::Plane` exists one enum over: a slot named for a kind it
  refuses. A reader of `SlotVarKind { slot: Plane, found: Plane,
  expected: Frame }` (P2's exact output) has to know the history. sure.
- `refactor.rs:362` `.unwrap_or(crate::OperandSlot::Input)` and
  `mate/member.rs:616` `.unwrap_or(part)`: two silent fallbacks where a read
  did not resolve, each inventing an address rather than refusing. Class:
  grep `unwrap_or(` beside `operation_of` in the touched area. likely.
- `edit.rs:5559` `stranded_by_repoint` recomputes `strict_ancestors` twice for
  every name-carrying node on each re-point (the whole document, two walks
  per carrier); `check_acyclic(new)` is also whole-document per write. Fine
  today, quadratic by shape. unsure.
- `edit.rs:6344-6361` the `SetParam` arm branches on `(slot, value)` four
  ways and routes an operand read to `set_operand` while a scalar read is
  re-spelled as a formula: two lowerings of `Operand` meet at one arm. Not how
  I would have cut it (one `lower` returning either a read or a formula).
  unsure.
- `SlotKind::Measured` (`operand.rs:193`) is "a scalar an operation defines",
  a predicate shaped for today's one scalar-defining node; D renames it
  observed. A transient kind with a permanent name. likely.
- `Doc::reads` unused (NOTE-3); `OperandSlot::label` says "first operand"
  where Python says `a` and `OperandSlot::At` says "body" where its word is
  `at`: three spellings per field (label, word, variant). likely.
- Q1 sweep (`verbatim|re-derived|ported from|mirror of`) and Q2 sweep over the
  touched files: hits are pre-existing (`refactor.rs` instance crossing,
  `check.rs` walk-roster prose); nothing new self-declares a copy. The
  undisclosed copy I did find is the kind check spelled twice,
  `check_read` (`edit.rs`) and `first_operand_read_fault` (`check.rs:700`),
  both over `SlotKind::admits`, each with its own half-port clause. likely.
- Q8: read `operand.rs` whole and `up_to_ids.rs` whole; `edit.rs` in the
  touched regions only (7,500+ lines).

## Exercised / not

Exercised: style Q1, Q2, Q3 (MINOR-2, MINOR-3), Q4 (NOTE-4, MINOR-5), Q5
(MINOR-4, NOTE-3), Q7, Q8 (partial). Not exercised: Q6 (no disclosed
deviation beyond the two filed rows, both scheduled as `work/intent/` items);
viewer and tour suites; Python wheel; FORK-7 (not this unit's).
