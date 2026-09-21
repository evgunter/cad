---
id: joindesync-swallows-the-certification-payload
kind: issue
title: boolean::ops describe_minted_edges drops a Certification { ResidualExceeded } payload into a bare JoinDesync - the refusal a user sees for the zip's chord-on-cylinder defect
status: open
opened: 2026-09-13
refs: [declared-surface-pairs-emits-duplicate-pairs, 2105]
priority: P1
cost: E
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
