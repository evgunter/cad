---
id: deletenode-strands-a-declare-payload-name
kind: unit
title: DeleteNode leaves a Declare whose payload names a dead node: DeleteWouldDangle reads inputs only
status: closed
pr: 2753
opened: 2026-09-06
refs: [2028, 2028]
branch: edit/delete-strands
closed: 2026-09-16
---


## What

Found by DOCM-7's R1 review (NOTE-8), probe
`r1_delete_doors_around_a_declared_union` on `docm/7-review-r1`.

Delete the UNION of a declared union and the edit succeeds, leaving the
`Declare` behind with a payload naming a node that is no longer in the
document. The document saves, loads and evaluates. Delete the `Declare`
instead and the edit refuses `DeleteWouldDangle`, because that one IS a
DAG edge.

`crates/editor-core/src/edit.rs:1530` — the delete door's dangle check
reads `node.inputs()` and nothing else, so a reference carried in a
PAYLOAD (`Node::payload_names`: a `Declare`'s pairs, a blend's
selection, a mate's two heads) is invisible to it.

## Why it is not a bug in the door

This is ruled, not overlooked. The insert door's own comment
(`edit.rs:1475`) states the D3 carve-out: payload name refs must point
at live nodes AT EDIT TIME, they are not DAG edges, and "later deletes
may strand them (N5), so this is the ONLY door that checks". N5's rung 1
(`ResolveError::NodeGone`) exists precisely to diagnose a stranded name,
and `DocEdit::Rebind` is its repair.

So extending `DeleteWouldDangle` to cover payload references would not
be a reuse of one predicate at a second door — it would reverse the
carve-out, for every node kind that carries a payload name, and make
rung 1 unreachable by the route it was written for. DOCM-7 therefore
files this rather than building it (the fix brief left the call to the
implementer).

## The class, and what a ruling would decide

The union shares this with the pair boolean, the fillet/chamfer
selection and the mate — it is not new. What DOCM-7 adds is a payload
name that points at a node the payload's own consumer depends on, which
makes the stranded state easier to reach by hand than it was.

The question for a ruling: whether a payload reference should be a
WEAKER edge (delete warns, or offers to rebind) rather than no edge at
all, and whether the answer differs for a name that points at the
consuming node's own space.

## Consequence for DOCM-7's own acceptance

A5 asked for the `declare` edge to be exercised through change and
removal. There is no edit that CHANGES or REMOVES a live node's declare
edge (DM6: inputs are not rewired), and deleting the `Declare` refuses
while the union is live. So A5's change/remove half is unexercisable
through the doors that exist; DOCM-7 discloses that rather than
inventing an edit for it.

## Where it stands

Open, unscheduled, on DOCM's slate.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

(At DOCM's exit sweep, `refs` names the PRs `DOCM-7` stood for: `DOCM-7` = #2028 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)

## Question for Ev (2026-09-16, EDIT orchestrator) — with a recommendation

**The question.** A payload name (a blend's selection, a `Declare`
pair, a mate head, a derived frame's face, a measure ref) is not a DAG
edge: `InsertNode` checks its node is live and nothing else does, so a
later `DeleteNode` strands it, `ResolveError::NodeGone` diagnoses it
at evaluation and `Rebind` repairs it. Should a payload name be a
weaker edge than no edge at all — and is the answer different when
the name points at the consuming node's own space (a `Declare` whose
pairs name the union that consumes it)?

**Recommendation: keep the carve-out, and make the delete door SAY
what it stranded.** `DeleteNode` stays legal when a payload names the
target; its `Applied.maintenance` (the column every accepted edit
already returns since EVAL-4) gains a typed row per stranded
`(node, name)` — the names whose minting node just left, computed at
the door by the same `Node::payload_names` walk the insert door uses.
No refusal, no silence: the edit is loud where it happens rather
than at the next evaluation, and the chrome's cascade affordance can
show the strand count beside its dependent count. The self-space case
needs no separate answer: deleting the union strands every pair of
its `Declare`, the report names all of them, and the orphaned
`Declare` is ordinary garbage the user cascades or deletes.

**Rejected: a full edge** (`DeleteWouldDangle` over payload names).
It reverses the ruled D3 carve-out, makes rung 1 of N5 unreachable
by the route it was written for, and **deadlocks the declared union**:
the union's input is the `Declare`, the `Declare`'s pairs name the
union, so under a full edge neither can be deleted alone and the
"any reference" relation has a cycle that `cascade_delete_order`
(which walks inputs only) cannot see. The self-space case is exactly
where a full edge fails, which is the answer to the row's second
question.

**Rejected: as-is.** A legal edit whose consequence is invisible until
evaluation is the limp-along shape the maintenance column exists to
end.

**Cost.** One maintenance variant, its `Display`, a row that goes red
when a strand goes unreported; a CHROME row to render it (filed on
their slate when this lands). No schema change: maintenance is
derived, not persisted.

## RULED (Ev, on the `[ev]` PR, 2026-09-16): the recommendation stands — DM7

Recorded as **DM7** in `crates/editor-core/REFERENCES.md`: a name is
not an edge, `DeleteNode` stays legal, and the accepted edit's
`Applied.maintenance` carries a typed row per stranded `(node, name)`.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/delete-strands`.

1. Read `Applied`'s maintenance column (`crates/editor-core/src/edit.rs`,
   the A11 cluster-record maintenance and EVAL-4's shape) and extend
   it — one typed row per stranded name: the node that carries the
   name, the name, and the deleted minting node. Computed at the
   `DeleteNode` door from `Node::payload_names` over every surviving
   node (the same walk the insert door checks with — ONE home for
   "which payloads carry a name"; do not write a fifth list, C6's S4
   table names the four that exist). `payload_read_sites` (a mate's
   operands) are the solve's to refuse and are NOT strands — say so
   at the site.
2. Replay reproduces it: maintenance is deterministic from the edit,
   so `Doc::replay` and `persist::load` produce the same rows (or the
   documented load boundary discards them — read
   `replay-and-load-keep-the-document-without-its-maintenance` and
   state which; do not close that row here).
3. `Display` in prose, F6-shaped; the `pncad-py` façade's maintenance
   surface gains the row if it renders maintenance today (LIB's file,
   mechanical, said in the PR).
4. Rows that go red: DOCM-7's R1 probe shape (delete a declared union;
   the report names every pair of its `Declare`) lifted from
   `origin/docm/7-review-r1` if it survives; a fillet whose target is
   deleted under a cascade that keeps the fillet's own name-space
   consumer; a delete that strands nothing reports nothing; a mutant
   that skips one payload kind is caught.
5. File the CHROME affordance (the strand count beside the cascade
   count) on `work/chrome/` in this PR — it is this ruling's chrome
   consequence and CHROME has not been asked.

## Built (2026-09-16)

PR 2753, branch `edit/delete-strands`. DM7 is built.

**Landed.** `Applied.maintenance` is a `Vec<Maintenance>` — a new enum
in `edit.rs` whose two arms are `Cluster(ClusterMaintenance)`, the A11
registry acts unchanged, and `Strand { node, name }`, one row per
surviving payload name whose minting node the edit removed. The
`DeleteNode` arm of `apply` computes the strands after the removal,
over the document as it then stands, through `Node::payload_names` —
the same single answer the insert door checks with, so no fifth list.
The strands lead the column and the reconciliation appends after them;
that order is stated as a CONTRACT on the field, with its pinning row
named. `ClusterMaintenance` gained a prose `Display` beside its own
type in `mate/solve.rs` and `Maintenance` delegates to it, so each
enum's arms are guarded by their own F6 census.
`Node::payload_read_sites` is deliberately not walked, and the
function says why at the site, with the walk's per-delete cost under a
cascade stated beside it.

The façade follows mechanically: `pncad::document` carries
`Maintenance` beside `Applied`; `pncad-py`'s `ClusterMaintenance`
pyclass becomes `Maintenance` with a `strand` tag and `node`/`name`
getters, with the stub, the tag inventory, the census and the `ty`
fixture in step. Every sentence promising a cluster-only column was
swept (`py/doc.rs`, `pncad.pyi`, `refactor.rs`, `eval4`'s header,
`docs/guide/selecting.md`).

Rows: `crates/editor-core/tests/dm7_delete_strands.rs` (seven,
including DOCM-7's R1 probe shape lifted from `origin/docm/7-review-r1`
into an assertion, and a carrier row whose expectation is DERIVED by an
exhaustive `match` over `Node` so a new name-carrying variant fails to
compile), plus one `display_contract` row per enum, plus the review's
four probes adopted from `review/strands-rv` authorship-preserving
(`rv_dm7_probes.rs`).

**Did not land, deliberately.** The strand row carries `(node, name)`
alone: the deleted minting node is `name.node` by construction, and a
third field repeating it is a disagreement waiting to happen. The
spec's CHROME filing was not made as a new file — the row already
existed as `work/offer/cascade-delete-shows-the-strand-count`, parked
on this unit, and this PR corrects its count definition in place rather
than opening a second file (`work/README.md` "One file, one item";
`docs/prompts/implementer-discipline.md` §6, "add your evidence to it
rather than opening a second").
`replay-and-load-keep-the-document-without-its-maintenance` is NOT
closed: replay and load discard the column by the documented load
boundary, which the PR states, and the new round-trip row asserts only
what makes the discard lossless.

**Filed by this unit.**
`work/edit/stranded-appearance-keys-are-not-reported-by-dm7` — an
appearance attachment is the same N5 shape one door over and DM7's
walk does not see it; a clause-scope question for Ev, measured by an
adopted probe, not built here.
`work/lib/maintenance-crosses-python-as-a-nine-attribute-union-class`
— the façade's union class, recorded as an instance of a class whose
every LIB row is closed.

## Closed (2026-09-16, EDIT orchestrator)

DM7 built and merged as PR #2753 after one opus style review with a
correctness arm (APPROVE-WITH-FIXES, three MINORs, all built at the fix
pass) and the orchestrator's read. Residue in its own files:
`stranded-appearance-keys-are-not-reported-by-dm7` (a clause-scope
question for Ev), the CHROME affordance row unparks at this merge.
