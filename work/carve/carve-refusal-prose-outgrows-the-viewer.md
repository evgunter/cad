---
id: carve-refusal-prose-outgrows-the-viewer
kind: issue
title: sweep: the loft, revolve, tube and blend refusals over 50 words (Ev's concision request)
status: closed
closed: 2026-09-23
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
---


## What

Refusal `Display` arms on this program's ground that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 69 | `sweep/src/loft.rs` `LoftError::ReversedStacking` | |
| 68 | `sweep/src/revolve/mod.rs` `RevolveError::MultipleAxisRuns` | |
| 64 | `sweep/src/revolve/tube.rs` `TubeError::WallGapCollapsed` | |
| 55 | `sweep/src/blend/mod.rs` `BlendError::NonpositiveSize` | (also BAND's ground) |

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

## Closed (2026-09-23)

All four arms were rewritten at the source by the CHROME concision
pass: `LoftError::ReversedStacking` (79 rendered words before, 51
after), `RevolveError::MultipleAxisRuns` (75 → 44),
`TubeError::WallGapCollapsed` (71 → 38) and `BlendError::NonpositiveSize`
(62 → 23). The same pass rewrote the rest of `ExtrudeError`,
`RevolveError`, `TubeError`, `SkinError`, `LoftError` and `BlendError`
(its recourse sentences shortened with every phrase the followability
suites pin kept), and moved the routed blend escalations onto the
payload view.

`editor-core/tests/refusal_concision_chains.rs`
`every_node_refusal_renders_within_the_budget` now renders every arm
listed above the way the feature tree draws it and holds it to the
75-word budget, with no stage prefix and no arena key outside the
kernel-bug arms it names. The rewrite and its census are in the
`chrome/concision-chains` PR and in
`work/chrome/error-and-check-text-overflows-its-region.md`,
section "The remaining chains".
