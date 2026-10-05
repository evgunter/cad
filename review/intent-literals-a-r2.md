# Review r2 — PR #4065, INTENT-LITERALS PR A (definitions)

Frozen head `e8336050a2`, base `b553731da5` (merge-base with main). I did not read
PR #4065's comments, any other `review/*` branch, or another reviewer's output.
My probes and the mutation driver are in `review/r2-intent-literals-a/`. Each
probe was run inside the PR head's `crates/<crate>/tests/all.rs` and then
removed.

## Verdict: APPROVE-WITH-FIXES

The core is sound, and I checked it by running it:
- definitions bind correctly at f64, Interval, Dual and `Sym<Interval>`;
- the pushforward matches a finite difference;
- cycles and the expansion bound refuse at both doors;
- tokens expand through nested definitions;
- the lifecycle cascade is right;
- nothing moved for documents without definitions.

What is left:
- a viewer behaviour change that silently drops a tolerance;
- a quadratic cost that every document pays, definitions or not;
- four gaps where a plausible bug survives the PR's rows (my probes close each one);
- stale Python read-side docs.

None of these is MAJOR.

## Findings

**MINOR-1 — The viewer's text door turns a constant expression into an input-less definition, and the tolerance is dropped with no notice.**
- Where: `crates/viewer/src/session.rs` `set_param_text` (`constant_count` ~:1940, `DefineVar` commit ~:1949).
- Claim: C8.
- Demonstrated by execution (`r2_literals_a_viewer.rs`). Start from `base_r = 50 mm ± N(σ=0.1 mm)` and type `2 mm + 1 mm`, then `-(5 mm)`.
- Each one commits without refusal as one `DefineVar`. The variable becomes `Defined` with no inputs:
  - `doc.free(var)` is `None`, so the distribution is gone;
  - the variable stops being an analysis axis (MC and stackup lose it);
  - nothing in the outcome tells the user.
- Before this PR, both texts refused `ParamNotANumber`.
- The door is also asymmetric. It folds a constant *count* expression to a value (`(_, _, Some(count))`), but not a constant length or angle. So `2 + 1` is a value, while `2 mm + 1 mm` is a definition.
- Related: typing `3` over a Length parameter now refuses `VarKindFixed`, saying "the definition offered is of kind count". The user typed a value, not a definition. It is typed and honest, but it misleads.
- Confidence: sure (behaviour); likely (that it is unwanted).

**MINOR-2 — `Doc::definition_order` is O(n²) and recomputed on every new document, including those with no definition.**
- Where: `crates/editor-core/src/doc.rs` `definition_order` (:1276). Each placement re-runs `var_order.iter().find(...)` from the start, and each step calls `definition_reads`, which allocates.
- Claim: C1, cost side; values do not move.
- Why every edit pays: the memo is empty on every clone, so every edit's output and every `var_env` (`evaluate` builds two) recomputes it.
- Measured (debug, cold `var_env`, free variables only): 500 vars 31 ms, 1000 vars 140 ms, 2000 vars 616 ms. That is quadratic. Before, `var_env` was linear in the free variables.
- Same shape elsewhere:
  - `var_definers` (:1190) scans every variable per call, so `reached_through_definitions` / `var_readers` is O(n²);
  - `unread_anonymous_vars` (:1235) is O(n·rounds);
  - the `DocDiff` closure (`diff.rs:110`) uses `Vec::contains`.
- On a chain of definitions the declare door costs O(n²) per edit: building a 100 / 200 / 400 chain took 0.08 / 0.39 / 3.1 s (debug).
- Why it matters now: PR C mints a variable per written slot, so documents will hold hundreds to thousands of variables.
- Confidence: sure (complexity, measured); likely (that it matters by C).

**MINOR-3 — Four plausible bugs survive every row the PR adds.**
- Claim: C9. Demonstrated by mutation (`mut.py`). Each mutant was run against `intent_literals_a|intent_vars|doc::tests|persist::check` (65 rows).
- These survived. Each is killed by an r2 probe, and the real code passes every probe:
  - M23, `param_source.rs:220`: expansion only one level deep (`encode(definition, &|_| None, out)`). Nothing pins `g := h, h := 2w ≡ 2w`. Killed by `r2_nested_definition_token`.
  - M25, `doc.rs` `expansion_nodes`: iterating `var_order` instead of `definition_order`. This under-counts a variable declared before the chain it reads, so an 8191-node expansion is **admitted**. Killed by `r2_bound_for_an_early_declared_reader`.
  - M14, `definition_order`: dropping the `|| !self.vars.contains_key(read)` dead-read clause. `g := h + 1 mm` (declared first), `h := 2w`, then delete `w`: `g` then refuses "h has no binding" instead of `DefinitionRefused{h, …}`. Killed by `r2_refusal_chain_through_a_dead_read`.
  - M27, `refactor.rs:3478`: inline comparing the host's variable with the part's *un-remapped* definition. Inlining into a host that already holds the same named `w` and `h := w + w` (at different ids) would refuse `VarNameConflict`. The PR's row inlines only into an empty host. Killed by `r2_inline_into_a_host_holding_the_definition`.
- Two more survive, and both look harmless:
  - M9: removing the memo reset after GC (`edit.rs:4866`);
  - M15: `diff.rs:110` closing over `other` only.

  Both are redundant code (see Style S4).
- Killed by the PR's rows, as expected: M1 (the `binding` refused-lookup), M16 (the load `DefinitionVarKind` arm).
- Confidence: sure.

**MINOR-4 — Python's read side silently omits defined variables, and its documentation still promises them.**
- Where: `crates/pncad-py/src/py/doc.rs` `params` (:1511) and `vars` (:1529).
- Claim: C8.
- Demonstrated by execution: after `declare_var("height", parse_expr("base * 2.0"))`, `doc.params` lists only `base`.
- `params`'s docstring (Rust and `.pyi:3948`) still calls it "the read side of `DocEdit.declare_var`, and the only door that answers a whole parameter back". `declare_var` now takes an `Expr`, and nothing lists a defined variable's *name*.
- The Rust `vars` docstring still says "the document's variables … the named ones and the anonymous ones". Only the `.pyi` was updated to say "free".
- The PR's sweep pattern (`\.free\(|free_named\(`) cannot match a `free_vars()` iteration, so it missed both.
- Confidence: sure.

**NOTE-1 — The edit door and the load door name different variables for one expansion fault.**
- Where: `edit.rs:4339` searches `definition_order`; `persist/check.rs:620` searches `var_order`.
- Claim: C4.
- On one over-bound document, the edit door names the first variable in definition order and the load door names the first in declaration order.
- `tags.rs` says "one fact at the edit and load doors", but the two are separate searches.
- Inspection. Confidence: sure.

**NOTE-2 — A redefinition from free to defined drops the distribution with no `Maintenance` record.**
- Where: `edit.rs` `DefineVar`'s `Defined` arm.
- Claim: C7/C8.
- VR3 says this is right: a defined variable holds no distribution. Only the doc of `DocEdit::DefineVar` says so. The viewer path in MINOR-1 is where a user meets it.
- Inspection. Confidence: likely.

**NOTE-3 — The bound limits a variable's expansion, not a slot's token.**
- Where: the `encode` comment ("The doors bound the expansion").
- Claim: C4.
- A slot that reads a 4095-node variable k times writes about k·4096 nodes into a flow token. That growth is linear, not exponential.
- My probe only evaluated such a slot on an extrude, which writes no token, so the token write is unmeasured.
- The comment over-claims slightly. Inspection. Confidence: likely.

**NOTE-4 — The bound of 4096 has no scheduled re-measure.**
- Claim: C4 / spec §9.
- §9 says "corpus max ×16". The PR answers that the corpus holds no definition today, which is fair for A.
- After C/D, every slot formula *is* a definition, and nothing in `work/` re-takes that measurement. (Style Q6: a claim resting on a measurement owes a scheduled re-measure.)
- Confidence: likely.

## What I ran (private `CARGO_TARGET_DIR`, foreground, `--profile default`)

- `cargo nextest run -p editor-core --profile default`, in 3 partitions: **2745 passed, 0 failed**. That includes the PR's 22 rows, `m10_p_fence` ×3 (corpus geometry bit-identical, f64 and interval) and my 14 probes.
- `viewer`, the `panel_edits|refusal_concision_edits|panel_display|r2` subset: 51 + 1 passed.
- `crates/pncad-py/run-python-tests.sh`: **930 OK**.
- 10 mutants (above).
- Not run: the workspace-wide suite, the `ci` profile at ε 1e-6 / 1e-12, `demos/tour`, clippy, fmt.

## Claims

- **C1** — exercised:
  - the full editor-core suite and the corpus fence are green;
  - the diff carries no golden, corpus or digest file;
  - `r2_no_definition_is_the_old_env` shows `definition_order == var_order` and that `var_env` binds exactly the free variables;
  - a free variable's `encode` arm is byte-unchanged (inspection).

  Cost moved (MINOR-2).
- **C2** — exercised by `r2_every_lane_binds_the_chain`, on `a := w·s`, `b := a·s + w`:
  - Dual: seed on `s` gives `b.deriv == 2·s·w` exactly;
  - Interval: the box encloses all four corners;
  - `Sym<Interval>`: the env encloses the nominal;
  - the axes are `[s, w]` only.

  A count definition drives a pattern count (`r2_a_count_definition_drives_a_pattern`: 3 → 5 instances, `structural`, and `diff.vars` holds `k`). Refused definitions carry the slot address (the PR row, plus M1). Not exercised: a full `Sym<Interval>` *evaluate* of a document whose slot reads a definition (I checked the environment only).
- **C3** — exercised:
  - memo across clone, save/load and redefinitions (`r2_definition_order_memo_follows_edits`);
  - the PR's row-3 recompute count;
  - diff closure with a count definition.

  No stale result found. The memo resets are redundant with clone-empties (M9 survives harmlessly).
- **C4** — exercised: cycles through redefinition (probe and rows); a side redefinition (`z := w + w` grows `d10` to 6143 → refuses at `d10`); an early-declared reader; a 4095-node chain loads. A valid 4095 document is admitted. NOTE-1 and NOTE-3 apply.
- **C5** — exercised: nested expansion is equal (`r2_nested_definition_token`); the PR's rows cover `h ≡ 2w ≢ 3w`. "Nothing reads a definition's name" holds: `encode` panics on a name leaf, and both doors refuse one (load: `NamedReaderInDefinition`).
- **C6** — exercised: the stackup sensitivity of `b` against a central FD (h = 1e-6) agrees to 3e-11 / 4e-13, and to 1e-12 against the analytic `2ws`, `s² + 1`. Seed on a defined variable refuses (row 2, killed by mutation in the author's list).
- **C7** — exercised (`r2_lifecycle_edges`): deleting a named definition, or redefining it free, removes exactly its anonymous input; rename-to-anonymous of an unread definition refuses. The delete → readers-refuse path is the PR's row, plus my dead-read chain.
- **C8** — exercised: viewer (MINOR-1) and Python (MINOR-4). Each viewer commit is one edit (`history` +1 per accepted text). Python `define_var` round trip: the PR's class passes.
- **C9** — exercised by mutation (MINOR-3).

## Style

- **S1 (Q1)** — `VarDecl` and `VarDef` are two identical enums (`Free(FreeVar) | Defined(Expr)`). `kind()` is written twice, character for character (`var.rs:91`, `:148`), and so is `defined()`/`defined_mut`. It is disclosed as B's staging, but until B lands it is two spellings of one fact, joined by a `From` impl. **likely**
- **S2 (Q1/Q2)** — Each definition fault is a variant twice (`EditError::Definition*` and `SnapshotError::Definition*`), and each `Display` sentence is written twice, verbatim ("expands, through the definitions it reads, to {nodes} expression nodes, past the bound"; the cycle arrow loop). The bound search is also written twice, and the copies have already drifted (NOTE-1). The `tags.rs` comment "one fact at the edit and load doors" is the only thing joining them. **sure**
- **S3 (Q7)** — The memo is an `OnceLock` inside a value type whose `pub(crate)` fields any crate code mutates. It has a hand-written `PartialEq` that is always `true` and a `Clone` that forgets, which grew `IMPL_FILES_TODAY`. Freshness is kept by convention: "clear at each variable-table write", five hand-placed resets. A census was added to close a cache question, which is the brief's "a lane closing X adds a hand-written census". I would compute the order at the env door, or key it outside `Doc`. **likely**
- **S4 (Q2/Q3)** — The five `new.definition_order = …default()` resets (`edit.rs:4265`, `4866`, `5359`, `5391`, `5500`) are dead in practice: `new` is a clone and starts empty. M9 shows nothing can see them, and the PR body states "the door also clears it" as a guarantee. `diff.rs:110`'s `self` pass is likewise unobservable (M15). **likely**
- **S5 (Q4)** — `Doc::free_vars` (`doc.rs:1384`) still says it is "the ONE iteration base every lane reads … so no two lanes can disagree on which variables there are". The environment now has a second base, `definition_order`. **sure**
- **S6 (Q5)** — `pncad-py` `Doc.vars`'s Rust docstring and `Doc.params`' "read side of `declare_var`" are stale (MINOR-4). **sure**
- **S7 (Q1)** — `edit_payload.rs:495`: `DefinitionUnknownVarName` puts the *read* name in `param`, while every other `Definition*` arm puts the *defined* variable there. So `param` means two things across one error family. **likely**
- **S8 (Q7)** — The viewer's `unreachable!("a continuous literal remembers the unit…")` in `set_param_text` holds by the tuple shape three lines up, not by a type. And the count-only constant fold is an extra special case (MINOR-1). **unsure**
- **S9 (Q2)** — `check_var_def`'s new comment, "a definition's floats are its expression's literals, finite by construction", is repeated at `persist/check.rs`'s `edit_non_finite`. Both are correct today, and they are two sites stating one rule. **unsure**
- **S10 (Q1, sweep)** — The PR's sweep (`\.free\(|free_named\(`) is blind to `free_vars()` iterations. My sweep (`def()\.free()|free_vars()|\.free(`) over `viewer`, `pncad-py`, `pncad`, `demos` found the two Python getters (MINOR-4) and `props.rs:928` (intended). **sure**

**Style questions exercised:**
- Q1: prose and constant greps over the added lines; `4096` appears only at its definition.
- Q2: comment-reconciliation grep.
- Q3: mutation.
- Q4: the `free_vars` premise.
- Q5: Python docstrings.
- Q6: the bound's measurement.
- Q7: memo, viewer.
- Q8: partial. I read `doc.rs`'s variable section (~:1160–1700) end to end, not the whole of `edit.rs`.
