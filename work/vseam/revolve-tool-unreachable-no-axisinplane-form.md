---
id: revolve-tool-unreachable-no-axisinplane-form
kind: issue
title: The revolve tool cannot be reached from the panels: no form authors a Datum::AxisInPlane
status: closed
opened: 2026-09-04
refs: [viewer-session-god-module-split]
closed: 2026-09-17
branch: viewer/axis-in-plane-form
---


Found by the whole-file read that opened `viewer-session-god-module-split`
(2026-09-04). Not a finding of that unit's diff — the unit has no diff
yet — which is the point: this is invisible to every per-unit review
because no unit's diff contains both halves.

## What happens

`add_revolve` requires a `NodeKindWanted::SketchAxis` seat
(`crates/viewer/src/session.rs:1767`, the `require_kind` at `:1677`;
the seat itself is `crates/viewer/src/seats.rs:161`), and `admits`
(`crates/viewer/src/session/refuse.rs:61`, the `SketchAxis` arm at
`:65`) satisfies that seat for
`Datum::AxisInPlane` and nothing else.

The panel that authors datums offers four kinds —
`forms::DatumKindChoice` is Plane, Frame, Axis, Point
(`crates/viewer/src/forms.rs`, `:124` at this writing; the four arms
are built in `crates/viewer/src/pane/create.rs`, `add_datum_ui`'s
`DatumSpec` match) — and
`AxisInPlane` is not among them. `add_datum_ui` is the only
`DatumSpec` construction site in `src/`; every
`DatumSpec::AxisInPlane` in the tree is in `crates/viewer/tests/`.

So the revolve tool ships, is listed, opens, and its seat can never be
filled from the running application. Only a headless test can author
the node it needs.

## Why the refusal does not say so

Nothing is wrong at any single door. The seat refuses correctly, with
the right wording, on the datum kinds the panel *can* author — it
refuses `WrongNodeKind` and names what it wanted. The user is told
"this is not a sketch axis" over and over, truthfully, with no way to
produce one, and the application never says that no way exists. This
is the failure mode the reachability of a tool should be a property
of, not a consequence of two independently-correct lists disagreeing.

## The class

It is the second instance of one shape on this board.
`work/chrome/add-profile-mints-no-frame` is the first: a form that
cannot mint the thing its own seat requires. Both are a *seat*
vocabulary and an *authoring* vocabulary maintained by hand against
each other, with no row asserting that every seat a shipped tool
opens is fillable through the panels. The general guard is one row
per tool: open it, and assert some authorable form satisfies its
seat. Whether that row is affordable is a question for whoever takes
this; the two instances are enough to say the pattern is not an
accident.

## Home

CHROME's, by charter — that program is "viewer chrome and coverage"
and already holds the sibling instance. Filed here because VIEW holds
the viewer floor while CHROME's `src`-touching slate is parked behind
this program's unit 1, and `work/README.md` forbids copying an item
between slates. Re-home by header edit when CHROME next moves; the
announce is owed either way, because the class guard above is a
coverage row and coverage is CHROME's word.


## Citations re-pointed after the 1c split (VIEW orchestrator, 2026-09-04)

This file was written against the pre-split tree. The `file:line`
citations above are corrected in place; this note exists so a reader
who remembers the old ones can tell a correction from a claim change.
Nothing about the finding moved — `stale-file-citations-after-the-split`
is the general case, and this is VIEW's own half of it being paid.

## Two citations this row carried that the tree had moved past

Both refreshed above, both noted rather than silently fixed. The enum
was named `forms::DatumKind` when this was filed and is
`forms::DatumKindChoice` since
`two-datumkind-enums-name-the-same-four-datum-kinds`; and the member
ORDER quoted here, `Plane, Axis, Point, Frame`, was the declaration's
until PR 2046 reordered it to form order, `Plane, Frame, Axis, Point`.
Neither changes what this row finds.

**What that row settled bears on this one's fix.** The add-datum form's
enum and the public `viewer::DatumKind` a datum DRAWING carries are two
types on purpose, and this row is the reason: giving the form an
`AxisInPlane` choice grows `DatumKindChoice` to five while the draw tag
stays at four, because an `AxisInPlane` is drawn as the axis it is. A
fix here adds a member on one side only, and that is correct.

## Closed

The add-datum form offers a fifth kind, `DatumKindChoice::AxisInPlane`
("axis in sketch"): a frame picked from the document's frames, a 2-D
origin and a 2-D direction in that frame's coordinates, and a line
saying a revolve needs its profile on the same frame. The button waits
on the pick. The lowering moved out of the button's closure into
`Drafts::datum_spec`, so it is testable without egui, and the
`partial_mirror!` roster over `DatumSpec` now offers every arm, with an
empty absent section that is still the growth alarm.

The class guard this row asked for is
`drafts::tests::every_datum_seat_is_fillable_from_the_add_datum_form`:
for every `Seat` whose wanted kind is a datum, some add-datum choice,
lowered from its default drafts, authors a node the seat's own `admits`
takes. It fails on the tree this row was filed against (no choice
authored a `SketchAxis`). **What it does not cover**: the `Profile` and
`Body` seats, which other forms and ops fill; it skips them by an
exhaustive match on `NodeKindWanted`, so a new wanted kind has to be
classified before it builds. It reads the lowering, not the widgets, so
a form that stopped DRAWING a choice would pass. Nothing drives the
egui forms headlessly today.

The add-datum door now gates an axis-in-sketch's `plane` by kind
(`NodeKindWanted::Frame`), as the add-profile door gates its plane.

The frame picker is shared with the add-profile form (`frame_picker`
in `pane/create.rs`), so `work/chrome/add-profile-mints-no-frame`'s
naming-by-node-number half now lands at one site for both forms.
