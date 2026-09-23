---
id: paths-refusal-prose-outgrows-the-viewer
kind: issue
title: profile: the path and corner refusals are the longest text the viewer shows (Ev's concision request)
status: open
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
---


## What

Refusal `Display` arms on this program's ground that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 222 | `profile/src/path.rs` `PathError::Escalated` | whole arm, across its per-predicate sub-arms (each ~50–80) |
| 156 | `profile/src/path.rs` `CornerReason::EnclosesLegCarrier` | |
| 127 | `PathError::SeamArrivalOffDirection` | |
| 109 | `PathError::ContinuationTargetOffRay` | |
| 94 | `PathError::JunctionCusp` | |
| 87 | `PathError::SeamTangent` | |
| 80 | `PathError::UnderflowedDirection` | |
| 76 | `PathError::JunctionTangent` | |
| 75 | `PathError::SeamArrivalLeverTooShort` | |
| 74 | `PathError::FilletOffsetLeverTooShort` | |
| 66 | `PathError::FilletArcFlattenedInStorage` | plus `FILLET_FLATTENED_RECOURSE` |
| 60 | `PathError::FilletCarrierBelowSceneResolution` | plus `FILLET_SCENE_RESOLUTION_RECOURSE` |
| 59 | `PathError::NonFiniteDirection` | |
| 51 | `PathError::ArcCenterNotEquidistant` | |

These reach the viewer through `NodeErrorKind::Profile*` and through
the profile editor's preview verdicts (`crates/viewer/src/pane/profile.rs`).

## The standard

The standard these arms are held to is stated once, in
`work/chrome/error-and-check-text-overflows-its-region.md` (section
"The standard a refusal is rewritten to"), with its word budget and the
test that enforces it.

## How the census was taken, and what it could not see

A static pass over every `impl Display for` block in `crates/*/src`,
split into match arms, counting the words of each arm's string
literals (a `{…}` placeholder counts as one word; a named recourse
constant such as `COINCIDENCE_RECOURSE` is NOT expanded, so a
constant-carrying arm is longer on screen than its count). A realistic
payload adds to every count: a nested refusal (`{e}`, `{source}`)
renders its own arm inside this one. Arms at or above 50 literal words
are listed. The pass does not prove each arm is reachable from the
viewer — most reach it through `NodeErrorKind`'s forwarding arms
(feature tree fault line, status line) or through the checks window —
and a `Display` written outside `impl Display` (a helper returning a
`String`) is not seen.
