---
id: reach-refusal-prose-outgrows-the-viewer
kind: issue
title: topo: SplitReduceError::UnderflowedSectorChord is over 50 words (Ev's concision request)
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
| 50 | `topo/src/splitting/mod.rs` `SplitReduceError::UnderflowedSectorChord` | |

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

`SplitReduceError::UnderflowedSectorChord` was rewritten at the
source by the CHROME concision pass (57 rendered words before, 27
after), and every `SplitReduceError`, `SplitJoinError` and
`SplitFinishError` arm lost its stage prefix (`split_reduce:`, `split
join:`, `split finish:`) and, outside the kernel-bug arms, its arena
keys. `SplitReduceError::CurvedBooleanUnsupported` rendered 141 words
(the whole SSI routing description); it now renders 38.
`m3_pr3_split`'s door-naming row was re-baselined to the new
invariant: no split arm names a stage.

`editor-core/tests/refusal_concision_chains.rs`
`every_node_refusal_renders_within_the_budget` now renders every arm
listed above the way the feature tree draws it and holds it to the
75-word budget, with no stage prefix and no arena key outside the
kernel-bug arms it names. The rewrite and its census are in the
`chrome/concision-chains` PR and in
`work/chrome/error-and-check-text-overflows-its-region.md`,
section "The remaining chains".
