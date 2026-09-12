---
id: frame-direction-refusal-lands-on-the-profile-without-naming-the-frame
kind: issue
title: A frame slot's direction refusal now surfaces on the profile node carrying only a role word, with no id for the frame that actually refused
status: open
opened: 2026-09-12
---



(WIRE orchestrator) From the review of PR 2435 (NOTE 1, `likely`),
reported by the fix pass rather than grown into it because the diff
crosses a crate boundary after the delta verdict.

PR 2435 moved the frame's `f64` placement onto the frame's own
`NodeResult` and made `profile_plane_f64` a READ of it. The refusal
moved with it in one respect and not the other: the *decision* is now
made once, at the frame, but it is *raised* at the reader that needed
it — `crates/editor-core/src/eval/wire.rs:1097`,
`Some(FramePlacement::Unreadable(r)) => Err(r.node_error())`. Raising
it there is deliberate and right (a frame nobody draws on must not
poison the document). What travels with it is not: `refusal`
(`wire.rs:777-784`) maps every `UnitVec3Error` arm onto a variant whose
only field is `role`, and `NodeErrorKind::DegenerateDirection`
(`crates/editor-core/src/eval/mod.rs:912-915`) carries no node id. So
the user reads, **on the profile node**, "the datum frame x axis has
zero length" (`mod.rs:1581-1583`) with nothing saying *which* frame —
and before this change the message sat on the frame node, which was
the right one. Three sibling variants (`NonFiniteDirection`,
`UnderflowedDirection`, `Escalated`) reach the same road the same way.

This codebase already has the convention for a profile-side refusal
caused by its frame, fifteen lines away in `section_of`:
`NodeErrorKind::DerivedFrameSection { profile, frame }`
(`wire.rs:4365`, `:4394`; declared `mod.rs:1233-1238`, `Display` at
`mod.rs:1753-1760`) names both nodes by id. `profile_plane_f64` has
the frame id in hand — it is the `plane: RecipeNodeId` parameter it
was called with.

## Why it was scoped out of 2435

The cheap-looking fix is not cheap: a new `NodeErrorKind` arm plus its
`Display` in `editor-core`, **two arms in
`crates/pncad-py/src/tags.rs`** (the tag map at `:696` and the detail
map at `:824` both match exhaustively), **and a roster string in
`crates/pncad-py/src/tests.rs`** — a new word in another crate's error
vocabulary and its census. That is a surface decision, not a rounding
of the diff that was under review.

## Open question for whoever takes it

Whether the right shape is a new variant that wraps the role *and* the
frame id (paralleling `DerivedFrameSection`), or an id field added to
the four existing direction variants — which would name the node for
every `*_ROLE` caller, not just this road (`DATUM_AXIS_ROLE`,
`PATTERN_DIRECTION_ROLE`, `TRANSFORM_AXIS_ROLE`,
`PLACEMENT_AXIS_ROLE`), and is the larger but more uniform answer. The
four roles' other callers raise on the node that owns the slots, so
for them the id is redundant-but-harmless; this road is the one where
it is load-bearing.
