---
id: unowned-refusal-prose-outgrows-the-viewer
kind: issue
title: topo/geom-brep: the refusals over 50 words on ground no program owns (Ev's concision request)
status: open
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
---


## What

Refusal `Display` arms on ground no program owns that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 89 | `topo/src/pcurves.rs` `PcurveMintError::SingularChartJoint` | |
| 75 | `PcurveMintError::LoopDiscontinuity` | |
| 70 | `PcurveMintError::OuterSpansPeriod` | |
| 56 | `PcurveMintError::LoopNotClosed` | |
| 50 | `PcurveMintError::LoopWraps` | |
| 54 | `geom-brep/src/certify.rs` `CertifyError::NotSecondOrderSeparated` | plus `COINCIDENCE_RECOURSE` |

`work.py territory` names no owner for `topo/src/pcurves.rs` or
`geom-brep/src/certify.rs`. `topo::BooleanError` (also unowned) was
rewritten whole by the filing PR and is held to the budget by its test.

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
