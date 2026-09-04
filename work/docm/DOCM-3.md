---
id: DOCM-3
kind: unit
title: Node::Union (n-ary), DocEdit::SetMembers, and pairwise-distinct inputs (DM4, DM5)
status: spec
opened: 2026-09-04
---


## Spec

`docs/DOCM-REFERENCES-DESIGN.md` DM4, DM5. `Node::Union { members }`
(n-ary, one body, fold in member order, `FromMember` naming keyed by the
member's own minting node); `DocEdit::SetMembers { node, members }` with
its four typed refusals; the pairwise-distinct-inputs check stated once
and called from `InsertNode` and `SetMembers`. `demos/tour/src/diefillet.rs`
rewrites its tool onto the new node and its "NAMED GAP" note goes; the
acceptance row is: remove one pip by `SetMembers` + `DeleteNode` in one
action and both fillets still resolve. `no-docedit-splices-a-deleted-node`
is parked on this unit (DM6). The union seat in the viewer is CHROME's;
the Python surface is LIB's, both filed at merge.
