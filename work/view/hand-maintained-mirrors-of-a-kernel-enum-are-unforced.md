---
id: hand-maintained-mirrors-of-a-kernel-enum-are-unforced
kind: issue
title: forms::BOOLEAN_OPS mirrors a kernel enum declared in another crate, and no compiler forces the mirror
status: open
opened: 2026-09-06
---


Filed by the `const ALL` unit, which converted every table it could and
is naming the one it could not rather than leaving the residue in a PR
body (`work/README.md`: a residue named only in a body dies with the
directory).

## What the unit fixed, and why this is left over

Nine of the viewer's ten `const ALL` tables enumerated a viewer enum,
and each is now projected from that enum's own declaration by
`crates/viewer/src/vocab.rs`'s `vocabulary!`: a variant cannot reach
the enum without reaching the list. That mechanism needs the
declaration to be HERE.

`forms::BOOLEAN_OPS` (`crates/viewer/src/forms.rs:44`) is not:
`BooleanOp` is declared at `crates/topo/src/boolean/mod.rs:138`. The
table lists all three of that enum's variants today and **nothing
forces it to keep doing so** — a fourth operation added in `topo` would
leave this table three long and the boolean form three buttons wide,
with no compile error and no red row anywhere. It is `pub(crate)` with
a production reader (`crates/viewer/src/pane/create.rs:887`), so it is
NOT an instance of the reader-count class that
`tool-kind-all-and-ordinal-have-no-production-reader` was about.

`forms::MATE_PRIMITIVES` (`crates/viewer/src/forms.rs:471`, reader
`pane/create.rs:132,147`) is the same shape one step weaker. It lists
three of `MatePrimitive`'s four variants
(`crates/editor-core/src/mate.rs:155`) **on purpose** — `Clocking`
exists so the kernel can refuse it, and a form offering it would be
offering a refusal — so completeness is not what it claims and forcing
it would force the wrong thing. What it shares with `BOOLEAN_OPS` is
the unforced mirror: a new primitive the panel SHOULD offer would not
appear here and nothing would say so.

## The class, stated

**A viewer table that mirrors a vocabulary owned by another crate.**
`crates/viewer/README.md`'s `forms` row calls these "hand-maintained
mirrors of a kernel or sketch enum" and that is exactly the property;
what is missing is anything that reads the two against each other.

Not every mirror on that row is unforced, which is why this item is
about the two `const` tables and not the whole row: `PathVerb` and
`ArcMode` mirror `sketch::PathStep` and `sketch::ArcSpec` and are held
to them by `PathVerb::of` and `ArcMode::of`, exhaustive matches over
the mirrored type, so a new step or mode fails to compile.
`DatumKind` mirrors `session::DatumSpec` partially and deliberately
(no `AxisInPlane` arm — the form does not offer it).

## Answers on the table

- **Export the vocabulary from the crate that owns it.** `topo` would
  declare `BooleanOp` through the same one-declaration construction it
  already uses elsewhere (`crates/profile/src/path/program.rs:215` is
  the precedent) and publish a `BooleanOp::ALL`; the viewer's table
  becomes a `map` over it plus a `label` match, both compiler-forced.
  Costs a public list on a kernel type for a chrome's benefit.
- **A row rather than a mechanism**: a viewer test that matches
  exhaustively over `BooleanOp` and asserts each arm appears in
  `BOOLEAN_OPS`. Cheap, local, and red on the day the kernel grows —
  but it is a second list again, in the suite.
- **Leave it and say so.** Three boolean operations is a closed
  mathematical set; `topo` gaining a fourth is not a thing that
  happens. The argument is real and is why this is an issue and not a
  unit.

Whichever answer is taken, `MATE_PRIMITIVES` takes the partial-mirror
version of it: it wants to know when the mirrored enum grows, not to
be regenerated from it.
