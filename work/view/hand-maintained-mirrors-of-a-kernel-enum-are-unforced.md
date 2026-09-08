---
id: hand-maintained-mirrors-of-a-kernel-enum-are-unforced
kind: issue
title: forms::BOOLEAN_OPS mirrors a kernel enum declared in another crate, and no compiler forces the mirror
status: open
opened: 2026-09-06
refs: [2046, dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum, boolean-op-has-a-third-hand-written-complete-list]
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

`forms::BOOLEAN_OPS` (`crates/viewer/src/forms.rs:67`) is not:
`BooleanOp` is declared at `crates/topo/src/boolean/mod.rs:138`. The
table lists all three of that enum's variants today and **nothing
forces it to keep doing so** — a fourth operation added in `topo` would
leave this table three long and the boolean form three buttons wide,
with no compile error and no red row anywhere. It is `pub(crate)` with
a production reader (`crates/viewer/src/pane/create.rs:892`), so it is
NOT an instance of the reader-count class that
`tool-kind-all-and-ordinal-have-no-production-reader` was about.

`forms::MATE_PRIMITIVES` (`crates/viewer/src/forms.rs:482`, reader
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

## Appended after PR 2046's style review (2026-09-06)

**The first answer is cheaper than this item priced it.** "Export the
vocabulary from the crate that owns it" was costed above as "a public
list on a kernel type for a chrome's benefit". That cost has already
been paid, once, for a neighbouring kernel vocabulary and for this very
crate: `crates/topo/src/contact.rs:67` publishes
`ContactClass::ALL`, `crates/viewer/src/matetool.rs:132` maps over it,
and `contact.rs`'s own doc gives the argument for why — a planted third
variant "failed this crate at its designed fences and left a downstream
`[Rest, Tangent]` literal green", so the slice is "the fix at the
source". That is the same shape, the same direction, and the same
argument. The option is precedented, not novel.

**And it buys two sites, not one.** `BooleanOp` has a THIRD complete
hand-written list, in a different order, at
`crates/editor-core/src/persist/kernel_wire/boolean_op.rs:35` — see
`work/issues/boolean-op-has-a-third-hand-written-complete-list.md`,
which also names eight unswept `const ALL` arrays of kernel enums
elsewhere in the workspace. A published `BooleanOp::ALL` retires the
`editor-core` copy as well as this one.

**A third instance inside `crates/viewer/src`**, filed separately as
`work/view/dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum.md`:
`pane/properties.rs`'s new-parameter radio row is a complete inline
mirror of `editor-core`'s `Dimension`. Whoever takes this item should
take that one — three instances is what the class has in this crate,
not two.

**A fourth, found by re-running this unit's census pass with its
anchoring bug removed** (see the closed item's method note):
`crates/viewer/src/pane/viewport.rs` maps `egui::PointerButton`'s three
buttons to `crate::input::PointerButton`'s three, by hand, in
production. Both sides are complete at three and nothing forces either;
a fourth button on either side is silently never produced. It is the
same class with the mirrored enum in the TOOLKIT rather than the
kernel, which is the one place a `pub` list cannot be asked for
upstream — so it is the instance that most likely wants the "a row
rather than a mechanism" answer. Not filed separately: it is this
class, and the file is here.
