---
id: two-partial-mirrors-in-the-viewer-have-no-growth-alarm
kind: issue
title: SUBJECTS_WITH_AN_EXPIRY_ISSUER and DatumKind are partial mirrors with nothing to tell them the mirrored enum grew
status: open
opened: 2026-09-12
---



Filed by the DOOR unit that closed
`mate-primitives-is-a-partial-mirror-with-no-growth-alarm`, whose own
row asked whoever took it to say whether one instrument covers all
three sites. Two of the three are VIEW's ground and are not fixed by
that unit, so they get a file rather than a sentence in a PR body.

## The finding

A DELIBERATELY PARTIAL mirror claims no completeness, so nothing may
force it to be complete — but it still wants to be TOLD when the enum
it mirrors grows, because a member that SHOULD be in it would
otherwise be absent with nothing saying so. `MATE_PRIMITIVES`
(`crates/viewer/src/forms.rs`) now has that alarm: `partial_mirror!`
holds a roster classifying every `MatePrimitive` variant as offered at
a seat of the list or as deliberately absent with its reason, over an
exhaustive match with no wildcard, so a new variant fails the build
until someone decides which it is.

Two siblings have none:

- **`SUBJECTS_WITH_AN_EXPIRY_ISSUER`** (`crates/viewer/src/frame.rs`,
  `:289`) names two of five `Subject`s. Nothing forces a decision when
  `Subject` grows: the only match over a subject in that file
  (`subject_of`'s, `:572`) has a `_ => Subject::Document` arm, so a
  sixth subject WITH an expiry issuer joins the enum, misses the list,
  and no row goes red — its messages are then swept only by
  `StatusUpdate::Clear`, silently.
- **`forms::DatumKind`** names four of `DatumSpec`'s five arms
  (`crates/viewer/src/session/author.rs`). The direction that IS held
  is kind-to-spec (`pane::create`'s lowering match is exhaustive over
  `DatumKind`); the direction that is not is spec-to-kind, so a sixth
  `DatumSpec` arm the add-datum form SHOULD offer arrives with no form
  edit and nothing says so.

## Does the DOOR instrument transfer

**`SUBJECTS_WITH_AN_EXPIRY_ISSUER`: yes, with one more macro arm.** It
is the same shape — a partial LIST of the mirrored enum's own variants
— and differs only in the element: `[Subject; 2]` is bare where
`MATE_PRIMITIVES` is `[(MatePrimitive, &str); 3]`, so the seat
assertion reads `list[seat]` rather than `list[seat].0`. That is the
second arm the DOOR macro's doc says to write at the moment a second
caller exists, and it is the moment to lift the macro out of `forms`
to somewhere both modules can reach (`vocab` is the crate's home for
this construction).

**`DatumKind`: the same skeleton, not the same arm.** Its partiality
is between two ENUMS rather than between an enum and a list, so
"offered" is not a seat but a counterpart variant, and the roster maps
`DatumSpec` arm to `DatumKind` variant or to a reason. The exhaustive
half is a match over `DatumSpec` (its arms carry `Expr`s and a
`RecipeNodeId`, so it is a `fn(spec: DatumSpec)` with `{ .. }` arms and
no const evaluation), and the seat half asserts against
`DatumKind::ALL`, which `vocabulary!` already projects. Whether that is
a second arm of one macro or a related second macro is the call
whoever takes this makes; what should not happen is a third
hand-written enumeration that nothing holds.

## What is not being claimed

That either site is wrong today. Both partialities are argued at their
own doc and both arguments hold; the defect is that neither survives
the enum growing.
