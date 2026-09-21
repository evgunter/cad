---
id: node-placer-field-docs-say-body-where-instances-are-accepted
kind: issue
title: Node::Transform and Node::Pattern field docs say "the body placed" / "the body replicated" although both placers accept Instances
status: closed
opened: 2026-09-08
priority: P4
cost: E
branch: door/node-placer-field-docs
pr: 2985
closed: 2026-09-21
---


(EVAL orchestrator) From EVAL-11's style review (PR 2195, finding 3).
`crates/editor-core/src/node.rs` declares `Node::Transform` as "a rigid
placement of an upstream body" with `input: /// The body placed.`
(~`:1640`) and `Node::Pattern` as "a pattern of an upstream body" with
`/// The body replicated.` (~`:1653`). Since the ruling in
`work/eval/transform-refuses-a-patterns-instances-value.md` (PR 2137,
EVAL-6, `wire::placeable_operand`) both placers accept `Instances` and
are shape-preserving over it, and `eval::node_value_kind` (EVAL-11)
now reads a transform's family through its input on that premise.
The declaration is the premise's home and still says "body" — the
doc-rotted-code-right case. The viewer-side members of the same class
are on CHROME's slate at
`work/chrome/body-seat-reads-through-the-placer-chain.md`; these two
field docs are the kernel-side members. `node.rs` is DOCM's.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/door/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the fix is written in the row and it is one PR on a file another program owns, which is DOOR's test. Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Closed (2026-09-21) — PR 2985, and it corrected the brief that dispatched it

The two field docs, the two variant lines above them, and a third
sentence two variants down (`Node::Part`) now say what the placers
accept and yield.

**The orchestrator's brief was wrong and the lane caught it.** The
dispatch said both placers are *"shape-preserving over `Instances`"*,
taken from this row's own text and from ruling 2137's summary. Read
against the code:

- Both placers read the same door (`placeable_operand`), so they do not
  differ in what they ACCEPT: a `Body`, a boolean's non-empty result,
  or an `Instances` taken whole — **three shapes, not two**; an empty
  boolean is the typed absence `EmptyOperand`.
- They DO differ in what they YIELD. `wire_transform` maps through
  `placeable.map(..)` and is shape-preserving. **`wire_pattern` ends
  `Ok(OpOut::plain(ValuePayload::Instances(instances), table))`
  unconditionally** — N bodies for a one-body master, N·M
  placement-major for an `Instances` master of M. Writing
  "shape-preserving" twice would have put a false sentence in the
  declaration other code reasons from.

Verified at the site before merging rather than taken from the report.
This is instruction 1 — *read the clause, not the row's summary* —
catching an error the orchestrator introduced by quoting the row.

**A third rotted sentence, in fence.** `Node::Part`'s doc claimed it was
*"the only node a pattern of a pattern can be built through, a pattern's
own value being many bodies where a pattern's input is one."* Both
halves are retired — `eval6_placers_over_instances.rs`'s `nested_doc`
builds `linear(linear(cube))` with no `Part` between the levels — and
the sentence now says what a `Part` between two patterns MEANS.

**No pin, correctly.** Nothing here changes behaviour, so there is no
runtime value an assertion could discriminate, and adding one would be
the §2 defect. What was owed instead and done: confirming no doc-test
and no prose census reads these sentences (the five things naming
`node.rs` by path are four `gated_to!` lists and a
`deny_unknown_fields` count — none reads prose).

**Fence:** `crates/editor-core/src/node.rs` is EDIT's, announced there.

**Filed, and the class was bigger than the row said:** the same rot at
the Python door (`work/bind/python-transform-door-doc-says-an-upstream-body`),
in a viewer test header (`work/vdoc/mate-tool-flow-header-says-a-patterns-input-is-one-body`),
and in an `editor-core` test header stating flatly that *"a pattern OVER
a pattern does not evaluate"* (`work/tint/msolve2-header-says-a-pattern-over-a-pattern-does-not-evaluate`)
— which is false, and is the sentence `nested_doc` refutes. Evidence
appended to `work/chrome/body-seat-reads-through-the-placer-chain` for
two VERBATIM copies of the field docs in `viewer/src/session/op.rs`
that its list did not carry.

**Adjudicated: the CHROME append was right**, though the lane called it
a coin-flip against opening a fourth row. That row already lists prose
sentences in its scope, so two more belong on its list (§6's
grep-first). CHROME may split them out; the note says so.
