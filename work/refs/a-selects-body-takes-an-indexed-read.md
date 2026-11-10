---
id: a-selects-body-takes-an-indexed-read
kind: issue
title: A selection's body is a plain read, so a member read by index (xs[i]) cannot be selected from, and Rebind { body } would reach every member's selections
status: review
opened: 2026-10-10
priority: P1
cost: M
pr: 6555
branch: reads/select-indexed-member
---


FORK-DM4 unit 1 (PR 4527) builds the indexed read `xs[i]` at every body seat
(`crate::BodyRead { read, at }`, index slots `SlotId::Index { seat, k }`).
INTENT stage 2 E (#4523) landed first and made a blend's, a shell's and a face
frame's body the selection's: `VarDef::Select(var::Select { body: VarId, names })`,
whose `body` is lowered as a Body seat (`edit.rs`, `mint_selection`). By the
two implementers' agreement the merge kept `Select.body` a bare `VarId`, so a
member of a family read by index cannot be selected from: fillet `xs[2]`'s edges
has no spelling.

Building it: give `var::Select.body` the `BodyRead` form, its index slots the
select definition's own (a definition, not a node), resolve the select against
the projected member (`eval/wire.rs`'s `reads_projected` / `instance_of`), and
keep the carried names keyed by the family read (DM4).

One hazard to pin with it, from E's implementer: `DocEdit::Rebind { body }`
rewrites every selection of one body read. Two selections of different members
of one family (`xs[0]`, `xs[1]`) share that body read, so a `Rebind { body: xs }`
would reach both; it must address the indexed read (read and indices), not the
read alone.

## Implementer's notes

Built on PR 6555 under Ev's ruling there (form 1 deleted, producers and consumers both); the design as built is REFERENCES DM3, and the PR body lists every site.

- `var::Select.body` is `BodyRead<VarId>`, its indices the selection's own `Count` variables (`edit.rs` `mint_selection`, `lower_index`). `xs[k]` on the family's own index chain is stored as `xs`. A member read's rank is the family's (`InputFault::IndexRank`, one enum with the node seat's) and a mate side's every index is a written count (`SelectionFault::SideIndex`), asked at the door and at load of every selection a seat reads directly or through a pose (`edit.rs` `selection_body_fault`, `seat_reads`).
- Reach: a member read reaches what its indices reach. Content key: a member read's index values. Evaluation: the member projected as a node seat's is, the names resolved in the member's own table; empty member `EmptyOperand`; a clearance seat reports the member's place.
- Deleted: `names_members`, `names_a_member`, the `Reach` exception, the whole-read branches in `eval/wire.rs`, `wire/pose.rs` and `wire/trace.rs`, `read_named_body`'s `whole` flag, and the sited conversion.
- A side states its member natively (`SitedFace.member`); `head_of` derives the family's spelling for the member walk. Flatten and inline unwind a body built from the family to the member read (`refactor.rs` `unwound_selection`); split crosses a member read under the family's spelling.
- Python `select` / `select_where` refuse a family read whole (`family`) and an absent member (`no_member`); every Python door pairing a body with a name takes `xs[i]`. The viewer's picks on a drawn member author `xs[i]` by the member's own name, and refuse where no member reads them.
- The form-1 shape is admitted as a per-member selection and refuses at evaluation; whether that answers "it shouldn't type check" is put to Ev on the PR.
- Filed: `a-rebind-at-a-per-member-built-family-reaches-every-members-selections`, `a-per-member-indexed-read-at-a-node-seat-names-members-twice`, `a-rank-change-leaves-readers-unsaveable`, `a-whole-body-measure-over-a-member-read-refuses-other-body`, `a-member-read-is-no-continuation-and-no-cell`, `sketching-on-a-members-face-fuses-the-whole-family`. Refreshed `an-indexed-read-of-rank-two-has-no-family`. Closed: `a-selection-of-a-member-of-a-family-with-an-empty-member-refuses`.
