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
| 54 | `topo/src/boolean/solid_contain.rs` `PointInSolidError::PartialConeFace` | rewritten by the filing PR from 165; the recourse is most of what is left |
| 50 | `PointInSolidError::PartialSphereFace` | rewritten by the filing PR from 165; likewise |

`PointInSolidError`'s other long arms were rewritten by the filing PR.
`ContainError::ArcLoopUnsupported` was left because
`sweep/tests/census_containment_cause.rs` pins its wording as the
census finding's own words, and it is short enough that the gain is
small.

## The standard (from `work/chrome/error-and-check-text-overflows-its-region`)

Ev reported (2026-09-17) that the viewer's error messages are far too
long, and ruled on 2026-09-22 that the fix is **at the source**: the
`Display` of each refusal the viewer shows is rewritten in the crate
that raises it, not summarised by the viewer. A good refusal says, in
the user's terms: what could not be done, the short reason, and what
they can do about it. **The recourse is the part never to drop.**
Developer detail (routing, dispatch tables, predicate names, bands,
doc paths, issue numbers, which lane is unbuilt) moves to the variant's
rustdoc, or stays in the payload `Debug` already carries, rather than
into nothing. Types, variants and payloads do not change; prose only.
Tests asserting the old text are re-baselined, never weakened into
something that cannot go red — asserting the recourse phrase is fine.

The worked example and its family (`topo::BooleanError`,
`topo::PointInSolidError`, and `editor_core::NodeErrorKind::Boolean`'s
wrapper) were rewritten by the concision PR that filed this row; the
torus × plane union refusal went from 277 words to 65, and
`editor-core/tests/refusal_concision.rs` pins it.

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
