---
id: atrest-findings-open-with-a-tier-stage-prefix
kind: issue
title: The at-rest findings open with a tier stage label ("tier 3:", "tier-3′ census:") the viewer shows the user
status: open
opened: 2026-09-24
refs: [error-and-check-text-overflows-its-region]
priority: P4
cost: E
---

## What

`topo::ValidationError`'s `Display` (`crates/topo/src/validate.rs`,
`impl fmt::Display for ValidationError`) opens sixteen arms with a
tier label: `"tier 3: "` on `Band`, `ApproxCertification`,
`ApproxLaneUnsupported`, `Pcurve` and the four `Ring*` arms, and
`"tier-3′ census: "` on every census arm (`UndeclaredContact`,
`ContactContradicted`, `StaleContactDeclaration`, `CensusEscalated`,
`CensusUnsupported`, `CensusLaneUnsupported`, `CensusUndecidable`,
`InstanceInterference`). The standard
(`work/chrome/error-and-check-text-overflows-its-region.md`, "The
standard a refusal is rewritten to") names stage prefixes developer
detail, and these reach the viewer's at-rest and product badges
verbatim.

`test_utils::refusal::stage_prefixes` does not see either: it wants
every token to open with a lowercase letter, and `tier 3`'s second
token opens with a digit, while `tier-3′` carries a `′` outside
`[a-z0-9_-]`, so the shape check reads both as English. `editor-core/tests/refusal_concision_at_rest.rs`
checks the word budget only, for that reason.

## Why it was not taken with ATREST-8

The census label is load-bearing outside the prose:
`crates/pncad-py/tests/test_validate.py`
`test_the_count_is_the_findings_and_it_is_deterministic` counts
`"tier-3′ census:"` occurrences as the finding separator, so dropping
it is a change to that row's instrument, not a re-baseline of a
sentence. ATREST-8 rewrote two census arms and kept the label on both
so the family reads one way until the whole family moves.

## The fix

Drop both labels from every arm in one change; re-derive the Python
row's count from `failure_count` and the `\n` join rather than the
label; extend `stage_prefixes` to see a digit- or prime-bearing
token, or add the two labels to its shape, so the next one reds.

