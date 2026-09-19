---
id: a-declare-orphaned-by-a-cascade-is-never-reported
kind: issue
title: A Declare orphaned by a cascade that deleted its consumer is silent forever
status: spec
branch: edit/orphaned-declare-report
opened: 2026-09-17
---


## What

Deleting a node cascades its dependents. A `Declare` is not a
dependent of the boolean or union that consumes it — the edge runs the
other way — so a cascade that deletes the consumer leaves the
`Declare` behind, and nothing ever mentions it again:

- the delete reports no strand, correctly: a sited pair's names are
  the MEMBERS' and the members are untouched, and a site is a node id
  rather than a name, which `stranded_references`
  (`crates/editor-core/src/edit.rs`) deliberately does not report;
- the next evaluation says nothing: a `Declare` with no consumer
  evaluates to its own payload and refuses nothing
  (`eval/mod.rs`'s `Declare` arm);
- no later door reads it, because the node that would have is gone.

Measured by
`crates/editor-core/tests/review_decl_r1.rs`'s
`a_declare_orphaned_by_a_cascade_refuses_nothing`: deleting a member
of a two-member declared union cascades the union away, the `Declare`
survives, the maintenance list is empty and the evaluation is clean.

## Why it is a row and not a bug of this unit

The state is not wrong — a `Declare` can legitimately exist
unconsumed, which is premise 5's ruled position (the load door asks
nothing new, and a site that is not the consumer's operand is the
evaluation's refusal rather than the edit's). What is missing is that
nothing ever tells the author the node is now inert. The same shape
predates sited declarations: the orphan was silent under the
member-space payload too. What sited declarations change is how easy
it is to reach — the `Declare` is now authored FIRST, so any cascade
through its consumer produces one.

## The shapes a fix could take

- report it as maintenance at the delete that orphans it (a new
  `Maintenance` arm — "this Declare has no consumer"), which is the
  DM7 door's vocabulary already;
- report it at the next evaluation, as a document-level finding rather
  than a node refusal;
- leave it and say so in `Node::Declare`'s docs, which is the cheapest
  and is what holds today by default rather than by decision.

Ground: `crates/editor-core/src/{edit.rs, eval/mod.rs}` — EDIT's.

Filed by the EDIT-DECL fix pass, PR #2809.

## Ruled and spec'd (2026-09-19, EDIT orchestrator) — middle tier, branch `edit/orphaned-declare-report`

**Ruling: the first shape — the delete that orphans a `Declare`
reports it, as maintenance, in DM7's vocabulary.** `Maintenance` gains
an arm, `OrphanedDeclare { declare: RecipeNodeId }`
(`crates/editor-core/src/edit.rs`): a `DeleteNode` whose removal took
the LAST consumer of a `Declare` reports that `Declare` once, at that
step. The rule is a TRANSITION, not a state: a `Declare` is legally
consumerless in the one-pass authoring window (inserted first, its
union next — DM4), so "any consumerless `Declare`" would fire on every
fresh declaration; what the door reports is "this delete made it
so". The consumers of a `Declare` are the nodes whose `inputs()` hold
it (a union's or boolean's `declare` edge is a DAG input, so the
cascade already orders the consumer before its members); the door
computes, for the node it deletes, which of its `Declare` inputs have
no surviving consumer in the document AFTER the removal — one pass of
the same shape as `stranded_references`, and read out of the same
post-removal document so the two reports cannot disagree about which
nodes are gone. DM7's posture holds verbatim: report, never refuse,
never repair — the repair is the author's `DeleteNode` of the
`Declare` or a new consumer, and evaluation stays silent (a
consumerless `Declare` evaluates to its own payload, as today).

**Why not the other two shapes.** A document-level finding at the
next evaluation cannot tell the orphan from the fresh declaration
without the transition, and the maintenance column exists so that a
legal edit's consequence is not deferred to evaluation (DM7's second
bullet). Leaving it and saying so in `Node::Declare`'s docs is the
state the row was filed against. This is an addition beside DM7, not a
change to what DM7 decides (a stranded NAME); the clause text is not
touched, the arm's doc carries the rule, and the orchestrator puts the
ruling to Ev for objection on the next `[ev]` PR.

**Order in `Applied::maintenance`.** Orphan rows follow the payload
and appearance strands of the same delete and precede the cluster
acts; the `maintenance` field's order contract gains that sentence and
the boundary is held by a row whose fixture produces both kinds (a
delete that strands a name AND orphans a `Declare` — a member that
mints a name another node carries and is a site of a declared union).

**Premises to verify before building.** (1) `Node::inputs()` lists the
`declare` edge for `Boolean` and `Union` (node.rs ~2629/2640) and no
other variant consumes a `Declare` — confirm by grep; a second
consumer kind makes the "last consumer" test read `inputs()`, not the
variant. (2) A `Declare` can be consumed by more than one node (two
unions over the same members, or a boolean and a union) — author that
fixture; if the insert door refuses it, say so and the multi-consumer
row becomes the two-step delete below. (3) `Maintenance`'s readers:
`crates/pncad-py/src/py/mate.rs` (three exhaustive matches) and
`tags.rs`'s tag map — the compiler finds every one; the binding gains
the tag `orphaned_declare` and a node accessor (`Maintenance::node`
answers the `Declare`), LIB's by announcement; `display_contract.rs`'s
`Maintenance` rows and `dm7_delete_strands`' order rows are the
suites that move. (4) `refactor::split`/`inline` delete through
`apply`, so a split whose cut takes a consumer but not its `Declare`
cannot happen (the cut is closed under the DAG in both directions,
and the `Declare` is an input) — confirm with one assertion in an
existing split row rather than a new fixture.

**Rows** (each red on `origin/main` first, then green):
`review_decl_r1::a_declare_orphaned_by_a_cascade_refuses_nothing`
re-headed to `…_is_reported_at_the_delete_that_orphans_it`: the
cascade's maintenance carries exactly one `OrphanedDeclare { declare }`,
at the UNION's step, and the evaluation is still clean; deleting a
consumer while a second consumer survives reports nothing, and
deleting the second reports the orphan once; deleting the `Declare`
itself (which cascades its consumers) reports no orphan; inserting a
`Declare` with no consumer and then deleting an unrelated node reports
nothing (the transition rule — the row a state-based implementation
reds on); the order row above; the `Display` sentence for the arm
(`display_contract`), naming the `Declare` and saying what it lost;
the Python tag and accessor rows in the binding's suite.

**Mutants** (each named with the row that reds it): the state-based
implementation (the unrelated-delete row); reporting at every step of
the cascade that still lists the `Declare` (the cascade row's "exactly
one"); reading consumers out of the PRE-removal document (the cascade
row — the union is still a consumer there); the arm placed before the
strands (the order row).

**Sweep.** `Maintenance::`, `stranded_references`, `no transients`,
`orphan` over `*.rs`/`*.md`/`*.py`/`*.pyi`; `edit.rs`'s
`stranded_references` doc ("Under the vocabulary as it stands there
are no transients to cancel") stays true of STRANDS — cite, do not
widen; `Node::Declare`'s doc gains one sentence naming the report.

**Territory.** `crates/editor-core/src/{edit.rs, node.rs}` (EDIT);
`crates/editor-core/tests/*` (TCOST/TINT); `crates/pncad-py/src/{py/mate.rs, tags.rs}`
and its suite (LIB, by announcement — a new tag word, said in one
line). Middle tier: one opus style review with a correctness arm, then
the fix pass.
