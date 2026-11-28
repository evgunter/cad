---
id: a-drive-keeps-no-node-result-from-one-leaf-to-the-next
kind: unit
title: A drive keeps no node result from one leaf to the next: one store per drive under the evaluator's memo rule, and receipts that count what a result rests on
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
---


Points 1, 3, 5 and 6 of the parent row's weighed state (`[ev]` PR
7368; `docs/ERROR-DESIGN.md` E6's leaf-protocol bullet): a node's
result (build, finish or refusal, with what it recorded) is served
from one store per drive under (the node's own id, the content key,
the naming key), each entry computed once; a leaf's receipt counts
what its result rests on, served or built; the drive's total counts
each distinct node result once, so the pinned totals of multi-leaf
drives re-baseline once; the store is a dial like `plain_memo`, and
verdicts, leaf receipts and the drive's total are the same with it on
or off and on one thread or many. A deferring session takes no prior.

Measured (M8, M9): on the tour 87% of build and 91% of finish time
inside node ops is on a key already seen, most of it the holes'
refusals, and every repeat of a key carries a byte-identical refusal
and count delta.

**Open inside the unit, to measure and not to ask:** whether handing
a leaf its parent box's replay is enough (no shared state). It needs a
document with two features that both refine, under the new split
rule; if cousin-only repeats are rare there, the parent-only prior is
the simpler final state (Ev was told on PR 7368 it would be taken and
logged).

After `a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state`.

**From the session-state unit, for whoever builds the store**
(`geom_core::sym::NodeRecord`, `NodeValue::sym`, `NodeError::sym`):

- A node's record (the identities it registered, its counts, its plain
  roots) rides its value. A served node's counts are not counted by
  the session, which counts what ran: the store adds the record's
  counts to the leaf's receipt, and the record's roots to the roots
  the leaf's `frozen` column is read over (`sym::leaf_need` reads the
  root scope's, which holds the roots of the nodes that ran).
- A node's scope reads the records of the values its reads see
  (`eval::read_records` over `wire::seen_reads`, in field order), and
  each record names the records it was itself built over
  (`NodeRecord`'s `over`). A served upstream node's record is read the
  same way, so nothing else is owed there.
- The parameter brackets stay the leaf's input (`Sym::param_over`,
  recorded as the environment is built, before any node). A node's
  decision read takes the brackets of the parameters its forms reach,
  and the content key hashes no bracket (the node-key unit's evidence
  on the session-state row): a served node is the leaf's own only
  where the leaf's box agrees on those parameters.
- A value built in one leaf resolves in another through the drive's
  table (`sym_drive_memo::a_value_built_in_one_leaf_means_the_same_in_another`).
  With `DriveConfig::plain_memo` off the table serves nothing, so the
  store has to be off with it or keep its own table.

**A question this unit settles (from the session-state unit's fix
pass).** A leaf resolves a symbolic id by its PRESENCE in the drive's
table: an id is its content's hash, so once any leaf of the drive has
minted that content inside a session, a leaf handed the same content
from outside every session reads it as known, and before that as an
unknown of its own (unpublished, unpooled, never a proof). Both
answers are sound and the second is the stronger: an order dependence
between leaves, reached only by a value from outside every session
(`geom-core`'s `a_drives_table_and_what_is_outside_it::an_outside_value_resolves_once_a_leaf_has_minted_its_content`
pins it as it stands). Whether a leaf should resolve an id by presence,
or only by reachability from what the drive served it, is this unit's
to decide.

**Also from that fix pass:** a part's run is a scope of its own and is
counted once per replay (`Evaluation::parts_sym`), so a part built in
each of N leaves is N results in the drive's total unless the store
serves it; a family's value carries its members' records; the
product's gather is on the product (`Product::sym`); a prior is read
only under the table it was built under (`PriorRefusal::Table`),
which is the door a served result comes through.

**Two gaps the session-state unit leaves here** (its merge pass,
2026-11-24):

- **No row serves one member of a family while its siblings run.** A
  member is a scope of its own inside the family's
  (`eval::eval_any`), and what it read goes on the family's record
  (`NodeRecord`'s `over`) when its scope closes. A member that is
  served and not run closes no scope, so the family's record has to be
  handed what the served member's record holds and what it was built
  over, in member order. No fixture in the tree has a family whose
  members are served singly; the rows that exist run every member
  (`editor-core`'s
  `a_nodes_result_at_the_symbolic_scalar_is_its_keys::a_reader_of_a_member_reads_that_members_record_and_a_reader_of_the_family_all`).
- **What a record holds beyond the table is not in any size.** A
  record names the records it was built over, one pointer per
  registering record upstream of it, shared and not copied
  (`NodeRecord`'s `over`). `DriveMemo::bytes_estimate` counts the
  drive's table and neither the records on a leaf's values nor the
  warm caches. A store that keeps values keeps their records, so its
  size is its own to measure.

**From the session-state unit's delta review (PR 7492), for whoever
builds the store:**

- **A family's record with a served member** is now the record a run
  of every member leaves: `eval_node` hands the enclosing scope a
  served step's held record in place of what ran (`sym::carry`), pinned
  by `a_family_with_a_member_taken_from_an_earlier_leaf_carries_that_members_record`.
  That is the evaluator's own prior; a store that serves a member
  through another door owes the same call.
- **One record at two addresses.** A record's `over` and its `==`
  compare the records it was built over by address first and by their
  own content second, and `NodeRecord::read` counts by address. A node
  and its upstream served from different leaves can hold one record at
  two addresses; an identity of the record's own (its content) would
  not depend on that history.
- **A second, different bracket for one symbol**, where a served
  node's record and the leaf's environment disagree: the last recorded
  stands, and which stands decides the early walk. A named question
  for the store; nothing checks it today.
- **A served value under a table that serves nothing**
  (`DriveConfig::plain_memo` off): the value names nodes the table
  holds and forms it does not serve.
- **A served result's `over` keeps every upstream record alive** for
  as long as the result is held, so for the drive's length.
- **A gather that refuses drops its record** (`product::gather_sources`
  writes it on `Ok` alone): on
  `work/boxfinish/a-documents-registrations-do-not-reach-its-products-gather`.
- **`sym::carry` finds the served member's entries by position.** It
  drops the last `ran.identities.len()` identities and brackets of the
  open scope's lists, on the premise that the member's scope closed
  last and left them there, and it joins the counts with a saturating
  subtraction, which would hide an imbalance and not report it. Sound
  for the one caller there is; a store that serves through another
  door should hand the scope the held record in place of running the
  member, so that nothing is subtracted (the orchestrator's read at
  merge, 2026-11-28).

**From the node-key unit's fix pass (PR 7470), two things the store
should know:**

- **A name is a function of the document only inside a session.** A
  value with no expression is named by its node's mint scope and
  ordinal inside a session, and by its minting thread outside one
  (`geom_core::sym::mint`). A walk that maps its units in parallel
  where decisions are thread-portable and runs them in order under a
  session (`topo::props`' face walk, the evaluator's level map) so
  names the same work two ways (`geom-core`'s
  `the_two_arms_of_a_walk_share_no_name`). A drive replays every leaf
  inside a session, on the serial arm, so every entry of one store is
  named one way; a store that ever took an entry built under no
  session would hold names no replay reproduces.
- **A held axis keys apart from the unboxed build at `Sym`.**
  `analysis::var_env_over` binds a `Fixed` axis `nominal + 0` and the
  unboxed environment binds `nominal`: one number, two expressions, so
  two content keys now that the key hashes the expression. The `+ 0`
  is the arithmetic a varying axis binds through (`nominal + offset`),
  kept so that a held axis and a degenerate varying one are the same
  bits, a negative zero included; binding a held axis as the bare
  literal at `Sym` alone would make the symbolic lane's numeric channel
  differ from the plain lane's there. Every leaf of a drive goes
  through `var_env_over`, so inside one store a held axis is one
  expression and nothing misses; the miss is between an unboxed build
  and a boxed replay, which no store shares.

**Note for the unit (2026-11-24, from a designer of the climb fork):**
the ladder's growth guard (`SymRetry::max_forms` in `geom-core`'s
`sym.rs`, checked per attempt per leaf) makes a decision's answer
depend on the asks made earlier in the leaf. A store that serves a
node skips its asks, so a later decision in that leaf can meet the cap
at another point than in a fresh leaf: "the same with the store on or
off" has to hold across the guard, or the guard's count has to belong
to the node. (BOXFINISH orchestrator)
