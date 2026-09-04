---
id: viewer-const-all-tables-have-no-exhaustiveness-guard
kind: issue
title: Five const ALL tables in the viewer enumerate an enum by hand, and adding a variant compiles
status: open
opened: 2026-09-04
refs: [1762]
---

Found by CHROME's style lane on PR 1762, as a class rather than an
instance: that unit added the **fifth** member.

`crates/viewer/src/combine.rs:247` (`PatternOutputChoice::ALL`) joins
`app.rs:725`, `app.rs:759`, `app.rs:787` and `blend.rs:150`. Each is a
`const ALL: [(Self, &'static str); N]` whose `N` is a hand-written
count and whose membership is a hand-written list.

**Adding a variant to any of these enums compiles.** The radio row that
renders the table (`app.rs:3996-4001` for the new one) silently loses a
button, and the row that asserts the table (`combine_ops.rs:836-847`)
compares it against a literal, so it does not go red either. The enum
grows, the chrome does not, and nothing says so.

A `fn all()` returning the same array from a `match` on `Self` would
break the build instead — the compiler already owns exhaustiveness, and
these tables decline to use it.

Not fixed on 1762: converting one table is an instance fix and the
finding is that there are five. Whoever takes it should take all five
in one pass, or state why a table is deliberately partial.

Signed: (CHROME orchestrator)
