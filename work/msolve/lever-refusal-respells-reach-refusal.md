---
id: lever-refusal-respells-reach-refusal
kind: issue
title: LeverRefusal re-spells ReachRefusal's arms plus instance/part by hand in of(): two parallel enums that drift when one grows an arm
status: open
opened: 2026-09-20
priority: P1
cost: E
---



## What

Found by MSOLVE-8's fix pass (PR 2896, style review). `LeverRefusal`
(`crates/editor-core/src/mate.rs`) is `ReachRefusal`'s arms
(`crates/editor-core/src/mate/reach.rs`) re-spelled one for one with an
`instance` and a `part` added to each, projected by hand in
`LeverRefusal::of()` — a by-hand projection between two parallel enums,
so an arm added to `ReachRefusal` compiles until the exhaustive match
in `of()` is reached and then owes a twin arm, a twin `Display`
sentence and a twin Python tag, none of which the type states. The
shape is the one `mate_payload.rs` closed for the Python side (one
record read off the enum) and `wire.rs`'s `refusal` closed for the
direction door (one map, both roads).

## The fix this wants

One carrier: `LeverRefusal { instance, part, refusal: ReachRefusal }`
(or the equivalent — a `Reach` arm wrapping the reach door's own
refusal beside the arms that are the lever's alone, `PartUnresolved`
and `NotAnInstance`), so the reach's vocabulary has one home and the
lever adds its subject rather than restating the words. The Python
tag `lever_refusal_tag` and its census row move with it
(`crates/pncad-py/src/tags.rs`, `test_binding_census.py`'s
`LeverRefusal` entry). MSOLVE's ground (`mate.rs`, `mate/reach.rs`);
the tag is LIB's and wants announcing.

## A second instance: `FaceRefusal` re-spells `FacePoseRefusal` (MSOLVE-9, 2026-09-24)

MSOLVE-9 (PR 2934) landed the same shape a second time.
`FaceRefusal` (`crates/editor-core/src/mate.rs`) is `FacePoseRefusal`'s
arms (`crates/editor-core/src/mate/reach.rs`) re-spelled one for one
with an `instance`, a `part` and a `face` added, projected by hand in
`FaceRefusal::of()`, with a twin `Display` sentence per arm and a twin
Python tag (`face_refusal_tag`, `crates/pncad-py/src/tags.rs`). The
lever's two own arms recur here too: `FaceRefusal::NotAnInstance` is
the solve's, not the reach's, and `PartUnresolved` wraps the resolver's
fault as the lever's does. The one-carrier fix above covers both
pairs — `FaceRefusal { instance, part, face, refusal: FacePoseRefusal }`
beside the arms that are the solve's alone — and should take both in
one unit, since the two pairs share the Python payload's `instance`
projection (`crates/pncad-py/src/mate_payload.rs`) and four tag words
pinned as one fact
(`crates/pncad-py/src/tests.rs`'s
`a_face_refusal_spells_the_facts_it_shares_the_way_their_own_maps_do`).
