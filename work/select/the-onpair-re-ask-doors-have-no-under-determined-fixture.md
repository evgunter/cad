---
id: the-onpair-re-ask-doors-have-no-under-determined-fixture
kind: issue
title: The OnPair re-ask doors (join, offset, split piece) have no fixture that reaches them under-determined since U-lever
status: open
opened: 2026-10-15
priority: P3
cost: M
---


Filed by U-lever (`u-lever-the-wedge-is-levered-at-the-faces-extent`).

Before U-lever the shoulder rows in `sweep/tests/wedge_end_doors.rs`
reached an under-determined `OnPair` seam by shortening the seam: the
must-carry sagitta was levered at the edge's own length. It is now
levered at the faces' extent, and a short seam between faces that run
past the radius reads the intrinsic tangency, so those rows assert the
tangency instead. What they no longer exercise is the doors that
re-ask an `OnPair` after changing its edge: the join's
`Body::reask_on` (`boolean::edge_join::joined_spec`), the offset doors'
`offset_restate::re_ask_on_pair`, and `split::tangent_piece`'s pair arm.
An `OnPair` still arises where the rule reads the pair under-determined
at the faces' extent: a G2 join (`κ_rel = 0`), or faces that are short
for their curvature, which the profile door tends to decide first
(`must_carry_rule`'s extrude rows). A fixture of either kind through
each re-ask door would restore the coverage.
