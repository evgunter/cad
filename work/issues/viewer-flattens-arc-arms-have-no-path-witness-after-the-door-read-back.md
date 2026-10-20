---
id: viewer-flattens-arc-arms-have-no-path-witness-after-the-door-read-back
kind: issue
title: viewer: sketch::flatten's arc-frame refusals (a centre or far side that is not a point) are no longer reachable from a path, since the path door refuses those arcs first at f64; their rows now pin the door's refusal
status: open
opened: 2026-10-20
---


Found by PR #5449 (the path door's arc read-back).

At a point scalar the path door now reads every arc it builds back
against its end vertex before it stores it. An arc whose centre
overflows lands at NaN and escalates on `path_arc_landing`; an arc at
~1e308 m misses at a magnitude no tolerance resolves and refuses
`ArcMissBelowSceneResolution`. So `crates/viewer/tests/path_authoring.rs`'s
`an_arc_whose_centre_overflows_refuses_at_the_preview` and
`an_arcs_far_side_past_the_range_refuses_though_its_frame_is_finite`
now pin the door's refusal, not `PreviewError::Unflattenable`, and
`a_far_millimetre_arc_draws_and_its_validation_refuses` became
`a_far_millimetre_arc_refuses_at_the_door`.

What that loses: those two rows were the witnesses for `sketch::flatten`'s
centre/start check and its minted-coordinate check on arcs ("deleting
`flatten`'s `centre`/`start` check leaves the row above green and reds
this one"). No path reaches either arm with an arc now, so nothing reds
if they are deleted. The arms are still reachable in principle from a
loop that does not come through the door (a table), and the vertex arm
(`Line(1e308)`) still has its row. Either give the arc arms a witness
that does not go through the path door (an in-crate test of `flatten`
on a hand-built `ProfileLoop`), or say in `flatten`'s docs that the
door makes them defensive.
