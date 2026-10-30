# INTENT — log

## 2026-10-03 — opened

Chartered by the session that ran the D10 ruling (PR 3990), to build
it. Opening slate: stage 1 (variables). The representation row goes to
a designer pair before any lane builds; the build, derived parameters
and the GUI row are parked on it.

Moved here from RECIPE: `d10-one-way-to-say-intent-is-unbuilt` (the
hold's trigger; its id is unchanged, so every parked row still
resolves), `equal-literals-lower-to-one-identity-token` and
`mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`.

## 2026-10-03 — the representation is settled (VARIABLES-DESIGN VR1–VR9)

A designer pair weighed `variables-are-identities-with-labels`
(blinding: `analysis/design-fork/intent-variables-2026-10-03`). They
converged on everything but where a written quantity becomes a
variable; the orchestrator took the edit-door lowering of an authored
`Formula` (the type then cannot hold a literal) and folded in the other
report's findings. An elaboration of D10 that changes no ratified text,
so it did not go to Ev; the design is `docs/VARIABLES-DESIGN.md`, with
a companion-table row. Stage 1 is re-sliced: the build starts with
`variables-replace-the-parameter-table`; the literal retirement, derived
variables and the GUI row are parked behind it.

## 2026-10-03 — VARS PR 1: the renames (orchestrator's read)

`docs/INTENT-VARS-1-SPEC.md` sliced `variables-replace-the-parameter-table`
into three PRs, each green. PR 1 renames `ParamName`→`VarName`,
`DocParam`→`FreeVar`, `DocParamValue`→`FreeValue` and the name-fault
types, with no behaviour or wire change (golden fixtures untouched).
Review tier: the orchestrator's read — a mechanical rename whose only
risk, the wire, is pinned by the golden tests. The Python classes keep
their names until PR 3 reshapes that surface (the binding census now
argues the three types by `BOUND_AS`). PRs 2 and 3 are H and draw a
concurrent dual.

## 2026-10-04 — VARS PR 2 to review (dual, concurrent)

PR 4011, frozen head `42e2330ddf`. Pre-draw fields: difficulty **H**,
task class **STRUCTURAL**; arm CONCURRENT (rule 1). Both reviewers run
in their own cloud sessions, since this container's disk cannot hold
two more build directories; the method is identical for both. Rulings
made at hand-off: the Monte-Carlo sheets move because draws follow
`VarId` order (by design: name order would let a rename move them) and
are re-rendered; the symbolic tier's two lost registrations on
r2_filleted_bracket are a claim for the reviewers, not an accepted
cost.

## 2026-10-04 — VARS PR 2 merged (PR 4011)

Both reviews delivered at frozen head `42e2330ddf`. Each ran in its own
cloud session from one brief (sha256 prefixes `ed0ba53e17eaf67c` and
`fe6b38288dbaa08d`, identical modulo the lane label). R1 returned
APPROVE-WITH-FIXES. R2 returned NOT-MERGEABLE-AS-IS: four editor-core
rows were red at the head, all in the slow set the PR gate skips, and
three of them came from the declare preimage holding the definition.
Rulings at adjudication:

- The declare preimage is the variable's kind only. A value, a law or
  a display unit is not identity, so two documents differing only in a
  nominal keep one id chain.
- Variables are ordered by declaration (`Doc.var_order`, saved and
  checked at load), never by id. An id is digest output and gives an
  order no author can see.
- Load refuses an unnamed variable until PR 3's walk 6 relaxes it to
  "anonymous ⇒ read".

The fix pass was large and run by the implementer lane. It also
re-blessed the sym_9 bracket's pins (154/160): the tier's reach depends
on symbol order, filed as `work/sym/sym-tier-reach-depends-on-symbol-order.md`.
The row is in `docs/DUAL-REVIEW-LOG.md`. Next is PR 3: readers read ids.
## 2026-10-04 — open PRs triaged against D10

At Ev's request a lane read every open PR against D10. Closed as fully
superseded: #3929 (`[ev]`, a measured part stays a product root: the
consume/read typing is moot once nothing consumes). Its row
`a-measured-part-is-not-a-product-root` moved here from RECIPE, parked on
the build (REACH's plate row still blocks on it by id); its one unfiled
finding is `a-failed-requirement-refuses-the-whole-product`. Partly
superseded and left open: FUSE's #3955 (note on FUSE's log). 22 others
are not superseded.

## 2026-10-04 — VARS PR 3 merged (PR 4027): readers read ids

The implementer ran in its own cloud session, and the PR went to a dual
review at frozen head `b07891634f`. Each reviewer ran in its own cloud
session from one brief, identical modulo the lane label (sha256 prefixes
`fe4eec8319d1d0a0` for r1 and `78488713d2fd5f35` for r2). Both returned
APPROVE-WITH-FIXES with no MAJOR, so the tally is 0.

The union and the rulings made at adjudication:

- A variable is spoken from the document wherever one is at hand,
  including a refusal re-spoken after a rename and an analysis box taken
  before one. Memoized pattern refusals hold their formula as an `Expr`;
  the `#hex` text scan is gone.
- An unnamed variable has one spelling, `#<16 hex>`.
- Split or inline across an unresolved reader refuses typed
  (`UnresolvedVarCrossesCut`).
- The door checks a post-condition: no stored node holds a `Name` leaf
  (`NameLeafWritten`).
- The id-free corpus digest is a committed guard.
- One expression walker and one lowering rule.
- Every surviving mutant has a row.

The viewer's "param" vocabulary keeps its names until it is moved in one
piece (`viewer-param-vocabulary-names-a-variable`). Two more items were
filed: the Python classes, and `range.rs`'s synthetic name. An
independent verifier session checked the fix pass: 8 of 9 mutants red and every probe held; the one survivor, the door post-condition, got its row in a second pass.

`variables-replace-the-parameter-table` is closed. The spec is in
`docs/doc-ledger/intent-vars-1-spec.md`. `no-dimensioned-literal-in-a-slot`
and `parameters-defined-by-formulas` are unparked; the literal retirement
is next.

## 2026-10-05 — the literal retirement's spec (`docs/INTENT-LITERALS-SPEC.md`)

A spec writer read the merged code and sized every literal site. VR4
cannot land before a slot can hold an anonymous *defined* variable, so
`parameters-defined-by-formulas` merges into this unit as its first PR.
The unit lands in four PRs, each green:

- A: definitions.
- B: authored `Formula` and stored `Expr`, with byte-identical goldens.
- C: a slot holds a `VarId`.
- D: `Expr` holds no float.

All nine of the spec's recommendations were accepted (§11). Two were
written into VARIABLES-DESIGN:

- A lone number at a slot's root is a typed value (VR6).
- A `Measure`'s arithmetic stays a formula in the node until stage 2
  (VR4).

Both elaborate D10 and change no ratified text. The spec found that C
makes separately typed equal radii lose their declared evidence; a
fixture that relied on it shares one variable instead. Ev has said to
proceed through the plan without asking.

## 2026-10-05 — INTENT-LITERALS PR A, definitions (`intent/literals-a`)

A variable may be defined by an `Expr` over other variables
(`VarDef::Defined`), authored as `VarDecl` and lowered at the edit door.
The doors refuse a definition cycle, a read the document does not
answer, and an expansion past `DEFINITION_NODE_BOUND` (4096 nodes); the
load door refuses the same in two new walks. The environments bind
definitions over their inputs in definition order, so a defined
variable carries its inputs' enclosure, tangent or symbol; the seed door
refuses one. Coincidence tokens and content keys expand definitions,
`DocDiff::vars` closes over them, and the anonymous lifecycle cascades
through them. The viewer's parameter text door defines a variable, and
retires `Refusal::ParamNotANumber`.

Filed: `viewer-value-doors-read-a-defined-variable-as-absent`.

## 2026-10-05 — INTENT-LITERALS PR B, authored Formula and stored Expr (`intent/literals-b`, PR 4072)

One tree, `ExprTree<L>`, serves two forms: the stored `Expr` reads
every variable by id, and the authored `Formula` may also write a name.
Nodes, programs, placements and measures are generic over the slot
form; a `DocEdit` carries the authored form, and every door lowers it
before anything else reads it, refusing an unlowerable name in the
existing vocabulary. The type now carries what `NameLeafWritten` and
load walk 4 (`NamedReaderInSnapshot`, with its definition twin)
checked, so all three are deleted. `parse_expr` is `parse_formula`.
Python's `Expr` class is `Formula`, a stored definition reads back as a
read-only `Expr`, and `ParamName`, `DocParam` and `DocParamValue` are
`VarName`, `FreeVar` and `FreeValue`. Goldens, corpus files and digests
are byte-identical.

Ruled by the lane: an edit refused for two faults at once may now name
its unlowerable name first (InsertNode, SetProgram, SetOffset; SetParam
and SetExpression keep their order); a name read at another kind is
`var_kind_mismatch` in Python rather than `unlowered_name`;
`GeomPred.datum_distance`'s comparand stays a stored `Expr`, so a
written name refuses where the predicate is built; the error tag words
(`unknown_param`, …) keep their spelling.

Closed: `python-param-classes-name-a-variable`.
### PR A review pass (both reviewers APPROVE-WITH-FIXES, no MAJOR)

- The viewer's text door folds constant text, continuous or count, to a
  value (`SetVarValue`), so a toleranced parameter keeps its tolerance.
  Only text that reads a variable defines one. A value typed over a
  definition frees the variable and writes through the value door in one
  action, so `3` over a length refuses `VarValueKindMismatch` whether the
  variable is free or defined.
- A defined parameter's row has the free row's field, showing its
  formula. A formula typed into it redefines the variable; a number
  frees it.
- The order memo is gone. `definition_order` is Kahn's algorithm over
  one adjacency, and the other definition walks read the same index.
- The edit and load doors ask one `Doc::expansion_fault`, and the
  definition sentences render once for both.
- `Doc.definitions` lists the named defined variables in Python.
- Each mutant the review found surviving has a row, and there are MC and
  symbolic-tier rows.

Filed: `definition-node-bound-is-re-measured-against-the-corpus-after-d`.

## 2026-10-06 — INTENT-LITERALS PR C, a slot holds a VarId (`intent/literals-c`)

A stored slot holds the id of the variable it reads. The edit door
lowers each authored slot in one walk that returns every fault at its
address: a lone variable, name or fresh entry is itself, a value mints
an anonymous free variable, and a formula mints an anonymous defined
one. `Node::authored` re-authors a node by reading its variables, and
`Node::written` gives the formulas the slots were written as. Equal
typed values are two variables, so their tokens and content keys
differ; sharing is said by reading one variable.

Ev revised Q3 and Q4 mid-PR (spec §11, VARIABLES-DESIGN VR8): only a
toleranced variable is an analysis axis. An untoleranced one is a
constant in every lane, and Sym binds it as its exact nominal. Measured
against B's head: the interval suite's slowest five are unchanged
(427/209/115/95/87 s against 422/213/114/90/87 s), the stackups on
`tolerance`, `mcplate` and `chaintol` list the same three rows, and the
Monte Carlo sheet is bit-identical. Sym theorems rise by one per written
formula (`work/sym/symbolic-reach-at-slot-variables-c.md`).

Ruled by the lane:
- An anonymous variable's mint reads what it holds, so sibling inserts
  that differ only in a typed value still mint different nodes.
- An anonymous definition's refusal is reported as the slot's own
  (`VarEnv::written`), not as a refusal of `#id`.
- Names refuse before dimensions, B's order.
- The path editor loads and compares the program as written, so a value
  moved since the load refuses `ProfileEditStale`.
- Retiring anonymous variables is no loss the person is shown.

Closed: `range-synthetic-name-mints-a-name`,
`symbolic-reach-at-slot-variables-c`. Filed:
`unproven-coincidence-lint-binds-every-variable-as-a-symbol`.
- 2026-10-06 — Note from ZIP: filed `the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms` (P0/H) on this slate as stage-4 input, on Ev's direction in chat. Two designers converged on retiring the declared-REST zip (`boolean/rest.rs`) with declared pairs. Before then the join gains a partner-edge chord, ring re-homing on a curved chart in aligned contact, and the `mekr` `NotSameFace` cause. The row carries the measurement. ZIP's REST-lane rows are parked on `d10-one-way-to-say-intent-is-unbuilt`. Units ZIP already started finish: the zip's admission check (PR 4127, a live wrong body; Ev, in chat, "finish it"), `Fusions` (PR 4116) and pins (PR 4130). (ZIP orchestrator)

## 2026-10-07 — the GUI's variables (`intent/gui-variables`, PR 4247)

`typing-a-value-mints-or-offers-a-variable`, with
`viewer-param-vocabulary-names-a-variable` and
`viewer-value-doors-read-a-defined-variable-as-absent` (both closed in
the PR). Nine GUI choices are built provisionally and put to Ev on the
PR (`needs_ev`). The evidence behind it:

### Rows (each goes red without its guard)

These are mutation-checked: each guard was removed in turn and the named row went red.

| Guard | Row |
|---|---|
| typing mints (`slot_typed_edit` at `set_slot`) | `panel_edits::a_literal_slot_edit_routes_through_setparam…`, `gui_variables::a_value_typed_at_a_shared_slot_makes_it_its_own` |
| kind filter in `equal_variables` | `gui_variables::a_typed_value_is_offered_the_variables_of_equal_value_and_kind` (+2) |
| offer stands only while the slot reads the typed variable | `gui_variables::an_offer_stands_only_while_the_slot_reads_what_was_typed` |
| typed text opens the offer | `gui_variables::typed_text_is_offered_and_a_formula_is_not` |
| decline closes the offer | `gui_variables::declining_keeps_the_typed_variable_and_moves_no_document` |
| a name is stored only on commit | `gui_variables::a_name_is_stored_only_when_committed` (was `a_proposed_name_is_stored_only_when_committed`; see Ev's ruling below) |
| probe refuses a defined variable as defined | `gui_variables::the_range_probe_says_a_defined_variable_is_defined` |
| exists-notice reads a defined variable | `gui_variables::the_exists_notice_reads_a_defined_variable_as_holding_its_name` |

Panel rows (the `app` feature suite, the real pane, with clicks through to the session):
- `properties_pane_tests::the_offer_is_drawn_and_its_button_makes_the_slot_read_the_variable`
- `properties_pane_tests::keep_separate_declines_the_offer_and_moves_no_document`
- `properties_pane_tests::the_naming_field_opens_empty_and_stores_only_on_commit` (was `a_name_is_proposed_in_the_pane_and_stored_on_commit`)

Accepting is also covered by `gui_variables::accepting_an_offer_makes_the_slot_read_the_variable`.

### Sweep

Pattern: every viewer identifier in the item's list and its siblings, i.e. `(Selection|ValueGestureName|Standing|BoundsTarget|Self|Subject|GestureTarget)::Param`, `SetParam(Unit|Text)?` under `SessionOp::`/`Self::`, `*ParamGesture`, `NoSuchParam`, `ParamRow`, `param_rows`, `param_(edit|unit_edit|showing|doors|unit_ui|bounds_ui)`, `add_param_ui`, `new_param_*`, `doc_param`, `set_param*` and `begin_param_gesture`, over `crates/viewer`, `demos`, `docs` and `work`.

- **Hits fixed:** every one in `crates/viewer` (src, tests, examples, README) and in 12 open items under `work/`.
- **Hits left:** `work/*/log.md`, closed items, `docs/MODEL-AB-LOG.md` and `docs/DUAL-REVIEW-LOG.md`. These are history, not citations to keep live.
- **Blind spot:** prose that says "parameter" for a document variable without naming an identifier.
  - I swept that with a phrase pass ("document parameter", "add-parameter", "parameter row", "(un)declared parameter"), then a word pass in the variable-centred viewer files.
  - The word pass excluded type parameters, function parameters and curve or ray parameters, and I reviewed every converted line. Four test files where "parameter" means a ray or curve parameter were restored whole.
  - What is left is "parameter" meaning a feature's slot, a function argument or a curve parameter.

### Runs (local, CARGO_TARGET_DIR outside the worktree)

| Run | Result |
|---|---|
| workspace `ci` profile, default ε | 12377 passed, 604 skipped, 0 failed |
| ε 1e-6, workspace `ci` profile | 12377 passed, 604 skipped, 0 failed |
| ε 1e-6, workspace slow set | 259 passed, **1 failed**: `editor-core name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time` (12.9 s for 1000 names). It **fails identically on bare main** (merge base `7c506747a`, same seed `0x5ae437797e2c1b75`, run alone: 13.6 s). This PR touches no editor-core code |
| ε 1e-12, whole workspace, default profile (slow set included) | 12636 passed, 344 skipped, **1 failed**: the same row (14.2 s). It **fails identically on bare main** at 1e-12 (12.8 s, run alone) |
| `cargo nextest run -p viewer --features app` | 1183 passed, 1 skipped, 2 failed: `gpu::tests::every_pass_builds_on_a_real_device` and `the_culled_passes_draw_every_face_that_faces_the_eye`, "NO WGPU ADAPTER" (this box has no Vulkan ICD; hosted CI has one) |
| doctests (`cargo test --doc --workspace`) | 172 passed, 6 ignored, 0 failed |
| Python (wheel built with maturin, `PNCAD_TY` set) | Ran 944 tests, OK |
| `cargo clippy --workspace --all-targets --all-features -D warnings` | clean |
| `cargo fmt --all --check` (+ benches) | clean |
| `scripts/gates/*.sh` + `payload-rung-sweep --check` | all pass |
| tour: fmt, clippy `-D warnings`, `nextest --release` | clean, clean, 97 passed |
| wild: fmt, clippy `-D warnings` | clean, clean |
| `work.py lint` | ok (0 problems, 0 warnings) |
| editor-core `--profile default` | not run: the diff does not touch editor-core |
| ruff (`check-python-lint.py`) | skipped locally (ruff 0.16.8 here, CI pins 0.16.1); no Python file changed |

### The review's fix pass

The single review (APPROVE-WITH-FIXES, `review/intent-gui-variables`)
found the offer outliving a drag, a redo reviving it, a naming field
that named whatever the slot read at commit, a non-bit equality, and a
mutant the formula row could not kill. The fixes change none of the
nine choices: each makes the code do what the choice already says.
New rows, each mutation-checked red without its guard:

| Guard | Row |
|---|---|
| the offer stands only while the slot reads the minted variable at the bits typed (`SlotOffer::stands`) | `gui_variables::a_drag_of_the_typed_slot_closes_its_offer`, `gui_variables::moving_the_typed_variable_closes_its_offer` |
| `perform` closes an offer for good once it stops standing | `gui_variables::a_redo_does_not_revive_an_offer`, `gui_variables::a_value_moved_back_is_not_offered_again` |
| an edit elsewhere leaves the offer (choice 2 rejects closing it on any later edit) | `gui_variables::an_edit_elsewhere_leaves_the_offer_standing` (guards the choice; no mutant: it pins that the fix is not the history-state key a first cut used) |
| the `is_typed_value` filter in `offer_after` (the review's surviving M7) | `gui_variables::typed_text_is_offered_and_a_formula_is_not`, now with `b` holding an equal typed value |
| bit equality in `equal_variables` | `gui_variables::a_negative_zero_is_not_offered_for_a_typed_zero` |
| `SetSlotVariable` refuses `NotOffered` | `gui_variables::accepting_what_is_not_offered_is_refused` |
| the naming field closes when its slot stops reading the variable it was opened for (`NameDraft::var`) | `properties_pane_tests::a_naming_field_closes_when_its_slot_reads_another_variable` |
| a refused rename keeps the field and its text | `properties_pane_tests::a_refused_name_keeps_the_field_and_its_text` |

A first cut keyed the offer to the history state its typing recorded;
that closed it on an edit anywhere, which is choice 2's rejected
alternative, so the key is the typed value's bits instead. An
undo/redo-specific clear was dropped as redundant: a step that changes
what the slot reads closes the offer through the same check, and the
mutant removing it survived every row.

Runs on the fix pass: `cargo fmt --all --check` and `cargo clippy --workspace
--all-targets --all-features -D warnings` clean; `scripts/gates/*.sh`
and `payload-rung-sweep --check` pass; viewer default 910 passed;
viewer `--features app` 1194 passed with lavapipe installed, so the two
GPU rows ran and passed; editor-core `--profile ci` 2776 passed (one
row, `sym_9_the_drive_writes_the_ladders_receipt`, was cut by the
run's own wall-clock limit and passed alone in 249 s); `work.py lint`
ok. The editor-core change is two doc citations of the renamed
`Refusal::NoSuchVariable`.

### Ev's ruling on choice 7 (2026-10-08)

Ev, on PR 4247: "there's no need to recommend a name now; `distance_1`
as a naming scheme is uninspiring enough that i think it doesn't really
beat requiring the user to type something". The **name…** field now
opens empty, and `props::proposed_name` and its stepping are gone. The
pane row `the_naming_field_opens_empty_and_stores_only_on_commit` pins
that the field opens empty, that **Name** on an empty field names
nothing, and that typed text stores nothing until it is committed.
Choices 1–6, 8 and 9 stand as built. Ev's question about anonymous
variables is open on the PR, and nothing here changes for it.

Runs on this change: with current main merged in (a merge commit; the conflicts
were this branch's renames against main's `VarId::new(0, …)` and
`Doc::ids`/`var_ids`, resolved by keeping both), `cargo fmt --all
--check` and workspace clippy `--all-features -D warnings` clean;
`scripts/gates/*.sh` and `payload-rung-sweep --check` pass; viewer
default 910 passed; viewer `--features app` 1194 passed (GPU rows run
on lavapipe); `work.py lint` ok. The branch changes no Python or
`pncad` file, so the Python suite was not re-run.

## 2026-10-07 — INTENT-LITERALS PR D, Expr holds no float (`intent/literals-d`)

A stored expression's leaves are variable readers, exact rationals
(`Ratio`, reduced, numerator and denominator at most 2^53), integers
and `turn`; no float is left in a document's expressions. A written
quantity is the authored `Quantity` leaf: inside a formula it mints an
anonymous free variable of its own, in pre-order before the variable it
defines, so two typed `5 mm` in formulas are two variables and their
tokens differ. Tokens carry `T_RATIO`, `T_INTEGER` and `T_TURN`;
`T_LITERAL` and `T_COUNT_LITERAL` are retired bytes. The f64 geometry,
and the interval enclosures, of the whole corpus are byte-identical
with ids masked; the id-bearing pins moved on `kitchen_sink` alone.

Ruled at review (orchestrator, on the lane's spec-undecided rulings):
- A bare number inside a formula is the exact `Ratio` its decimal
  spells, and refuses `ConstantOutOfRange` where none in range does
  (`1e-20`, `0.30000000000000004`): never a hidden variable (VR6). One
  copy of the rule, `Ratio::from_decimal`. The whole text one such
  decimal is a value, its written double. `Formula::scalar` is always
  a written `Scalar` quantity (§1), so inside a definition it mints a
  variable; `Formula::ratio` is the constant.
- A count of integer constants beside an operand that is no count
  reads as the scalar it equals, whichever side it folds on
  (`turn/4`, `2*3*w`, `w*2*3`); a count reading a variable is promoted
  only by `scalar(n)`.
- `INT/INT` with no space is one ratio, except as the right operand of
  `/`, so `w/2/3` is `(w/2)/3`; `unparse` brackets a `p/q` divisor. A
  ratio's parts are integers (`2/3.5` refuses saying so).
- `turn` is reserved (§1).

Ruled by the lane:
- A declared definition mints its own id before its quantities, so a
  refusal speaks the id it is minted at.
- `Doc::unparse` writes an anonymous reader as what it holds; the bare
  `unparse` keeps `#<16 hex>`.
- `GeomPred::DatumDistance` holds a `Formula`, evaluated with no
  document: a name refuses `EvalError::Unlowered`.

Closed: `no-dimensioned-literal-in-a-slot`,
`equal-literals-lower-to-one-identity-token`,
`definition-node-bound-is-re-measured-against-the-corpus-after-d`
(4096 stands). Opened: `typing-a-value-mints-or-offers-a-variable`
(its trigger fired). Re-parked: `operations-define-output-variables`
(stage 2 A) on FORK-1's PR 4222, which still gates it once this
row's trigger fires.

## 2026-10-07 — stage 2 sliced (`docs/INTENT-STAGE2-SPEC.md`)

A spec lane sized stage 2 (operations and one dependency) at main
`9eaf8eab2f`, measured before stage 1's C and D. Stage 2 lands in six PRs,
each green, in this order:

- A `operations-define-output-variables`: no node id moves.
- B `operands-are-reads`: behaviour-preserving, with byte-equal roots.
- C `the-product-is-an-explicit-list`: closes both parked product rows.
- D `measure-is-an-operation`.
- E `select-defines-face-and-edge-variables`.
- F `a-mate-reads-face-variables`: A12 retires.

C precedes D, E and F because a measure or a mate that reads its operand
would otherwise drop it from A10's sink set.

Five FORKs are open for designer pairs:

1. The output signature: kinds D10 does not list, and DM3.
2. How the product list is kept: A10's maintenance.
3. What a selection is: SELECT-DESIGN §4.
4. Re-pointing an operand: DM6.
5. Whether a geometric slot may read a measured value.

The six rows are filed parked behind stage 1, so they do not count toward
the load. At stage 1's close the stage's ~25 points either fit the budget
or split into their own program.
## 2026-10-07 — FORK-6: split moves a variable with its readers (orchestrator's ruling)

The inline lane (`inline-merges-variables-by-equal-value`) stopped at a
case the ruling did not decide: split copied a named variable the cut
alone reads into the part and left an unread twin in the remainder, so
under "never merge by value" inline-of-split always refused and A4's
acceptance broke. One Opus and one Fable designer weighed it (fork-log
row 84; reports on `design/intent-s2-fork6-A` and `-B`) and converged
after one round. Ruled: a variable's side is the union of its readers'
sides (node slots and remaining definitions); all cut, or no readers
and every read moving, it moves (`DeleteVar` in the remainder,
declared in the part under its name); readers or reads on both sides
refuse `UncutVarReference`; otherwise it stays (VR7). The move is not
a VR7 deletion: nothing is stranded and the name exists once before
and once after. The promise is inline(split(d)) = d up to minted ids
and A10's regrouping; split(inline(h)) keeps an unread free named part
variable in the host. Not sent to Ev: no ratified decision moves. A4's
acceptance is kept and re-worded as VR1 forces ("minted ids"), and A4
Split and VR9 each gain a descriptive line, landing with the lane's PR.

## 2026-10-07 — stage 2 spec updated to the FORK-2b, FORK-4 and FORK-5 rulings (PR 4216)

- **C is now "the product is the world"** (FORK-2b, #4220). It lands `PlaceInWorld { body, pose }` and a derived product. The audit's C-side retirements ride C rather than B, because each is stated over world placements: D-2's narrowed closure, `InstanceConsumed`, `PlacedUnderTwoRoots` and N4. The product is checked once, at the migration.
- **B builds the one slot door for operands** (FORK-4, #4221).
- **D refuses a construction reading an observed variable** (FORK-5, #4218).
- **F retires A5's minting lift.**
- **A and E stay pending FORK-1 and FORK-3** (#4222).
- The consuming-model audit's 14 hits are mapped in spec §11.

## 2026-10-08 — FORK-1, FORK-1b and FORK-3 ruled (PR 4222)

- **FORK-1:** approved as written (typed output ports; `Body`, `Bodies`, `Profile`; a split two bodies; an instance one `Body` per world placement).
- **FORK-1b** (Ev's lattice comment; designer pair, three rounds, row 86): approved. Poses are frames up to their kind's symmetry, D10's five kinds, incidences constructed, a 2-D value lives in its node; the revolve defines `body` and `axis`; the tube reads a `Frame` from unit B (`tube-spine-reads-an-axis-origin`). Ev added that the mates' `Subgroup` should be shared: one type is a pose's symmetry and what a mate folds.
- **FORK-3:** sets. Ev clarified that "the role played by edges is replaced by sharing variables" meant dependency-graph edges.
- Unit A is unblocked (stage 1 finished with PR D, #4277).

## 2026-10-08 — unit A: outputs are logged, so ids move (orchestrator's ruling)

The unit A lane found that spec §2's two asks could not both hold: an output logged `Minted::Var` and no node id moving. An id's ordinal is the mint log's length plus one, and an insert's preimage holds its slot variables' ids, so one logged output moves every later id, digests included. Ruled: log outputs as ordinary variables, ordered as minted, as VR1 and N1 already say, and let ids move in A. Q6 was a spec recommendation, not ratified text. The alternative, keeping outputs out of the ordinal count, would change N1's log invariant and VR1's order for a checkpoint B discards anyway. A's check is now equality up to an id bijection, with content keys and geometry digests bit-equal. Two smaller calls were also accepted. An instance defines one `body` in A, and its per-world-placement signature is C's. A transform's port is `Bodies` over a `Bodies` operand and `Body` otherwise.

## 2026-10-08 — unit B: operands are reads (branch `intent/s2-b-reads`)

Every operand field holds a `VarId` read of an output, typed by the kind its slot admits; `Node::inputs` is gone and `Doc::upstream` is the one dependency relation the schedule, the content key, acyclicity, roots, split closure, the cascade and the viewer tree read. The tube reads one `frame` (`tube-spine-reads-an-axis-origin` closed). The orchestrator's rulings on the lane's questions:

- **Q1, the slot door: one door, the slot typed by kind.** DM6 says an operand slot is written by the one slot door every slot has, so no `SetOperand` beside `SetParam`. `SlotId` gets a total `kind()`, `dimension()` becomes partial (scalar slots only), and `SetParam`'s payload widens to a scalar formula or a read (the `Operand` shape), with `From<Formula>`. `SetMembers` is the list spelling of the same door through the same write path. `SetStructuralParam` stays.
- **C1, a part over a split: accepted.** `Part { of: split, SplitHalf(h) }` reads the half its selector names.
- **C2, one wrong-kind refusal.** No `OperandVarKind`: `SlotVarKind` (and its `SnapshotError` twin) widens to carry kinds, its scalar wording kept.
- **C3: accepted.** `Maintenance::StrandedRead { node, slot, var }`; a re-point that takes a declared name out of its carrier's reach reports `Strand { took: Took::Reach }`. Each has a row (`intent_s2_b_reads`, `dm7_delete_strands`).
- **Q2, a read of a split's port is that half.** The port is typed `Body` (FORK-1), so evaluation projects it as `Part { SplitHalf }` does, bit-equal bodies and names; `Part { SplitHalf }` stays in B, its retirement filed (`part-split-half-retires`), because retiring it moves roots.
- **Q1/C2 as built** (the orchestrator accepted the shape): `SlotId::Operand(OperandSlot)`; `SlotId::kind() -> SlotKind { Is(VarKind), Placeable, Measured }`, total, and `dimension() -> Option<Dimension>`, with `expr_dimension()` for an expression slot; `SetParam { node, slot, value: SlotValue, fresh }` with `SlotValue::{Formula, Read}`, the wire field `expr` → `value`; one `SlotVarKind { var, node, slot, found: VarKind, expected: SlotKind }` at both doors. `UnknownOperand` folds into `UnknownSlot`, and `SlotDimensionMismatch.expected` is a `SlotKind`, so an expression at an operand refuses there. A read at a scalar slot is the formula of the one variable it resolves to, so every formula rule holds of it (an output is still `VarIsAnOutput` until unit D). The three read refusals (`OperandUnresolved`, `AmbiguousOutput`, `DefinesNothing`) carry a `SlotId`, so one resolver serves both families. Python: `DocEdit.set_param(node, word, value)` takes a `NodeId` or `Var` at an operand word, the operands' field words join the slot alphabet (`section`/`member` have no reading: their position is `set_members`'), the program slot's word is `program` (freeing `profile` for the operand), and `slot_var_kind`'s `expected`/`found` now read the slot's kind and the variable's.

Lane calls the rulings left standing:

- **A pre-B file** spells operands as node ids, which parse as variable ids and refuse `SnapshotError::OperandUnminted` with the regenerate recourse rather than `Unreadable`: the bytes are well-formed, so only the read walk can tell.
- **DM5 is over the operations read**: two outputs of one operation are that node reached twice (a boolean over both halves of one split refuses `DuplicateInput`, as `Boolean { a: X, b: X }` did).
- **`DeclaredNameNotUpstream`** asks the read relation: a name is out of a carrier's reach when the carrier or a node downstream of it minted it, or a node inserted after it that it does not read.
- The viewer's body and instances seats read the kind of the variable a node named alone defines (`Doc::read_of_node`), closing `body-seat-reads-through-the-placer-chain`.

## 2026-10-08 — unit C: the product is the world (branch `intent/s2-c-world`)

`Node::PlaceInWorld { body, pose }` reads one `Body` and defines one `Body` output, `copy`; the product is the copies in the placements' document order, and nothing places as a side effect. `roots.rs`, `Doc::roots`, `SetRoots`, `RootFault`, A10's maintenance (`on_insert`, `on_delete`, `on_set_members`) and `PlacedUnderTwoRoots`/N4's once-per-product rule are deleted. `DocEdit::place(body, pose)` is the authoring door.

- **Copy names** are the body's names under a new segment `RoleSeg::Placed { of }` minted at the placement (content tag 51), as a pattern copy's are qualified by the copy; `StableName::in_copy` / `copy_of` are the pair. Accepted by the orchestrator. Selections and `Rebind` resolve at their minting node's own table; only a mate head lifts to the product table (`assembly::resolve_face`), and no committed document stores a `Placed` name.
- **Spec reworded to what C builds** (orchestrator): §4's Instances sentence (one `body` output, the part's product as one body; placement semantics are stage 3's) and test 6 with the migration note (every migrated placement is at the identity; a root instance keeps its offset).
- **Duplicate adds no `Transform`** (orchestrator, `Transform` retires in #4326): a pattern of two, its second instance projected and placed, so the copy is a body of its own; a second placement of the same body waits on `a-feature-gesture-re-points-every-copy-of-its-target`.
- **Errors.** `NoBodyRoots` is `EmptyProduct { unplaced }`; a placement whose read is unresolved is `StrandedPlacement { placement }`.
- **A mate whose member no placement reads mints nothing at the at-rest gate**: F's Q8 rule landing early (orchestrator accepted). `mate6_gather_mints::the_gate_skips_a_mate_on_an_unplaced_member_and_mints_the_placed_ones` goes red if the skip swallows a mate whose members are placed.
- **Instances** keep one `body` port (the part's whole world); FORK-1's per-placement signature is filed as `an-instance-defines-one-body-per-part-placement`, its open choices put to the orchestrator.
- **A copy carries its body's contact records and declaration rows** verbatim: a rigid placement keeps every arena key.
- **Which cut placement carries a remainder name** (split's rebind): the placed body's own names, those a transform below it carried whole, and a pattern copy's where a `Part(Instance(k))` pick's index matches the name's outermost `Instance` index; any other name refuses `NameOutsidePartWorld` rather than re-anchoring silently. Matching the pick's `k` by evaluating it at the document's values is a positional pick: interim, retiring with `[ev]` #4341's `Member` keys (orchestrator).
- **Split.** The part's world is the cut's placements; the remainder places the instance once, after the kept placements (no splice). D-2's consumer-ward closure narrows: a remainder read of a body the cut places re-points to the instance's body; any other read of a cut body is `SeveredEdge`. A name the part's world does not carry is `SplitError::NameOutsidePartWorld`.
- **Inline.** `InstanceConsumed` retires: a reader of the instance re-points to the inlined body (`InstanceReadUncarried` when nothing carries it); a posed host placement reads through the part's single identity placement, else `PlacementPoseCrosses`. `UnplaceableFrame` names the part's placement.
- **Pre-C files** refuse `Unreadable` naming `roots`. The corpus builders place their pre-C roots in root order, and `intent_s2_c_world` test 6 checks each regenerated product against its recorded pre-C digest.
- **Closed here:** `a-measured-part-is-not-a-product-root` (test 7; the tour's cut plate is its product), `a-failed-requirement-refuses-the-whole-product` (test 8) and WIRE's `one-shape-placed-n-times-has-no-product` (the tour's chain places nine copies).
- **No slot reads a world copy** (review MAJOR-1; the orchestrator reversed its ruling that the copy stays readable): `EditError::ReadsWorldCopy` at the door and `SnapshotError::ReadsWorldCopy` at load, recourse "read the body the placement reads" (D10: construction never reads the world). Split's closure no longer re-points reads of a cut placement's copy; inline refuses a reader whose heir would be a posed part placement's copy (`Uncarried::Posed`). The gather, export, checks and the mate-head lift read copies internally, never through a slot.
- **The posed path has its row** (review MAJOR-2): `intent_s2_c_world::a_posed_placement_moves_its_copy_and_its_records_rigidly` kills the pose-blind and the record-dropping mutants. `Doc::unplaced` lists `Bodies` outputs too, and `EmptyProduct`'s recourse names the `Part` pick.
- **Second review** (sequential arm, after the first fix pass): split and inline share one re-point rule (`world_heir`; `SplitError::RemainderReadUncarried` where inline refuses `Uncarried::Posed`/`Bodies`); inline re-points a later identity host placement of the instance instead of dropping it; no measure is sited at a world placement (`MeasuresWorldCopy`); the door and the load ask one read predicate (`edit::read_fault`). The viewer's Transform gesture stays as built, a known interim gap (orchestrator): `the-transform-gesture-moves-a-placed-body-through-a-transform` (CHROME). An instance's body still passes its part's posed world to host constructions, stage 3's to close (`a-placement-is-the-bundle-of-mates`).
- **Filed:** `inline-refuses-a-posed-instance-a-placement-pose-could-take` (INTENT; closed, superseded by stage 3), `a-feature-gesture-re-points-every-copy-of-its-target` (CHROME), `a-split-or-pattern-gesture-leaves-its-target-placed` (CHROME; closed on the orchestrator's ruling that the literal reading stands), `the-world-still-speaks-of-roots` (RECIPE), `python-cannot-read-what-a-placement-places` (BIND)., and on review `a-world-gauge-instance-offset-did-not-move-into-its-placement` (INTENT, Q9 disclosed and not built), `deleting-a-placed-feature-unplaces-its-target` (CHROME), `a-transform-drops-the-contact-records-a-placement-carries` (WIRE), and on the second review `a-split-name-picks-a-pattern-copy-by-its-evaluated-index` (INTENT) and `the-transform-gesture-moves-a-placed-body-through-a-transform` (CHROME).

## 2026-10-08 — stage 5 sliced (`docs/INTENT-STAGE5-SPEC.md`)

A spec lane sized stage 5 (assertions and the at-rest lints) at main `047d10d5d`, written against stage 2's final shapes. Stage 5 lands in three PRs, each green:

- A `an-assertion-relates-by-equality` (M): `AssertionRelation` with `=`.
- B `interference-at-rest-is-a-finding` (H): interference between copies leaves A5's refusal, with the quieting rule's interference half.
- C `the-at-rest-census-is-a-check` (H): A5's gate becomes a check resident with the rule's contact half, and `Separation` retires into it.

A and B wait only on stage 2 (A on D; B on C, D and E). They can be built in parallel, before stages 3 and 4. C waits on A, B and stages 3 and 4.

Five FORKs are open for designer pairs:

1. What "the same measure at the same site" is.
2. The site of an interference finding.
3. What "does not straddle zero" admits. This one re-words D10.
4. The shape of the census as a check, and where it refuses. This one re-words DS6's waiver paragraph.
5. Whether `Separation` retires. This one re-words DS6.

The three rows are filed parked.
## 2026-10-08 — the parked rows re-pointed at the stage that releases them

99 rows outside `work/intent/` were parked on `d10-one-way-to-say-intent-is-unbuilt`, which closes only when all six stages are built. Each was read with the code it cites (five reader lanes, one per group of programs) and re-pointed at the earliest thing that releases it. Two umbrellas are filed, `intent-stage3-is-built` and `intent-stage4-is-built`; each closes when its stage's units merge, and the stage specs point their units at it. No row waits on stage 5 or 6, so none is filed for them, and no row needs the whole program, so none keeps the D10 umbrella.

**Counts.** `measure-is-an-operation` 1; `select-defines-face-and-edge-variables` 1; `a-mate-reads-face-variables` 2; `stage3` 13; `stage4` 67; `none` 15 (workable now). Total 99.

**Workable now (15).** Opened, with the reason in a dated section of each row. Five look already done and want a measure-and-close, not a build: `a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync`, `reflex-corner-edge-in-face-poses-zip-a-ring-parallel-to-its-section-loop`, `a-subtract-through-a-pinch-line-drops-the-pinch-row-at-its-cut`, `gauge-of-recomputes-the-clusters-per-placement-lookup`, `declared-flush-intersect-refuses-in-one-operand-order`. `tangent-lever-row-escalates-containment-at-eps-1e-6` is red on main at ε = 1e-6. MSOLVE and ZIP go from `blocked` to `ready` because each now has a dispatchable row.

**Calls overruled or worth a second look.** The readers called `the-rest-lane-reads-a-nested-struts-site-at-its-holders-tip` and `a-sharp-plate-offset-over-a-rounded-one-refuses-unpaired-loose-ends` workable. The orchestrator moved both to stage 4: the first's site is `rest::enumerate_segments`, inside the declared-REST zip stage 4 deletes, and the second reaches the chord join only after that zip declines. Borderline calls that stood: `topo-mints-indeterminates-outside-the-funnel` (stage 4) has steps 4–5 that could split off as workable; `a-boxed-rotation-refuses-not-rigid-at-every-placer` (stage 3) releases only its placement half, since a boxed `Pattern` angle still refuses after it; and the three MSOLVE-15 rows (rider, roll, unit variants) are stage 3 because the plan re-cuts MSOLVE-15 against D10's frame ground.

| item | old block | new block | reason |
|---|---|---|---|
| `recipe/a-measure-merges-free-instances-into-one-relative-freedom-component` | `d10-one-way-to-say-intent-is-unbuilt` | `measure-is-an-operation` | relative_freedom_components walks Node::inputs(); unit D makes a Measure an operation defining an observed scalar only assertions read, which removes the edge |
| `recipe/a-reshapings-values-strand-what-a-value-edit-would-not` | `d10-one-way-to-say-intent-is-unbuilt` | `select-defines-face-and-edge-variables` | it is the strand report on names that kept readers hold (edit.rs undrawn_kept_pieces); unit E moves names into Select-defined Face/Edge variables that own N5, so where strands are reported moves there |
| `msolve/a-placer-row-states-what-a-poisoned-row-cannot` | `d10-one-way-to-say-intent-is-unbuilt` | `a-mate-reads-face-variables` | the raise site is member.rs check_reference's head walk through Pattern counts and Part indices, replaced by a typed unresolved read once mate sides read Face variables |
| `msolve/split-and-inline-over-a-mate-read-at-a-union-are-unmeasured` | `d10-one-way-to-say-intent-is-unbuilt` | `a-mate-reads-face-variables` | refactor.rs reads member_of through the head walk that descends unions; unit F retires that walk (A12's reading edges), and names this row as its own |
| `msolve/a-box-independent-mate-fault-bisects-the-whole-leaf-budget` | `d10-one-way-to-say-intent-is-unbuilt`, `the-box-driver-carries-no-part-resolver` | `intent-stage3-is-built`, `the-box-driver-carries-no-part-resolver` | the fix is a terminal class in drive.rs classify_replay for MateFault arms (Unpinned, dangling head, unresolved part), a vocabulary stage 3 rewrites (placement as a bundle of mates, subgroup overconstraint); keeps the-box-driver-carries-no-part-resolver |
| `msolve/a-box-over-a-solved-clocking-widens-thirty-thousandfold` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | the widening comes from member_of re-measuring a residual met by construction; stage 3's "a mate places and never checks", overconstraint by subgroup algebra without measuring, retires that re-measure |
| `msolve/a-clocking-rider-is-levered-unreduced` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | the fix site (Lever::Roll, the coincidence rider) is deleted by #3681's MSOLVE-15, which is re-cut against D10's frame and mate ground at stage 3 |
| `msolve/a-declaring-mates-alignment-is-never-read` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | places compares gauge refs; declaring mates exist only between gauges, and A11 (2) gauges and A11 (4) declaring mates retire at stage 3 |
| `msolve/a-face-base-puts-its-reference-on-local-y` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | what a frame read off a face means (a Frame against a Plane pose that forgets in-plane motion) is stage 3's pose-kind work, which re-specifies face_base and point_at_frame |
| `msolve/a-face-frame-cannot-turn-its-roll` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | the kernel half landed; what remains is MSOLVE-15 (rider, Clocking and standoff deletion over MateFrame offsets), re-cut where mate frames become Frame variables at stage 3 |
| `msolve/a-far-meeting-point-fails-membership-by-its-own-rounding` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | the false Contradictory comes from the membership re-measure (mate_member_translation_in_plane), which stage 3's measure-free overconstraint retires |
| `msolve/a-mate-frame-axis-is-decided-against-a-length-band` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | compose_offset re-mints base∘offset; stage 3 makes frames and directions variable kinds with their own witness, the "witness that survives composition" this row asks for |
| `msolve/a-mate-frame-is-written-in-the-reading-instances-coordinates` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | what a mate's literal frame means is ASSEMBLY A3, which D10 retires; spaces, "a part has no location" and the per-space computing frame are stage 3 |
| `msolve/an-identically-zero-margin-escalates-at-a-fine-eps` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | the escalation is member_of and check_offsets re-measuring zero-by-construction residuals; stage 3 decides overconstraint by subgroup algebra and closes checked offsets as assertions |
| `msolve/mate-primitive-unit-variants-load-from-a-null-payload` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | it dissolves when MSOLVE-15 replaces Coaxial/Clocking with Coaxial { roll } (#3681), a build re-cut on stage 3's mate ground |
| `recipe/placement-step-slots-are-spelled-three-ways` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | two of the three spellings (step 0 of transform/gauge/offset, PlacementStep) belong to gauges, offsets and Transform-as-placement, which stage 3 retires; MateFrameStep is re-cut with frame variables |
| `topo/a-boxed-rotation-refuses-not-rigid-at-every-placer` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage3-is-built` | the fix site is boxed placement (transform_rigid on a Transform/instance/mate frame); stage 3 makes frames variables, placement a bundle of mates, and retires Transform-as-placement |
| `band/declared-joint-kind-zero-margin-reads-smooth` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | judge_joints/junction_reverses classify declared tangent_joints (validate.rs) into cusp and Tangent records; stored tangent-joint flags retire at stage 4 |
| `carvetail/half-revolve-caps-are-never-an-operand` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the F7 gate refuses UndeclaredCoincidence on value-decided coplanar caps; stage 4 turns that into Zero glue plus the lint, changing what the row asks for |
| `carvetail/loft-walls-keyed-per-segment-on-a-declared-carrier` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | a declared continuation and the declared merge rung, which stage 4 replaces with canonical carrier forms at the door |
| `cleave/a-plane-datum-through-a-named-edge` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | It exists to emit the split's on-plane declaration; under D10 a plane read from an Edge variable is a structural coincidence, decided at stage 4's door (the declaration retires) |
| `cleave/coincidence-intent-has-too-many-spellings` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the spellings it lists (FacePairDeclaration/DeclaredPair, split declarations, carried/ON-set records, FlushFinding) retire at stage 4's one recording door (D10 Coincidence) |
| `cleave/split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | its designed fix (SplitDeclarations, the undeclared pinch refusal) is replaced by D10: a split's ON verdict that makes pieces touch is recorded at the one door and linted |
| `cleave/split-refusal-detector-names-a-shared-parameter-coincidence` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | this is the unproven-coincidence lint's job (report with the edit that makes it one construction); the split refusal and declaration it hangs on retire at stage 4 |
| `cleave/the-rest-lane-reads-a-nested-struts-site-at-its-holders-tip` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | rest::enumerate_segments is the declared-REST zip, which stage 4 deletes (the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms); a witness needs a declared Rest contact |
| `cleave/topo-mints-indeterminates-outside-the-funnel` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | held step 2's payload sites (CarrierEqError::Undeclared, UndeclaredCoincidence) and contact_verify's declared-contact contradictions are what stage 4 retires; steps 4–5 could be split off as workable |
| `emit/a-declaration-on-a-face-one-fold-step-cut-and-partly-merged-cannot-be-routed-by-names` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | route_declarations/look_through_fold route declared pairs, which stage 4 retires (booleans glue on Zero); the question then has no subject |
| `emit/a-pair-boolean-names-a-declared-covered-pair-by-operand-order` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | naming a declared covered pair; declarations retire at stage 4, re-ask it as naming a Zero-glued covered pair |
| `emit/union-refuses-in-some-member-orders-and-publishes-in-others` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the class left is DeclareResolve/ConsumedByFold on routed declarations (wire.rs look_through_fold), removed when stage 4 retires declared pairs |
| `fuse/a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | StaleContactDeclaration comes from the declared-contact records remap_contacts carries; stage 4 retires the declared-contact seats and records found contacts at the one door |
| `fuse/a-carried-vertex-on-face-row-at-a-pinch-is-unprobed` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | asks whether carried declared Rest v-on-f rows survive remap_carried; those declared-contact rows retire at stage 4 |
| `fuse/a-kissing-convex-corner-result-ships-an-undeclared-vertex-on-face` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the tier-3′ UndeclaredContact{VertexOnFace} refusal becomes an unproven-coincidence finding recorded at the stage-4 door |
| `fuse/a-pinch-line-crossing-a-face-interior-drops-the-pinchs-records` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | a missing v-v contact record behind tier-3′ UndeclaredContact; stage 4 replaces the record/refusal pair with the recording door and a finding |
| `fuse/a-two-pinch-union-ships-a-pinch-its-records-do-not-declare` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | an unrecorded self-touch refused UndeclaredContact at tier 3′; at stage 4 that refusal becomes a finding and contacts are recorded at the one door |
| `fuse/a-vertex-on-face-row-follows-its-face-not-the-part-it-rests-on` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | a lineage bug in the declared v-on-f rows of remap_contacts (Stale/UndeclaredContact); stage 4 retires those records and refusals |
| `fuse/coincident-shell-has-no-fixture-for-its-unpaired-and-mixed-arms` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | shell_witness::on_verdict reads pairs settled by shared source or verified declaration (canonical-form equality at stage 4), and Unpaired is unreachable only because of UndeclaredCoincidence |
| `join/a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the volume is right; the fault is tier 3′ UndeclaredContact/StaleContactDeclaration on the contact records, which stage 4 replaces with one-door records and unproven-coincidence findings |
| `join/a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | only a declared VertexVertex contact or a coincidence read can link the keys; stage 4 retires the first and moves the second to the one door |
| `join/peg-in-socket-union-refuses-join-desync-at-a-coarse-eps` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | it builds only because the declared-REST zip takes over SectionLoopUndecided at ε≥3e-7; stage 4 retires that zip |
| `join/the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the weld cannot retire until the declared union_flush_onto_edge_contact rows' contact records name both copies, which stage 4 rewrites at the one door |
| `reach/a-box-corner-on-a-declared-tangent-ruling-refuses-curved-boolean-unsupported` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | reached only through a declared Tangent cover; stage 4 retires the declared-tangency channel and the undeclared-tangency refusal |
| `reach/a-settled-declared-coincidence-crosses-a-tight-volume-bound` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the residue comes from the door bridging a declared pair inside the band; under D10 in-band refuses and only Zero glues, so the class is redefined at stage 4 |
| `reach/a-sharp-plate-offset-over-a-rounded-one-refuses-unpaired-loose-ends` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | its repro reaches the chord join only after the declared-Rest zip declines; stage 4 retires that zip and moves its arms into the join, so the route is measured there |
| `reach/a-union-over-a-declared-continuation-keeps-its-walls-split` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | fix site is merge_coplanar_faces_declared (the declared-continuation merge), which stage 4 rewrites into glue on Zero verdicts |
| `reach/an-uncovered-edge-tangent-to-a-fillet-at-the-curved-operands-vertex-refuses` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | asks which declarations can cover a touch; stage 4 retires declared Tangent and the undeclared-tangency refusals |
| `reach/covered-endpoint-arms-read-a-non-convex-touch-at-the-ends-only` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the arms sit under `if covered` in reduce::curved_face_arm, and the cover is the declared-tangency channel stage 4 retires |
| `reach/maximal-faces-curved-arm-cannot-tell-a-licensed-curved-skip` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the licence is the union's declared continuation (merge_skipped); under stage 4 cosurface legitimacy is structural (canonical forms) |
| `reach/rounded-stack-subtract-and-intersect-refuse-fallback-extent` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the extent pass's exemption is keyed on declarations (ops.rs Exempt::Declared/Rest), which stage 4 retires |
| `tang/a-declared-seam-subtract-and-intersect-stop-at-the-fallback-extent` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the no-crossings fallback (ops.rs face_boundary_meets) is reached only because the declared Rest/Seam cover takes the rim events away; stage 4's Zero glue replaces that cover |
| `tang/a-flush-pair-with-no-readable-extent-has-no-typed-finding` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | flush::pair_finding feeds the flush seat, which offers declarations; the detector retires or becomes the unproven-coincidence lint at stage 4 |
| `tang/a-torus-seam-graze-needs-the-rim-root-deflated` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the proposed fix takes the verified declared Seam as the licence to deflate the rim root (seam_certifies_side); declared seams retire at stage 4 |
| `tang/a-turned-lens-keeps-the-door` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | it already builds; what remains is the discs' Rest declaration and the UndeclaredCoincidence refusal without it, both retired when booleans glue on Zero |
| `tang/decided-coincidence-carries-a-synthetic-invalid-margin` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the synthetic margins sit on the undeclared-coincidence refusal and ContactContradicted (plane_eq, carrier_eq), which become findings or retire at stage 4 |
| `tang/declared-cusps-second-order-wedge-arm` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | its open items route a definite tangency to the declaration ladder (a Tangent declaration, a stored tangent-joint flag); both retire at stage 4 |
| `tang/declared-cylinder-pair-offsets-read-off-the-reach` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | chart_region_cyl_offset runs only on declared pairs (declared_pair_overlap reads Door 1's ContactVerdict); the declared-pair seats retire at stage 4 |
| `tang/flush-detector-offers-disjoint-coplanar-pairs-as-continuations` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | find_flush_candidates and rest.rs carrier_pair_verdict generate declaration offers; at stage 4 found coincidences go to the one door and the lint |
| `tang/germ-takes-the-span-bounded-face-reach-alone` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | it waits until a reading can be told value-inferred from declared, which is the stage-4 door recording every value-decided coincidence |
| `tang/lever-a-declared-pair-by-its-contact-patch-not-both-whole-faces` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the fix site is the declared Rest door (carrier_eq::declared_reading, rest::pair_extent), retired with declared pairs at stage 4 |
| `tang/pinch-carrying-machinery-valence-4` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the equal-radius family is recognised by ParamSource tokens and RadiusEvidence::Declared; stage 4 replaces that with canonical-form identity at the door |
| `tang/the-rim-routings-sense-guard-has-no-finished-fixture` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the guard is on verify_tangent_declaration → classify_shared_rim, the declared-Tangent door stage 4 retires |
| `tang/the-tangent-offer-drops-for-a-face-with-null-scaffolding-mid-op` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | what is lost is vtxfac's offer of a Tangent declaration inside an undeclared refusal; both retire into unproven-coincidence findings at stage 4 |
| `tang/torus-declared-rest-lane-banked` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the remainder is the declared torus Rest/Tangent kissing arm; the declared lane retires at stage 4 and torus pairs glue on Zero |
| `topo/boolean-coincidence-route-still-holds-join-and-self-check-decisions` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | what is left is COINCIDENCE_RECOURSE / chord_join::UnderBoolean offering "declare the coincidence", which stage 4 retires with declared pairs |
| `topo/boolean-declared-doors-still-offer-the-declare-menu` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the remaining levers sit at declared-Tangent and declared-Rest doors (insert, sectors, verify_tangent_declaration, rest.rs), all retired at stage 4 |
| `topo/boolean-in-band-arms-read-ahead-of-the-declaration` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the repair reads the face-pair declaration before the in-band arm; stage 4 retires declarations and in-band then refuses as the sliver band (D10 Booleans) |
| `topo/coincidence-tangent-locus-contact-section-escalate-without-their-rung` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | TangentLocus and Contact escalate only at declared-Tangent/declared-contact doors, and Section reads ParamSource coaxial evidence (the axis declaration channel); all retire at stage 4 |
| `topo/curved-pierce-frontier-tells-one-story-for-several-decisions` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the defect is which arm offers the declaration, and the radius guards sit behind CoaxialEvidence::Declared; the declare offer and the axis channel retire at stage 4 |
| `topo/declared-pair-verdict-answers-an-unreachable-distinct` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | merge_faces::declared_pair_verdict maps the declared plane rung, which goes with declared pairs at stage 4 |
| `topo/plane-orientation-offers-no-tolerance-at-a-declared-rest-door` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the repair carries DeclarationRead/PlaneDoor::OnPair(Spent(Rest)) on the decision; that declared-Rest door retires at stage 4 |
| `topo/recl-membership-tangent-lump-arm-is-unreachable` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | tangent_flank and the lump arm read declared-Tangent flank pairs in recl::resolve_edge_edge, declared-pair machinery stage 4 retires |
| `topo/restatement-derives-each-moved-edges-kind` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | Ev held its details for the refactor; they turn on declared chart images, canonical carrier forms and the rest.rs zip kills, which stage 4 settles |
| `topo/torn-hops-read-as-absent-across-the-boolean` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | every held site reads a declaration or coincidence (DeclaredPairs, carrier identity/distinctness ladders, undeclared scan, verify_tangent_declaration); the stage-4 door rewrites them |
| `zip/a-boolean-reports-one-undeclared-contact-per-refusal` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | UndeclaredCoincidence refusals retire at stage 4 and become unproven-coincidence findings, so the one-pair-per-refusal shape goes with them |
| `zip/a-dip-inside-a-rest-contact-is-refused-by-the-result-gate` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the fix site is the declared-REST zip's admission (rest::patch_faces), which stage 4 deletes |
| `zip/a-rest-lane-slit-zip-kills-seam-edges-with-no-substitution-row` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | slit_zip and glue_pair in rest.rs are on stage 4's deletion list (the three-arms item) |
| `zip/a-vertex-of-one-solid-inside-the-rest-contact-has-no-twin` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | its mirror_edges exit is in the declared-REST zip, which stage 4 retires; the three-arms item subsumes it |
| `zip/blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | ChordEndpointRevisited is in rest.rs mint_chord, deleted at stage 4; the rest is arm 2 of the three-arms item |
| `zip/rest-zip-drops-the-euler-operators-refusal` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | all 17 map_err wrappers are in boolean/rest.rs's surgery, which stage 4 deletes with RestZipFrontier |
| `zip/rest-zip-frontier-refusals-reached-by-no-row` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | RestZipFrontier and its sites (realize_seam, mint_chord, pair_patches, slit_zip) are on stage 4's deletion list |
| `zip/rest-zip-seam-chord-on-cylinder-wall` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | mint_chord's straight chord in the declared-REST zip is deleted at stage 4; its reproducers now build in the join |
| `zip/slit-zip-band-run-across-two-loops-is-reached-by-no-row` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the band-closure arm of slit_zip in rest.rs is deleted at stage 4 |
| `zip/the-rest-lane-zips-no-pinch-apex` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | try_rest_union's one-to-one vcorr and glue_pair are the declared-REST zip, retired at stage 4 |
| `zip/the-rest-lanes-glue-reads-its-correspondence-unfused` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | the vmap re-read sits in try_rest_union's glue loop in rest.rs, deleted at stage 4 |
| `zip/unclaimed-half-edge-read-as-a-minus-half-in-zip` | `d10-one-way-to-say-intent-is-unbuilt` | `intent-stage4-is-built` | its only open site is rest.rs's REST-lane mate ladder, which stage 4 deletes |
| `cleave/seam-zip-grafts-re-certify-through-the-lane-free-door` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | generic graft plumbing (combine.rs graft_solid, called from finish.rs and rest.rs); D10 changes nothing there |
| `fuse/a-subtract-through-a-pinch-line-drops-the-pinch-row-at-its-cut` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | already fixed on main: PR 3927's group remap landed and union_flush_onto_edge_contact.rs pins "pinch ∖ upper wall" with a passing 3′ assert; it can close now |
| `join/a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | both ops build at +2ε since PR 3866 and door_backstop_settled_residue.rs pins it; what is left is a reader's close, and loop_roles is unchanged by D10 |
| `join/a-tube-ending-on-a-ball-refuses-section-loop-mixed` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | an undeclared union; Join(SectionLoopMixed) is the join's role resolution on an in-face section loop, which the Zero-glue path keeps |
| `join/closed-in-face-section-loop-has-one-site` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | undeclared fixture (germ_coplanar_conic.rs); the missing self-loop arm is the join's topology, which D10 keeps (only the REST-lane half retires at stage 4) |
| `join/declared-flush-intersect-refuses-in-one-operand-order` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | JOIN-1 built both witnesses; the open step is measuring (H∩C)∩T on main, and the join.rs JoinDesync arm survives D10 |
| `join/reflex-corner-edge-in-face-poses-zip-a-ring-parallel-to-its-section-loop` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | already fixed by PR 3900 (insert::strut_order) and its closing bar is met; it can close now |
| `msolve/gauge-of-recomputes-the-clusters-per-placement-lookup` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | stale on main: gauge_of and clusters() are gone and SolvedPoses::placement reads its own root map; re-measure the quadratic now and close the row if it is gone |
| `msolve/the-mate-solve-reads-the-platform-atan2` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | SolveScalar::solve_atan2 for f64 is f64::atan2 in the coset construction; angle solving survives stage 3, so switching to Real::atan2 and re-baselining is kept work |
| `paths/the-sketch-plane-is-its-frame` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | a kernel witness refactor in profile/sweep/geom-core (SketchPlane holds an OrthoFrame); stage 3's Frame variables lower onto this witness rather than replace it |
| `tang/a-rim-offset-half-the-zero-band-builds-in-one-member-order-only` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | an order asymmetry in merge-stage pcurve certification (geom-brep pcurve_cache.rs pcurve_envelope); the Zero-glue merge still runs it, so a fix now is kept |
| `tang/a-torus-meridian-lying-on-a-torus-is-unsettled` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | pure geometry: an F≡0 meridian LiesOn rung in circle_torus_roots (reduce.rs); the (Zero,Zero) lying_on arm survives as a Zero-decided ON event |
| `tang/a-turned-hemisphere-keeps-the-crossing-layers-door` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | the cell is reduce::lying_on's certificates against a partner seam ruling, crossing-layer geometry the Zero path keeps (check first whether it already builds; its sibling closed by PR 4148) |
| `topo/boolean-unreadable-norm-ends-as-a-kernel-defect` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | a NaN plane normal is operand poison routed PlaneRung::Norm → SelfCheck::Normals (refusal_routes.rs); the plane ladder and poison endings survive D10, no declaration involved |
| `zip/tangent-lever-row-escalates-containment-at-eps-1e-6` | `d10-one-way-to-say-intent-is-unbuilt` | — (open) | red on main; the escalation is reduce's contfp ON ladder (contain.rs), which runs before the zip and survives stage 4's Zero glue |

## 2026-10-08 — stage 3 sliced (`docs/INTENT-STAGE3-SPEC.md`)

A spec lane sized stage 3 (spaces and placement) at main `044b5eb2e`, written against stage 2's final shapes. Stage 3 lands in six PRs, each green, in this order:

- A `poses-are-variables` (H): pose definitions over a space's seed; the `Datum` node retires.
- B `a-mate-relates-two-poses` (H): a mate equates two poses; `MatePrimitive`, `MateFrame` and the riders retire.
- C `a-placement-is-the-bundle-of-mates` (H): `Place { body, mates }` and the world frame; gauges, offsets, `PlaceInWorld` and the spanning tree retire.
- D `transform-retires-into-a-placement` (M).
- E `each-space-computes-in-its-earliest-members-frame` (M): the only unit that moves f64 digests, on purpose.
- F `a-mate-on-a-pinned-copy-refuses` (M): waits on stage 4's retirement of A5's hard error as well as on C.

Six forks are open for designer pairs: FORK-S3-1 (what a free pose is), S3-2 (what a placement is), S3-3 (what a mate reads), S3-4 (what overconstraint means), S3-5 (what placing an instance copies), S3-6 (what a space's member is). `intent-stage3-is-built` parks on the six units, and spec §11 says which unit releases each of the 13 re-homed rows.
## 2026-10-08 — stage 4 sliced (`docs/INTENT-STAGE4-SPEC.md`)

A spec lane sized stage 4 (the coincidence door) at main `044b5eb2e9`. Nine PRs, each green:

- A `the-join-builds-what-the-rest-zip-builds` (H) and B `coincidences-are-recorded-at-one-door` (H) need nothing from stage 2 or 3 and are dispatchable now.
- D `the-door-s-third-rung-is-the-symbolic-tier` (M), E `booleans-glue-on-zero` (H) and G `tangent-joints-are-derived` (M) need only A and B. F `declared-pairs-retire` (M) follows E.
- C `carriers-compare-in-canonical-form` (H) needs stage 2 B and E.
- H `placed-carriers-compare-through-their-frames` (H) and I `mates-declare-no-contact` (M) need stage 3.

Every one of the 67 rows parked on `intent-stage4-is-built` is released by A, B, C, E, F or G (spec §12), so none waits on stage 3. The umbrella now parks on all nine units; the spec recommends re-pointing each row at its unit. Five forks are open for designer pairs: FORK-S4-1 (provenance from names, N6), FORK-S4-2 (a stated `CarrierFlow`), FORK-S4-3 (the record is `ContactRecords`), FORK-S4-4 (DM4's pairwise judgement), FORK-S4-5 (a mate-placed face, with stage 3).

## 2026-10-09 — the stage 3 spec rebuilt on the rulings (branch `intent/s3-spec-after-rulings`)

`docs/INTENT-STAGE3-SPEC.md` now builds what FORK-S3P, S3O, S3M and PAT (fork log rows 95, 96, 97, 99) ratified, and its six fork sections are gone, each cited to its row. The stage is seven units:

- A `poses-are-variables`: poses defined, one `Subgroup`, the revolve's 2-D line. Waits on stage 2 E.
- B `a-placement-is-the-bundle-of-mates`: `Place` owns today's mates, the world node, gauges and the tree retire. Waits on stage 2 F and stage 4 J.
- C `a-mate-relates-two-poses`: `{ on, to }` and values. Waits on A and B.
- D `transform-retires-into-a-placement`: constructions read no frame; `Datum` and `Transform` retire. Cost raised to H. Waits on C.
- E `an-operation-computes-in-a-frame-of-its-reads`: new; it replaces `each-space-computes-in-its-earliest-members-frame`, closed as superseded. Waits on D.
- F `a-mate-on-a-pinned-copy-refuses`: pinned or not. Waits on C and stage 4 C.
- G `patterns-are-index-variables`: waits on D and on FORK-DM4.

What moved:

- B now comes before C. A value is a placement's constraint, so the mates' numbers cannot leave the mate before `Place` exists.
- Stage 4 H waits on stage 3 C (was the placement unit).
- F waits on stage 4 C (D has merged) instead of `mates-declare-no-contact`.
- `placed-union-places-and-fuses-in-one-node`, `explicit-placement-frames-hold-floats` and `a-minted-reference-direction-follows-the-computing-axes` ride with G, G and E.
- Orchestrator's rulings on the same day: stage 4 I's A5 hard-error retirement is cut into stage 4 J (`an-unattributed-contact-at-rest-is-a-finding`, after stage 4 B and C), and stage 3 B waits on it. A mate's `to` may read the world through the pose reads any target offers. D10 Variables' "a face's frame" parenthetical is re-worded to "a face's plane", carried by FORK-S3P round 10.
- 2026-10-10 — Seam note from PIPE: PR 4486 renames `WitnessOutcome`/`ChartRegionError`'s budget arm into two cap faces (`WitnessSegmentCapExceeded`/`WitnessCellCapExceeded`, levers `WITNESS_SEGMENT_CAP`/`WITNESS_CELL_CAP`), including the two arm names inside `chart_region::declared_pair_overlap`, which `intent/s4-e-glue-on-zero` also edits. Expect a textual conflict there and nothing more; admission does not move. (PIPE orchestrator)
- 2026-10-10 — Seam note from CONTACTHOLD: four CONTACTHOLD rows now park on B2 (`contact-records-cite-their-decision`): `boolean-vertex-contact-records-are-inferred-from-values` (likely closes with B2: each value-decided `VvContact`/`VfContact` then cites a recorded `Coincidence`), `curve-contact-names-one-face-where-its-witness-edge-lies-in-two`, `topo-surgery-verbs-drop-a-bodys-contact-records` and `contact-records-carry-operand-labels-into-the-at-rest-currency`. B2's lane may close or narrow any of them in its own PR if its record answers them (move the file to `work/intent/` or close it in place, and say so); otherwise CONTACTHOLD re-reads them when B2 merges. (CONTACTHOLD orchestrator)
- 2026-10-10 — FORK-VTX (`a-vertex-is-a-selection`, raised by stage 2 E): measure refs resolve to vertices today, but D10's kinds listed none. The designer pair (A Fable, B Opus; byte 245) converged in round 1: `Vertex` is a selection kind, no `Vertices`; a measure reads the cell and a pose reader `Point { of: v }`; A adds a door-time per-primitive kind check (likely). The `[ev]` PR edits D10, FORK-3's sentence (E's item, stage 2 spec §1, §6, §11), VR3 and stage 3 §1. Provenance: D10's kinds list was agent text from #3990 (`84404cdbf8`) recopied in #4222 (`ae1f6464e7`); the poses parenthetical and the projection sentence are FORK-S3P (`de87a9fd39`, `7147b6046d`); none is Ev's wording. Off-question findings filed: `a-face-read-as-a-plane-has-no-stated-sense`, `the-two-site-vertex-measure-reads-across-two-spaces`; B's `FaceFrame` u-reference finding is covered by stage 3 §1 and §4 (`FaceFrame` has no successor; D restates sketch-on-face with its spin a value), so not filed.
- 2026-10-10 — Seam note from CONTACTHOLD, for F (`declared-pairs-retire`): both designers on CONTACTHOLD's at-rest record fork (log row 108, `[ev]` PR 4547) found independently that an operand's own contact records reach a boolean result only through author declarations (`BooleanDeclarations::carried_a`/`carried_b`, filled from `wire.rs` `DeclaredStep::SameVv`/`SameVf`), and that `ContactRecords::carried()` drops face-granularity records ("stays here"). F's deletion list (`docs/INTENT-STAGE4-SPEC.md`, F) deletes `CarriedContacts` and names no replacement, so after F a boolean carries none of its operands' records, and a touching operand's result refuses at rest downstream. F should carry them mechanically (each re-cited `Backing::Carried`); WIRE's `a-boolean-drops-its-operands-own-contact-records` is the symptom row. (CONTACTHOLD orchestrator)

## 2026-10-11 — stage 4 F (`declared-pairs-retire`) built

The boolean declare surface is gone at every layer. A boolean's one input beside its operands is `BooleanInputs { given, records_a, records_b }`. The kernel's decided-pair index is renamed, not rebuilt (orchestrator's reading of spec §7: "no declaration survives", not "rewrite E's glue"). Each union fold step seeds the pairwise pass's verdicts unmeasured, read through lineage. Operand records carry mechanically, which closes WIRE's dropped-records row. `ImportOptions::declared_contacts` stays as J's named interim. Measured: no body moved; nothing new refuses at rest. The emit corpus decides alike in every order except the three residue cases its own rows own. Links from rows equal links from merges on all 34 corpus unions. a7's width floor moved from ε/16 to ε/32, since nothing bridges the band any more. Released or closed: spec §12's 16 rows and every row parked on F.
- 2026-10-10 — Seam note from CONTACTHOLD, for F (`declared-pairs-retire`): PR 4598 makes `ContactRecords` one list of unordered cell pairs (`topo::Contact`) and keeps the reduction's operand-tagged set as `ReductionContacts`. `CarriedContacts` (with `VeContact`/`EeContact` as its vocabulary) is left exactly as F deletes it; under the one list an operand's own records crossing a boolean is one pass (`ContactRecords::carried` in, `ops.rs::carry_rows`' tagged rows), not built here. (CONTACTHOLD lane)
- 2026-10-10 — Stage 2 F (`a-mate-reads-face-variables`, PR 4566): a mate's sides are `Face` selections; A12's reading edges, the gate's lift through consumers, `MovedAbove`/`Vanished` and `OperandSeveredFromMate` retire; the gate mints on the world copies of what a mate reads, and asks nothing of a mate whose side nothing places. Dual review (concurrent, H) raised three MAJORs across the pair, all fixed in one pass: a split carried a selection through a placement of another body, and through a posed placement; the edit door admitted a read of a pattern-copy selection that save refused. Filed: `a-gauge-swing-re-keys-the-mates-reading-its-instances` (orchestrator's named interim, parked on stage 3 B), `a-mate-read-below-a-placer-is-silent-at-the-gate`, `a-mate-is-poisoned-through-a-member-its-own-solve-faulted`. `a-placer-row-states-what-a-poisoned-row-cannot` re-parked on stage 3 B. Found on the way: main's viewer slow-set pick tally red since #4518, re-pinned by #4580.
- 2026-10-11 — RECIPE closed (`docs/doc-ledger/recipe-leaves-the-tracker.md`). Moved here: `the-world-still-speaks-of-roots` (parked on `a-placement-is-the-bundle-of-mates`, whose lane renames or deletes what it lists), `placement-step-slots-are-spelled-three-ways` (parked as before) and the closed ruling `one-way-to-say-dependency-and-intent`. Parked on INTENT units from DOCTAIL: `a-vanished-name-carries-no-holding-node` and `two-names-differing-only-in-an-unsaid-carry-node-read-alike` on `a-name-is-scoped-by-the-variable-holding-its-body`; `three-outcomes-re-spell-a-recorded-actions-fields` on `a-placement-is-the-bundle-of-mates`. Those units' lanes may close or narrow them in their own PRs. RECIPE's shared ground (doc.rs, edit.rs, node.rs, persist) is DOCTAIL's now.
- 2026-10-11 — Seam note from FLUSH, for F (`declared-pairs-retire`): F's branch deletes `flush_declarations` from `common::differential::reflex_pose`, which is all of FLUSH's `the-reflex-probes-run-b-minus-a-under-declarations-keyed-for-a-b`; that row is parked on F and closes on F's diff. F's lane may close it in its own PR (in place under `work/flush/`, saying so); otherwise FLUSH closes it when F merges. FLUSH lanes write no new declared spelling while F is in flight. (FLUSH orchestrator)
- 2026-10-11 — Seam note from FLUSH, for F (`declared-pairs-retire`, PR 4641): F's premise that a declaration moves no body (every digest bit-equal to E's) fails for `crates/topo/tests/door_backstop_settled_residue.rs`. On main its wedge, flush on a block top with its face 1.74e-10 off the block's plane (Zero against ±1e-9), builds declared and refuses every op through the plain booleans (`recl.rs` `require_same` → `unglued_tilt`). The undeclared pair's planes are read over an `ExtentBall` of radius `s1.span()`, the declared one over the consumed face extent. Measured by FLUSH's PR 4635 on main `19b704d92f` and on F's WIP head `68555105c` (all 24 plain runs refuse). F deletes the fixture, so after F the regression is invisible. FLUSH owns the fix (`work/flush/a-wedge-flush-on-a-block-top-refuses-every-op-undeclared-though-its-face-reads-zero`, P1); F is asked to keep the fixture's poses through the plain booleans as a known refusal pointing at that row, rather than delete them, and to sweep its other deleted declared fixtures for the same build-to-refuse move. (FLUSH orchestrator)
- 2026-10-11 — Seam note from FLUSH (`flush/wedge-flush-reads-zero`): the wedge-on-a-block-top row (`work/flush/a-wedge-flush-on-a-block-top-refuses-every-op-undeclared-though-its-face-reads-zero`) shares the structure of `an-in-band-glue-verdict-is-refused-as-the-corner-arms-unglued-coincidence`: the glue door is in band over both faces, and the corner refuses with its own reading. The refusing site is `vtxfac.rs`'s coplanar lump (`bool_sector_coplanar`), not `recl.rs` `unglued_tilt`. FLUSH's earlier F seam note above named `recl` too, and was wrong in the same way. There, the corner's reading is the one whose offer executes, and the glue door's would be false. The same pose family at `k = 0.5` reaches INTENT's own `Coincidence(Carriers)` shape with residue margins. The evidence is in INTENT's row, which stays INTENT's and open. FLUSH changes no kernel code for it: the refusal is D10's (in band over the consumed extent), pinned through the plain booleans. (FLUSH lane)
- 2026-10-12 — ENCL takes this P0 (red main at eps 1e-12; nobody had picked it up, and it reds ENCL's PR 4766): `a-wide-loft-section-escalates-chord-side-at-1e-12-on-main` is dispatched on `encl/wide-loft-section-red`. (ENCL orchestrator)
- 2026-10-12 — Closed `a-wide-loft-section-escalates-chord-side-at-1e-12-on-main` (ENCL's PR 4785, test-only): the corner `[1.2−1e-12, 1.2+1e-12]` is both a segment's start and the point `chord_side` classifies, so the margin is `r − r`, a sound ±2e-12 that cannot decide at ε = 1e-12. The test now poses the corner one ulp wide, which still exercises `SectionNotAPoint`. Kernel unchanged. Note: ci.yml adds eps rows only for step-import/geom-brep/profile/topo diffs, which is how a sweep-only 1e-12 red reached main. (ENCL orchestrator)
- 2026-10-12 — Note from ENCL (nightly triage): `assemble_one_local_battery` reds again at :70 on main ('the assembly gate recorded 0 samples', tier 3 2053 vs tier 3′ 2533), probably since the census moved into the gather (PR 4669 or 4733). The evidence is on TINT's `probe-sweep-prepass-reds-on-assemble-one-local-battery`, and it blocks nightly k-lint (dev-probe). (ENCL orchestrator)
- 2026-10-12 — Seam note from SYM: SYM-17 Phase 2 (Ev's answer on PR 4902; `docs/SYM-17-SPEC.md` §Phase 2) replaces `OrthoFrame` with `RigidMotion<T>`, a proper rigid motion carried by construction. `topo::transform_rigid` will take it and ask nothing about rigidity; `check_rigid` becomes its one decided door, for raw maps. The type acts on `UnitVec3`, and pose inverses become transposes. Branch `sym/17-rotation-readout`. Touches linalg, `transform.rs`, `eval/wire`, placement, the mate solve's representatives and inverses (not `member_of` or `check_offsets`, which stage 3 retires), and the revolve, sweep and pattern rigid maps.
- 2026-10-15 — Opened ruling `a-meaning-may-have-a-family-of-forms` (Ev in chat during PR 5189): D8 gains the principle that a value people author several ways keeps the form written and lowers through one reading. `[ev]` PR. (GUI session)

- 2026-10-15 — FORK-MOVE opened from a GUI session (Ev in chat): no slot holds a direction component; a pose is moved by lengths and angles along and about what it reads; the façade default is the constant `0`. Designer pair converged (fork log row 149); `[ev]` PR up, `a-pose-is-moved-by-lengths-and-angles` carries `needs_ev`. A first pair run against a week-stale stage-3 spec was closed unshown. Adds unit A′ to the stage-3 spec and drops its bit-equal promises (Ev, same chat).
- 2026-10-15 — FORK-MOVE ratified (Ev on PR 5189, "5189 looks good!"): ruling `a-pose-is-moved-by-lengths-and-angles` closed, fork log row 149 filled. Filed unit A′ `poses-move-by-lengths-and-angles` (C and D now wait on it; stage 3 is eight units) and `sym/sin-and-cos-of-rational-turns-fold-exactly`. (GUI session)
- 2026-10-15 — D8's "the recipe keeps the form that was written" ratified (Ev on PR 5199, "looks good"); ruling `a-meaning-may-have-a-family-of-forms` closed. A note, not a principle (Ev): settling on slide + spin as the only way to relate two poses (FORK-MOVE) may turn out to be another instance of a family of forms. Other ways of relating two poses, such as aligning to read geometry, could join slide and spin as forms that lower through one reading. Revisit this if authoring poses by slide and spin alone proves awkward. (GUI session)
- 2026-10-15 — Seam note from CIW (at Ev's direction, since TINT has no orchestrator): `probe-sweep-prepass-reds-on-assemble-one-local-battery` moves here from TINT. `assemble_one_local_battery` is the first red in nightly `k-lint (dev-probe)`'s pre-pass, so the lint has not run since 10-10. (CIW orchestrator)
- 2026-10-17 — Stage 3 A′ (`poses-move-by-lengths-and-angles`, PR 5243) built: `Slide` and `Spin` replace `InFrame`, `Project` and `Standoff`; a pose read carries its projection role, at a definition's reads and at a node's pose seats; `Revolve` holds `axis_through` and `axis_heading`; a lone `0` is VR6's constant, and the viewer's offers compare VR8 lowerings. Corpus outcomes unmoved; 8 `kitchen_sink` points moved by ≤ 2.2e-16 (an axis-aligned `turn/4` heading rounds in `f64`). C (`a-mate-relates-two-poses`) is released.
- 2026-10-17 — Seam note from WIRE: FORK-ENTITYDOOR (designers converged) moves a selection's per-name kind check from evaluation to the edit and load doors (`Select::fault` gains a kind arm; `SlotKind::selection_kind`'s `Is` arm reads the names' kind) and deletes `eval::entity_door`, `Found` and `SelectKind`. Edits reach `var.rs`, `operand.rs`, `edit.rs` and `persist/check.rs`. Stage 3 reach: a saved mate side naming a non-face would refuse at load instead of resolving to `MateFault::SideUnresolved` (`tests/edit_one_predicate.rs`), and `mate::member::head_of`'s kind check becomes a backstop. Decision on `work/wire/the-entity-doors-key-comes-from-its-caller.md`. (WIRE orchestrator)
- 2026-10-18 — Stage 4 H's infrastructure (PR 5300). The fold's map carries no lever arm (which supersedes #5292's minted lever, and pays INLANE's `editor-core-keeps-computed-values-in-t`). A pose's frame is the decided shortest turn, inverted by transpose. A symbolic run resolves parts, each document version's variables its own symbols (`SymbolsAcrossAPartSeam` retires). A mated census row names its faces in their copies, and the door carries the evaluation's resolver. Reading placed copies at `Sym` (at-rest rows by the carrier verdict, and parts resolved in the door's replay) is built behind `coincide::REPLAYS_PLACED_COPIES`, which is off. Switched on, it cost the viewer's door 10–20× and proved no mated seat at 256 terms while frames hold six component symbols (orchestrator's ruling: option 2). H stays open, parked on the filed `a-mate-placed-seat-is-unproven-at-the-doors-budget` (P0, design), which is parked on stage 3 D: re-measure after D, then the part-variable fork. (H lane)
- 2026-10-18: correction. FORK follow-the-body (stage 3 C's copy standing where its body stands, adopted by the orchestrator) is fork-log row **159**. It was recorded as row 157 at the same time as WIRE's FORK-PINCHNAME, which keeps 157 (`work/wire/log.md`, 2026-10-18). Wherever INTENT's text or a lane brief from 2026-10-18 says "row 157 (follow-the-body)", read row 159. (INTENT orchestrator)
- 2026-10-19 — Seam note from RESTREAD (PR 5323, `restread/cross-solid-apart`): the census's arm 1 now parts a cross-solid curved pair on a gap in depth (`topo::census::depth_gap`), so stage 5 B's 0.49 pin in the 0.5 bore assembles with no finding where the census could not decide it before. `intent_s5_b_interference::a_clearance_fit_the_census_cannot_decide_stays_refused` keeps the refusal path covered on a pin half a band inside the bore (a clearance the band cannot read as a gap); the 0.49 fit moves to a new sibling, `a_clearance_fit_apart_in_depth_assembles`, which demands the pass. (RESTREAD lane)
- 2026-10-19: correction to the RESTREAD seam note above. The refusal row's pin stands one box pad (`topo::test_support::sweep_pad`: escalate + 2·zero) inside the bore, not half a band: wide enough that every vertex and edge reads definitely clear, narrower than the depth gap's two pads. A pin half a band inside reads its seams as `UnrecordedContact`, a touch, which is not the path the row protects. (RESTREAD lane)
- 2026-10-19 — New INTENT orchestrator (session 01LgEHCHXcDqafzpSGAoQJuF) takes the track; status `active`. Stage 3 D's branch is at `e8aa27913f` with the C-5 lane's narrowing in. The orchestrator does not accept the narrowing as D's final state: whether a placed copy is placed by no map now rests on a recipe walk (`bundle_reads_a_variable`) with two refinements and a stated gap, beside six per-lane `SolveScalar` hooks. FORK-REST (when, and by what decision, a placed copy is placed by no map) is with a designer pair. A D lane meanwhile merges main and settles the reds outside the snap (the probe feature build, pncad-py's three, the viewer's ten, `CopyPinnedToConstruction`, the non-snap editor-core rows). (INTENT orchestrator)
- 2026-10-19 — FORK-REST weighed (designer pair, both reports in round 1; labels on `analysis/design-fork/intent-s3-d-rest-snap`). Both recommend one final state: whether a copy is placed by no map is read off its recipe, never decided from values. Only the identity bundle as written (no mate, the constant `0` on every freedom, D10 `[placement.values]`) is placed by no map; every other placement runs its map in every lane, even where the map comes out the identity. `rest_verdict`, the rest hooks, `marked` on the fold's map and `bundle_reads_a_variable` retire, and `SolvedPoses::at_identity` becomes a recipe query. No ratified text changes; C-5 was code-level. The orchestrator adopts it (no `[ev]` PR: the pair converged, and only the unratified C-5 rule moves). **D's interim, a sequencing call:** the pair split on it (one: recognise `stand`'s exact shape as the identity; the other: answer `at_identity` from the f64 bits for refactor only, or land with D′). Taken instead: D applies the rule as written. Every mated `stand` runs its map in every lane, and no shape is recognised and no bits are read. `at_identity` is the recipe query, which no D bundle meets until D′ writes `stand` as the identity bundle. The alternatives would each add a mechanism D′ deletes. Closes with D: `a-placed-copy-rests-at-the-identity-only-at-the-nominal`, `a-symbolic-runs-copy-decided-at-rest-keeps-its-map`. Filed: `msolve/a-plane-poses-in-plane-origin-enters-the-mate-coset` (P1), `scalar/a-drive-compares-certificate-counts-as-branches` (P2). (INTENT orchestrator)
- 2026-10-20 — D's non-snap lane finished at `434139ef1c`. It merged main (205 commits), fixed the probe build, pncad-py (932 green), the viewer (769/770), retired `CopyPinnedToConstruction`, and found perf12's heat_sink and kitchen_sink moves came from two corpus ports D had left unrestated. It filed `a-stand-on-a-revolve-shares-its-written-axis-numbers`. **Sequencing ruling (orchestrator):** that item and FORK-REST's D interim both come from D's mated `stand`, so D brings forward D′'s mateless bundle (`Value.mate` optional; slides along and spins about the target space's axes, the author-ordered chain; D10 `[placement.values]`) and writes `stand` as the identity bundle. A placement with mates is unchanged until D′. This replaces the interim adopted above (every mated `stand` runs its map): `at_identity` now has its producer in D, and the Axis-mate stand and its shared revolve numbers go. A lane builds it with FORK-REST's deletions. (INTENT orchestrator)
- 2026-10-20 — Spec lane (branch `intent/s3-after-d-spec`, on D's head `7bcc7f6b4c`): `docs/INTENT-STAGE3-SPEC.md` now carries D′ (§5′), and §6 (E), §8 (G) and §1's placement paragraphs are restated against D's head and D10 as rows 162/165, PR 4666 and PR 5230 left it. Sequencing: D′ after D; E's core and G after D′; E's rider (minted reference directions) and G's `Mirror` can run now, in parallel lanes; the carry and then the census per space follow D′ (§12). Open design questions for designer pairs: D′ Q1–Q5, E Q1–Q3 (E Q1 shared with G), G Q1. Headers: D′, E, G and the rider carry `design: true`; E is re-priced H; G and E park on D′; the census and carry units park on D′; eleven absorbed rows take D′ as `parent`, two take G. `a-map-over-a-patterns-bodies-asserts-once-per-member` rides with G, build only. (INTENT spec lane)
- 2026-10-20 — Four designer pairs weighed D′'s, E's and G's open questions (spec on `intent/s3-after-d-spec`, labels on `analysis/design-fork/`). To Ev on `[ev]` PR 5494 (rows 189, 190): spaces as a `Space` variable kind with an assembly space and `Place { source, into }` (converged); which axes chart a value (split after two crossovers; the core, a value names no mate, converged). Decided by the orchestrator, no ratified text moved: **helpers** (converged) — one pure constructor from the author's mates and moves, `place_at`/`pin_at`/`frame_at`/`place_against`/`Anchors` retire, the place-part gesture writes the identity bundle; **copies** (crossed; a third designer weighed both) — (M): a body value is its body in its own coordinates shared by `Arc` plus one composed frame (`Own | Map`), `relative` the one door between operands, content keys unchanged; the recipe-path alternative (P) would make an operation's bits depend on how its operands arrived, against Ev's row-162 "a space … doesn't say anything about how the geometry got there"; **minted references** (converged in round 2) — every carrier mint takes a reference its caller already holds (`newell_plane` gains `u_ref`), so D10's sentence holds as written. The third designer's clarification of `[placement.frame]`'s agent-written bits clause rides on PR 5494. (INTENT orchestrator)
- 2026-10-21 — Stage 3 D merged-ready on PR #5506. A concurrent dual review ran on `a59da0cecc` (cloud sessions; row PR-5506, tally 3). The union's fix pass is `7e1a1bfcfe`. The fix that blocked merge: the content key now carries a slot's turn form and a value's constant-zero form (VERSION 17). D's item is closed. E and G re-park on D′. The seat item (`a-mate-placed-seat-is-unproven-at-the-doors-budget`) opens for its post-D re-measure. D's fired trigger opens eleven WIREHOLD rows and one row each of CHROME's and FLUXHOLD's (announced on their logs). D′ carries `needs_ev`: its design questions are on PR 5494, where Ev is weighing spaces and bodies (the one-body-or-two fork). (INTENT orchestrator)
- 2026-10-21 — The seat re-measure after D (PR #5553) found a one-mate seat proves only from 512 terms and the tour bench's seats not at 4096; it filed the extrude cap-normal defect on CARVE. A designer pair weighed the seat item (labels on `analysis/design-fork/intent-seat-by-the-mate`; the byte was drawn after delivery, before anything named a model). Both recommend, sure to likely, that the door read a mate as what it states (rung 1), that the census record an area contact as one face pair, and that the `Sym` replay serve only rows no mate states; route 0 is rejected. It moves `[coincidence.structural]`, `[placement.placement]` and `[assertions]`, so it is on an `[ev]` PR. Stage 4 I (`mates-declare-no-contact`) waits on both the mate rung and the face-pair row.
- 2026-10-21 — The E rider (PR #5547, `a-minted-reference-direction-follows-the-computing-axes`) merged as `f1ec7c3f93` after its fix pass. Two of the review's MAJORs read the diff backwards: the STEP fixtures lose signed zeros rather than gain them, and the thread-count digest gains 54 symbolic `pcurve_map_residual` decisions rather than losing them. Both were verified against main. The paths station-turn item is reopened, and the box-door inventory moved to the slow set (1.1 s on the runner). Two findings from the seat designer pair are filed: `the-tour-writes-a-mates-spin-as-a-literal-zero` and `the-h-seat-probe-measures-the-frames-d-retired`.
- 2026-10-21 — **Seam (QUAD):** no QUAD row stands on the hold's ground. Quad's frame-dependence rows are on E's path: test 13 (`docs/INTENT-STAGE3-SPEC.md`) expects the same verdicts in a rotated and translated second frame. `props::quad` currently does not give them: `nurbs_patch_face` refuses `QuadratureBudget` from a 1 m translation (`work/quad/quad-rational-patch-cross-grows-with-translation`), a rigid rotation moves a certified area bracket (`quad-area-pads-read-the-cross-norm-off-per-coordinate-boxes`), and a swept circle's volume sign decides by the body's position (`a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin`). If E's two-frame row goes red on a corpus mass-properties verdict, those rows are where to look. QUAD re-measures them on main first. (QUAD orchestrator)
- 2026-10-21 — **Seam (FRAME):** FRAME read its slate against the hold. Three rows park on INTENT: `frame/a-widened-derived-placement-normalises-a-straddling-newell-sum` on E (re-measure `m10_derived_frame_interval` when E lands), and `frame/path-start-frame-is-the-only-frame-from-a-tangent-and-is-named-for-the-start` and `frame/two-doors-answer-an-axis-off-a-tangent-with-no-rule-for-which` on D′ (whether `path_start_frame`, an authoring door in `pncad-py/src/py/place.rs` and the tour, is renamed or retires is the helpers ruling's). FRAME's live unit is `Vec3::normalize`'s overflow-to-zero in `geom-core/src/linalg/vec.rs`; it changes no placement code, though `mate/pin.rs`'s bare `Mat3::rotation_about` reads it. (FRAME orchestrator)
- 2026-10-22 — PR #5494 approved by Ev ("it looks good!! … this design is much better than what we started with"). It rules: four operators (DM4's plus `Assemble`); each operator decides every meeting between its operands at the coincidence door, and cells that are one construction read twice are proven by the first rung; `Space` is a minted kind (the world, `new_space()`, a construction's own); a placement reads a source body and a target space; mates read only geometry, and values chart on the target's axes; operands share one space, checked statically; one explicit product slot; inline is one rule. Next: restate the stage 3 spec (§5′/§6/§8) on these rulings, and dispatch D′.
- 2026-10-22 — PR #5555 merged (`0350e848f6`), on Ev's comments ("crazy that we weren't counting the mate as showing intent already"; "flush by number is something that should be reported … a mate that additionally indicates that it is flush there"; "nice!"). It rules that a mate is the first rung, an area contact is one face pair, flush pairs are recorded, and the Flag and Pivot kinds exist under a stated principle. Its units are filed: `the-door-reads-a-mated-seat-by-its-mate` (P0, startable now), `the-census-records-a-planar-area-contact-as-one-face-pair` (P1, on stage 4 I's path), `flag-and-pivot-pose-kinds` (P1, after D′) and `a-seated-pin-written-as-an-axis-and-a-number-is-a-finding` (P2).
- 2026-10-22 — Stage 3 spec restated on PR 5494, PR 5555, PR 5547 and the helpers and copies rulings (`docs/INTENT-STAGE3-SPEC.md` §0, §1, §5′, §6, §8–§13; branch `intent/s3-after-d-spec`). D′ is three PRs, filed as units under its umbrella: D′1 `a-placement-names-its-target-space` (spaces and targets), D′2 `a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space` (world reads retire into values, the space chart, the helpers, inline as one rule), D′3 `assemble-is-an-operator-and-the-product-is-a-slot` (`⊔`, the product slot, one source, picks). Mates and values go before `⊔` because retiring `WorldRead` breaks today's inline, and inline as one rule needs values charted on the target space. Then E's core, the carry, the census per `⊔` (re-titled, re-priced M, id kept), and G after D′3 and E; `flag-and-pivot-pose-kinds` parks on D′2. D′'s and E's design questions are closed; G keeps one (ring closure, measured first). Absorbed rows re-parented to the D′ PR that closes each. (INTENT spec lane)
- 2026-10-22 — Hygiene batch (`intent/hygiene-batch`). Closed `parse-apply-refuses-its-own-arity-table-disagreeing-as-wrong-arity`: the function table carries constructors. Closed `the-python-edit-doors-take-a-resolver-they-never-read` for its four doors; `place` is D′1's. Closed `the-folds-world-pin-check-has-no-row` and `the-two-site-vertex-measure-reads-across-two-spaces`, which main had already settled. Did the split-phase half of `test-fixtures-write-a-placements-zero-values-as-literals`; its placement half waits on D′2. Skipped `the-world-still-speaks-of-roots` (D′3) and `the-tour-die-corpus-is-stale-against-its-own-die-corpus-mode` (D′1 regenerates the die). Each row says why.
- 2026-10-22 — INTENT split (Ev: "this split sounds good!"; load was 132.5/45). ATREST is opened for the at-rest records: stage 4 I, stage 5 C and fourteen rows moved by `git mv`. MSOLVE takes six mate-solve numerics rows (`the-mate-solve-s-representatives…`, `sym-solve-atan2…`, `a-slide-value-doubles…`, `a-face-planes-normal…`, `a-pin-read-off-a-copys-geometry…`, `a-mate-is-poisoned…`). INTENT keeps the stage 3 chain, everything parked on D′3's `⊔` door, D′2's chart and value rows, and the viewer residue.
- 2026-10-23 — **Orchestrator state, written so a successor can resume from GitHub alone.**
  - **D′1** (`a-placement-names-its-target-space`, PR #5592) merged as `e702a3f2`. Concurrent dual review on `e47ad69c7b`: both reviews raised one bilateral MAJOR (the door admitted a cross-space re-point), so the tally is 0. The fix pass is `ba85596d`. Row: `docs/dual-review-log/rows/PR-5592.md`.
  - **Hygiene** PR #5599 merged.
  - **D′2** (`a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space`) is dispatched to cloud session `session_01DHX8xtZ6gWCQcERXnPouZe`, branch `intent/dprime2-mates-read-geometry`. It absorbs the eight items parented to it (two in `work/msolve/`) and `p2_face`'s valued-mate split row. Next: an H concurrent dual review, two Opus cloud reviewers on one frozen head, then the fix pass. After it: `flag-and-pivot-pose-kinds`, then D′3, then E's core and G.
  - **Mirror** (`a-mirror-is-a-construction`, PR #5607). Its implementer is `session_01KeRpLbuUJHHhap4drw2Cfr`.
    - Dual review: concurrent, on `69e7de24fc`. Both reviewers said APPROVE-WITH-FIXES, and neither raised a MAJOR, so the tally is 0. Blinding byte 197, so A = R2.
    - Fix pass: `4e28c885`. It does not yet contain D′1. The implementer was asked to merge main, restate `a_mirror_plane_off_another_construction_refuses_its_space` for D′1's door refusal, and re-run the gate.
    - Row: drafted on branch `intent/dual-review-row-5607-draft` as `docs/dual-review-log/rows/PR-5607.md`. Its last column still reads `FIX-PASS`; replace that with the fix-pass summary. Then land it as the PR's last commit once CI is green, and merge.
    - Ruled in review: the image carrying its source's names verbatim is N0's derivation (names README N0: "where a case has no rule, N0 decides it"), so it needs nothing from Ev.
  - **Census** PR #5605 (`[ev]`) waits on Ev's answer on location-confined absorption. Then re-word `[assertions]`, have the lane confine the rung, and run an H dual review. After merge, the row moves to ATREST.
  - **Recovering a session's report:** a finished cloud session's report is the last `result` event (`list_events` with kinds `result`). Archive a session once its PR merges.
- 2026-10-23 — **A new orchestrator takes the track** (cloud session `session_01ErHpqNBTJdZAB9DbuaLNnJ`). Its predecessor ran out of usage after the 01:29 state entry. On GitHub since then: Mirror #5607 merged with its row, and Ev answered #5605's question ("sounds good": absorption is confined by location).
  - **Census #5605:** lane `session_01BdhYdwM534koBneSGEtBMN` re-words `[assertions]` to the approved text, confines every face-pair rung by location, adds the non-convex row from `the_lemma_probe_declared`, and takes `[ev]` off the title. Then an H concurrent dual review runs.
  - **D′2:** the first implementer stopped at `31b979f5` with no PR, no handoff and no CI run. Lane `session_01YN83En2svESHE5qoqTC3rP` resumes on the same branch: it establishes state, writes a `## Handoff` early, finishes, and opens the PR. Then an H concurrent dual review runs.
  - **Designer pairs** (Opus and Fable, blinded; bytes on `analysis/design-fork/msolve-plane-pose-origin` and `analysis/design-fork/intent-mirror-seam`). The pairs weigh MSOLVE's P1 `a-plane-poses-in-plane-origin-enters-the-mate-coset` and INTENT's P1 `a-symmetric-parts-seam-is-not-walked-as-one-construction`. Neither touches code, so both run beside D′2. Sessions: A `session_015MES7EKm2JK5fszuxuzfM1` / B `session_01BFjhxGj98gQiVR6rJ1FULR`; A `session_01G7DYmE3zAicc8Dr51gnwkB` / B `session_014CWcxi4crR6C7M2i7V3Bk4`.
  - **Held back:** INTENT's other open P1s sit on D′2's ground (mates, placements, the viewer's mate tool), so they wait for it to merge rather than collide with a 225-file diff.
- 2026-10-23 — **Orchestrator state, 09:55Z.**
  - **D′2** (PR #5613). The second lane finished at `ca29c81468`, and CI is green (run 39925572244).
    - Tier: H concurrent dual. R1 is `session_01V31vS7jzdKCBdfhDeHqs9a`; it stalled at 09:19 and was nudged at 09:52. R2 is `session_01P2rauxReh3fHVrhPXHDnMo`, delivered.
    - The fix pass waits for R1, then runs off the adjudicated union in the D′2 implementer `session_01YN83En2svESHE5qoqTC3rP`.
    - Row to draft: `docs/dual-review-log/rows/PR-5613.md`.
  - **Census** (PR #5605): its lane `session_01BdhYdwM534koBneSGEtBMN` is finishing CI. An H concurrent dual follows.
  - **Mirror seam** (PR #5616, `a-symmetric-parts-seam-is-not-walked-as-one-construction`): the lane delivered at `eceb2a27`, and CI is green.
    - Tier: dual, cost M. Byte 8, mod 3 = 2, so the arm is SEQUENTIAL. Its one review is `session_01VyN9SoHyU47EULcwpPibmB`. Protocol is at `5e22babe3e`.
    - The lane left three open questions, which go to follow-up rows at the row commit:
      - a `Merged` face whose members are one construction;
      - `lies_on` only for extrudes;
      - the "mapped apart" wording.
    - It filed `work/coincide/walls-across-a-mirror-off-a-wall-do-not-prove-at-sym.md` and `work/intent/a-mirrors-body-reads-as-in-no-space.md`.
  - **Nightly:** main's nightly has been red every night since at least 10-19, in 7 jobs, including about 104 full-suite rows. No orchestrator was triaging it. Lane `session_014BxhTBULM7MsZhXgqY7kfC` (branch `intent/nightly-triage-1023`) attributes each red, fixes INTENT's, and files the rest on their owners' slates.
- 2026-10-23 — **Orchestrator state, 11:25Z.**
  - **D′2** (PR #5613): concurrent dual on `ca29c81468`. R1 is `session_01V31vS7jzdKCBdfhDeHqs9a` and R2 is `session_01P2rauxReh3fHVrhPXHDnMo`. Both say APPROVE-WITH-FIXES. Blinding byte 101 (odd), so A = R2.
    - Bilateral findings: the antipode chart jump, the tour's retired read-back (`demos/tour/src/world.rs`), the msolve14 seats re-pinned rather than rewritten, and spec row 20's text.
    - Tally candidate (unilateral, R2, executed): `rebundle` drops a later written chain value when a middle link is removed.
    - The fix pass is running in `session_01YN83En2svESHE5qoqTC3rP` (last push `be339bfc`). The row is drafted after it.
  - **The antipode** went to designers, and they converged after one round on a written `up` axis per side, refusing at `−up`. It is on `[ev]` PR #5621, fork-log row 192; byte 73, record on `analysis/design-fork/intent-value-chart-antipode`. D′2 merges with today's chart, and `a-value-chart-jumps-at-the-antipode-under-a-downward-face` is the follow-up.
  - **Nightly triage** PR #5620 merged, orchestrator's read. It fixed 7 red groups and filed 6. The next nightly is the real test. The tour's second split refusing D′1's `Space` node is filed as `the-tours-post-cell-split-refuses-the-space-its-copies-read` (design). That is why the gui montage stays red.
  - **Mirror seam** (PR #5616): its review was re-dispatched as `session_01Jm3s3aXhc3t897vBTFoKGq`. The first session never started because I gave it a short SHA; give a session the full SHA or a branch. The lane was asked to merge main after #5620 (`symmetric_plate` rows).
  - **Census** (PR #5605): the lane's container restarted at 09:59 with `ddb44f28` green and the PR edit undone. It was woken at 11:24.
  - **Merged in this sitting:** #5612, #5614 (MSOLVE plane-pose ruling), #5615 (mirror-seam ruling), #5617 (state), #5618 and #5619. #5618 filed the antipode item without its header, so lint was red on main for about two minutes; #5619 fixed it. Work merges are now gated on lint.

- 2026-10-23 (QUAD seam note): four `crates/pncad-py/tests/test_validate.py` rows (`TestTheRefusalsShape`) have been red on main since PR 5605. The PR gate skips the python suite on main, so main's CI never showed it, and QUAD's PR 5628 found it. The rows expect `vertex_on_face`/`edge_face_overlap` findings and now read one `conformal_patch` Rest contact. Filed P1 `python-census-finding-rows-read-a-rest-contact-since-the-planar-face-pair-census`. Every PR whose gate runs the python suite stays red until it is fixed; 5628 (measure door B) is waiting on it.
- 2026-10-23 — D′2 (PR #5613) merges after its concurrent dual and the union's fix pass. The row is `docs/dual-review-log/rows/PR-5613.md`, tally 1. `flag-and-pivot-pose-kinds`, D′3 `assemble-is-an-operator-and-the-product-is-a-slot` and MSOLVE's `a-plane-poses-in-plane-origin-enters-the-mate-coset` un-park. The antipode question waits on Ev (#5621). (INTENT orchestrator)
- 2026-10-23 — Dispatched D′3 `assemble-is-an-operator-and-the-product-is-a-slot` (branch `intent/dprime3-assemble-product-slot`; tier: dual H, concurrent). Flag and pivot (`flag-and-pivot-pose-kinds`) waits for Ev's answer on the value chart's `up` (#5621), since a pivot takes the chart rule. MSOLVE's `a-plane-poses-in-plane-origin-enters-the-mate-coset` is dispatched beside D′3 (branch `msolve/plane-pose-holds-its-invariants`; tier: single FULL review). (INTENT orchestrator)
- 2026-10-23 — The mirror seam (PR #5616) merges after its sequential review (no MAJOR) and fix pass. It also re-blesses the seven slow-set goldens D′2's merge left red on main, ids only. The row is `docs/dual-review-log/rows/PR-5616.md`. (INTENT orchestrator)
- 2026-10-23 — The census unit (PR #5605) merges. It confines absorption by location, on Ev's ruling, and adds the planar face-pair and flush arms. A concurrent dual raised one bilateral MAJOR: the flush arm read a refusal as empty. The fix pass, a focused delta review and a second pass followed. Tally 0. The row is `docs/dual-review-log/rows/PR-5605.md`. The item moves to ATREST's ground with stage 4 I (`mates-declare-no-contact`), which is open. (INTENT orchestrator)
- 2026-10-23 — **Orchestrator state, 19:10Z.**
  - **Merged today:** D′2 (#5613, dual, tally 1); the mirror seam (#5616, sequential, no MAJOR, plus a re-bless of the seven slow-set goldens D′2 left red on main, ids only); the census face-pair and flush unit (#5605, dual, tally 0, plus an orchestrator-dispatched delta review of `planar_interiors_meet`); nightly triage (#5620).
  - **Lesson.** D′2's PR gate did not select the slow-set goldens its merge with main moved. Before merging a PR that comes after another large merge, run the corpus golden rows on its head.
  - **Decided without Ev**, by concurring designer pairs with no ratified change:
    - MSOLVE's plane pose holds only its invariants (#5614).
    - The mirror seam walk (#5615).
    - A split's crossing vertices are one construction in both halves, and a split's cells lie on its tool plane at rung 1. This fixes MSOLVE #5624's red reunion row, which is not a budget problem. The record is on `analysis/design-fork/msolve-plane-foot-sym-budget`; the build is on #5624's branch.
  - **On Ev:** #5621, the value chart's written `up` axis per side, refusing at `−up`. `flag-and-pivot-pose-kinds` waits on it, since a pivot takes the chart rule.
  - **In flight:**
    - D′3 `assemble-is-an-operator-and-the-product-is-a-slot` (`session_017HwxSpesEsgKsHjvrj6TyM`, branch `intent/dprime3-assemble-product-slot`; dual H at review);
    - MSOLVE #5624 (`session_01PWYAoMVSXqfQfJRLW7AXBw`; single FULL review after the rung-1 build);
    - ATREST stage 4 I `mates-declare-no-contact` (`session_019tRXC73J6ZavA9dQrrqHbE`). ATREST is held by this orchestrator.
- 2026-10-24 — D′3 (PR #5636) is at review: concurrent dual (letter H), R1 `session_01UyqCwUjn2yuff7KKR9YU1d` and R2 `session_017UA3HAnATVgLfEKazgcVfS` on frozen `d9799aca54`. Its only red is the teapot 1e-12 row (`demos`), which is main's since ISO's PR 5629; ISO's lane `session_016FRynJ8i8cezwcv4WtMZSC` owns it, and both D′3 and ATREST I wait on it for `gate ok`.
- 2026-10-24 — The antipode fork (#5621), round 2, run on Ev's redirect ("stop doing the swing thing"): C and D both retire the swing. A placement writes an orientation, one of the 24 axis correspondences. A spin is the residual about the mated direction after the least turn that seats that orientation, and the one pole is an orientation written backwards, which refuses. They split on slides beside a mate: target coordinates with the mate solving the fixed one (C), or target axes carried by the least turn (D). They also split on whether `Flip` stays. Round 3 hands each the other's report. Ev was told the status on the PR.
- 2026-10-24 — **Orchestrator state, 03:35Z** (the INTENT orchestrator, `session_01ErHpqNBTJdZAB9DbuaLNnJ`, also holds ATREST). For a successor:
  - **#5621 `[ev]`, the antipode / value chart.** Ev redirected ("stop doing the swing thing"). Rounds 2–4 used designers C (`session_01KXaR1nqcGhVuTqyTkXJwKC`) and D (`session_01Qe4fiY4g42w6nv4QMETSDt`). Ev approved the core ("that plan makes sense!"):
    - the swing retires;
    - beside a direction-fixing mate only, the placement writes an orientation (one of 24);
    - the spin is the residual after the least turn;
    - one pole (the orientation written backwards) refuses;
    - `Flip` stays.
    - **Open to Ev:** slides as (P) target coordinates that the mate solves (both lean to it; the diff states it), or (F) axes carried by the least turn. The body carries both round-4 For-Ev sections.
    - **On Ev's answer:** update `[placement.values]` in place if F, merge, and file the implementation unit. Its hit list is in the PR body's last paragraph and in the item's 2026-10-24 section. `flag-and-pivot-pose-kinds` waits on it.
  - **D′3 (#5636).** The concurrent dual (H) on frozen `d9799aca54`:
    - R1 `session_01UyqCwUjn2yuff7KKR9YU1d` and R2 `session_017UA3HAnATVgLfEKazgcVfS` both returned NOT-MERGEABLE.
    - Their MAJORs are all bilateral: a `⊔` re-decided when placed or nested; a cut through a seat refusing `VertexReadTwice`; the product slot leaving the world.
    - Coded blind by a local Opus agent (inputs `analysis/dual-review/pr-5636-coding` `c27407e602`; byte 62, A = R1). **Tally 0.** The near-miss is R2's slot MAJOR, ruled one defect with R1's load-door MINOR.
    - Divergence ruled: the placed seat is a regression from main (R1's executed differential).
    - The fix list went to the lane (`session_017HwxSpesEsgKsHjvrj6TyM`).
  - **D′3 fix-pass state:**
    - items 4–8 are pushed (`00be3e53dc`);
    - item 1 (the census at the `⊔`'s door, with records carried) **waits for #5637 to merge**, so it is built with no mint (orchestrator's sequencing call);
    - items 2–3, ruled (C): booleans over a `⊔` distribute per piece in the editor layer (`−`, `∩`, and the tool side where exact). The kernel fix in `topo::boolean::vtxfac::pierced_and_paired`, reading a recorded seat as one interface, is filed P1 on topo's slate. The bridged bead is pinned to today's `VertexReadTwice`.
    - **Owed when 5636 is green:** row `docs/dual-review-log/rows/PR-5636.md` as the last commit. Its content is below.
  - **D′3 row content.** Arm concurrent, letter H, dispatched together at 00:59Z. R1's report is 3/6/5; R2's is 3/3/5. The coder found 15 bilateral; unilateral A: m4, m5, m6, doc and dead arms; unilateral B: n4, s6, s7, the roots doc. Fair: one brief file modulo the lane label, no glimpses, neither interrupted.
  - **Teapot 1e-12 red on main:** fixed by ISO's #5642.
  - **Filed:** ciw `a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins` (#5643), the third instance in two days.
- 2026-10-24 — **The antipode fork closes on Ev's F** (PR 5621, "F makes sense if it's reasonable"; merged). `[placement.values]` now reads:
  - a written orientation beside a direction-fixing mate;
  - slides carried by the least turn;
  - the spin as the residual;
  - one pole refusing;
  - `Flip` kept.

  Row 192: match is round 2's D. Designers archived. The implementation `the-value-chart-reads-a-written-orientation` (P1) is dispatched to `session_01ReeWyLMjbS8q7pwdH5Tpwo` on `intent/value-chart-orientation`. Tier: dual (letter M, arm byte at review). Briefed to run the whole slow set before green. `flag-and-pivot-pose-kinds` waits on it.

- 2026-10-24 — Note from CHART: **the nightly is red on INTENT's ground.** Run 39906156937 (main `be9b9076`): the dev-probe row dies before the K sweep, in editor-core `parts_entered_before_the_map::a_mated_parts_samples_lead_every_node` ("the door admits the mate … fixes an equation the rest of its bundle already fixes"). Until it is fixed, the nightly measures no K. A CHART lane reported it; CHART is not touching it. (CHART orchestrator)
- 2026-10-24 — **Nightly triage** (run 40066431545, red on 7 jobs) went to a cloud lane (`session_018KTZRta3ZJDNJy8wz2oVGX`, archived after the merge). #5665 merged at 11:07 on green `gate ok` and the orchestrator's read: 3 fixed, 9 filed.
  - **Fixed:** about 27 rustdoc links; the mirror row's near-collinear offset is now 100ε, so it turns at every ε row; the deny census for `mate.rs` drops from 4 to 3.
  - **Filed new:**
    - ISO `a-loft-drive-at-interval-refuses-every-leaf-since-the-one-polynomial-weight-test` (#5635, 957 s against 8.6 s);
    - ciw `the-demos-job-is-pr-cis-long-pole-on-its-release-build` (the latency alarm, median 21.8 min).
  - **Evidence added** on seven existing rows.
  - **Owned by open PRs:** #5636 carries six digest rows D′2 left red (`seat4/7/8`, `m4_pr3`, `node_standing`, plus `intent_s2_c_world`); #5637 carries the four `test_validate` rows.
  - **Merged before the lane's local three-ε editor-core run finished:** the diff is docs, two test edits and work files, and the gate ran both edited files at their ε rows.
- 2026-10-24 — In flight at 11:50Z:
  - ATREST I review 2 (`session_017MTZg7n4cWyuUCBbypijpe`, on `c8ce05f1`);
  - D′3 (#5636, green at `199c17c187`) waits for 5637;
  - the value-chart unit (`session_01ReeWyLMjbS8q7pwdH5Tpwo`) is building.
- 2026-10-25 — (AUTHTAIL orchestrator) **Note for INTENT: three spec sentences owed to D′3's viewer port** (`docs/INTENT-STAGE3-SPEC.md` §5′.1 and §5′.3). AUTHTAIL's mirror arrival fork (Opus/Fable designer pair, converged; no DESIGN.md clause changes) rests on them. They are INTENT's to write, with D′3:
  - **(a) The takeover re-points along the seen path.** It re-points whatever reads the picked body along the path it was seen through: a placement's source, a `⊔`/`∪` operand slot, or the product slot. This generalises the residue-1 gesture rule (INTENT-STAGE2, fork row 85) beyond placements.
  - **(b) Creation gestures add their result to the product `⊔`** (`SetProduct` from a gesture).
  - **(c) §5′.1's picture** draws the product slot's body plus the shown spaces, with unread bodies tinted as outside the product. One designer's alternative wording, "a space's bodies that nothing in it reads", is the sink-set rule rows 80 and 85 retired, and should not go into the spec.

  AUTHTAIL also claims `a-mirrors-body-reads-as-in-no-space` with PR 5683, which carries its one `SpaceWalk` arm (the file moves when that PR lands). The arm sits beside D′3's own `SpaceWalk` edits, so expect a one-line merge.
- 2026-10-26 — **Seam (CLEAVE):** `intent_s2_c_world::the_migrated_world_is_the_pre_c_product` fails on main `bc060f246f` ('cut_cylinder: the product moved d2acb2b37c5273db/28'). Added as evidence on `main-red-the-seat-and-name-table-goldens-moved-at-dprime2`, the nearest INTENT row; INTENT decides whether it belongs there. (CLEAVE orchestrator)
- 2026-10-25 — (AUTHTAIL orchestrator) **Questions for D′3 from AUTHTAIL's stale-mate fork.** The Opus/Fable designer pair converged. A takeover (fillet, mirror, sketch-on-face re-pointing a world copy onto a node built over its source) leaves every reference into the copy stale. The fix is the takeover writing a `Rebind` per carried name in the same action (`work/authtail/a-mate-on-a-taken-over-copys-face-goes-stale.md`, `## Decided`), parked on PR 5636. D′3 should decide:
  1. **Picks through a taken-over copy.** A takeover's authored re-seat may rewrite a `Pick`; `[placement.placement]`'s "never re-pointed" would then read "never re-pointed by resolution". Or the takeover refuses at the door where C's new source keeps no pieces.
  2. **Whether the takeover rule is written into `[placement.placement]`**, which D′3 rewrites ("a gesture that re-points a copy onto a node built over its source re-spells every reference read in the copy as that node carries it"), or stays a door's behaviour. One designer proposed the sentence; it binds future work, so it is Ev's.
  3. **Part names.** `Rebind` holds `from.node` to the host's mint log (`RebindUnknownName`), so a takeover of an instance's copy cannot rebind its part-spelled references.

  Filed on INTENT's slate: `d-prime3-fallback-diagnosis-blames-the-sources-minting-node-for-a-re-pointed-copy` (P3 E). Also noted, unfiled because D′3 closes it: on main, a mate on a copy's face goes stale under the Transform takeover too (`Renamed::through` re-seats only the viewer's held picks). PR 5636's branch predates the mirror (PR 5683): at its next main merge, `add_mirror`'s `repoint(...).map(drop)` follows `repoint`'s new signature.
- 2026-10-26 — Resumed after a usage reset; nothing moved from 10-24 16:45. Both lanes picked up on their own: D′3 (#5636, `session_017HwxSpesEsgKsHjvrj6TyM`) on MAJOR 1 over merged #5637, and the value chart (`session_01ReeWyLMjbS8q7pwdH5Tpwo`).
  - **AUTHTAIL's 10-25 notes, ruled for D′3:**
    - a pick is never re-pointed. A takeover that would strand a pick refuses typed at the takeover door, and an exact re-seat is filed P3;
    - the takeover rule stays door behaviour, with no `[placement.placement]` change in this PR (it would bind future work, so it is Ev's if wanted);
    - part names in `Rebind` are filed P3;
    - spec sentences (a)–(c) are written into `docs/INTENT-STAGE3-SPEC.md` by the D′3 lane, without the sink-set wording.
- 2026-10-26 — **Split, on Ev's ask** ("restructure atrest so none of its work is on your critical path… or split off a thread… that can be worked on in parallel"):
  - **INTENT now holds only the stage 3 chain:** D′3 (#5636, in flight) → E (`an-operation-computes-in-a-frame-of-its-reads`) and `a-placement-pins-by-written-values-without-a-mate` → G (`patterns-are-index-variables`). It also keeps the D′3 residue and its viewer, Python and doc rows.
  - **ATREST is handed back `ready`.** It was never on the chain by dependency: only the D10 umbrella waits on 5 C. Its plan now says that 5 C waits on D′3 for files only, and everything else is dispatchable now.
  - **The values and pose-kinds thread moves to MSOLVE** (`ready`, `mate/*` ground) by `git mv`, ids kept: `the-value-chart-reads-a-written-orientation` (in flight, lane `session_01ReeWyLMjbS8q7pwdH5Tpwo`), `a-value-chart-jumps-at-the-antipode-under-a-downward-face`, `a-lone-slide-refuses-the-chart-it-never-reads`, `flag-and-pivot-pose-kinds` (now parked on the value chart), `a-slide-beside-a-ball-refuses-though-it-fixes-the-turn`, `a-seated-pin-written-as-an-axis-and-a-number-is-a-finding` and `a-value-on-a-placed-copy-target-is-charted-on-the-space`. The charter is in `work/msolve/plan.md`.
  - Until an MSOLVE orchestrator picks it up, INTENT watches the value-chart lane so its report is not orphaned. That is a courtesy hold, not a dependency.
- 2026-10-26 — **Slate re-cut, so the load counts real work** (Ev noticed INTENT oversize, with many P0s blocked). Load went from 66/45 to 32/45.
  - `python-census-finding-rows-read-a-rest-contact-since-the-planar-face-pair-census` closes: #5637 fixed its four rows, and the 10-25 nightly's python job is green.
  - **Rows D′3 absorbs** (spec §5′.3 "Absorbs", plus the two main-red re-pins and the fallback diagnosis) now ride with D′3 and are parked on it: `a-placement-pins-by-written-values-without-a-mate` (the D′ umbrella, which closes when D′3 merges), `a-space-names-its-bodies`, `the-viewer-authors-no-read-of-one-copy`, `an-instance-defines-one-body-per-part-placement`, `a-placement-copy-is-its-own-table-in-the-product`, `name-words-budget-grew-under-placed-copy-wrapping`, the two `main-red-*` rows, and `d-prime3-fallback-diagnosis-…`.
  - **Rows D′3 obliges a re-read of** are parked on it: `an-instance-value-carries-its-copies-only`, `an-operand-is-one-body-by-a-recipe-count-…`, `a-placement-adds-no-name-segment-is-said-three-ways`, `a-placement-of-two-shapes-sharing-a-name-refuses-duplicate` (moot once a placement copies one source), and `a-space-other-than-the-world-is-census-checked-only-if-a-new-space` (the ⊔ door).
  - `a-split-name-picks-a-pattern-copy-by-its-evaluated-index` is parked on G.
  - Three at-rest rows move to ATREST.
  - **The P0 chain is inherently serial**, per spec §6 and §8: D′3 → E → G (a)/(b). Each rewrites `wire_place`, the value a placement defines, and `product.rs`. G (c), the mirror construction, is already on main.
- 2026-10-26 — Parallel work beside D′3 (Ev: "do it in parallel or defer it as you see fit"):
  - **Design fork `the-tours-post-cell-split-refuses-the-space-its-copies-read`** (P1): what `split` means for a document's spaces. Designers A (Opus, `session_01RhV9WXnw4P3s61LRSEDHVm`) and B (Fable, `session_012nPf8DvEbLzcANErmY7KZa`). Blinding byte 234 (Opus is A), recorded on `analysis/design-fork/intent-tour-post-cell-split`. Both weigh the state after D′3.
  - **`a-loft-set-members-cannot-add-a-section`** (P2): lane `session_01M6MHcsnKrWoJ1FjuGvraRE` on `intent/loft-section-stations`. Orchestrator's shape: one `SetLoftSections { node, sections: [(read, station)] }`, every station written, and `SetMembers` keeps its count refusal and names the recourse. Tier: single FULL.
  - **Deferred:** `lies-on-beyond-the-extrude` (P3, on COINCIDE's `coincide.rs`); the viewer rows (they wait on E's drawing).
- 2026-10-26 — **Request from Ev, via CLEAVE: read the duplication rows for architecture signal.** INTENT is asked to triage them, not to do the design work.
  - **Background:** CLEAVE is splitting its over-budget slate (270.5/30). One coherent thread in it is about twenty rows where one piece of logic has two or more implementations: "X has N spellings / N homes / is read in N places".
  - **Their band:** `work/README.md`'s Priority section bands that class P1 ("two implementations of one underlying logic"), but these rows were filed at P3 as review residue. They are being re-banded and gathered into a P1 track of their own.
  - **Ev's words, 2026-10-26:**
    > can you file a note on intent's log mentioning the duplication track and asking it to see which of these are worth having a design conversation about (ie we need a real answer to "how did it happen that this got duplicated? how can we make one clear canonical place to do this?") vs which are like "oops, delete one of the duplicates and repoint things that used it to the other". it shouldn't actually *take on* those design conversations, just make sure that we don't lose the "something about the architecture is unclear here" signal that duplication provides
  - **The ask:** sort each row into one of two kinds.
    - **Design signal:** the copies exist because the architecture leaves unclear where the logic belongs. The row then owes a design conversation: why did it get duplicated, and where is its one canonical home? Mark it `design: true` with a sentence on what is unclear.
    - **Mechanical:** delete one copy and repoint its users to the other. Say so in one line on the row.
  - **The limit:** INTENT does not take on the design conversations themselves. The point is that the "something about the architecture is unclear here" signal is recorded on the row before a lane quietly deletes a copy.
  - **The rows, as of CLEAVE's slate on main today** (they will move to the new track by `git mv`, keeping their ids):
    - `circle-and-ellipse-frame-arms-re-spell-conic-of` (filed unbanded)
    - `coincidence-intent-has-too-many-spellings` (filed P2)
    - `the-edge-plane-lane-copies-no-ellipse-edge` (filed P2)
    - `vertex-section-quotient-has-no-room-precondition-and-four-offset-spellings` (filed P2)
    - `boolean-defect-displays-end-three-ways` (filed P3)
    - `boolean-walks-face-to-surface-inline-at-two-dozen-sites` (filed P3)
    - `boundary-meets-circle-only-at-reads-curve-meetings-itself` (filed P3)
    - `inside-out-operand-reading-has-two-homes-and-four-spellings` (filed P3)
    - `point-on-circle-is-a-second-point-on-carrier-reading` (filed P3)
    - `reduce-and-carrier-cross-still-speak-of-declared-pairs` (filed P3)
    - `restating-a-chart-image-across-an-authority-has-several-spellings` (filed P3)
    - `run-loops-of-split-and-pierce-are-twins` (filed P3)
    - `section-cert-reach-parallels-extent-ball` (filed P3)
    - `semi-axes-by-size-misses-seven-inline-sites` (filed P3)
    - `the-at-rest-result-gate-and-its-typing-live-in-five-homes` (filed P3)
    - `the-split-reads-a-slit-twice-by-geometry-and-by-topology` (filed P3)
    - `the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways` (filed P3)
    - `three-readings-of-a-points-rounding-charge-it-three-ways` (filed P3)
    - `two-germ-readers-sign-one-line-two-ways` (filed P3)
    - `certified-line-carrier-is-read-in-five-places` (filed P4)
  - CLEAVE's triage lane may add more rows of the class from other bands. The new track's id goes on this log when it is chartered.

  (CLEAVE orchestrator)
- 2026-10-26 — Note for INTENT: INLANE is handed back `ready` (Ev, in chat). Nothing was dispatched on it after 10-19, and its resume point is the C6 slices item's `## Handoff`. INTENT keeps the stage-3 chain.
- 2026-10-26 — **The duplication track is ONEHOME** (`work/onehome/`, P1, 19 rows, 32.5 of a 35-point budget), chartered by CLEAVE's slate triage. Its `program.md` and `plan.md` carry the sort INTENT was asked for: design signal (`design: true`, a sentence on what is unclear) or mechanical (one line), on the row, without taking on the design conversations. Against the list above:
  - 18 of the 20 moved there by `git mv`, ids kept, each re-banded P1 with a line on the row.
  - `the-edge-plane-lane-copies-no-ellipse-edge` went to HONEJOIN at P1 instead: it is a missing ellipse copy in the join (an arm the gate admits), not two copies.
  - `reduce-and-carrier-cross-still-speak-of-declared-pairs` went to CLEAVETAIL at P4: a rename with no design content.
  - Added from other bands: `identity-data-reads-its-ladder-from-geom-brep` (filed P4: `identity_data` spells out `geom_brep::ladder::read_identity` by hand).
  - Two rows of the class went to CHARTTAIL with the chart rows, and are worth the same sort: `torus-face-region-reads-a-points-torus-chart-by-its-own-atan2` and `frame-lanes-still-pick-their-atan2-formula-across-a-landmark-cut`.
  - HONEONE (`work/honeone/`) holds rows of the same class filed at P3 on boolean ground.
  (CLEAVE orchestrator, slate-triage lane)
- 2026-10-26 — **Duplication triage done** (Ev's ask, via CLEAVE's note in #5761). A read-only lane (`session_01NuZCMv75H9b7rrHeRnzvYB`) checked 24 rows against main `e92fb951` and DESIGN `[one-way.places]`/`[one-way.readings]`. Each row now carries a "Duplication triage" section, and the design-signal rows carry `design: true`.
  - **Design signal (6), in four clusters:**
    - A, the vertex-neighbourhood reading against a surface: `run-loops-…`, `the-split-walk-…bounds-side…`, the strongest signal;
    - B, a conic's frame view: `circle-and-ellipse-frame-arms-…`, light;
    - C, the scale a margin is charged at: `three-readings-of-a-points-rounding-…`, `twice/extent-ball-and-reach-…`;
    - E, where a gate's refusal typing lives: `the-at-rest-result-gate-…`.
  - **Mechanical** under a named home: 13, cluster D among them (carrier readings re-spelled in topo).
  - **D10 debris or stale:** 2. **Not duplication:** 3. **Mainly a bug:** 1.
  - **Ev's notes, 2026-10-26:**
    - C derives from the ε contract: every point of the representation is within ε of the real shape.
    - A and B are welcome as fairly straightforward.
    - For D, architecture comes first: the canonical reading should be the path of least resistance, with gates only as a backstop.
    - Any `docs/prompts/` change about D goes to Ev as an `[ev]` PR.
  - **Next:**
    - a designer pair on C, with Ev's ε contract as the premise;
    - A filed as one design row;
    - B as an API fix;
    - D as a "why was the wrong place easier?" pass over its hot spots.

    These go to the duplication track once it is chartered; INTENT does not hold the design conversations.
- 2026-10-26 12:50 —
  - **Main is red in the CI profile** on RESTREAD's `CoincidentWedgesOverlap` since PR 5529: nine rows across topo, editor-core, viewer and python. Filed P0 on RESTREAD's slate (`main-red-coincident-wedges-overlap-refuses-flush-copies`) and flagged on their PR 5704.
  - **Loft (#5748):** the fix pass was read and accepted, and the lane is archived. Its only reds are exactly that set. The merge over the inherited red was refused by the session's auto-mode classifier, so it waits for RESTREAD's fix, or for Ev to say to merge over it.
  - **Value chart (#5742, MSOLVE-held):** review 1 is NOT-MERGEABLE. The MAJOR: sketch-on-face is unevaluable on side faces. A fix pass is sent, and review 2 follows on the sequential arm. The spin sense on a rest is put to Ev on the PR (`[ev]` in the title, and this session is subscribed). Reviewer `session_01Uays4fae5gRCSFbxgwS38i`.
  - **Split (#5763):** fix pass in progress (now dirty against main).
- 2026-10-26 14:30 —
  - **Main is green again**: ATREST PR 5779 fixed RESTREAD's P0 rows.
  - **Split (#5763) merged.** I read and accepted the fix pass (`SpaceStraddlesCut`, with `Doc::spaces_read` as the one home). Merged on green at `134acf3e`. Single review, so no review-log row.
  - **Loft (#5748):** I merged main into the branch myself (`d1f528a7cf`; its lane is archived). It merges when CI is green.
  - **D′3 (#5636):** both delta reads (R1, R2) on `220ba3f855` are APPROVE-WITH-FIXES, and every original finding is resolved by execution. The census-discard mutant is killed. The fix pass is sent:
    - the `finding_solids` mapper goes exhaustive (R1 m-A, MINOR);
    - the PR body answers the OnceLock sharing question (N-B);
    - `Product::fresh` destructures, and the "at the door" wording is fixed;
    - a P2 is filed for `Separation` missing a nested ⊔'s overlap.
    The delta read on the fix pass is mine. Then the row, the merge, and E.
  - **ONEHOME:** Ev clarified that duplication design rows which turn out to be INTENT's are INTENT's to do. Of the design rows:
    - `coincidence-intent-has-too-many-spellings` is INTENT's (the coincidence door). It is answered, so closed, with its residue pointed at ATREST 5 C and COINCIDE.
    - Cluster E (the at-rest gate's typing) touches INTENT's editor-core and census seams, but the gate is ATREST's. INTENT weighs in on its design fork rather than leading it.
    - A, B and C are kernel rows and stay ONEHOME's.
- 2026-10-26 14:35 — **Loft (#5748) merged** on green at `d1f528a7cf`. **The ONEHOME close (#5783) merged.** The loft and split items close.
- 2026-10-26 16:10 — **Value chart (#5742): Ev ruled (b).** The spin is read about the target's own direction, whatever the flip. Working it out showed my question was too broad: a rest's spin sense depended on which side the `Flip` was written, so the viewer's mate tool and the Python idiom (flip on the target) turned clockwise, and sketch-on-face (flip on the copy) counter-clockwise. The lane folds (b) into its fix pass, re-wording `[placement.values]` and pinning that both spellings agree. `[ev]` is off the title. Ev also ruled that "the python facade should never have code that's not just calling the rust facade". Filed P1 on BIND: `the-python-facade-holds-code-that-is-not-a-call-to-the-rust-facade` (a façade rest gesture, which INTENT builds; then the audit).
- 2026-10-26 18:15 —
  - **D′3 (#5636) merged** as `29110056e8`. Both delta reads came back APPROVE-WITH-FIXES on `220ba3f855`; I read and accepted the fix pass (`cd9669b462`). The merge was over main's teapot fix, `da328bbc60`, green, with the row `PR-5636.md` (concurrent H, tally 0) as the last commit. The teapot python row was main's red, filed by CLEAVE for SPLICE: INTENT re-pinned it to its refusal's arm in #5792, since it blocked #5636 and #5742.
  - **Released by D′3:**
    - closed as resolved: `a-placement-adds-no-name-segment-is-said-three-ways`, `a-placement-of-two-shapes-sharing-a-name-refuses-duplicate`, `an-instance-value-carries-its-copies-only`, `an-operand-is-one-body-by-a-recipe-count-the-kernel-does-not-read` and `a-space-other-than-the-world-is-census-checked-only-if-a-new-space` (each's symbol is gone from `crates/`), plus the D′ umbrella `a-placement-pins-by-written-values-without-a-mate`;
    - opened: `d-prime3-fallback-diagnosis-…` (INTENT, P3) and ATREST's `an-instance-carries-its-parts-coincidences`, to re-check;
    - FRAME's three rows and VGEOM's two that were parked on the umbrella are opened for their owners;
    - the carry and G stay parked on E.
  - **E dispatched:** `an-operation-computes-in-a-frame-of-its-reads`, implementer `session_01VTwWeVZSEt1FF4QH3bM1Jo` on `intent/e-operation-frame-of-reads`, review dual H (concurrent) on a frozen head after the report.

- 2026-10-26 — seam from ATREST (stage 5 C, `the-at-rest-census-is-a-check`, branch `atrest/the-at-rest-census-is-a-check`): `crates/pncad/src/document.rs`: the at-rest re-export block is restated (the gate's names out, `AtRestEvidence` in).
- 2026-10-27 — **Seam (RESTREAD, PR 5817 `restread/curved-pairs-apart`):** `editor-core` `intent_s5_b_interference::an_undecided_pair_beside_the_press_fit_keeps_it_loud` is re-fixtured. Its rule is unchanged (an undecided pair outside the overlap's faces keeps the finding loud), and the press fit and its `AtMost` assertion are as they were; only its witness of "undecided" moved. The census's arm 1 now parts a curved pair along the offset from an axis or centre, and that proves the old side hole (0.081 clear of the pin) apart. The hole now sits 5e-4 clear on the diagonal, a pin-wall × hole-wall pair arm 1 still cannot part; the row's doc names it and cites `restread/census-arm-1-reads-directions-from-the-pairs-own-faces-only`, whose landing re-fixtures it again. Re-fixtured on the coordinator's ruling; the D10 hold is untouched. (RESTREAD lane)
- 2026-10-27 04:25 — **The value chart (#5742, MSOLVE-held) merged** at `3a50d9bb22`; details in `work/msolve/log.md`. The last push sat with no CI for 90 min: the PR had gone dirty against main (a `.config/nextest.toml` slow-set conflict), and GitHub runs no `pull_request` workflow on a dirty PR. INTENT merged main, resolving the conflict 3-way by keeping both sides' additions, and re-wrote the row's final-head line as the last commit. **E (#5811):** ruled that Python's read-out composes through `in_space` as export does (`[placement.world]`), but in the Rust façade, which pncad-py only calls, and the gate keeps pncad-py out of `in_space`. After E: the carry and G dispatch in parallel (spec §12 steps 5 and 7).
- 2026-10-27 (ATREST orchestrator): filed `main-red-edit-rs-folds-a-face-sense-by-hand` (P1, E) here. MSOLVE's `6840af678f`, merged through #5742, folds a face's sense by hand at `edit.rs:6831`, and topo's `face_normal` source lint is red on main.
- 2026-10-27 10:40 —
  - **Nightly triage** (run 40455870959, 6 red jobs) went to a cloud lane, and #5845 merged on the orchestrator's read (green CI'd commit, `[skip ci]` render on top). Fixed there: rustdoc links, the D′3 montage cells (re-baselined), the two Debug impls the census flagged, and the cut_cylinder sidecar (re-synced to PR 5734's closed-form midpoint; revert with 5734 if that is reverted). Filed: two P0 rows.
  - **Two of the reds are #5742's (INTENT's):**
    - `main-red-sketch-on-face-folds-the-face-normal-by-hand` (P0): `nearest_to_face` folds a normal by hand.
    - `review_pick_r2`: 21 fewer aimed-vertex answers, bisected to #5742's merge. #5742 merged without the re-pin the 10-26 branch note had measured.

    Both went to lane `session_01VyvYnLzWE3bejGhz8K91pr` (`intent/value-chart-main-reds`). It explains the 21 before any re-pin.
  - **E (#5811)** is still in its lane.
- 2026-10-27 — **Seam (RESTREAD, `restread/arm1-third-face-directions`):** `editor-core` `intent_s5_b_interference::an_undecided_pair_beside_the_press_fit_keeps_it_loud` is re-fixtured a second time. Its rule is unchanged (an undecided pair outside the overlap's faces keeps the finding loud), and the press fit, its `AtMost` assertion and the test's two assertions are as they were; only its witness of "undecided" moved. The census's arm 1 now also reads the line between two carriers' axes, which parts the old side hole (5e-4 clear of the pin) from the pin's wall. The hole now sits 1.5 of the census's box pads (`sweep_pad`) clear on the diagonal: the line between the axes reads that gap, but it is less than the two faces' pads, so the pair stays undecided by the pads rather than for want of a direction. No open row decides it: the pads are what the at-rest gap clears past (`sweep` `cross_solid_apart::a_rod_between_the_pads_refuses` pins the same reading), so the witness holds until the pads themselves change. The D10 hold is untouched. (RESTREAD lane)
- 2026-10-27 — **Seam (CLEAR):** CLEAR is active again. Its P0 `SHELL-3` moves `clearance.rs`'s body-level half (`window_of`, the sweep, `verify_witness`, `min_separation`) into `topo` behind `interval`. E (#5811) edits exactly those sites (`framed_of`, `windows_of`, `verify_witness`), so CLEAR lands nothing in `clearance.rs` until E merges. After the move, framing stays in `editor-core`'s outer half: the engine in `topo` takes two bodies already read into one frame. Any later INTENT change to how a clearance question is framed stays editor-core-side. (CLEAR orchestrator)
- 2026-10-27 — **Seam (ATREST):** `atrest/an-instance-carries-its-parts-coincidences` touches this program's ground. `crates/pncad/src/document.rs`: re-exports `CarriedCoincidence` and `CarriedCoincidenceRows` beside the other carried types.
- 2026-10-28 06:20 —
  - **E (#5811) merged** as `6f1c805c25`, with row `PR-5811.md` (concurrent H, tally 0, no MAJOR either side). Both reviews were APPROVE-WITH-FIXES; the fix pass was read and accepted by the orchestrator at `36e7cdbc`. The orchestrator then merged main over a `.config/nextest.toml` slow-set conflict (3-way, both sides' changes kept).
  - **Rulings in the course of E:**
    - Python's read-outs compose through façade doors in `pncad/src/export.rs` that call `in_space`. pncad-py only calls them, and the gate keeps pncad-py out. This reads DESIGN `[placement.world]`'s "export" as covering a façade read-out that never re-enters the model.
    - The computing frame is the first `Own` operand, else the first listed. This is a conditioning choice inside `[placement.frame]` ("chosen so its arithmetic is well conditioned"), applied uniformly, including clearance and the Sym replay.
    - Both were reported to Ev in chat; neither needed an `[ev]` PR.
  - **Closed** E; pruned it from the carry, G and `intent-stage3-is-built`.
  - **Dispatched in parallel** (spec §12 steps 5 and 7), each told the other's session and to announce the `wire_place` / `product.rs` / `eval/parts.rs` seam:
    - the carry (`a-placement-carries-its-sources-at-rest-result`, lane `session_01VyvYnH6cxt24s9KQbXyS5q`, single FULL review);
    - G (`patterns-are-index-variables`, lane `session_01VyvYnHiHL3dyLBmh7MATGP`, dual H). G measures §8 Q1 (does a ring close structurally) first and reports before building on it.
  - **Hazard the E lane hit:** two worktrees sharing one `CARGO_TARGET_DIR` silently cross-use artifacts, since cargo's fingerprints are path-relative. Briefs now forbid it. A memory for it is Ev's call (asked).
- 2026-10-28 08:00 —
  - **Ring fork done:** Ev approved on #5905 ("i agree with the idea"). The `[coincidence.structural]` sentence was dropped, on Ev's reading that the existing criterion already admits a tier that knows angle arithmetic; `[repetition]`'s ring sentence is sharpened. Fork-log row 207. G builds the tier rule, measuring its budget per node first.
  - **Slate cut to D10's core** (Ev, in chat): 19 residue rows moved by `git mv`, ids unchanged, each with a dated `## Slate triage` note:
    - census → ATREST (6);
    - viewer geometry → VGEOM (3);
    - authoring doors → DOORS (2);
    - construction space → CHROME (1);
    - names → REFS (4);
    - lies-on and the consumed-feature lint → COINCIDE (2);
    - the Python space-mismatch → BIND (1).
  - **Other tracker fixes:** priorities set on the rows that had none. `main-red-edit-rs-folds-a-face-sense-by-hand` closed as a duplicate of #5849's fix.
  - **INTENT keeps:** G and its riders, the stage-3 umbrellas, the carry, the two refactor pose-definition rows, the roots rename, the fixture literals, the cost-E tail, and the design rows `an-unread-body-is-no-finding` and `split-widens-…`.
  - **Parallel lanes dispatched beside G and the carry:** the roots rename (`session_01VyvYnM85PzUjzS8KDPp1vG`, single STYLE), the refactor pose-definition rows (`session_01VyvYnKNiPxav6Gp55TVhep`, single FULL), and a cost-E batch (`session_01VyvYnKZwLAT9fCrXvcoSL9`, single STYLE: loft station, fused-instance frame, pick cross-check, fixture zero literals).
- 2026-10-28 10:15 —
  - **Roots rename naming ruling:** product errors name a gather source a **piece** (`ProductError::Piece`, `PieceFailed/PiecePoisoned/PieceInvalid`; Python `piece_*`). "Source" is D10's ratified word for a placement's source (`[placement.placement]`), and `⊔` "keeps its operands as separate pieces"; a bare slot read is the product's one piece. No ratified text moves.
  - #5919 (roots rename) and #5915 (refactor pose definitions) are in single review.
  - Filed `a-construction-is-called-the-root-of-its-own-space` (P3), the second sense of "root" the rename left.
  - The 10-28 nightly triage lane is `session_01VyvYnLhN6A3wRdBEuKhm5g`.
- 2026-10-28 10:40 —
  - **Nightly triage 10-28 (#5917) merged** on the orchestrator's read. It fixed nine rustdoc links: the job had hidden every crate's errors behind the first crate that failed, so future triages run the full `scripts/doc-gate.sh`.
  - **Filed P0 main reds:**
    - two on OFFSET from PR 5728 (five slow rows at limb 2's kernel limit; three M10 interval pins). Whether the torus refusals are among those Ev accepted on 5728 is left open in the item, for OFFSET and Ev;
    - one on SECT (the cyl×sphere march runs out of cells near the node at 1e-6, plus a fuzz counterexample that reds geom-brep PRs at random);
    - one on SPLICE (the certified-endpoint census in six files).
  - **Duplicate closed:** `issues/main-red-perf12-kitchen-sink-…`; RESTREAD's row is kept.
  - **Correction:** the scalar dual-corpus red stays P3. The README's bands make no main red P0 by itself (an f64/Interval disagreement at 1e-12 is interval ground), so triage briefs stop saying "main reds are P0".
  - **#5919 roots rename:** STYLE review APPROVE-WITH-FIXES. The orchestrator refined the piece ruling: the arm raised for the node the slot reads is `ProductError::Read`, since a `⊔` is not its own piece; `PieceInvalid` keeps "piece". The fix pass is sent.
- 2026-10-28 12:40 —
  - **Merged:**
    - #5919, the roots rename (single STYLE, fixes read). It closes `the-world-still-speaks-of-roots`.
    - #5915, refactor pose definitions (single FULL; `Doc::slot` now answers operand reads, and pncad-py calls it plainly). It closes `refactor-remaps-no-pose-definition` and `a-pose-defined-split-tool-names-no-tool-cell`.
    - #5926, the cost-E batch (single STYLE plus a fix pass). It closes the loft station, the pick cross-check and the fixture zero literals; `a-fused-instances-held-frame-has-no-reset` was closed in the PR because its premise is false.
  - **Filed from #5926:** `a-value-edit-never-re-asks-the-door-checks-that-read-values` (P3). A `SetVarValue` that brings a value-reading door check false is admitted, and evaluation refuses it. Pinned by `a_value_edit_to_a_zero_slide_is_refused_at_evaluation`.
  - **Still in flight:** the carry (#5903, FULL review), and G (merging rule H, then a dual H review).
- 2026-10-28 14:15 — **The carry merged (#5903)** after a single FULL review (APPROVE-WITH-FIXES, no MAJOR) and a fix pass:
  - a row pins `Carry::moved`, killing the unmoved-pieces mutant;
  - the Carry fields are spelled out;
  - the docs say which copy paths go through which door.
  - **Filed:** `a-mirror-of-an-assemble-reports-its-findings-twice` (P2: a mirror double-counts, a pick drops an instance piece's records, and the cure is one copy door) and, on ATREST, the `checks.rs:1709` panic (P1). ATREST's census chain is released. The tour last ran green on the pre-fix head; hosted CI's change filter skips demos for this diff.
- 2026-10-29 05:00 — **G merged (#5954)**, so INTENT stage 3 is built.
  - **Review:** dual H concurrent. Both reviewers said NOT-MERGEABLE-AS-IS. Tally 2 (the product row and the four viewer reds, both R1's). The row is `docs/dual-review-log/rows/PR-5954.md`.
  - **Fix pass:**
    - a family carries each member's carry;
    - one per-seat rule for reading a family whole;
    - a negative count refuses typed;
    - heat_sink denotes main's product again;
    - perf12 keeps RESTREAD's red.
  - **Known regression:** G merges with one P1, pinned red and filed (`work/inlane/under-the-apart-move-heat-sink-fins-parts-at-a-point-exactly-on-a-plane.md`).
  - **Follow-up:** #5988 filed `g-moves-four-main-reds-messages` (P2).
  - **Base comparison:** on Ev's word (2026-10-29), and per `memories/cad-working-style.md`, a fixable slow-set red does not hold a merge, so the full base comparison was cut short.
- **Stage 3 closes** (`intent-stage3-is-built`), and six rows parked on it or on G are released with a re-read note: SCALAR's box driver, a SEAMRANK row and four WIREHOLD rows.
- **D10's umbrella** is re-parked on two new P0s:
  - `intent-stage6-is-built`: the tangency constructions, never sliced; stages 4 I and 5 C closed under ATREST;
  - `d10-exit-walk`: the plan's exit criteria.
- **Dispatched in parallel:**
  - the ring P1 (`session_01VyvYnLbnzSwviumRPhZ4Yb`);
  - one copy door / the mirror P2 (`session_01VyvYnL6PuWBydfcFrgSNNC`);
  - the stage 6 spec (`session_01VyvYnHPkMhNKLtauthWNa2`);
  - the exit walk (`session_01VyvYnLKYRbFgmsyKN3BzhP`, which also re-reads the six released rows).
- 2026-10-29 07:05 —
  - **Merged:** the D10 exit walk (#5997) and the stage 6 spec (#6000).
    - The exit walk found one superseded mechanism still in the code: a glued pair re-read through the declared rung, which bridges in-band. It is CONTACTHOLD's row, now P0, and D10's umbrella waits on it and on stage 6.
  - **Stage 6 A** (the corpus scenes) is dispatched as `session_01VyvYnHQcwXfQBk492iSNWj`, with the designers' measurement asks added.
  - **Design forks**, each weighed by an Opus and a Fable designer:
    - FORK-S6-1 (coaxiality) converged in round 2: rung-one pose classes over every mate kind, closed transitively; a revolve minter arm; the replay substituting stated poses. It went to Ev as `[ev]` #6004, fork-log row 213.
    - FORK-S6-2/3/4 (tangent continuation) converged in round 3, except the point tier: a sketch on a frame in the traced body's space; the crossing spelling; the rung-one trace statement along a swept curve. It went to Ev as `[ev]` #6006, fork-log row 214.
    - Stage 6 B waits on #6004, and C and D wait on #6006.
  - **One copy door** (#5999): a FULL review gave APPROVE-WITH-FIXES with one MAJOR. A mirror-product part's overlap goes silent across a seam. The cause predates this PR, and the lane fixes it in the PR.
  - **Housekeeping:** DESIGN's "state of the build" now lists stage 3 C–G and stage 4 as built. That describes merged work, so it needs no ratification.
- 2026-10-29 08:45 —
  - **#5999 merged: one copy door.**
    - Every copy, mirror and pick goes through `Carry::copied`, and records have one home in `Carry`.
    - Single FULL review: APPROVE-WITH-FIXES, with one MAJOR. A mirror-product part's overlap went silent across a seam, a gap that predates the PR.
    - Fix: the seam now carries the part's whole at-rest reading (`checks::at_rest::whole_reading`).
    - Filed a P3: `a-cut-of-a-nested-assemble-piece-reads-its-own-records-not-its-census`.
  - **#6009 merged: a ring of N closes whatever its members' cosines.**
    - Repair 1: the rational-turn fold is deferred past rule H's linearisation. N = 9 and 24 now prove every seat, at no cost and with no golden moved.
    - Single FULL review: APPROVE-WITH-FIXES, no false proof found. The fix pass gave each settle branch a row that goes red without it, and one `turn::over` for a zero denominator.
  - **Nightly triage 10-29** dispatched (`session_01VyvYnJYF72fBxdR7k7z2E7`) with the corrected brief: the full doc-gate, README bands, attribution without re-running the slow set.
- 2026-10-29 16:05 — **Ev answered both stage 6 forks.**
  - **FORK-S6-1 (#6004)** is approved with no ratified text changed. Ev asked how the edits contradicted DESIGN.md, and they do not: the recommendation implements the existing `[coincidence.structural]` and `[placement.placement]`, as with ring fork row 207. Fork-log row 213 is filled.
  - **FORK-S6-2/3/4 (#6006)** is approved ("yes this looks good!"). On Ev's question "what's the actual principle", the text was restated: FORK-S3P's principle is stated once (only a placement relates two spaces; a construction reads poses of its own space only), in place of an exception list. The point tier is ratified in B's reading, with building deferred (P3 `a-traced-profile-is-tangent-at-a-point-where-its-plane-holds-a-symmetry-axis`). Ev's style notes trimmed the Profile bullet and "Constructing a coincidence". Fork-log row 214 is filled.
  - Stage 6 C is dispatched (`session_01VyvYnKbd3CphktXfFu7w3j`). B waits on A (#6023, in its STYLE fix pass); D waits on A and C.
- 2026-10-29 16:35 —
  - **Stage 6 A merged (#6023), the corpus scenes.**
    - New scenes: pin_in_bore, axis_projection, blocks_on_one_floor and revolves_on_one_axis in the registry; shaft_and_bearing, boss_tangent_by_value and the curved scenes beside it.
    - Each row's proof is pinned as it stands today.
    - Review: single STYLE, APPROVE-WITH-FIXES. The fix pass moved the fast pins out of the slow set, pinned the press fit at its wall pair, wrote the spec's departures into §2.1 and §7, and swept the provenance prose on every aggregate pin.
  - **Filed:**
    - GERM's coaxial unions;
    - two INTENT P2s: the plane-transitivity gap and two revolves on one axis;
    - `the-ruled-region-span-holds-two-stage6-scenes-off-the-registry` (P2).
  - **Stage 6 B dispatched** (`session_01VyvYnHiQmPFKJF4LW5roUJ`) to #6004's converged design.
- 2026-10-29 21:25 — **Stage 6 B merged (#6041).**
  - Rung one reads every mate kind, closed transitively. A revolve minter arm and the replay's stated-axis substitution land with it.
  - Dual H review, tally 2:
    - R1's MAJOR, introduced here: refused bundles fed the classes. Fixed: only admitted bundles are read.
    - R2's MAJOR, pre-existing: a revolve's wall classification has no door row. B no longer builds on it; filed P1 on COINCIDE.
  - Row: `docs/dual-review-log/rows/PR-6041.md`.
- **Stage 6 C** (#6043): dual H review in, tally 0, fix pass running.
- **The D10 hold, brought up to date at Ev's request.**
  - `d10-one-way-to-say-intent-is-unbuilt` now states where the build stands. Its old text said the code still had every mechanism D10 retires.
  - `plan.md`'s stage state is updated.
  - A lane re-reads the ~26 rows held on the hold (`session_01VyvYnH4C7X55BN67GaEqMo`). It releases those whose ground is built and re-parks the rest on the specific remainder.
- 2026-10-29 23:30 — **Stage 6 C merged (#6043).** A profile drawn on a frame reads a face's trace, with the crossing vocabulary and typed refusals; the façade is `pncad::trace`.
  - Review: dual H, tally 0. Row `docs/dual-review-log/rows/PR-6043.md`.
  - Fix pass:
    - pncad-py's word parsing and dimension checks moved into the façade;
    - free AtX/AtY;
    - torus, Sym, loft and refusal rows;
    - one outward normal and one extent table;
    - six P3s filed.
  - The fix-pass head's `test (app)` was red only on the fast-set budget for two untouched m10_6 rows (1.052 s and 1.002 s). The re-run on the row commit was green.
- **Stage 6 D dispatched** (`session_01VyvYnKSgWzCxCxYDXAhVSQ`), curve tier only, per #6006. Stage 6 now waits on D and E (the viewer's tool).
- **2026-10-30, Ev answered #6064** ("this at least makes sense to start? ... we probably should treat cylinders the same way regardless of how they were made eventually but it doesn't have to be now"). It merges as written. The eventual direction is filed as `a-cylinder-reads-its-symmetry-by-how-it-was-made` (P3, design: the kind-by-value vs constructed-parallel question goes to the designer pair first).
- **2026-10-30, stage 6 D merged (#6060)**, dual H, tally 0, no MAJOR (row `PR-6060.md`).
  - One fix pass: traces compared by carrier, a red row per arm, the unreachable leg check and the symmetry lattice dropped for what they decide, and the `ReadTheAxis` prose corrected.
  - The reviewers found that the #6006 premise ("that face's kind's symmetry") is not what the code reads. It went to Ev as `[ev]` #6064, which is open.
  - `intent-stage6-is-built` waits on E alone. D10's umbrella waits on stage 6 and on CONTACTHOLD's declared-rung P0.
- **2026-10-30, stage 6 E dispatched** (`session_01VyvYnLgN4x5RNGfT71WRpC`), single FULL review. E is an INTENT row at P0 on the viewer's ground, so the sitting continues rather than handing the track back; check-in 103 had planned the handback. The crossing is announced on DOORS', VGEOM's and FORMS' logs, none of which has an orchestrator.
- **2026-10-30, the non-D10 tail rehomed** (Ev, 2026-10-30: reprice the rows below P0 and move each to the program it belongs in). 22 rows moved by `git mv`, ids and bodies unchanged, priority and cost set: `a-revolve-traced-tangent-has-no-axis-read-off-the-face` → COINCIDE (P3), `g-moves-four-main-reds-messages` → GREEN (P1), `split-widens-a-product-reading-one-of-several-cut-copies` → DOCTAIL (P1), `the-ruled-region-span-holds-two-stage6-scenes-off-the-registry` → CHARTTAIL (P2), `a-construction-is-called-the-root-of-its-own-space` → DOCTAIL (P4), `a-cut-of-a-nested-assemble-piece-reads-its-own-records-not-its-census` → HELDRECORDS (P1), `a-cylinder-reads-its-symmetry-by-how-it-was-made` → COINCIDE (P3), `a-line-past-a-cones-apex-crosses-its-phantom-nappe` → TRACE (P1), `a-lofts-later-section-may-not-be-drawn-on-a-frame` → TRACE (P3), `a-spline-or-approximating-face-has-no-trace` → TRACE (P3), `a-takeover-could-re-seat-a-pick-its-result-keeps` → DOORS (P3), `a-takeovers-rebind-cannot-reach-part-spelled-names` → DOORS (P3), `a-traced-profile-is-tangent-at-a-point-where-its-plane-holds-a-symmetry-axis` → COINCIDE (P3), `a-traced-profiles-first-pass-and-pen-cost` → TRACE (P4), `a-value-edit-never-re-asks-the-door-checks-that-read-values` → DOCTAIL (P1), `an-arc-runs-to-a-crossing-only-about-a-centre` → TRACE (P3), `an-assemble-of-a-family-counts-it-one-part` → WIRE (P1), `assembly-md-fences-a3-as-the-code-before-d10` → MSOLVE (P4), `a-remainder-reader-of-a-named-pose-over-the-cut-refuses` → DOCTAIL (P3), `a-ring-about-a-tilted-axis-slides-each-member-along-it` → MSOLVE (P1), `a-traced-profile-in-an-indexed-family-refuses-at-the-analysis-lanes` → TRACE (P2), `set-program-on-a-traced-profile-with-named-steps-refuses` → TRACE (P1). TRACE opened for the seven traced-profile rows, which no live charter fits. INTENT keeps its three P0 rows and `an-unread-body-is-no-finding`.
- **2026-10-30, stage 6 E (#6071), single FULL review.** One reviewer (`session_01VyvYnLT`, probes `review/intent-s6e-R1` @ fe6201c48) returned NOT-MERGEABLE-AS-IS at 1/3/4.
  - **MAJOR:** a planar cap picked as the sketch plane wrote a frame off the cap's edge, so the door reported `ReadTheAxis`.
  - **MINOR:**
    - frame moves were literals computed off the evaluation;
    - the frame decision lived in the viewer;
    - the plane pick did not set the plane.
  - **Ruled and fixed in one pass:** the frame's z is always the traced face's axis, and the plane pick places the origin by a vertex read off it (`Through{Axis{traced}, Point{vertex}}`). This is decided in the façade (`pncad::trace::traced_frame`, Python `Evaluation.traced_frame`) and refused typed where the plane is not square to the axis. The cap, top-cap and offset-cylinder rows prove `Structural`. The viewer builds through `drawn_profile`/`Chain`.
  - **A class the review named (S1):** the viewer's session repeated the body-operand block (`on_body` → `seat_read` → `body_of`). The pass folded it into `picked_body` for `sketch_on_face`, `add_mirror` and `face_read`.
  - **Filed:** the preview drawn off its frame is `vgeom/a-profile-drawn-on-a-frame-is-drawn-at-the-worlds-xy` (raised to P2, since this tool's preview shows it).
  - The orchestrator's read accepted the pass: the frame is built only from reads, with no literal.
- **Stage 6 closed 2026-10-30** (A #6023, B #6041, C #6043, D #6060, E #6071). D10's umbrella now waits only on CONTACTHOLD's bridging-rung row (P0).
  - The track stays `active`: `an-unread-body-is-no-finding` turned out to be D10 text and is now P0 (below).
- **2026-10-30, `an-unread-body-is-no-finding` raised P2 → P0**, and D10's umbrella now waits on it. `[operations]`' "an unread body is an unread variable, a finding" is D10 text (`[ev]` D′, 679b4abbf7) with no check; the exit walk read only superseded mechanisms. Designer pair next. The rest of INTENT's non-D10 tail is being repriced and rehomed (tracker lane `session_01VyvYnJwve9f1FDKZ7Bhh4R`, Ev's ask).
- **2026-10-30, Ev answered #6075 (merged).**
  - The unread-body sentence is reworded: what the person made (inserted or named) and nothing uses is a finding, and byproducts never are. Ev's deliberate-vs-byproduct comment drove round 2.
  - An unasserted `Measure` is reported ("simplest; maybe drop later"). Fork-log row 218 is filled.
  - `an-unread-body-is-no-finding` is dispatched, single FULL review.
- **2026-10-30, the unused check merged (#6081).** It is the last D10 sentence: what the person made (inserted or named) and nothing uses is a certified, advisory finding, default Warn. The corpus count was 5 over 50 documents; the gallery's two are filed as `the-gallery-documents-carry-what-nothing-uses`.
  - Review: single FULL (`session_01VyvYnH4`, probes `review/intent-unused-6081` @ e751f844), APPROVE-WITH-FIXES 0/3/4.
    - A named world was reported (the exemption sat on one pass of two).
    - The claim that "it answers when evaluation refuses" was false under the default config: any check's refusal discards the whole report. Filed as `coincide/a-checks-refusal-discards-every-residents-findings`.
    - A misplaced doc block.
  - One fix pass. The orchestrator's read accepted it: one "used" closure, with the world exempt there.
  - The lane pushed two census-only commits (the façade re-export and the binding census) after the freeze; the reviewed code is the frozen head's.
- **2026-10-30, CONTACTHOLD #6079 merged** (the last superseded mechanism). D10's umbrella stays open: DESIGN's state-of-the-build still lists one unbuilt D10 item, "an assertion quieting an unproven row no `Gap = 0` says". That is ATREST's `a-gap-over-adjacent-cells-states-nothing-of-the-row`, raised P2 → P0. The umbrella now waits on it alone, and its designer pair is next.
