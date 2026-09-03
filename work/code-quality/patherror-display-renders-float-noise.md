---
id: patherror-display-renders-float-noise
kind: issue
title: `PathError`'s Display arms render scalars with `{:?}`, so refusal sentences carry round-tripped float noise
status: open
opened: 2026-08-30
github: 1282
refs: [1267]
---

## From GitHub issue 1282

opened 2026-08-30, 0 comments.

A **class**, split out of the BLEND-7 review (PR #1267) rather than swept in there.

`Real` carries `Debug` and no `Display`, so every arm of `impl Display for PathError<T>` reaches its scalar payloads through `{:?}`. For `f64` that is the shortest round-tripping form, which is exactly right for a diagnostic dump and wrong in a sentence a person reads: an 8 mm radius that arithmetic produced renders as

> …tangent setback 0.008000000000000002 m exceeds the 0.0034999999999999996 m the anchor pins…

The same applies to `ProfileError` and to the other doors' error types that carry scalar payloads.

## What PR #1267 did

Only the arm it added (`FilletEnclosesLegCarrier`) renders through a small private helper, `path::num`, which prints the shortest decimal that still names the same number to a relative 1e-9 and passes non-`f64` `Debug` forms (intervals, duals) through untouched. It is a display choice only — the payload keeps the exact scalar, and nothing branches on the string.

## What is open

Whether to apply the same treatment across the existing arms, and where the helper belongs if so (several crates have the same shape, so `geom-core` beside `Real` is the obvious home). Worth deciding once: a per-arm trickle would leave the two spellings side by side indefinitely, which is its own smell.

— Filed by the BLEND-7 implementer lane, adjudicating both blinded reviews.

## Home

`work/code-quality/` — this is a structural finding about two spellings of one job (`{:?}` scalars versus `path::num`) living side by side across crates, which is the register's stated subject.
