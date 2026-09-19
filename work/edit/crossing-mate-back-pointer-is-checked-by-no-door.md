---
id: crossing-mate-back-pointer-is-checked-by-no-door
kind: issue
title: An interface crossing's mate id is a node reference no door checks
status: open
opened: 2026-09-19
refs: [instantiate-part-crossings-are-names-payload-names-does-not-list]
---

(Found by the sweep of `edit/instance-crossing-names`, which made an
instance's crossing `outer`s payload names and left the third field of
the same crossing unexamined.)

## The finding

`InterfaceCrossing::Mate` (`crates/editor-core/src/node.rs`) carries
three references into THIS document's id space, not two: `outer` (now
a payload name, checked at the insert door and repaired by `Rebind`),
`inner` (the part's, deliberately out of reach), and `mate` — *"the
crossing mate, in the remainder"*, a `RecipeNodeId` of a node in this
document.

Nothing checks `mate`:

- `Node::payload_names` cannot list it, because it is a node id and
  not a name;
- `Node::payload_read_sites` — the door for *"the nodes a payload's
  references are READ AT that are not also DAG inputs"*, whose doc
  says *"the insert door checks these are live exactly as it checks a
  payload name's head, and for the same reason: a never-existed id is
  a typo"* — has no `InstantiatePart` arm, so `instantiate_part_with`
  accepts a record naming a mate that never existed;
- no delete reports one. Deleting the crossing mate leaves the record
  pointing at a gone node, silently, where deleting the `outer`'s
  minting node now reports `Maintenance::Strand` (DM7).

That the id is unchecked is visible in the fixtures: `asm_r2b_assembly`
(`row5_b`, `row5_c`, `row6`) each spell `mate: RecipeNodeId(7)` /
`(9)` / `(4)` in documents holding no such node, and all three pass.

## Why it may matter

The id is read: `NodeErrorKind::CrossingUnverified { mate, .. }`
(`eval/wire.rs`) names it in the refusal a user reads, and the Python
binding publishes it (`pncad-py`'s `py::refactor`,
`InterfaceCrossing.mate`). A dangling id there is a refusal naming a
node that is not in the document.

## The ruling this wants

Whether the crossing's `mate` is a READ SITE in
`Node::payload_read_sites`' sense — checked at the insert door, and
therefore also whether its delete deserves a report of its own (a
read site's does not today: *"a delete that strands one is the
solve's to refuse (A12), not this door's to report"*, per
`payload_read_sites`' doc). If it is, the arm is one line and the
three `asm_r2b_assembly` fixtures need real mate ids; if it is not,
the reason belongs on the field, which today says only what the id
denotes.
