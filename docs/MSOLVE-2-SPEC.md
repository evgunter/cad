# MSOLVE-2 — The member chain: nested copies through `Part`, sibling distinctness at every level (spec)

**Program:** MSOLVE (`work/msolve/plan.md`), unit `MSOLVE-2`
(`work/msolve/MSOLVE-2.md`). **Ruling of record:** Ev on PR 1731
(`work/msolve/nested-pattern-mate-heads-refuse.md`, "both in"): the
member vocabulary extends through identity-transparent nodes as one
walk; the pattern-of-transform half landed in MSOLVE-1 (PR 1929), this
unit lands the nested-copy half. **Track:** kernel change — one style
review plus a correctness arm (§Review). No A/B row.

## What the tree says now, and what it changes about the task

- `mate/solve.rs`'s `walk` (MSOLVE-1) admits any number of
  `Transform`s and AT MOST ONE `Pattern` level; `Member.copy` is
  `Option<(RecipeNodeId, u32)>`; `Placer` and `Walk` carry the chain
  the offset folds over; `Member` has an explicit `Ord` over
  `(instance, copy, at)`.
- **A pattern over a pattern does not evaluate.** `wire_pattern` takes
  `body_operand`, and a pattern's value is `Instances`, so
  `Pattern { input: pattern }` refuses `WrongOperand` — the same fence
  `Transform` over a pattern hits (`work/issues/
  transform-refuses-a-patterns-instances-value.md`). The nested shape a
  user can actually build is **`Pattern` over `Part { of: pattern,
  select: Instance(i) }`**: `Node::Part` projects one body out of the
  instances with every name VERBATIM (its doc: "a pass-through in
  `Transform`'s sense, contributing no role segment"). So the third
  identity-transparent node MSOLVE-1's report flagged is not a side
  decision; it is the only road to a nested copy, and the same road
  makes a transform over ONE copy reachable (`Transform` over `Part`),
  which MSOLVE-1's A3 could not build.
- A nested copy's name is `{ node: P2, path: [Instance { i2, of }] }`
  with `of = { node: P1, path: [Instance { i1, of: master }] }` — the
  outer pattern wraps the name it saw at its input, and the `Part`
  passed that name through unchanged.
- `PartSelect::Instance(Expr)` is an EXPRESSION in a structural slot.
  The name carries a literal index; the Part carries an expression.
  Admission must not evaluate; the offset already does (the pattern's
  slots are evaluated at the document's parameter bindings in
  `derived_offset`).

## What the unit builds

**1. `Member.copy` becomes the chain.** `copy: Vec<(RecipeNodeId,
u32)>` — the pattern copies the walk consumed, OUTERMOST first, empty
for a plain member. `Member` loses `Copy`; the explicit `Ord` stays
`(instance, copy, at)` in that order with the chain compared
lexicographically, and the doc at the impl keeps its argument (a
document whose members are at most one level deep keys exactly as
before: a one-element chain orders as the `Option` did). `Placer::
Pattern` and `copy` no longer spell the index twice: the member's
chain IS the walk's pattern placers, taken from one source — collapse
them (S12 from MSOLVE-1's style review).

**2. The walk admits `Part { Instance(_) }` and repeats the pattern
arm.** At a node that is not the name's head, the walk continues
through a `Transform` (composing) or through a `Part { of, select:
Instance(_) }` (pose-neutral, identity-transparent: continue at `of`
with the same name, contributing NO placer). At the head, a `Pattern`
consumes `Instance { i, of }` and continues at its input under `of` —
now at every level, not once. Anything else stops the walk as today,
naming the node. A `Part { SplitHalf }` is a different body and stops
the walk. Structural throughout: no expression is evaluated.

**3. The offset folds the whole chain, and checks the `Part`.**
`derived_offset` folds every placer's map outermost first, as today,
over the longer chain. Where the walk passed a `Part` selecting
instance `e` (an expression) directly above a pattern whose copy the
name says is `i`, the offset evaluates `e` at the document's bindings
(the way it evaluates the pattern's own slots) and refuses TYPED when
it does not equal `i` — a new `MateFault` variant naming the mate, the
side, the part node, the name's index and the evaluated one. The name
is the authority on which copy; the Part is checked against it, never
the reverse. (A document where they disagree would otherwise be placed
by the name and gathered by the Part.)

**4. Fewer walks.** `solve_document` walks each reference once for
`by_pair`; carry the `Walk` (member and chain) in the pair's value so
`pair_left_factor`/`derived_offset` fold the chain in hand rather than
re-walking (S13). `head_of`'s and `member_of`'s public shapes stay.

**5. `mate/member.rs`.** The member vocabulary — `Member`, the walk,
`member_of`, `head_of`, the placers, `derived_offset` and its
constants — moves out of `solve.rs` into `mate/member.rs` with no
logic change beyond items 1–4 (S19). `pub` surface and the re-exports
`refactor.rs`, the viewer and `pncad` import stay where they are.

**6. The viewer's tool** reads a nested copy's MASTER through every
level (`picked_member`: descend `Instance { of }` to the innermost
name, the one headed at the instance) and admits a pick on a `Part`
over a pattern, and on a `Transform` over such a `Part`. Everything
else in the tool is unchanged: the frame is read at the member's
instance and divided by its placement; the composed offset is the
solve's.

**7. The rows.** `crates/editor-core/tests/msolve2_member_chain.rs`,
through ordinary doors, asserting on the product's face frames with
MSOLVE-1's whole-frame oracle (lift it into the shared test fixture
rather than copying it):

- a nested copy `(i2, i1)` seats at `M2(i2) ∘ M1(i1) ∘ placement`,
  with a rotating inner or outer rule so the two maps do not commute;
- **loop closure at each level**: two mates from one base to sibling
  copies of the INNER pattern (different `i1`, same `i2`) — the second
  declares and a consistent pair verifies, an inconsistent one refuses
  at the closing mate; the same with siblings of the OUTER pattern
  (same `i1`, different `i2`); and one mate to `(i2, i1)` and one to
  `(i2', i1')` differing at both levels;
- a `Part`-selected copy read AT the `Part` (no outer pattern) is a
  member; a mate read at the pattern with `Instance(i)` and one read at
  a `Part` selecting the same `i` are two members over one copy
  (different operands), the second declaring — state that at
  `Member.at`;
- `Transform` over `Part { Instance(i) }` over a pattern, mate read at
  the transform: seats (the shape MSOLVE-1's A3 could not build);
- the `Part`-index check: a `Part` whose expression evaluates to a
  different copy than the name says refuses the new variant, naming
  both indices;
- `mate1_r1_probes`' nested half, `mate1r2_probes` P5 and
  `msolve1_transform_aware::a10` now place — move their expectations
  with the ruling and say so in the PR;
- every existing suite unchanged; a document with at most one pattern
  level solves bit-for-bit (`a7` and the `asm_r2a` bit rows are the
  witness).

**8. The docs.** A11 (5)'s member sentence and A12 in
`crates/editor-core/ASSEMBLY.md` state the walk as landed (any number
of transforms and `Part` selections, any number of pattern levels, the
member's identity its instance, its copy chain and its operand); the
`Mate` variant's doc and `member_of`'s doc drop the MSOLVE-2 fence.
Present tense, no history.

## Acceptance

- **A1** A nested copy seats at the composed pose, non-commuting maps,
  measured on the product's face frames.
- **A2** Loop closure at the inner level, the outer level and both:
  second mate `Declaring`, consistent pair passes the gate,
  inconsistent pair refuses at the closing mate (name the fault).
- **A3** `Part` in the walk: a pick/mate read at a `Part` is a member;
  two operands over one copy are two members; `Transform` over `Part`
  over a pattern seats.
- **A4** The `Part`-index mismatch refuses typed with both indices.
- **A5** One walk per reference per solve (a counter in tests, or the
  call graph read by the reviewer); `Placer`/`copy` are one source.
- **A6** `mate/member.rs` exists; `solve.rs` shrinks by the vocabulary;
  no behaviour change attributable to the move (the bit rows).
- **A7** Every existing suite unchanged but the three expectations the
  ruling moves; the no-nesting solve bit-for-bit.
- **A8** The viewer admits a nested-copy pick and a `Part` pick and
  seats them after commit (the `mate_tool_flow` shape).

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted
  CI is the verification of record; poll it in the foreground; never
  end a turn with background work active.
- Merge-only; push early and often; the PR through the GitHub MCP
  tools (no `gh`). Private `CARGO_TARGET_DIR` and scratch outside the
  worktree; `git status` before every `git add`; never `git add -A`;
  build narrowly, disk is shared.
- Fence: `crates/editor-core/src/mate/*` (the new `member.rs`
  included), `node.rs` docs, `crates/viewer/src/matetool.rs`, tests,
  the shared test fixture for the oracle, `ASSEMBLY.md`. Nothing in
  `eval/*` (the `Part` node's own evaluation is not this unit's; if its
  index check needs a shared evaluator, call the existing slot
  evaluation the pattern's offset already uses), nothing in
  `refactor.rs` beyond what `Member` losing `Copy` forces.
- Do not evaluate in the walk; do not make `Part` contribute a placer;
  do not give the chain an `Option` wrapper; do not keep a second
  spelling of the copy chain.
- **Stop clause.** If `Member` losing `Copy` forces a change to
  `refactor.rs`'s crossing collector or `assembly.rs` beyond a
  `.clone()`; if the `Part`-index check cannot reuse the pattern's
  slot evaluation; or if a nested document cannot be built through
  `DocEdit::InsertNode` at all — STOP, write what you measured in the
  PR as a draft, and end your turn.

## Out of scope

`Pattern` or `Transform` accepting `Instances` directly
(`work/issues/transform-refuses-a-patterns-instances-value.md`);
`Part { SplitHalf }` as a member road (a split half is a different
body); the `DanglingHead` catch-all's typed cause (MSOLVE-3); the
gate's `Vanished` on a mate read below a pattern
(`assembly-gate-refuses-vanished-on-a-mate-read-below-a-pattern`),
which this unit must not make worse and should measure once on a
nested document, reporting what it finds.

## Review

One style review plus a correctness arm, claims verbatim:

- **C1** A1–A3 on nested documents the implementer did not build,
  with non-commuting rules at both levels, measured on the product.
- **C2** Loop closure holds at every level (A2), and the spanning
  tree's edge choice on one-level documents is unchanged (the `Ord`
  argument, read against `solve_cluster`).
- **C3** The walk evaluates nothing; the offset checks the `Part`'s
  index against the name and refuses typed (A4); one walk per
  reference (A5).
- **C4** `mate/member.rs` is a move (A6); bit-for-bit on the no-nesting
  solve; nothing outside the fence moved.
- **C5** The viewer reads the innermost master and seats (A8).
