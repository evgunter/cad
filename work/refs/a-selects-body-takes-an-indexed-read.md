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

Built on PR 6555 under Ev's ruling there (form 1 deleted, producers and consumers both); the design as built is REFERENCES DM3.

- `var::Select.body` is `BodyRead<VarId>`, its indices the selection's own `Count` variables (`edit.rs` `mint_selection`, `lower_index`). `xs[k]` on the family's own index chain is stored as `xs`. A member read's rank is the family's and a mate side's every index is a written count (`edit.rs` `selection_body_fault`, asked at the door and at load).
- Reach: a member read reaches what its indices reach (`repetition.rs` `Reach::of_var`). Content key: a member read's index values (`eval/wire/pose.rs` `key_words`).
- Evaluation: the member is projected as a node seat's indexed read is (`eval/wire.rs` `reads_projected`, `member_projected`, `project_selection`), and the names resolve in the member's own table (`selection_table`); an empty member refuses `EmptyOperand`.
- Deleted: `repetition::names_members`, `names_a_member`, the `Reach` exception, the whole-read branches in `eval/wire.rs` (`select`, `resolve_name`), `wire/pose.rs` (`read_off`) and `wire/trace.rs`, and `read_named_body`'s `whole` flag.
- A family read plainly at a selection's body is read per member, as at every body seat, so no door reads it whole; there is no `Bodies`-kind variable for `SlotVarKind` to refuse there.
- Producers: a sited name (`SitedFace`, `SitedRef`) of one member lowers to `xs[i]` and the member's own name (`operand.rs` `selection_at`), the inverse of `mate::member::head_of`; flatten and inline unwind a body built from the family to the member read the same way; split crosses a member read under the family's spelling (`mate::member::spelled_whole`). Python `select` / `select_where` refuse a family read whole (`SelectRefusal`, `family`) and take `xs[i]`; the viewer's picks on a drawn member author `xs[i]` and the member's own name (`session.rs` `picked_body`, through `select::member_entity`).
- Filed: `a-rebind-at-a-per-member-built-family-reaches-every-members-selections`, `a-per-member-indexed-read-at-a-node-seat-names-members-twice`. Closed: `a-selection-of-a-member-of-a-family-with-an-empty-member-refuses`.
