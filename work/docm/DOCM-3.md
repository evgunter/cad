---
id: DOCM-3
kind: unit
title: Node::Union (n-ary), DocEdit::SetMembers, and pairwise-distinct inputs (DM4, DM5)
status: closed
opened: 2026-09-04
closed: 2026-09-04
pr: 1803
branch: docm/3-union
---


## Spec

`docs/DOCM-REFERENCES-DESIGN.md` DM4, DM5. `Node::Union { members }`
(n-ary, one body, fold in member order, `FromMember { member, of }` naming keyed by the
member EDGE, the amendment's key); `DocEdit::SetMembers { node, members }` with
its four typed refusals; the pairwise-distinct-inputs check stated once
and called from `InsertNode` and `SetMembers`. `demos/tour/src/diefillet.rs`
rewrites its tool onto the new node and its "NAMED GAP" note goes; the
acceptance row is: remove one pip by `SetMembers` + `DeleteNode` in one
action and both fillets still resolve. `no-docedit-splices-a-deleted-node`
is parked on this unit (DM6). The union seat in the viewer is CHROME's;
the Python surface is LIB's, both filed at merge.

## Closed (2026-09-04)

Merged as PR 1803 (ordinal 1801, sample #127). `Node::Union` (n-ary,
fold of the pair verb, tag 31), `RoleSeg::FromMember { member, of }`
with `names/emit_union.rs`, `DocEdit::SetMembers` over `Node::list_input`
(`Loft` too), DM5 as `Node::input_fault` with three callers; the die's
tool is one union and removing any pip leaves both fillets resolving.
Residue with its own file: `n-ary-union-has-no-declaration-channel`
(touching members refuse with no in-node recourse; the ruling to bring
to Ev). `no-docedit-splices-a-deleted-node` stays parked, now on that
item. The union seat is CHROME's, the Python doors LIB's, filed in the
PR body.
