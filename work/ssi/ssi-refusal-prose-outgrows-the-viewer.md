---
id: ssi-refusal-prose-outgrows-the-viewer
kind: issue
title: geom-brep: SsiError::FitSampleBudget is over 50 words (Ev's concision request)
status: closed
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
cost: E
priority: P4
closed: 2026-10-01
pr: 3651
branch: ssi/diagnoses
---


## What

Refusal `Display` arms on this program's ground that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 55 | `geom-brep/src/ssi.rs` `SsiError::FitSampleBudget` | |

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

## The same arm advises raising ε (ENCL sweep, 2026-09-28)

`SsiError::FitSampleBudget` ends "raise the tolerance, or wait for the
compaction work ...". D4 ¶1 (i) (Ev, `[ev]` PR 3352) keeps loosening ε
only at a kernel approximation limit that names no other recourse. There
it is a last resort and says the refusal may indicate a kernel bug worth
reporting. A spent fit-sample budget is that kind of limit ("wait for the
compaction work" is not a lever the user holds), so the rewrite this row
asks for should end the arm as the loosening clause followed by
`geom_core::KERNEL_LIMIT_LAST_RESORT`, with the payload's value if it
gives one. The ENCL kernel-limit lane found it with a second sweep pass
for loosening spelled without the word "loosen", and left it here as
SSI's ground.

## Closed (2026-10-01, PR 3651)

Landed in `ssi/diagnoses`. Every SSI refusal now names its operand and decision, and ends by `certify::recourse`. The PR body records each decision; review was a single FULL review with one fix pass.
