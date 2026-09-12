---
id: mate-primitives-is-a-partial-mirror-with-no-growth-alarm
kind: issue
title: MATE_PRIMITIVES is a deliberately partial mirror with nothing to tell it the mirrored enum grew
status: open
opened: 2026-09-11
---



Filed by the `BooleanOp::ALL` unit, which closed
`hand-maintained-mirrors-of-a-kernel-enum-are-unforced` and is naming
the half of that item its fix does not reach, rather than letting it
die with the closed item (`work/README.md`: a residue disclosed only in
a `## Closed` section is invisible to the re-homing sweep).

## The finding

`crates/viewer/src/forms.rs`'s `MATE_PRIMITIVES` lists three of
`MatePrimitive`'s four variants (`crates/editor-core/src/mate.rs:155`)
**on purpose**: `Clocking` exists so the kernel can refuse it, and a
form offering it would be offering a refusal. Completeness is not what
the list claims, so the fix the closed item took — publish the owner's
`ALL` and map over it — is the WRONG fix here, and the site now says
so.

What the site still has no answer for is the other half. A primitive
the panel SHOULD offer, added to `MatePrimitive` tomorrow, would not
appear in this form and nothing anywhere would say so. A partial mirror
wants to be TOLD its enum grew; it does not want to be regenerated from
it.

## The shape an answer has

Not a projection. Something that reads the two against each other and
reds when the mirrored enum changes — a row over an exhaustive match on
`MatePrimitive` that asserts each variant is either offered here or
named as deliberately absent, so a new variant fails until someone
decides which it is. That is the "a row rather than a mechanism" answer
the closed item priced, taken for the case where the mechanism is wrong
rather than for the case where it is merely expensive.

`crates/viewer/src/frame.rs`'s `SUBJECTS_WITH_AN_EXPIRY_ISSUER` (two of
five `Subject`s) and `forms::DatumKind` (four of five `DatumSpec`
arms) are the same shape one crate closer, and whoever takes this
should say whether one instrument covers all three or whether the
partiality of each is too different to share one.
