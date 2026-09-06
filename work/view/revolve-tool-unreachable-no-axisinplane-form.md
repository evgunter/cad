---
id: revolve-tool-unreachable-no-axisinplane-form
kind: issue
title: The revolve tool cannot be reached from the panels: no form authors a Datum::AxisInPlane
status: open
opened: 2026-09-04
refs: [viewer-session-god-module-split]
---


Found by the whole-file read that opened `viewer-session-god-module-split`
(2026-09-04). Not a finding of that unit's diff — the unit has no diff
yet — which is the point: this is invisible to every per-unit review
because no unit's diff contains both halves.

## What happens

`add_revolve` requires a `NodeKindWanted::SketchAxis` seat
(`crates/viewer/src/session.rs:1196`, the `require_kind` at `:1200`;
the seat itself is `crates/viewer/src/seats.rs:161`), and `admits`
(`crates/viewer/src/session/refuse.rs:61`, the `SketchAxis` arm at
`:65`) satisfies that seat for
`Datum::AxisInPlane` and nothing else.

The panel that authors datums offers four kinds — `DatumKind` is
Plane, Axis, Point, Frame (`crates/viewer/src/forms.rs:52`; the four
arms are built in `crates/viewer/src/pane/create.rs:354-363`) — and
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
