---
id: a-declare-orphaned-by-a-cascade-is-never-reported
kind: issue
title: A Declare orphaned by a cascade that deleted its consumer is silent forever
status: closed
closed: 2026-09-19
pr: 2874
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

## Built (2026-09-19)

`Maintenance::OrphanedDeclare { declare }` is the arm, and `apply`'s
`DeleteNode` reports it: `orphaned_declares` (`edit.rs`) takes the
removed node's `inputs()` and the POST-removal document — the same
document `stranded_references` is read out of — and names every
`Declare` among those inputs that no live node's `inputs()` still
hold, in document order. Report, never refuse, never repair. The rows
land after the strands of the same delete and before the cluster
acts; `Applied::maintenance`'s order contract says so and
`dm7_delete_strands::an_orphaned_declare_follows_the_strands_of_the_same_delete`
holds the boundary with a delete that produces both kinds.

**The spec's "deleting the `Declare` itself reports no orphan" row is
not buildable, and the row that replaces it says why.** A `Declare` is
its consumer's DAG input, so deleting it means cascading the consumer
first, and the consumer's step is the SAME `(document, edit)` pair as
deleting that consumer for any other reason. `Applied::maintenance` is
a function of the document and the edit, so the two cases cannot
report differently: the cascade reports the orphan at the consumer's
step and its next step removes the subject.
`dm7_delete_strands::cascading_a_declare_away_reports_the_orphan_and_then_removes_it`
pins that, the arm's doc states the transient, and a caller who wants
a cascade's net effect reads the document it ended at.

Premises, checked: (1) holds — `Node::declare_input`'s match is
exhaustive and only `Boolean` and `Union` carry one, and both list it
in `inputs()`; consumption is still read through `inputs()`, so a
future consumer kind counts the day it compiles. (2) holds — nothing
refuses a second consumer of one `Declare`, and
`an_orphan_is_reported_by_the_delete_that_takes_the_last_consumer`
authors two unions over one declaration. (3) holds — the binding's
three matches and the tag map are the readers; the tag is
`orphaned_declare`, `Maintenance.node` answers the `Declare`, and
unlike `stranded_appearance` this arm is REACHABLE from Python
(`Doc.declare_all` + `Node.boolean(declare=)` + `DocEdit.delete_node`),
which `test_document.py`'s
`test_deleting_the_consumer_reports_the_declaration_it_orphaned`
exercises. (4) holds — `split`'s closure check refuses a cut that
takes a declared union and leaves its `Declare`, asserted in
`asm4_split_inline::row3_severing_cut_refuses_naming_the_edge`.

Rows: `review_decl_r1::a_declare_orphaned_by_a_cascade_is_reported_at_the_delete_that_orphans_it`
(re-headed), four in `dm7_delete_strands`, the `Display` case and
census entry in `display_contract`, the split case in
`asm4_split_inline`, the Python row and the census entry.

## Built — fix pass (2026-09-19, PR #2874)

The style review (`review/orphan-rv`) came back MERGEABLE; its probes
and the one row it filed are merged here authorship-preserving, and
the orchestrator's rulings are built.

**The transient is ruled.** The lane's shape stands at `apply`: it is
a function of `(document, edit)` and reports what that one delete
did. The NET over an ACTION is the CASCADE door's answer —
`Session::commit_action` in the viewer; Python has no cascade door —
and nothing computes it. So the cancellation is filed where that door
lives: `work/offer/cascade-delete-shows-the-strand-count.md` gains a
`## Widened (2026-09-19, PR #2874)` section saying the affordance owes
the net of strands AND orphans over the doomed set, with the
reviewer's one-line filter and the probe that measures it. The arm's
doc states the transient once; `pncad.pyi` states it once too,
because Python is the external consumer with no cascade door.

**One home for "who consumes this node".** `roots::consumer` is that
home — the first live node whose `inputs()` hold the id, walked in
document order — and `roots::is_sink` is its predicate half.
`orphaned_declares` calls `is_sink` instead of its own
`doc.nodes.values().any(…)` copy, and iterates the DELETED node's
inputs rather than the whole document order, so the doc's cost
sentence is now true of the code. `apply`'s `DeleteNode` dangle check
and `refactor::inline`'s `InstanceConsumed` check call `consumer` for
the witness they name.

**The set is at most one today, said so.** `Applied::maintenance`'s
"in the document's node order" clause becomes "at most one today —
`Node::declare_input` is an `Option` — in the deleted node's input
order should a kind ever hold two", and
`dm7_delete_strands::no_delete_can_report_two_orphans_today` is the
guard that reds the day a kind holds two.

**The arm is nobody's but the delete door's.** Nowhere does the code
call it DM7's: it is "the delete door's orphan report, beside DM7's
strands (ruled at EDIT's wave 11; for Ev's objection)". The transition
rule is stated once, in the arm's doc, and pointed to from
`Node::Declare`, `DocEdit::DeleteNode`, the order contract and the
`.pyi`.

**The `Display` sentence is true now.** It said "nothing reads the
declaration"; the root set reads it — the same delete re-roots the
`Declare` into `doc.roots()` — so it says "no node consumes the
declaration". `display_contract` pins the new clause, the `.pyi`
paragraph is re-worded the same way, and
`an-orphaned-declare-joins-the-product-root-set` (the reviewer's row,
filed on this slate) is cited from the arm's doc as the open question
of whether a `Declare` may be a root at all.

**The probes are adopted and the probe suite deleted.** Into
`dm7_delete_strands`: the at-most-one guard, the mixed
`Boolean`/`Union` consumer pair (both orders, folded into the
two-consumer row), the transient's cancellability at the cascade
door, `SetMembers` cannot orphan, and the re-rooting row the filed
issue cites. Into `asm4_split_inline::row3_severing_cut_refuses_naming_the_edge`:
the declare edge refused in BOTH directions, plus the closed cut
accepted. `crates/editor-core/tests/rv_orphan_probes.rs` is gone.

**MINOR-1/NOTE-3.** `pncad.pyi`'s `last_maintenance` contract sentence
names all three kinds and their order ("Empty after an edit that moved
no mate graph, stranded no name and orphaned no declaration"); the
`DeleteNode` arm's comment says the input list feeds both
`roots::on_delete` and the orphan door.

**LIB's row cited and appended.**
`work/lib/maintenance-crosses-python-as-a-nine-attribute-union-class.md`
gains a `## Widened (2026-09-19, PR #2874)` paragraph: a seventh
variant, and `node` answering a second question.

**Disclosed deviation.** The asm4 fixture: the spec asked for one
assertion in an existing row, and the build added a four-node fixture
inside that row because `part()` has no union. Kept, and now widened
with the mirror cut and the closed cut.

## Closed (2026-09-19, EDIT orchestrator)

Built and merged as PR #2874 (middle tier: one opus style review with
a correctness arm, then the union fix pass). The delete that takes a
`Declare`'s LAST consumer reports `Maintenance::OrphanedDeclare {
declare }` — a transition rule, never a state (a fresh declaration is
legally consumerless in the one-pass window), read out of the
post-removal document exactly as the strands are; report, never
refuse, never repair; the order contract is strands, then orphans,
then cluster acts. DM7's clause text is untouched (the arm calls
itself the delete door's report beside DM7's strands, ruled at EDIT's
wave 11 and put to Ev for objection). One spec row could not be built
as ruled and the orchestrator ruled at the fix pass that the built
shape stands: deleting the `Declare` itself cascades its consumer
first, and `apply` — a function of `(document, edit)` — reports the
orphan at the consumer's step and removes its subject at the next;
the NET over a cascade is the cascade door's answer (the viewer's
`commit_action`; Python has no cascade door and its docstring states
the transient), filed on CHROME's `cascade-delete-shows-the-strand-count`
row with the reviewer's one-line cancellation measured. The review
(0 MAJOR, 1 MINOR) found the consumer scan copying `roots::is_sink`'s
body one file over; the fix pass gave "who consumes this node" one
home (`roots::consumer`, `is_sink` over it) and closed the class
across the crate (five sites, three fixed), narrowed the walk to the
deleted node's inputs, said the set is at most one today, corrected
the Python contract sentence, and cited the LIB row the seventh
variant widens. Filed by the review and left standing:
`an-orphaned-declare-joins-the-product-root-set`. Territory crossed
by announcement: four TCOST/TINT suites, LIB (`pncad.pyi`,
`py/mate.rs`, `tags.rs` — tag `orphaned_declare` — `tests.rs`, two
Python suites), FIX (`refactor.rs`, one line), CHROME and LIB rows
appended.
