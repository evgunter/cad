---
id: contact-refusal-prose-outgrows-the-viewer
kind: issue
title: topo: the containment refusals left over 50 words after the concision pass (Ev's concision request)
status: open
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
---


## What

Refusal `Display` arms on this program's ground that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 58 | `topo/src/boolean/contain.rs` `ContainError::ArcLoopUnsupported` | |

`PointInSolidError` was rewritten whole by the filing PR and every arm is
held to the rendered budget by `editor-core/tests/refusal_concision.rs`.
`ContainError::ArcLoopUnsupported` was left because
`sweep/tests/census_containment_cause.rs` pins its wording as the
census finding's own words, and it is short enough that the gain is
small.

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

## The at-rest route no longer renders these whole (2026-09-24, ATREST-8, PR 3185)

A `topo::ValidationError` that carries a `ContainError` (through `CensusUnsupportedCause::Containment` and `RingNestingUndecided`) used to render it whole
behind the at-rest and product badges, composed counts up to 80
words (the ATREST-8 review's measurement). It now classifies each
variant to a short reason and one recourse in the viewer's terms
(`crates/topo/src/validate.rs`, `classify_*`), so that route is
measured by `editor-core/tests/refusal_concision_at_rest.rs` and no
longer by this row. This row's subject is unchanged: the sentence
itself, as the callers that hold a `ContainError` (through `CensusUnsupportedCause::Containment` and `RingNestingUndecided`) directly still read it.
