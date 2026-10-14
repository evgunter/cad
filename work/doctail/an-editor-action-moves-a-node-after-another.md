---
id: an-editor-action-moves-a-node-after-another
kind: issue
title: editor: one client action moves a node after another — re-points the operand slots and unwraps each moved selection's From{read,of} to its of (the blend-crossing recourse's reorder, PR 4963)
status: open
opened: 2026-10-14
priority: P3
cost: M
---



Ev's ruling on `[ev]` PR 4963 (2026-10-14,
`work/band/a-concave-fill-and-a-whole-shell-inside-a-bands-region.md`)
makes a blend that a later feature crosses refuse as an unbuilt case,
with the reorder as its recourse: in the API
`fillet(subtract(body, hole))` becomes `subtract(fillet(body), hole)`
(`FILLET3_CLEARANCE_RECOURSE`, `FILLET3_RING_RECOURSE`,
`crates/sweep/src/blend/mod.rs`; `crates/sweep/README.md`, predicate 2).
The ruling's build order runs "an editor reorder action (re-point plus
the selection unwrap, one action)" beside any step.

**What it is.** One client action that moves a node after another, built
from the explicit slot writes DM6 allows (`crates/editor-core/REFERENCES.md`,
"No edit infers a re-point": "delete and reconnect" is a client's offer of
explicit slot writes). Moving the blend `B` before the feature `F` whose
output it reads:

- re-point `B`'s input slot to `F`'s input, and `F`'s input slot to
  `B`'s output, and every reader of `B`'s output to `F`'s, through the
  one slot door (`edit.rs`, `check_written_node`), acyclicity checked
  as for any write;
- unwrap each of `B`'s edge selections written on `F`'s output: such a
  name is `From { read, of }` at `F` (N1, `names/README.md`), which does
  not resolve upstream, so it becomes its `of`. A name `F` minted (an
  edge `F` split, a seam) has no `of`; the action reports it stranded
  as a DM6 write does, and the person re-picks it (re-querying gives
  the whole parent edge);
- rewrite `F`'s own selections, written on `F`'s old input, which now
  read `B`'s output: each becomes a name in `B`'s output (a `From` at
  `B` over it), and one `B` consumes is stranded and reported;
- rewrite the selections of `B`'s former downstream readers, which now
  read through `F`: a name `B` minted (a band face) becomes a `From` at
  `F` over it, and one `F` consumes is stranded and reported.

**Open.** Whether it is one `Edit` or a client composition of the
existing slot writes, and how it reads when `F` is not the direct input
(a chain of features between the edge's node and the blend). The action
makes several slot writes and selection rewrites at once, while DM6's
"delete and reconnect" is explicit slot writes "the person choosing
each": whether one reorder action, chosen once, meets that, or each
write must be offered to the person, is DM6's question to answer before
building.
