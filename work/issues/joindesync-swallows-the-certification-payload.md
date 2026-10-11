---
id: joindesync-swallows-the-certification-payload
kind: issue
title: boolean::ops describe_minted_edges drops a Certification { ResidualExceeded } payload into a bare JoinDesync - the refusal a user sees for the zip's chord-on-cylinder defect
status: closed
opened: 2026-09-13
refs: [declared-surface-pairs-emits-duplicate-pairs, 2105]
priority: P1
cost: E
closed: 2026-10-11
pr: 4625
---


## What

Split out of `declared-surface-pairs-emits-duplicate-pairs` for
findability (both reviewers of PR #2105 looked for this finding by its
payload text and found nothing). `crates/topo/src/boolean/ops.rs`'s
`describe_minted_edges` error arm maps `set_edge_curve`'s
`Certification { ResidualExceeded { Surface2Residual, sample 1 } }`
into `JoinDesync { "minted-edge description failed certification" }`,
discarding which edge, which check and which sample. On the peg-in-bore
scenes A/B (PR #2105, row 0) that bare `JoinDesync` is what a user
sees for the REST-zip's `Line` chord minted on a cylinder wall
(`work/curved/rest-zip-seam-chord-on-cylinder-wall.md`); the payload
would have named the edge and the residual.

## Fix

Carry the certification payload (edge key, check, sample, residual)
through `JoinDesync`'s `what` or a sibling variant; one row asserting
the rendered text names the edge. E.

## Home

Unowned at filing — `boolean/ops.rs` is S-BOOL's glob; on the CURVED
handover list. Filed by the CURVED orchestrator at the merge-door
dual's adjudication.

## Built (branch cleave/folded-graze-join-desync)

`describe_minted_edges`' refusals carry the attachment gate's refusal:
`DescribeRefusal::Failed` and `MergeCoplanarError::KeptBoundaryUndescribed`
hold it as `cause`, and the Boolean refuses a definite failure as
`BooleanError::EdgeUndescribed { edge, failure, cause, fitted }`,
whose text names the check, the sample and the miss and ends in the
kernel-defect ending (a fitted carrier's in the certification's own
last resort), with the edge in its `Debug` (`JoinDesync`'s `&'static str` has
~100 construction sites, so the payload rides the sibling this issue
named). An exact carrier's in-band surface residual is the input's and
refuses as `Escalated { EdgeOnFaces }` (`work/cleave/a-minted-edge-along-a-folded-graze-refuses-join-desync.md`).
Pinned by `boolean::ops` `a_describe_refusal_carries_its_certification`.

## Closed (PR 4625, 2026-10-11)

`describe_edges` now keeps the certification payload. An in-band surface
residual on an exact carrier is `EdgeOnFaces`; every other failure is
`EdgeUndescribed { edge, failure, cause }` with the defect (or fit)
ending, at both the Boolean and merge doors.
