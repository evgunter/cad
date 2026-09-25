---
id: atrest-refusal-prose-outgrows-the-viewer
kind: issue
title: topo: the at-rest validation findings are too long for the checks window (Ev's concision request)
status: closed
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
priority: P1
cost: D
parent: ATREST-8
closed: 2026-09-25
pr: 3185
---


## What

Refusal `Display` arms on this program's ground that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 103 | `topo/src/validate.rs` `ValidationError::LaminaWedge` | |
| 65 | `ValidationError::TangentNotIntrinsic` | |
| 62 | `ValidationError::InstanceInterference` | |
| 60 | `ValidationError::ScaffoldAtRest` | |
| 52 | `ValidationError::CensusLaneUnsupported` | |

These are what the checks window lists as findings.

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
