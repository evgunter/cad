---
id: split-sides-are-not-finished-bodies
kind: issue
title: The split's sides are plain bodies: its door takes a finished operand but hands back sides no at-rest gate read
status: open
opened: 2026-10-06
priority: P3
cost: M
---



Left by `split-answers-an-inside-out-operand-with-two-inside-out-halves`
(branch `cleave/split-operand-gate`), which makes the split's doors take
`AtRestBody` on the way in. The way out did not move: `SplitResult`'s
sides are `SplitPart<T>` over plain `Body<T>`, gated at tier 2 only
(`split_direct`'s `validate_closed`, `SplitFinishError::ResultInvalid`).
The Boolean's result is an `AtRestBody` gated by
`T::gate_at_rest_kept` (`boolean-door-adopts-the-finished-body-type`).

So a split side that a later door takes finished pays tier 3 there (the
editor's seat finishes every split side a Boolean or a second split
consumes), and the type does not say the side is finished.

Why the sides are not gated at tier 3 today, per `split`'s own doc: a
pinch side's touching pieces carry contacts the split declares nowhere.
Unmeasured: which sides `T::gate_at_rest_kept` refuses today. What closes
it: gate each side with `T::gate_at_rest_kept` and hand back
`AtRestBody`, with the contacts the split mints declared where the gate
needs them, or a measurement that says which sides fail and why the type
stays plain.
