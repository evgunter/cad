---
id: offset-refusal-prose-outgrows-the-viewer
kind: issue
title: topo: TransformError::NurbsPlaceholder is over 50 words (Ev's concision request)
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
| 56 | `topo/src/transform.rs` `TransformError::NurbsPlaceholder` | (also SHELL's ground) |

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

`TransformError::NurbsPlaceholder` was rewritten at the source by the
CHROME concision pass (63 rendered words before, 23 after); the
variant's rustdoc keeps why the refusal is by variant. The rest of
`TransformError` lost its `transform:` prefix in the same pass.

`editor-core/tests/refusal_concision_chains.rs`
`every_node_refusal_renders_within_the_budget` now renders every arm
listed above the way the feature tree draws it and holds it to the
75-word budget, with no stage prefix and no arena key outside the
kernel-bug arms it names. The rewrite and its census are in the
`chrome/concision-chains` PR and in
`work/chrome/error-and-check-text-overflows-its-region.md`,
section "The remaining chains".
