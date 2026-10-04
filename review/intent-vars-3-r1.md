# Review r1 — PR #4027 "INTENT-VARS-1 PR 3: readers read ids" @ b07891634f

Base `5f768f6b8c` (merge-base with origin/main). Private target dir; editor-core runs use `--profile default`, slow set included. I did not read PR #4027's comments or any other `review/*` branch. My probe file is `review/intent-vars-3-r1-probes.rs.txt`. To run it, drop it into `crates/editor-core/tests/` and register it in `tests/all.rs`.

## Verdict: APPROVE-WITH-FIXES

Every central claim I checked holds under execution:
- a rename moves no token and no memo entry, and is not structural;
- delete leaves readers unresolved, including on the incremental path;
- lowering happens at one door and mints one id;
- authored-log replay is exact;
- split and inline remap correctly;
- geometry and the pins moved only by ids.

The fixes are one C2 gap and a set of test holes:
- a refusal re-spoken after a rename keeps the variable's old name;
- four of the PR's new mechanisms survive mutation of the entire shipped suite in every language.

Nothing found is MAJOR.

## Green runs (head, unmutated)
- editor-core, default profile: **2705/2705** pass. That is the shipped 2698 plus my 7 first probes; 108 were skipped by the profile.
- `intent_vars_3_readers`: 11/11.
- viewer + pncad + pncad-py (Rust, no `app` feature): 1096/1096. This ran under editor-core mutants M3, M6 and M8, so those crates' own code was at head.
- Python suite (`run-python-tests.sh`): **927/927**, `test_variables.py` 4/4.
- Not run by me: viewer `--features app`, `demos/tour` clippy, the ε = 1e-6 and 1e-12 rows.

## Findings

**MINOR-1 (C2) — a variable named in a refusal is not re-spoken after a rename.** Demonstrated by execution; confidence **sure**.
- `EditError::respoken` (`crates/editor-core/src/edit.rs:2091`) re-speaks every node a refusal names, but leaves each `var: SpokenVar` field (`SlotVarKind`, `SlotUnresolvedVar`, `PayloadVarKind`, `VarNameTaken.holder`, …) as it was raised. `SpokenVar` has no `respoken`.
- The viewer's `Refusal::respoken` (`crates/viewer/src/session/refuse.rs:436`) passes `DrivenByExpression { params: Vec<SpokenVar> }` and `ParamNotANumber { var }` through as `unspoken`.
- Probe `p_r_respoken_keeps_the_old_variable_name` refuses a `SlotVarKind` on `ang_old`, renames it to `ang_new`, labels the node, then calls `err.respoken(&doc)`. The output is `ang_old is declared angle but Fillet "blend" (41d8064506e7) …`: the node is re-spoken and the variable is stale.
- Viewer reach is narrow: `frame::batch_refusal` re-speaks a batch's refusal against the batch's end document, so it bites when a rename shares a batch with the refusal. That is **likely** rather than demonstrated.
- This is the same class as the memoized-text fix the PR did make (`HeldNodes.vars`). That fix covered `Speaker::held` and missed the respoken path.

**MINOR-2 (C7, C9) — inline's reader remap is tested nowhere.** Shown by a surviving mutant; **sure**.
- Mutant M6 deletes `remap_node_vars(node, &var_map)` in `inline` (`crates/editor-core/src/refactor.rs:3435`).
- It survives the full editor-core suite (2698/2698), viewer + pncad + pncad-py Rust (1096/1096) and the Python suite (927/927).
- Row 12 only exercises inline's `AnonymousVarCrossesCut` refusal. No shipped test inlines a part whose carried nodes read a variable, so neither "re-points readers at the host's ids" nor Q4's merge-by-name is covered under ids.
- The code itself is right. Probe `p_o_inline_carries_and_merges` checks two cases, a host without `d` and a host already holding a `bit_eq` `d` under an extra variable. In both, the reader lands on the host's id, the result saves and loads `bit_eq`, and it evaluates clean. That probe goes red under M6.
- Split's remap (M7) is caught by row 12.

**MINOR-3 (C3, C9) — the measure content key's id input is unguarded.** Surviving mutant; **sure**.
- Mutant M5 replaces `h.write_u64(var.0)` with nothing (`crates/editor-core/src/eval/mod.rs:6289`). This is the PR's sweep hit 1, and the mutant survives the full editor-core suite.
- Consequence: two value leaves reading different variables at equal literal bits would share a key. Under a seed or a parameter box that could serve the wrong derivative from the memo. I reasoned that consequence but did not construct it, so it is **likely**.
- The fix is in place, but no row pins it.

**MINOR-4 (C4, C9) — lowering on `Recording::insert` is untested.** Surviving mutant; **sure**.
- Mutant M3 removes `lowered(...)` from `apply_insert` (`edit.rs:4596`), which is the `Recording::insert` path. It survives the full editor-core suite and the viewer, pncad and pncad-py Rust suites.
- Row 7 drives `apply` → `apply_with` only.
- On head the path is correct. Probe `p_i_recording_insert_lowers` checks that a named node inserted through `Recording::insert` mints `apply`'s id and gives a `bit_eq` doc, and it goes red under M3.

**MINOR-5 (C4, C9) — the door's two new "unresolved reader" refusals are never driven through the door.** Surviving mutant; **sure**.
- Mutant M8 makes `check_reads` skip `Dead` and `Unminted` faults (`edit.rs:781`). It survives the targeted editor-core set and all 1096 viewer, pncad and pncad-py Rust tests.
- So nothing applies an edit that writes `Expr::var(dead_or_unminted)` and expects `SlotUnresolvedVar` or `PayloadUnresolvedVar`. Under M8 the door would store a reader that `save` then refuses (`ReaderOfUnmintedVar`).
- On head the door does refuse: probe `p_f` got `SlotUnresolvedVar`.

**MINOR-6 (C4) — the spec's door post-condition does not exist.** Inspection; **sure**.
- Spec §5 says a `Name` in a stored document is guarded by "the door post-condition and load walk 4, plus `UnloweredName`". `door()` (`edit.rs:4616`) has no post-condition. The only `named_reads` sites in the edit layer are `lowered` and `expr_dim_of`.
- The invariant holds today by inspection of each writing path:
  - `InsertNode`, `SetProgram` and `SetOffset` go through `check_node_slots`;
  - `SetParam`, `SetStructuralParam` and `SetExpression` go through `set_slot`.
- So it rests on every future writer remembering `check_reads`. The deviation is not disclosed in the PR body.

**NOTE-1 (C1) — geometry did not move; the GUI renders did change.** **sure** for what I measured; **likely** for the tour.
- I re-checked the golden independently. `golden.cad` and `plate_param.pncad`, JSON-normalised (ids → token, `Param`/`Var` → READ, mint log sorted), are **identical** base vs head.
- The `[render]` commit re-baselined 8 `demos/renders-gui` cells. I pixel-diffed 3 of them (`datum-extrude-profile-1230e269`, `instantiatepart`, `gauge-instantiatepart4-mate3`). In all three the 3-D viewport is unchanged; every changed pixel is in the side panel.
- What changed there is the node-id text, and the **parameter rows are now in declaration order** rather than alphabetical (e.g. `depth, length, thickness` → `length, depth, thickness`).
- The PR body says "the tour is not re-baselined", which is true of `demos/tour`'s frames. It does not mention the renders-gui cells, or that the panel order is a user-visible change.
- The PR's "id-free corpus digest" was a one-off measurement and is not committed. Nothing re-takes it.

**NOTE-2 (C2) — unparsed formula text depends on the speaker.**
- Documentless `Display` (`NodeError::to_string`, `Speaker::TAG`) now writes a reader as `#3da0c3a822ef260e`, where it used to show the name. Probe `p_d`'s `PLAIN` line shows this.
- Spoken with a document, it re-speaks correctly across a rename. Probe `p_d` checks this and goes red under mutant M4, as does the shipped `pattern_spacing_and_step`.
- Confidence: **sure** (behaviour), **unsure** whether any user-visible surface renders documentless.

**NOTE-3 (C4) — `SetExpression` checks the whole rebuilt slot.**
- A `SetExpression` that writes a live sub-leaf into a slot also holding a dead reader refuses `SlotUnresolvedVar` (probe `p_f`).
- That is consistent with "the slot the edit writes", but it is broader than the PR body's "only leaves the edit writes are checked". **sure**.

**NOTE-4 (C5, C6) — exercised and held.** **sure**.
- Incremental evaluation after `DeleteVar`, and after a re-declare, keeps the reader refusing `UnresolvedVar` (`p_a`).
- A value change after a rename recomputes and gives the new radius (`p_b`).
- `DeleteNode` of an anonymous variable's last reader removes the variable and reports it; with two readers, replacing one keeps it (`p_c`).
- An authored log with rename → re-declare `w` → reader → delete replays `bit_eq` and saves/loads as a log over an empty snapshot (`p_h`).
- `var_order == vars` on both sides of a split and after inline (`p_o`).
- Load walks 4–6 run in spec order (`check.rs` `Walk::run`).

## Mutation table (C9)

"Targeted" means the editor-core suites named in my runs; "full" means the whole editor-core suite. Rows marked *also binding* were also run against viewer + pncad + pncad-py Rust.

| mutant | shipped result | killed by |
|---|---|---|
| M1: skip the anonymous GC in `door` | killed | row 9 |
| M2: `RenameVar` structural | killed | row 1 |
| M3: `apply_insert` unlowered | **survives** (full, also binding) | my `p_i` |
| M4: `Speaker::formula` no-op | killed | `pattern_spacing_and_step`, my `p_d` |
| M5: measure key without the id | **survives** (full) | — |
| M6: inline without remap | **survives** (full, also binding, Python) | my `p_o` |
| M7: split without remap | killed | row 12 |
| M8: door ignores dead/unminted | **survives** (targeted, also binding) | — |
| M9: `DeleteVar` leaves `var_order` | killed | rows 2, 10 |
| M13: `T_VAR` token without the id | killed | row 5, `seat7`, the `param_source` census |

## Style

Questions exercised: Q1, Q2 (grep pass), Q3 (via mutation), Q4, Q5, Q6, Q7, Q8 (partial).

- **S1 (Q1) — two spellings of an unnamed variable.** `SpokenVar` displays an unnamed variable as `variable <12 hex>`, while `unparse`, `Speaker::formula` and Python `Expr.text` write `#<16 hex>`. A refusal can carry both forms in one sentence (a `SlotVarKind` on an anonymous variable beside a formula). Spec Q1 ruled the `#` form for text only; nothing reconciles the two. **likely**.
- **S2 (Q7) — names restored by string rewriting.** `Speaker::formula` (`spoken.rs:567`) re-parses rendered text, scanning for `#` plus 16 hex digits, to restore names. Memoized refusal text holding ids, then patched back by a string scan, is fragile: any `#`-prefixed 16-hex run in a formula's text would be re-spoken. Storing the `Expr`, or a list of ids beside the text, would be the typed form. It is "not how I'd do it", and it was justified at length in the PR body. **likely**.
- **S3 (Q1) — two walkers over a node's expressions.** `node_exprs_mut` (`edit.rs`, after `exprs_mut`) answers payload *or* rows; `doc::node_exprs` (`doc.rs:1442`) answers rows *and* payload. Today they agree only because `Measure` and `Assertion` carry no rows. Both also miss the `Explicit`-rule count expression that `check.rs`' `first_payload_read_fault` docs admit is unwalked. Look for a third copy in `refactor::remap_node_vars` / `node_var_reads`. **likely**.
- **S4 (Q2, Q4) — wrong or stale recourse text.**
  - `UNDECLARED_PARAM_RECOURSE` ("declare it first") survives, although spec §2 lists it as deleted. The viewer's `NoSuchParam` (`refuse.rs:783`) still renders it.
  - For a *deleted* variable, "declare it first" is wrong advice: a re-declare mints a new id the reader never reads.
  - The door's own unresolved arms say `READ_A_HELD_VAR` (`edit.rs:1937`), so one fact has two recourses.
  - **sure** that the constant remains; **likely** that it is wrong advice.
- **S5 (Q4) — stale comment.** `eval/parts.rs:636-646` still says a parameter box and a seed are "a set of THIS document's parameter **names**" and "collide by name". Both are id-keyed now. **sure**.
- **S6 (Q5) — `DocEdit` header omits the new doors.** The top-level doc (`edit.rs:34-57`) enumerates the "variable family" as Declare, Define and the three `SetVar*`. It omits `RenameVar` and `DeleteVar`. **sure**.
- **S7 (Q2) — a redundant kind check.** `lower_names` refuses to lower at a mismatched kind, and then `check_reads` re-derives that same mismatch from the leftover name (`lowering_scope` + `expr_dim_of`). The `(Some, Some)` arm maps to `VarReadFault::Kind` even when `declared == referenced`; that is unreachable today but wrong if reached. It is one rule split across two functions to keep a refusal order. **likely**.
- **S8 (Q6) — `range.rs::synthetic_name` mints a name in the kernel.** That is what VR2 ("the kernel mints no name") and §8's Q2 ruling forbid. The spec allowed it to stay internal, and the PR body keeps it with no scheduled follow-up; it owes a `work/` item. **likely**.
- **S9 (Q6) — a measurement-backed claim with no guard.** The PR's "geometry did not move" rests on an id-free corpus digest that is not committed. A claim resting on a measurement owes a guard or a register. **likely**.
- **S10 (Q3) — row 2's main assertion was weaker than its fixture.** It evaluates from `None`. The incremental path, which is the stale-memo risk spec §5 names, is untested there; my `p_a` covers it and passes. **sure** (that it is a coverage gap).
- **Q8.** `edit.rs` (6203 lines) is the largest touched file. I read lines 1–700 and every changed region, not the full file. One thing seen in that read: each `DocEdit` arm and every exhaustive match grew two more arms this PR, and the "no wildcard" lists (`writes_a_mates_datum`, `exprs_mut`, the `EditError` address/subject lists) are now five parallel hand-lists of the same 30 arms. **unsure**.

## Claims exercised

| claim | exercised | result |
|---|---|---|
| C1 | yes | holds (NOTE-1). The ε rows and the tour frames I did not rerun. |
| C2 | yes | holds except MINOR-1. |
| C3 | yes | holds; the sweep is clean on my own grep shape. The guard is missing (MINOR-3). |
| C4 | yes | holds; replay exact. MINOR-4/5/6 are test and guard holes. |
| C5 | yes | holds. |
| C6 | yes | holds; probes plus shipped rows. |
| C7 | yes | holds (`p_o`), but untested in the shipped suite (MINOR-2). |
| C8 | yes | holds. Python 927/927 and the viewer rows pass. Python never reads stored slot expressions back, so `Expr.params`/`.text` lose nothing. Viewer `app` features not run. |
| C9 | yes | mutation table above. |

Accidental glimpses: none.
