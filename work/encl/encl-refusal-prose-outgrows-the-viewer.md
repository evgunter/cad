---
id: encl-refusal-prose-outgrows-the-viewer
kind: issue
title: geom-brep: the offset meter and offset fit refusals are too long for the viewer (Ev's concision request)
status: dispatched
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
priority: P0
cost: D
branch: encl/refusal-prose
---


## What

Refusal `Display` arms on this program's ground that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 111 | `geom-brep/src/offset_meters.rs` `MeterError::NormalFloor` | plus `OFFSET_METER_LADDER` |
| 51 | `MeterError::CurvatureHeadroom` | |
| 78 | `geom-brep/src/offset_fit.rs` `OffsetFitError::BoundNotFinite` | (and a second arm, 65) |
| 67 | `OffsetFitError::RefinementStalled` | |
| 61 | `OffsetFitError::Limb` | |
| 57 | `OffsetFitError::SampleCapReached` | |

`offset_meters.rs` is also OFFSET's and SHELL's ground; filed here as
its first-listed owner.

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

## The stage prefix is gone; the lengths are not (CHROME concision fix pass, 2026-09-23)

Every `OffsetFitError` arm opened with the function's name
(`fit_offset:`), which is on screen under the shell op's
`replace_face` refusals. The CHROME pass reworded each opening into a
sentence ("the offset fit's refinement loop stopped …",
`crates/geom-brep/src/offset_fit.rs:565-690`) and changed nothing
else; the lengths in the table above, and the constant names the arms
still hand the reader (`OFFSET_FIT_BUDGET`, `OFFSET_FIT_SAMPLE_CAP`),
are this row's.
