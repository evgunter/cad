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
