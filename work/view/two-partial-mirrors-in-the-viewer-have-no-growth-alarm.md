---
id: two-partial-mirrors-in-the-viewer-have-no-growth-alarm
kind: issue
title: SUBJECTS_WITH_AN_EXPIRY_ISSUER and DatumKindChoice are partial mirrors with nothing to tell them the mirrored enum grew
status: closed
opened: 2026-09-12
closed: 2026-09-14
branch: view/partial-mirror-alarms
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
  (`joined_subject`'s, the wildcard arm at `:574`) has a
  `_ => Subject::Document` arm, so a
  sixth subject WITH an expiry issuer joins the enum, misses the list,
  and no row goes red — its messages are then swept only by
  `StatusUpdate::Clear`, silently.
- **`forms::DatumKindChoice`** names four of `DatumSpec`'s five arms
  (`crates/viewer/src/session/author.rs`). The direction that IS held
  is kind-to-spec (`pane::create`'s lowering match is exhaustive over
  `DatumKindChoice`); the direction that is not is spec-to-kind, so a sixth
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

**`DatumKindChoice`: the same skeleton, not the same arm.** Its partiality
is between two ENUMS rather than between an enum and a list, so
"offered" is not a seat but a counterpart variant, and the roster maps
`DatumSpec` arm to `DatumKindChoice` variant or to a reason. The exhaustive
half is a match over `DatumSpec` (its arms carry `Expr`s and a
`RecipeNodeId`, so it is a `fn(spec: DatumSpec)` with `{ .. }` arms and
no const evaluation), and the seat half asserts against
`DatumKindChoice::ALL`, which `vocabulary!` already projects. Whether that is
a second arm of one macro or a related second macro is the call
whoever takes this makes; what should not happen is a third
hand-written enumeration that nothing holds.

## What is not being claimed

That either site is wrong today. Both partialities are argued at their
own doc and both arguments hold; the defect is that neither survives
the enum growing.

## The rename this row's subject went through

`two-datumkind-enums-name-the-same-four-datum-kinds` renamed
`forms::DatumKind` to `forms::DatumKindChoice`, and the citations above
are spelled for the new name. Nothing else about this row moved: the
spec-to-kind direction is still unheld and the instrument this row asks
for is still unwritten. The rename only removes an ambiguity that would
have bitten whoever writes it — `viewer::DatumKind`, a public type, is
the tag a datum DRAWING carries, and a roster in `forms.rs` naming the
bare word would have read as either.

That draw tag is **not** a third site for this instrument. It has no
`ALL`, it is not a partial mirror of anything (it is a partition of
`DatumValue`'s five arms onto four drawings, `AxisInPlane` sharing
`Axis`'s tag), and its growth is already forced by `draw_one`'s
exhaustive match.

## The citation this row carried that resolved to nothing

The first bullet above named `subject_of`'s match at `:572` as the
evidence that nothing forces a decision when `Subject` grows. **There
is no `subject_of` in `crates/viewer/src/frame.rs`, and there never
was on this tree** — the only `subject_of` anywhere is a test helper in
`crates/geom-core/tests/bounds_census.rs`, unrelated. The function is
`joined_subject` (`frame.rs`, `:570`) and the `_ => Subject::Document`
arm is at `:574`. That is not a line that shifted: it is a NAME that
resolves to nothing, the class
`doc-comments-name-symbols-that-do-not-exist` holds, and it cost the
argument its evidence — a reader who went looking for `subject_of`
would have found nothing and had no way to check the claim. Corrected
in place above; the claim itself was true and is unchanged.

## Settled: one macro, two shapes of it, and the third site is a file

**`SUBJECTS_WITH_AN_EXPIRY_ISSUER` takes the DOOR instrument**, as this
row predicted, and the bare-list arm is the second arm the macro's own
doc said to write when a second caller arrived. `partial_mirror!` moved
to `crates/viewer/src/vocab.rs` beside `vocabulary!`, which is the move
the same doc named.

**`DatumKindChoice` takes the same macro and NOT the same arm**, and
the reason is sharper than "the same skeleton". Its offering is an enum
whose `ALL` is PROJECTED from the declaration, so there is no second
copy of the membership for a seat assertion to hold: naming
`DatumKindChoice::X` as a counterpart already says the radio row draws
it. What it wants is the exhaustive half alone, with the counterpart
named per offered arm so the roster IS the spec-to-kind mapping. That
is the `onto` shape. The seat half — the per-seat `assert!`, the
out-of-bounds growth signal, the count check — would be inert there,
and writing it would have been the third hand-written enumeration this
row warned against.

**The alarm holds nothing between `DatumKindChoice` and
`datums::DatumKind`.** What it mirrors is `DatumSpec`, so the coming
`revolve-tool-unreachable-no-axisinplane-form` fix — which grows the
form enum to five while the draw tag stays four — moves `AxisInPlane`
from the roster's absent section to its offered section and touches
nothing else. Demonstrated rather than argued: with that change
simulated, the only reds are `pane::create`'s three matches over
`DatumKindChoice`, which are the work that row has to do anyway.

**A third site, filed not fixed.** `session::author::PatternRuleSpec`
mirrors two of `PatternKind`'s three arms with the same deliberate
partiality and no alarm, and it is the one of the three whose mirrored
enum lives in ANOTHER crate. `partial_mirror!`'s `onto` arm names its
counterpart as a value, so a payload-carrying offering like that one
does not compile through it; the macro's doc states the restriction and
`patternrulespec-is-a-partial-mirror-with-no-growth-alarm` holds the
decision.
