# MSOLVE-7 — The member walk's residue: one environment, one seat, one account, one attribute (spec)

Unit of the `msolve` program. Item `work/msolve/MSOLVE-7.md`. Answers
four rows on `work/msolve/`, read them in full before the code:
`mate-solve-rebuilds-the-nominal-environment-per-check` (EVAL-10's
style review), `axis-datum-names-the-pattern-where-the-evaluation-
names-the-transform` (EVAL-11's disclosure), `part-over-a-nested-
pattern-reads-the-flat-index-at-check-reference` (EVAL-6's residue)
and `mate-primitive-accepts-a-stray-field-the-module-docs-say-refuses`
(CENSUS-INERT-DENY's sweep). **Track:** kernel change to the solve's
cost, to where one refusal is seated, and to what the mate wire
accepts; no verdict moves — one style review plus a correctness arm
(§Review). No A/B row.

## What the tree says now

1. **The nominal environment is rebuilt per check.** `check_reference`
   and `derived_offset` (`crates/editor-core/src/mate/member.rs`) each
   open with `doc.param_env::<f64>()`, and `solve_document`
   (`mate/solve.rs`) calls the first twice per live mate (the read
   loop, `MateSide::A` then `B`) and `fold_pair` calls the second twice
   per spanning-tree edge. The environment is a pure function of the
   document (a `BTreeMap` built from `doc.params`), so the copies
   agree; what is paid is the allocation per call and one more place
   the sentence "the nominal environment is the document's own, under
   no box and no seed" has to stay true. The evaluator closed its own
   copy of this class in EVAL-9/10: one environment per evaluation,
   carried on `wire::LaneEnv::nominal`.

2. **One condition, two seats.** `pattern_map`'s circular arm asks
   `axis_datum(doc, *axis)` for the rule's axis operand and wraps its
   refusal with `here` — the PATTERN's id. `axis_datum` answers a
   non-datum operand through `crate::eval::node_value_kind`, which
   walks a `Transform` chain to its first non-transform node and
   refuses `MissingInput { input }` when a transform's input dangles.
   So the recipe road seats that refusal at the pattern, while the
   evaluation (`eval_node`) seats the same `MissingInput` at the
   TRANSFORM and poisons the pattern through it. Unreachable through
   `apply` (dependents cascade on delete; the load validator holds
   liveness), so no row pins it and none can be written through the
   doors — which is why it is a seat to correct, not a verdict.

3. **The flat index is decided; its account is what remains.** EVAL-6's
   fix pass landed the decomposition: `check_reference` folds the
   name's copy chain below a `Part` through every pattern level down
   to the next `Part` with `names::flat_body_index` (the evaluator's
   own placement-major layout) and refuses `PartSelectsAnotherCopy
   { named: flat, selected }` when the `Part`'s index disagrees; the
   four-case row is `msolve1_transform_aware::a12_a_part_over_a_
   nested_pattern_agrees_in_the_flat_index` (`k = j`, `i ≠ 0` is a
   disagreement, as the row's doc says). The item's residue is two
   questions for MSOLVE: does the check carry its own account of the
   index space, and should the walk name a copy by `(j, i)` rather
   than by a flat `Part`. **Ruled here:** the walk keeps naming by the
   flat `Part`. `Part(k)` is the document's vocabulary and ruling 2137
   made `k` select the flat body over a nested `Instances`; a `(j, i)`
   spelling would be a second vocabulary for one selection, and the
   check already speaks the flat one through the evaluator's single
   layout function. The account is owed at the check, not a new
   name.

4. **One hole in the mate wire.** `MatePrimitive` (`mate.rs`) derives
   `Deserialize` under `rename_all = "snake_case"` with no
   `deny_unknown_fields`; its `PlanarRest { offset }` is a struct
   variant with a named field, so a stray key on it loads and is
   dropped — `{"planar_rest":{"offset":1.0,"stray":2.0}}` parses to
   `PlanarRest { offset: 1.0 }`, established by execution on the row.
   `persist/mod.rs`'s module docs rule the opposite for the format:
   a NEWER document carrying a field this build lacks refuses where
   the owning wire type carries the attribute, and a stale reader must
   not silently drop data. `Alignment` and `MateFrame` carry it;
   `MatePrimitive` is the one field-bearing mate type that does not.

## What the unit builds

**1. One environment per solve.** `solve_document` builds
`doc.param_env::<f64>()` once and passes `&ParamEnv<f64>` down:
`check_reference(doc, env, …)` and `derived_offset(doc, env, …)` take
it, and `pattern_map` / `transform_map` / `node_slots` keep the `&env`
they already take. `fold_pair` receives it from its caller. No cache,
no `OnceCell`, no field on a struct that outlives the solve: a
parameter, the same shape EVAL-9/10 gave the evaluator. The sentence
about the nominal environment moves to the one site that builds it
(`solve_document`'s doc) and the two former sites point there.

**2. The refusal is seated where it was raised.** `axis_datum` returns
`Result<&Node<P>, Refused>` — the `(node, kind)` pair the derivation
road already uses — and seats each arm itself: a missing axis operand
and a wrong operand kind at the PATTERN (they are the pattern's own
wiring, as the arm's comment says today); a dangling transform input
found while classifying the operand at the TRANSFORM whose input
dangles, which is where `eval_node` seats it. To know that transform,
`crate::eval::node_value_kind` returns the raising node beside the
kind (`Err((at, kind))`) — a change in `eval/mod.rs` announced as a
seam in the PR body, with both callers updated (`member.rs`
`axis_datum`; `eval/wire.rs`'s operand door, which keeps its own
seating and must be shown unchanged in what it reports). `pattern_map`
stops wrapping `axis_datum`'s result with `here`. The `wire_operand_
door` row that greps the two function heads keeps matching.

Because the condition is unreachable through `apply`, the row is a
unit test beside `axis_datum` on a hand-built `Doc` (the same way
`msolve6`'s A4 seeded an unboundable face): a circular pattern whose
axis operand is a `Transform` over a dangling input refuses
`PlacerRefused { placer: <the transform>, error: MissingInput { input:
<the dangling id> } }` from `derived_offset`, and the same document's
`eval_node` reports `MissingInput` at the same transform — one row,
both roads, one seat.

**3. The index space's account, and the item closed by citation.**
`check_reference`'s doc names the index space in one place: a `Part`
over a nested `Instances` selects the flat body `j·M + i` of the
placement-major layout `names::flat_body_index` defines, and the check
folds the name's chain through that function so the two readers
cannot drift; the `(j, i)` question is answered at the item (ruled
above) and not in the code. If the doc already says this, the unit
changes no word there and the PR cites the paragraph; the item closes
either way with the citation and the row's name.

**4. The attribute, the census, the row.** `#[serde(deny_unknown_
fields)]` on `MatePrimitive`; the `deny_unknown_fields_census`'s
`ATTRIBUTE_SITES_TODAY` entry for `mate.rs` moves from 2 to 3 (a
re-baseline: the sibling row proves the attribute has a field to
deny); one row in `crates/editor-core/tests/` that the stray key on
`planar_rest` refuses through the LOAD door (`persist::load` on a
document whose mate alignment carries it, the typed `Unreadable` arm
the format uses for a stray field), and that the same alignment
without the key loads. The persist module docs' sentence stands; the
row is its proof for this type.

## Acceptance

- **A1** `param_env` is built exactly once per `solve_document`,
  pinned by a row that counts `doc.params` reads through a probe or
  by the signatures alone if no probe exists — say which.
- **A2** The two-roads row of §2: the transform is the seat on both.
- **A3** No verdict moves: every mate row in the workspace passes
  unchanged (`asm_r2a`, `asm_r2b`, `mate1_*`, `msolve1..6`, `docm4`,
  `fix_pattern_mate_crossing`, viewer `msolve3/4`, Python
  `test_assembly_author`), and the PR names any expectation that had
  to move, with why.
- **A4** The stray-key row refuses at the load door; the census
  re-baselined and the sibling row green.
- **A5** The four items closed on the PR's branch with `## Closed`
  sections citing the rows, `work.py lint` clean.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground; never end
  a turn with background work active. Four Cargo workspaces (`.`,
  `benches/`, `demos/tour/`, `demos/wild/`): a public signature change
  is checked in all of them before the push. `pncad-py`'s clippy with
  `--features python --all-targets` too.
- Merge-only; push early and often; the PR through the GitHub MCP
  tools (no `gh`). Private `CARGO_TARGET_DIR` outside the worktree;
  `git status` before every `git add`; never `git add -A`.
- Fence: `crates/editor-core/src/mate/member.rs`, `mate/solve.rs`,
  `mate.rs` (the attribute only), `eval/mod.rs` (`node_value_kind`'s
  error shape only) and `eval/wire.rs` (its one caller only),
  `crates/test-utils/tests/deny_unknown_fields_census.rs` (the
  constant only), tests, the four items. Nothing in the edit door,
  the evaluator's placement, `names/*`, the viewer or Python.
- Comments state the invariant (discipline §4): no "this used to be
  built per call" anywhere in `src`; the argument is the PR body's.
- **Stop clause.** If seating the transform needs `node_value_kind`
  to change what it CLASSIFIES rather than what it carries; if a mate
  row's verdict moves under one environment (it cannot — the
  environment is pure — so a moved verdict is a finding, not a
  re-baseline); or if the attribute refuses a checked-in `.pncad`
  (C5's four files) — STOP, write what you measured in the PR as a
  draft, and end your turn.

## Out of scope

Memoising anything across solves; the reach (MSOLVE-6, landed); the
clash margins' arm (MSOLVE-8); a `(j, i)` copy vocabulary (ruled
against above); sweeping other crates for the attribute's complement
(the census row's disclosed blind spot stays disclosed — file it on
CENSUS's slate if the lane meets it).

## Review

One style review plus a correctness arm, claims verbatim:

- **C1** A1: one environment per solve, and the environment passed is
  the document's own nominal one (no box, no seed) — compare a mate's
  offset under the old and new roads on a document with a parameter
  the pattern reads.
- **C2** A2: the refusal for a dangling transform input below a
  circular pattern's axis is seated at the transform on both roads,
  and every other `axis_datum` refusal keeps its seat (the pattern).
- **C3** A3: enumerate the mate suites and confirm no verdict moved.
- **C4** A4: the stray key refuses through `persist::load` with the
  format's own arm, and no checked-in document refuses.
