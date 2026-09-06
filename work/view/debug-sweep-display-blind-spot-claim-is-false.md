---
id: debug-sweep-display-blind-spot-claim-is-false
kind: issue
title: the Debug sweep's stated blind spot counts 20 Display impls where there are 36 and calls them all enum matches when three are over structs
status: open
opened: 2026-09-06
refs: [2093]
---



Found by the style review of #2093.

Implementer discipline §5 makes the blind-spot sentence part of the
receipt: *"A sweep whose blind spot is unstated is an unverified claim,
not a negative result."* #2093 states one, and it is checkable, and it
does not hold.

The claim, in the PR body and in
`work/view/debug-for-docsession-is-a-fourth-hand-maintained-walk.md`'s
*Sweep, and what the pattern could not match*:

> every `impl Display` in `crates/viewer/src/` (20 of them) is a
> `match` over an enum's variants, which is exhaustive by construction
> unless it wildcards

Two things are wrong.

**The count.** `rg -n 'impl.*Display for' crates/viewer/src` gives
**36**, not 20. No stated rule produces 20: 34 excluding the two
inside `evalseam`'s inner module (`crates/viewer/src/evalseam.rs:501`, `:526`), 19
distinct files, 32 spelled `core::fmt::Display`. The enumeration rule
is not given, which is the same omission the citation receipt in the
same PR is careful to avoid.

**The shape.** At least three of the 36 are over structs, not enums,
and each reads its fields by hand:

- `crates/viewer/src/frame.rs:320-324` — `Message`, reading
  `self.text`;
- `crates/viewer/src/frame.rs:706` — `Withdrawal<'_>`, reading
  `self.withdrawn` and `self.kind`;
- `crates/viewer/src/frame.rs:1847` — `Disagreement`, whose own
  doc argues that **both** halves of what it renders are load-bearing
  — exactly the sentence that would be falsified by a third field
  added to the struct and silently not rendered.

Whether any of the three is worth changing is a separate question.
What is wrong here is that the blind-spot paragraph reports a negative
result over a set it did not enumerate, and the negative result is
false. A reader of that paragraph now believes `Display` under
`crates/viewer/src/` has been looked at and is clean.

The same paragraph's second half is answered separately in
`work/view/field-censuses-inside-view-survived-the-debug-sweep.md`.
