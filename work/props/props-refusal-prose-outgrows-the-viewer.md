---
id: props-refusal-prose-outgrows-the-viewer
kind: issue
title: props: the mass-properties and measure refusals over 50 words (Ev's concision request)
status: open
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
---


## What

Refusal `Display` arms on this program's ground that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 66 | `geom-brep/src/props/mod.rs` `PropsError::NotIsoRectangle` | |
| 62 | `PropsError::QuadratureBudget` | |
| 55 | `editor-core/src/measure.rs` `MeasureUnavailableAt::NeedsEnclosure` | |
| 54 | `editor-core/src/measure.rs` `UnevaluatedReason::WindowSuperset` | |
| 54 | `geom/src/curves/fit.rs` `FitError::BudgetExhausted` | |

`props/sign-hull` (#2468) was open when this was filed.

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

## A `Debug` struct on screen (CHROME concision fix pass, 2026-09-23)

`MinClearanceRefusal`'s `Display` (`crates/editor-core/src/measure.rs:568-575`)
renders the clearance engine's refusal as "the clearance engine refused
`budget` (Depth { max_cell_depth: 20 })": the class is a code
identifier in backticks and the payload is the engine refusal's
`Debug` form. It reaches the feature tree through
`NodeErrorKind::MeasureClearanceRefused`. The file is in open PR #2702,
so the rewrite was filed here rather than made; the feature tree's
guard (`crates/editor-core/tests/refusal_concision_chains.rs`) finds a
`Debug` struct by its shape and admits it on exactly that row
(`FILED_DEBUG`), and removing the entry is the check that this is done.

## The at-rest route no longer renders these whole (2026-09-24, ATREST-8, PR 3185)

A `topo::ValidationError` that carries a `PropsError` (through `MassPropsError::Face`) used to render it whole
behind the at-rest and product badges (that composed length was not
measured before the change). It now classifies each
variant to a short reason and one recourse in the viewer's terms
(`crates/topo/src/validate.rs`, `classify_*`), so that route is
measured by `editor-core/tests/refusal_concision_at_rest.rs` and no
longer by this row. This row's subject is unchanged: the sentence
itself, as the callers that hold a `PropsError` (through `MassPropsError::Face`) directly still read it.
