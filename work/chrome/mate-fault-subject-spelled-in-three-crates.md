---
id: mate-fault-subject-spelled-in-three-crates
kind: issue
title: Which mate a MateFault is about is spelled per-arm in three places, none citing another
status: open
opened: 2026-09-04
refs: [1769]
---

Found by CHROME's style lane on PR 1769, as a class.

"Which mate is this fault about" is answered by a hand-written match
over `MateFault`'s arms in **three** places:

- `crates/viewer/src/tree.rs:286-292` — `blamed_mates`, added by PR 1769;
- `crates/pncad-py/src/py/mate.rs:568-581` — the same seven-arm
  or-pattern, in the same order, over the same field;
- `crates/pncad-py/src/tags.rs:394-402` — a third per-arm enumeration.

**None cites another**, and a prose sweep over both crates found
nothing disclosing the duplication — so this is an undisclosed copy,
the majority case the style brief's Q1 says only the DATA can find.

**The natural home is the kernel enum**: a `MateFault::subject()` next
to the arms it reads, so adding an arm forces the answer once instead
of three times. `crates/editor-core/src/mate.rs` is fenced by CHROME's
`keep_out`, which is why PR 1769 wrote a viewer-local copy rather than
moving it — so this is a **fence artifact**, and fence artifacts are
exactly the thing that goes unscheduled unless someone files them.

Cost of leaving it: the next consumer of `MateFault` writes a fourth
copy, and an arm added to the kernel enum silently gets a different
answer in each. PR 1769's copy is `match`-exhaustive, so IT would fail
to compile — the other two should be checked for the same property.

Signed: (CHROME orchestrator)
