---
id: patherror-escalated-runs-past-the-word-budget
kind: issue
title: PathError::Escalated renders 65 words, over the 75-word budget once a payload lands
status: open
opened: 2026-10-01
---


(PROPS's recourse-grammar lane, from the re-derived census
`work/props/props-refusal-prose-outgrows-the-viewer` describes: every
`impl Display for` block in `crates/*/src`, split into match arms,
counting each arm's string literals with a `{…}` placeholder as one word
and named recourse constants NOT expanded. The standard — 75 words on the
RENDERED text, with its test — is stated once in
`work/chrome/error-and-check-text-overflows-its-region.md`, section "The
standard a refusal is rewritten to".)

## What

`profile::PathError::Escalated` (`crates/profile/src/path.rs`) counts **65
literal words**, the longest arm left in the tree. A realistic payload
adds to that: the arm renders an escalation whose own `Display` is a
sentence of its own, so what the viewer draws is well past the budget.

Three sibling arms in the same `Display` also still name an UNVALUED
tolerance arm, which is the other half of the same grammar:
`JunctionTangent` and `JunctionCusp` end "otherwise move the geometry (or
lower the tolerance)", and they CARRY their margin and lever arm as
payload — so the value D4 ¶1 (i) asks for is in hand at the site.
`profile::validate`'s `FILLET_OFFSET_LEVER_RECOURSE` ends "or lower the
tolerance" the same way.

## Why it is filed here

`crates/profile/src/path.rs` and `validate.rs` are `paths`' (and
`round`'s) by territory. PROPS' unit retired the unvalued tolerance arm
from the three shared constants in `geom-core`
(`COINCIDENCE_RECOURSE`, `NO_DECLARATION_RECOURSE`,
`SPLIT_PLANE_RECOURSE`) and put the valued form in one home,
`geom_core::Indeterminate::ending(levers)`; a site holding an escalation
composes it from there. These arms hand-spell theirs instead, so they did
not move with the constants.
