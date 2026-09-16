---
id: quoted-parameter-name-in-error-prose-has-no-decision
kind: issue
title: editor-core quotes a parameter name in error prose at some doors and not at others, and nothing decides which
status: open
opened: 2026-09-16
---


Disclosed by `edit/error-prose`, which swept `{binding:?}` inside every
`impl Display` in `editor-core/src` and repaired the sites where the
rendered payload was a variant identifier or a struct dump. This is what
that sweep left standing, with the reason it was left and the question
that decides it.

## The residue

A `{x:?}` whose payload is a plain `String` renders the string plus
`Debug`'s quotes. It carries neither of F6's two fingerprints — no
variant identifier, no field-name punctuation — so it is not the class
the sweep repaired. It is nonetheless a `Debug` rendering in a user's
sentence, and it is spelled inconsistently across one crate:

| door | spelling |
| --- | --- |
| `EditError` (`edit.rs`) | bare — `parameter {}` on `name.0` |
| `PersistError` (`persist/mod.rs`) | quoted — `document parameter {:?}` |
| `NonFiniteSite`, `SnapshotError` (`persist/check.rs`) | quoted |
| `RangeRefusal` (`range.rs`) | quoted |
| `SplitError`, `InlineError` (`refactor.rs`, FIX's) | quoted |
| `ParseError` (`parse.rs`) | quoted |
| `EvalError` (`expr.rs`) | quoted |

`EditError`'s own `Display` header already records the split and
declines to decide it: its category prefix and its quotes were removed
deliberately, and the comment says its neighbours *"keep their spelling
until someone decides for them"*. Nothing schedules that decision, which
is what this file is.

## Why it is not simply "quote nothing"

The two spellings answer different questions. A bare name composes with
a sentence that already frames it (`parameter width is declared
length`); quotes delimit a name the reader supplied and may have
mistyped, which is why `ParseError` quotes — `{name:?} is not a
parameter this document declares` is about the exact bytes. A blanket
sweep either way would lose one of those.

`ParamName` is a `String` newtype with no `Display`, so the choice today
is made per call site with `{:?}` or `name.0`. A `Display` on
`ParamName` would move the decision to one place, which is probably the
shape of the answer; whether the quotes ride it, or ride a second door
for the did-you-type-this case, is the decision.

## The hazard that makes it more than taste

`Debug` on `String` is its prose today because `String` cannot grow a
field, and every site in the table renders either a `String` field
(`key`, `found`) or a `ParamName`'s `.0`. That is an assumption the
rendering sites make and do not state: a metadata key re-typed from
`String` to a structured key, or a `{name:?}` written over the newtype
instead of over its `.0`, starts dumping braces with no edit to the
sentence.

`crates/pncad-py/src/prose_census.rs` bounds that exposure where it can
see it — it resolves the field's declared type, so a field re-typed to a
struct reds the census. It cannot see the positional form: a bare
`{:?}`, which is how most of the table's sites are written, reaches its
`UNDECIDED` roster as `<positional>` and is decided by nobody. So the
instrument covers the named half of this residue and not the half the
crate actually uses.
