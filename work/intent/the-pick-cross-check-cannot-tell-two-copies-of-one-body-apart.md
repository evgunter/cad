---
id: the-pick-cross-check-cannot-tell-two-copies-of-one-body-apart
kind: issue
title: The viewer's pick cross-check reads a face by name, and two copies of one body wear the same names
status: closed
opened: 2026-10-23
priority: P3
cost: E
refs: [assemble-is-an-operator-and-the-product-is-a-slot]
pr: 5926
branch: intent/cost-e-batch
closed: 2026-10-28
---

Found by D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`). A copy carries its source's names verbatim, so two placements of one body draw two faces under one stable name. The viewer's id-buffer cross-check (`idpass::compare`) agrees an id answer with the analytic pick by NAME, so an id answer on the other copy reads as agreeing: the name is the same face of the body, on either copy. `crates/viewer/tests/frame_policy.rs:2300` (`a_twin_copys_face_wears_the_same_name`) pins this: both copies' ids map to the name, and the check is quiet on either.

Which copy a pick is on is its node (`PickHit::node`), which the cross-check does not compare. A mismatch between the two copies (the id buffer answering the left copy while the analytic pick lands on the right) goes unreported. Shape of a fix: compare the node as well as the name, so the check keys its agreement by (node, name).
