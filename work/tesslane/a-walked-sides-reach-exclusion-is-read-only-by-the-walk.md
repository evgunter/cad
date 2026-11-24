---
id: a-walked-sides-reach-exclusion-is-read-only-by-the-walk
kind: issue
title: the walk excludes a shared side its neighbour cannot reach (may_make); the CDT arms' one shared-side test does not
status: open
opened: 2026-11-22
priority: P3
cost: C
---


Filed by unit 4c (`tesslane/ruled-density`) when the walk took main's one
shared-side test.

## Finding

The trimmed lane has one test of a shared side,
`TrimPolygon::faces_holding_side` (`crates/mesh/src/trimmed.rs`): a side
joining two polygon points that are not neighbours on their loop and that
one face across the loop's edges holds both of. The CDT arms
(`TrimmedFace::emit`, PR 7198) cure every side it returns by inserting a
point and rebuilding the face. The walk (`trimmed::walk_face`) reads the
same test and then a second reading of its own, `trimmed::may_make`: it
cuts a side only where the face across may REACH it, that is where the
side's midpoint lies within `δ·(1 + cert::ALLOWANCE_SHARE)` of that face's
surface, plus two of the certificate's rounding allowances at the
magnitude of the points read (`cert::rounding_allowance`). Past that, no
triangle the face across certifies holds the side.

## Is the exclusion true of every arm?

Yes, for every lane that passes its triangles through `cert::verdict` at
the run's δ: the rectangle lane, the trimmed lane's cone, cylinder,
sphere, torus, spindle and spline arms, and the walk itself (each reads
`cert::FaceVerdict`). A planar face's triangles lie in its plane, so it
holds a side only where the side's midpoint is within rounding of it. A
memo-replayed patch was certified at the same δ (the memo key folds it).
The argument is at `may_make`. It reads no ε: the cut is a function of
the body and δ (DESIGN `[tolerance.chordal]`;
`split_rulings::the_walks_cut_is_the_same_at_every_eps`).

## How often main's rule fires where the exclusion would hold

Not counted. A CDT arm that inserts a point for a side no face across can
reach spends a retry and lays a point the face does not need; it is never
wrong. Counting it needs an instrumented run over the suites, which this
unit did not keep.

## What would change

The CDT arms could read `may_make` beside `faces_holding_side`, and skip
the sides it excludes. That moves their meshes, so it is TESSLIE's call
with a count in hand, not a change this unit makes.

## The premise the exclusion rests on (2026-11-22, restated 2026-11-23)

`may_make` compares the neighbour's TRUE surface distance with
`δ·(1 + ALLOWANCE_SHARE)`, while the neighbour's pass is on its COMPUTED
bound. Its first form decided the comparison on the run's band, so its
soundness rested on `Kε` covering the certificate's allowance plus the
distance's rounding, which the admission floor does not guarantee at the
smallest admitted ε; and it let ε choose between two meshes, which DESIGN
`[tolerance.chordal]` forbids. It now reads no ε: the side stays uncut only
past `δ·(1 + ALLOWANCE_SHARE)` plus two certificate allowances at the
magnitude of the points read, one for the neighbour's bound and one for
this distance's own rounding. What it does not cover: the neighbour's
verdict reads its triangle's third corner and its chart's anchor too, and
where those stand farther from the origin its allowance is larger than
the one read here (by at most `δ·ALLOWANCE_SHARE`). A side excluded on
that difference is the census's (`OverusedSide`: a wrong refusal, never a
wrong `Ok`). No row reaches it. Stated at the site.
