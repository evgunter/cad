# EVAL-11 — `node_value_kind` reads a placer's family through its input: a transform of a pattern is "instances" on both roads (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 11). **Item:**
`work/eval/node-value-kind-answers-a-transform-by-node-kind.md` (EVAL-6's
residue). **Track:** E — one implementer lane, one style review, fix
pass, record-at-merge; no A/B draw. The correctness claim is one row
(below), so no separate correctness arm. **Branch:**
`eval/11-transform-value-kind`. **Difficulty:** S.

## The claim

**`eval::node_value_kind` (`crates/editor-core/src/eval/mod.rs`) answers
"body" for every `Node::Transform`, and a transform's value family is its
INPUT's.** Since EVAL-6 the placers are shape-preserving over the value
(`Body → Body`, `Instances → Instances`), so a transform of a pattern
evaluates to `"instances"` while the recipe-side word — the one
`mate/member.rs`'s `axis_datum` puts in `WrongOperand::found` when a
circular rule's axis operand is not a datum — says `"body"`. The function's
own doc says so and cites the item; `msolve3_placer_refused` pins the two
roads to agree for the two families it reaches (a datum, a body) and not
for this one.

## What lands

1. `node_value_kind` takes the document and the node id
   (`node_value_kind<P>(doc: &Doc<P>, id: RecipeNodeId) -> &'static str`,
   or the node plus the doc — the lane picks the honest signature and says
   why) and reads a `Transform` through its input: the family of a
   transform is `node_value_kind` of its input. `Pattern` stays
   `"instances"` whatever its input (a pattern of a body and a pattern of
   instances both evaluate to `Instances`). A missing input (a dangling
   id) answers what the evaluation would refuse with — read
   `eval_node`'s poison/missing-input path and give the same word or say
   why the recipe road cannot; never panic. The recipe is a DAG by
   construction — say where that is enforced (the insert door) so the walk
   needs no cycle guard, or add the guard if it is not.
2. The one caller, `crates/editor-core/src/mate/member.rs` `axis_datum`,
   passes the document and id — MSOLVE's file, one line, **by announced
   seam** (the orchestrator's log line and the PR body).
3. **The row**: `msolve3_placer_refused` (or a sibling beside it) gains
   the families the item names: a circular rule whose axis operand is a
   transform of a pattern — the evaluation's `WrongOperand::found` and
   the recipe road's word are both `"instances"`; a transform of a
   transform of a body — both `"body"`. Written red-first (the PR body
   shows the transform-of-pattern row red at the merge base).
4. The function's doc drops the "one kind this cannot answer" paragraph
   (its premise is gone — Q4) and keeps the correspondence sentence: the
   families the row checks, and which are by inspection.
5. Nothing else moves: `rg -n 'node_value_kind' crates` — every caller
   listed. The viewer's `combine::denotes_body` (CHROME's) answers the
   analogous question for the GUI's body seat and is tracked at
   `work/chrome/body-seat-reads-through-the-placer-chain.md`; not
   touched here, named in the PR body.

## Sweep

`rg -n 'Node::Transform' crates/editor-core/src` — every match arm that
answers something about a `Transform` from the node alone; disposition
per hit (a shape-preserving placer's answer is its input's unless the
arm is about the placement itself). Blind spot: an arm reached through a
helper that matches on a payload rather than the node.

## Review

One style lane (`docs/prompts/reviewer-style-lane.md`). Claims: the
transform-of-pattern row is red on the base, green on the head
(reproduce); the two roads agree on every family the row reaches; the
sweep's hit list is real (run it shaped differently, e.g. `Transform {`);
Q2 on the walk's missing-input answer; Q4 on the function doc; Q6 — any
sweep hit left is a file.

## Records at merge

`work/eval/log.md` entry with the MSOLVE seam announcement; the item
`closed` with `pr:`; this spec deleted per `docs/DOC-LEDGER.md`.
