---
id: one-arc-carrier-type-in-geom-core
kind: unit
title: One arc carrier type in geom-core, shared by profile's segments and geom-brep's SketchSegment; sweep's copy goes (byte-identical)
status: dispatched
opened: 2026-09-30
priority: P1
cost: M
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
branch: claude/clever-bardeen-4itqb3
---


Unit of the #3453 ruling (q5): the same arc is spelled four times on
its way to an edge today:
- `profile::Segment::Arc { centre, radius, sweep }`;
- `profile::SegmentKind::Arc { center, radius, sweep, turn }`;
- `sweep::SweptKind::Arc`;
- `geom_brep::SketchSegment::Arc { a, b, centre, radius, sweep }`.

`geom-brep` restates the type because `profile` and `geom-brep` are
siblings. This unit gives them one type.

## Spec (PATHS orchestrator, 2026-09-30)

**Goal.** One arc carrier value in `geom-core`, which both `profile`
and `geom-brep` already depend on. Every other spelling wraps it or
re-exports it.

**Final state:**
- `geom_core::Arc2<T: Real> { centre: Point2<T>, radius: T, sweep: T }`.
  It carries:
  - `map`, which follows the repo's `Point2::map` / `SketchPlane::map`
    convention;
  - `reversed` (negates `sweep`; centre and radius untouched);
  - the arc's one evaluation, `point_from(a, s)`. That is the anchored
    rotation `SketchSegment::eval` computes today, moved verbatim. It
    is the only place the locus is spelled.

  The name is the implementer's to choose. Say why in the PR.
- `profile::Segment = Line | Arc(Arc2)`, and
  `SegmentKind = Line | Arc { arc: Arc2, turn }`. `turn` is
  validation's decision, so it stays on the validated side.
- `geom_brep::SketchSegment = Line { a, b } | Arc { a, b, arc: Arc2 }`,
  whose `eval` calls `point_from`.
- `sweep::SweptKind` collapses into the validated kind, or into an
  `(Arc2, turn)` under traversal. Its field-by-field copies go:
  `sketch_segment_of`, `canonical_sketch_segment` and `swept_kind`.
- The hand copies of the anchored rotation or of `‖a − c‖` in
  `swept.rs`, `seg.rs` and `path.rs` become calls to the one spelling
  where the arithmetic is the same. Where it is not, say so and leave
  the copy.

**Byte-identical, with no exception.** This is a move and a merge of
fields; no arithmetic changes. Every golden, digest, K CSV, Sym ledger
and render is unchanged. If one moves, stop and report it.

**Not this unit:**
- the stored bulge (5a);
- the consistency checks (5a);
- constructed carriers (5b);
- any new registration;
- `radius` staying or going (it stays, per the ruling).

The `pncad` / `pncad-py` surface is unit 6's; re-export what the move
requires and nothing more.

**Review tier: single FULL review.** The change is mechanical, but it
crosses five crates and claims byte identity.

**Claims to falsify:**
- (C1) Byte-identical everywhere: f64, Interval and Sym ledgers.
- (C2) Exactly one spelling of the anchored rotation remains, and every
  evaluator of an arc calls it.
- (C3) No field-by-field copy between arc types remains.
- (C4) The crate layering holds: `geom-core` knows no loop, program or
  sweep.
