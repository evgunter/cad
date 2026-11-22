---
id: a-validated-arcs-turn-is-a-sign-that-admits-zero
kind: issue
title: A validated arc's turn is a Sign, so SegmentKind::Arc { turn: Zero } can be written and a check holds it off
status: open
opened: 2026-11-22
---


Found by PR 7384's review (2026-11-22). `profile::SegmentKind::Arc`'s
`turn` is a `geom_core::Sign`, three-valued, while a classified arc's
turn is never `Zero`: the classifier refuses it typed before an
`ArcGeom` exists (`seg::full_turn`'s `Gate::FullTurnSense` and the
chord arm's own turn read, each `Sign::Zero => ShapeIssue::DegenerateTurn`).
Every reader then treats `Zero` as something it cannot meet:
`sweep::swept::turn_negates` reads it as the positive arm on purpose
("total rather than loud") so that `turn_axis`, `arc_span`,
`centre_on_material_side` and `Traversed::half_turn` cannot part on it.

A two-valued type exists: `profile::ArcSweep { Ccw, Cw }` (`sugar.rs`),
the authoring sense. Using it (or a two-valued turn of its own) for the
validated arc makes `Zero` unwritable, and removes the "Zero takes the
positive arm" convention from `turn_negates`. Not done in PR 7384: every
reader of a validated turn in `profile`, `sweep`, `editor-core`'s rows,
`viewer` and the value-channel digest (which hashes the turn) changes
with it, and the revolve's tube passes a `Sign` it decided.
