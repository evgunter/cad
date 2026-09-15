---
id: two-datumkind-enums-name-the-same-four-datum-kinds
kind: issue
title: two DatumKind enums in the viewer crate name the same four datum kinds
status: closed
opened: 2026-09-06
closed: 2026-09-14
branch: view/two-datumkinds
---


Found by the style review of PR 2046 (`view/const-all`).

## Two enums, one concept

- `crates/viewer/src/datums.rs`, `DatumKind` (filed as `:276`, which
  is a doc line of the same item; the declaration was `:328` when this
  was taken up and is `:344` now) — `pub enum DatumKind { Plane,
  Frame, Axis, Point }`, re-exported at the crate root
  (`crates/viewer/src/lib.rs:129`, `pub use datums::{DatumDraw,
  DatumKind}`), so it is `viewer::DatumKind`. It tags what a datum
  draw IS.
- `crates/viewer/src/forms.rs`, the form choice (filed as `:93`, also
  a doc line of the same item; the declaration was `:108` when this was
  taken up and is `:124` now) — `pub(crate) enum DatumKind { Plane, Frame,
  Axis, Point }` behind the `app` feature. It tags what the add-datum
  form is OFFERING.

Same name, same four members, same crate. Nothing maps one to the
other and nothing holds them together; `crates/viewer/tests/
datum_draw.rs:23` imports one and `crates/viewer/src/pane/create.rs:14`
imports the other, so a reader meeting `DatumKind` in this crate has to
work out which they have.

## What PR 2046 did to it, without saying so

That unit reordered the form's enum from `Plane, Axis, Point, Frame`
into `Plane, Frame, Axis, Point` so the enum would carry its form
order. The result is that the two enums were variant-for-variant
identical in name, membership AND order — the drift at its smallest it
has ever been, with nothing recording that or holding it there. The PR
body argues the reorder is inert (it is: no `Ord`, no discriminant, no
serde, and `ALL`'s order is unchanged) but does not mention the twin.

## Settled: two types, and the name collision goes

They are not one thing, and the identity of their members is an
arithmetic coincidence rather than a fact:

- `datums::DatumKind` **partitions the datum VALUES by how they are
  drawn**. It has four members over `DatumValue`'s five arms because
  `AxisInPlane` is a line in space and is drawn as the axis it is
  (`datums.rs`, `draw_one`'s `AxisInPlane` arm), so it shares `Axis`'s
  tag. A surjection onto drawings.
- The form's enum **selects what a plain-numbers form can author**. It
  has four members over `DatumSpec`'s five arms because `AxisInPlane`
  needs a frame PICK first. An injection into specs.

The same arm, `AxisInPlane`, is both the value that collapses and the
spec that is not offered — for two entirely unrelated reasons. That is
the whole of why the memberships match.

**Merging would make a false claim and this board already schedules
its counterexample.** A merged type's `ALL` is read by the radio row,
so "drawable" would imply "offered by the add-datum form". A datum
that drew distinctly but needed a pick to author would then arrive as
a radio button with no `DatumSpec` to lower to. That is not
hypothetical: `revolve-tool-unreachable-no-axisinplane-form` asks for
`AxisInPlane` to become authorable, and its fix grows the form's enum
by one while the draw tag stays at four.

**So: two types, and the hold is the NAME.** The form's enum is now
`forms::DatumKindChoice`, which is this crate's own spelling for a
form's choice as distinct from the thing chosen among
(`PatternKindChoice` beside the kernel's `PatternKind`,
`blend::BlendKindChoice`); it was the one form choice that took the
bare name, and the only one that collided. Both declarations now carry
the relationship: why they are two, and why they have the same four
members today.

There is **no mechanical hold**, because there is no invariant to
hold. A test asserting the two stay identical would hold the
coincidence and would have to be deleted the day the revolve row
lands, which is the correct outcome rather than a regression.

## What happened to the string tags and to `ALL`

Both stay exactly where they were. The words (`Plane = "plane"`, …)
are table data on the form side — `pane::create::add_datum_ui` walks
`DatumKindChoice::ALL` for them — which is the test
`crates/viewer/src/vocab.rs` states. `datums::DatumKind` keeps its
hand-written `label` match and gains no `ALL`: nothing iterates it for
words, and it is a classification of drawings rather than a
vocabulary the chrome offers, so it does not become a `vocabulary!`
declaration either. This file's "the survivor should be declared
through `vocab.rs`" applied to the merge outcome, which is not the one
taken.

Public surface is unchanged: the renamed enum is `pub(crate)` behind
`app`.
