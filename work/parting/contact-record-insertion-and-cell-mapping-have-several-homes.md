---
id: contact-record-insertion-and-cell-mapping-have-several-homes
kind: issue
title: The insert-or-union-cites rule for a contact record lives in three places, and each kind's cell mapping is respelled at every site that tags rows
status: open
opened: 2026-11-01
priority: P4
cost: M
---

Found by the PR 6225 delta review (2026-11-01, Q1, likely). That PR
gave record insertion one home inside the reduction's `ContactAcc`
(`reduce::record`, used by `vv`, `vf`, `ef`) and one `tagged` helper in
`ReductionContacts::rows`, but the class has more sites:

- the insert-or-union-cites rule, each with a linear `find`, in
  `ops.rs` (`Rows::push::add`) and `records.rs` (`ContactRecords::push`);
- the per-kind cell mapping, hand-spelled again for carried rows in
  `ops.rs` (the carry's row tagging): no contact type (`VvContact`,
  `VfContact`, `EfContact`, …) has a `cells()` method, so every site
  respells it;
- inside `ContactAcc`, the operand → `(seen_ab | seen_ba, list)` match
  is written three times in a row (`vf`, `rests_in`, `placed_on`) while
  `seen_ef` keys on the `Operand` instead: two spellings of per-operand
  keying in one struct.

Give each rule one home the others call. Also from the same review
(S5, likely): the end-rests-inside arm of `settle_in_plane`'s holding
read changes no outcome in any suite (only a hand-built unit pins it);
either find a reachable pose where it spares an in-band midpoint read,
or remove it.
