---
id: kind-mirrors-have-no-single-declaration
kind: issue
title: a fieldless kind enum is a hand-mirror of its error enum, and only a source-scan row sees the phantom direction
status: open
opened: 2026-09-04
---


Filed by the `boolean-error-has-no-fieldless-kind` lane at the moment
it disclosed the residue; PR 1490's review named the direction and
this is its durable home.

**The shape.** A fieldless kind enum beside a payload-carrying error
(`profile::PathError`/`PathErrorKind`, `Attr`/`AttrKind`,
`topo::BooleanError`/`BooleanErrorKind`) is TWO hand-written
declarations plus a hand-written projection. The compiler sees one
direction only: an arm added to the error enum reds the exhaustive
`kind()`. A variant added to the KIND enum alone is a phantom —
nothing constructs it, so no test reaches it and the only build it
reds is some downstream exhaustive map, in another crate, if one
happens to exist. Nothing at all objects to an arm projected to the
WRONG kind, which type-checks.

**What exists today.** `BooleanErrorKind` carries a source-scan row
(`crates/topo/src/boolean/mod.rs`,
`the_kind_enum_names_exactly_the_error_arms`) that reads both
declarations and the projection out of the file and asserts the three
name lists agree. It closes the phantom direction and the mis-pairing
direction for that one pair, and its own blind spot is stated at the
row: it is a text scan of one file, so a macro-expanded variant or an
arm whose formatting it misreads is invisible to it.

`PathErrorKind` (`crates/profile/src/path.rs:1044`) and `AttrKind`
have NO such row — their phantom direction is unguarded, and
`PathErrorKind`'s own doc says so.

**The fix shape.** A `transition_table!`-style single declaration: one
table generating the error enum, the kind enum and the projection, so
neither direction can drift and the source-scan rows retire. It is
worth deciding once, for every pair in the tree, rather than growing a
per-pair scan row. `NodeErrorKind` (SMELL-UV's §D row) inherits the
same decision.
