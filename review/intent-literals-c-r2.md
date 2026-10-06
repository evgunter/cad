# Review r2 — PR #4146 "INTENT-LITERALS PR C: a slot holds a VarId"

Frozen head `4b06b7e9595a4d2f4be8b5fd74142c483a9c2a89`; base
`35e4ac1829c60187ab0978ec550a2d5ca8b32d26` (merge-base with `origin/main`).
Private target dirs (`tgt-r2` for head, `tgt-r2-base` for a base worktree);
every run used `--profile default`, in the foreground. The probes are
committed beside this report (`review/intent-literals-c-r2-probes.rs`; they
were mounted into `crates/editor-core/tests/all.rs` locally). I did not read
PR #4146's comments or any other `review/*` branch, and I caught no
accidental glimpse of either.

## Verdict: APPROVE-WITH-FIXES

No MAJOR. Geometry did not move: f64 and, measured here, Interval both
hold with ids masked. Lowering, the lifecycle, the fresh table, split and
the mint's P1 fix all held under every probe I ran. Three MINORs:

- a D6 regression in the anonymous mint's preimage;
- a VR8 guard row that cannot go red under the mutant it names;
- the viewer hides the retirement of a *toleranced* anonymous variable.

## Findings

**MINOR-1 — A definition's display unit enters the mint preimage (D6 regression).** Claims C4/C3.
`crates/editor-core/src/mint.rs` `Held::of` (l.167–186) serializes
`Held::Defined(&Expr)`. In C a stored `Expr` still holds `Lit` leaves with
their `display_unit`, and `MintingEdit::insert` no longer erases display
units (the `erase_display_units` calls were removed). So two inserts that
`bit_eq` cannot tell apart, `w + 125 mm` and `w + 0.125 m`, now mint
different anonymous ids and therefore different node ids.

- **Demonstrated by execution** (probe `r2_d6_definition_display_unit_moves_the_node_id`):
  - head: nodes `11675160046155690227` and `9976061531424071179`.
  - Lone values, by contrast, still mint one id (`3500702910806010398` both times).
  - base: the same pair mints one id, `9941897939934870733` twice (probe `r2_d6_base`).
- The row that guarded this, `mint::tests::the_display_unit_is_not_part_of_what_an_insert_hashes`, was narrowed to lone values. So the case is unguarded.
- PR D probably heals it, since a definition then holds no float leaf. But §11 requires each PR to be green and right on its own.
- Confidence: **sure** (behaviour); **likely** (that it matters before D lands).

**MINOR-2 — The VR8 Sym row cannot go red when the rule is removed from the Sym lane.** Claims C9/C6.
`crates/editor-core/tests/intent_literals_c_slots.rs`
`an_untoleranced_variable_is_a_constant_in_the_symbolic_lane` (l.750) says it
"breaks if the lane binds an untoleranced variable as a symbol".

- **Mutant:** in `analysis.rs` `var_env_over` (l.1075) I disabled the untoleranced-constant arm (`&& false`), so every untoleranced variable binds `nominal + Sym::param_over(id,[0,0])`, the pre-§11 behaviour.
- **What happened** (`--profile default`):
  - The row passed, and so did every other row in `intent_literals_c_slots`, `intent_vars_3_readers` and `seat6_param_source`.
  - A zero-width symbol pair still decides `x − y` Zero numerically, and the row asserts only `Sign::Zero`. It never asserts that the binding is a constant or that no symbol was involved.
  - The mutant shows only as wall time: `m10_4_stackup_interval::*` rows each ran more than 20 CPU-minutes before I killed them.
- So the rule's Sym half is guarded only by a timeout in the slow set.
- Confidence: **sure**.
- The other mutant I ran (Held::of drops the value, reopening P1) went red in `asm_parent_held_names::sibling_versions_mint_two_node_ids_and_neither_resolves_the_others_names`. Ruling 1 is guarded.

**MINOR-3 — Retiring a toleranced anonymous variable is silent in the viewer.** Claims C8/C5.

- `crates/viewer/src/frame.rs` `maintenance_notice` (l.1512–1517) returns `None` for every `AnonymousVarRemoved`, and `session.rs` l.2579 filters them from the Apply-time stranded count.
- The comment's premise, "nothing the person sees went with it", fails when the variable carried a distribution. Spec Q6 has typed text re-lower a slot (`SetParam`), and the path editor's dropped steps re-lower too. Either way a toleranced dimension's distribution is discarded, and VR8 says that tolerance makes the variable an analysis axis. The person is told nothing.
- Ruling 5 is disclosed, but its premise covers only untoleranced variables.
- Confidence: **likely** (by inspection; I did not drive the GUI).

**NOTE-1 — `SlotReadsUnmintedVar` does not exist.** Claim C2/C9 premise.
Spec §4 ("Slot-reads-minted") and §8 row 14 name `SlotReadsUnmintedVar { node, slot }`.

- The code refuses with the pre-existing `SnapshotError::ReaderOfUnmintedVar { node, var }` (`persist/check.rs` l.404), which carries **no slot address**.
- `the_load_door_reads_every_slots_variable` asserts that error.
- The deviation is not disclosed in the PR body, and the spec text is now stale.
- The brief's claim list repeats the spec's name, so the dispatch premise is off too.
- Confidence: **sure** (inspection and the row).

**NOTE-2 — `Node::written`'s doc promises more than it does.** Claim C3.
`node.rs` l.4355–4366 says "a document rebuilt by re-inserting its nodes in
order is the document, ids included". `Doc::written` replaces each anonymous
variable by a value literal, which in C can carry no distribution.

- Re-inserting therefore splits an anonymous variable shared by two slots (via the fresh table, or read across nodes by id) into two.
- It also drops a distribution.
- The users are `diefillet::corpus_text`, `test_support` and the path editor's base. The path editor maps unchanged arguments back to their reader (`carry_unmoved`), so it is safe.
- Confidence: **likely** (inspection; my cross-node-read probe confirms the sharing exists).

**NOTE-3 — `range` on a slot now widens the slot's variable, which may be shared.** Claim C7.
`range.rs` l.653: a slot reading a named `w`, or an anonymous variable shared through a fresh table, is widened in place.

- Every reader of that variable then moves, while `CertifiedRange::field` names only the slot.
- Before C such a slot refused `SlotIsNotALiteral`.
- This conforms to the spec, but it is a quiet semantic change. Neither the module doc nor VR8 says that a slot's range is its variable's range.
- Confidence: **likely**.

**NOTE-4 — The PR body contradicts itself about the tour.**
It says "No tour frame or narration moved", and also "The only text lines that move are the symbolic-identity tallies (+1023 decisions …)". Confidence: **unsure** which is meant.

**NOTE-5 — VR8 and §11 text are faithful to the code.**

- `is_axis` (`analysis.rs` l.335) is the rule's only home.
- `analyzed_box`, the stackup entries (`stackup.rs` l.1080) and `var_env_over` all read it.
- MC draws `varying()` of the box.
- `seed_env` still seeds an untoleranced variable.
- `range` gives the variable a distribution in the derived document, so the Sym tier binds it as a symbol there.
- I swept every `free_vars()` reader and every `FreeVar::Continuous {..}` match; none treats an untoleranced variable as an axis by another path.
- The Sym `DriveMemo` keys on syntax (a `Lit`'s bits versus a `Param` symbol), and it is drive-scoped. So a Zero decided with `x` as a constant cannot be replayed over a widened `x`.
- Spec §7 ("Every anonymous continuous free variable is an axis") and §10 Q3/Q4 still state the superseded rule. §11 overrides them by the spec's own terms.
- Confidence: **likely**.

## Style

- **Q1 — "Hide anonymous retirements" is spelled three times.** At `viewer/src/frame.rs:1517`, `viewer/src/session.rs:2579` and the partition in `viewer/src/pane/profile.rs:1363`. Nothing ties them together. One more place to look: any OpOutcome consumer in `pncad-py`. **likely**
- **Q1 — "This slot reads its own anonymous free variable" is spelled three times.** `doc.slot(..)` plus `var_name(..).is_none()` plus `free(..).is_some()` appears in `viewer/src/props.rs:1027` (`slot_edit`), `props.rs:1337` (`slot_unit_edit`) and `viewer/src/session.rs:~364` (`carry_unmoved`). It is a predicate wanting a home on `Doc`. Sweep `tree.rs` and `sketch.rs` for a fourth copy. **sure**
- **Q2/Q7 — `lower_value` is not quite "one walk".**
  - `edit.rs:961` lowers in a walk. On failure it re-lowers every addressed row through `Lowering::fault` (l.866) to find the address. That second lowering is a second spelling of the fault, and it runs against `new` after the walk's partial mints.
  - The doc comment asserts that "the one such field" (a count beside a listed rule) is the only formula `try_map_slots` maps and `rows` does not address. Nothing enforces that.
  - The anonymous GC (`doc.rs:1233`, `node.exprs()`) and the split carry (`refactor.rs` `node_var_reads`, `VarCarry::author`) also rely on `exprs()` covering every stored `VarId`. **unsure**
- **Q5 — `mint.rs` module doc l.14–18 is stale.** It says "neither the name (VR2) nor the value is in the preimage". `MintingEdit::DeclareAnonymous` puts the value in, and that is ruling 1. The test doc at l.621 says the variable's preimage is "its kind alone". Both now state the opposite of the code. **sure**
- **Q5/Q7 — The FIXED-axis vocabulary is vestigial.**
  - `analysis.rs` l.288 and l.307 (and `axis_box_mass`) document "an axis built with no distribution". `analyzed_box` can no longer build one.
  - `BoxAxis::Fixed` and `OffsetInterval::FIXED` survive only for zero-width toleranced axes.
  - Under VR8 the module header's FIXED paragraph and these branches describe a state the door no longer produces. **likely**
- **Q3 — `an_untoleranced_variable_is_a_constant_in_the_symbolic_lane`** asserts in a direction the mutant also satisfies (MINOR-2). `a_toleranced_variable_is_an_axis…` pins `symbolic_zero` counts; the untoleranced row should be its mirror and is not. **sure**
- **Q4 — The D6 premise was cited and then invalidated.** `Lit::eq` (`expr.rs` l.818) still says the display unit is "never part of expression identity (D6)". The mint is now an identity that reads it (MINOR-1). This is the "code drifted from something meant to hold" case, not doc rot. **likely**
- **Q6 — Two deviations are undisclosed and unscheduled:** the error named in NOTE-1, and the narrowing of `the_display_unit_is_not_part_of_what_an_insert_hashes` to lone values. **sure**
- **Q8 — Not exercised.** `edit.rs` (~6000 lines, the largest file touched) was read only over its lowering, door, GC and fresh-table regions, not end to end. **sure**

## Claims exercised

| Claim | Exercised | How |
|---|---|---|
| C1 | yes | `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked` passes untouched at head. I also built an **id-masked Interval digest** (per document and in total) on base and on head: all 29 corpus documents are bit-identical (`fixture+all 85fab9a01bfc01e7` on both). So the re-blessed Interval fence moved for ids only. I diffed the `m10_p_fence` and `edit_placement_corpus_bits` re-blesses. **Not run:** tour frames, MC sheet. |
| C2 | partly | Ran the `intent_literals_b_door` sweep (passes). Probes: a fresh entry that reads itself → `FreshUnheld`; an entry read only by an unread entry → `FreshUnread`; a stale (retired) anonymous id at a new slot → `SlotUnresolvedVar`. All typed. Python and viewer doors not run. |
| C3 | partly | `intent_literals_c_slots` row 8 and viewer `profile_edit`, `panel_edits`, `edit_maintenance` (51 rows) pass. NOTE-2 by inspection. |
| C4 | yes | Same value in two edits gives distinct ids. P1 mutant goes red. D6 regression (MINOR-1). `MintLogOrder` read, not probed. |
| C5 | yes | A shared fresh entry survives rewriting one reader and retires with the last. An anonymous variable read across nodes by id is not retired, and the document loads. Load refusal row passes. |
| C6 | yes | `is_axis` sweep, Sym-mutant run, `DriveMemo` keying read (NOTE-5, MINOR-2). |
| C7 | partly | Split rows and `docm9_range` pass. NOTE-3 by inspection. |
| C8 | partly | Viewer rows pass. **Python not built or run** (no wheel built in this session). |
| C9 | partly | Two mutants: P1 caught; VR8-Sym not caught by its row (MINOR-2). |

Rows run green at head (`--profile default`): `intent_literals_c_slots`,
`intent_literals_b_door`, `m10_4_stackup*`, `intent_vars_3_readers`,
`docm9_range`, `seat6_param_source`. That is 91 tests, 91 passed.
