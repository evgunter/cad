---
id: indeterminate-error-arms-sweep
kind: issue
title: The ~40 Indeterminate-carrying error variants the escalation channel makes unnecessary to match on: a deletion sweep
status: open
opened: 2026-09-05
refs: [k-stats-escalation-channel-and-redo, escalation-channel-misses-op-minted-indeterminates, 1969]
---

## What

`grep -rn 'source: Indeterminate\|cause: Indeterminate\|(Indeterminate)' crates/*/src`
finds roughly 40 error-enum variants across `sweep`, `topo`, `profile`,
`geom-brep` and `editor-core` that carry a funnel escalation out of an
op (`ExtrudeError::ExtrusionEscalated`, `LoftError::StackingEscalated`,
`RevolveError::AxisEscalated`, `BlendError::Escalated`,
`ChartRegionError::Escalated`, `PlaneEqError::Escalated`,
`StructureRefusalKind::Indeterminate`, `NodeErrorKind::Escalated`, …).
PR #1969's escalation channel (`NodeValue::escalations`,
`NodeError::escalations`) makes MATCHING on them unnecessary for the
question "did a predicate escalate, and on what margin": the
subdivision driver reads the log first (`crates/editor-core/src/drive.rs`,
`classify_replay`).

## What this item is

The deletion sweep the k-stats spec explicitly named as a different
unit: which of those variants still carry information a consumer needs
(the op's own context around the escalation — Display text, the
recourse sentence, the site) and which are pure wrappers a consumer
could read off the log instead. Not the two arms in `classify_replay`:
those stay load-bearing until
`escalation-channel-misses-op-minted-indeterminates` lands (the log
does not carry op-minted `Indeterminate`s or the mate solve's).

## Acceptance

A hit list of the variants with a disposition each (kept: names why;
deleted: its consumer now reads the log), a sweep pattern with its
blind spot stated, and no behaviour change in what a consumer can
learn.
