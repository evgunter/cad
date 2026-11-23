---
id: tube-escalated-runs-past-the-word-budget
kind: issue
title: TubeError::Escalated renders 52 words
status: open
opened: 2026-10-01
priority: P4
cost: E
---


(PROPS's recourse-grammar lane, from the re-derived census
`work/props/props-refusal-prose-outgrows-the-viewer` describes: every
`impl Display for` block in `crates/*/src`, split into match arms,
counting each arm's string literals with a `{…}` placeholder as one word
and named recourse constants NOT expanded. The standard — 75 words on the
RENDERED text, with its test — is stated once in
`work/chrome/error-and-check-text-overflows-its-region.md`, section "The
standard a refusal is rewritten to".)

## What

`sweep::revolve::tube::TubeError::Escalated`
(`crates/sweep/src/revolve/tube.rs`) counts **52 literal words**, and it
renders an escalation, so a realistic payload adds that escalation's own
sentence on top.

## Why it is filed here

`crates/sweep/src/revolve/tube.rs` is `carve`'s by territory.

- 2026-11-02 — moved from CARVETAIL to LATHE by `git mv` when CARVETAIL merged into LATHE, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and CARVETAIL had fallen far under it).
