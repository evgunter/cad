---
id: sketch-plane-lift-choice-has-no-home
kind: issue
title: SketchPlane::map's doc states a lift rule with no home: each caller decides in prose and none says which it chose
status: open
opened: 2026-09-15
priority: P1
cost: D
---


Carried out of `affine3-try-map-the-fallible-walk-has-no-kernel-door`
by the affine-try-map lane, which implemented that row's first folded
finding (`SketchPlane::try_map`) and leaves this one unaddressed. The
carrying row closes when `Affine3::try_map` lands, and a finding left
inside a closing row is gone (`work/README.md`: disclosing a residue is
not scheduling it), so it gets its own file here.

## The finding

`SketchPlane::map`'s doc (`crates/profile/src/lib.rs`, the paragraph
headed **What the lift means**) sets out two spellings and ends by
saying **a caller chooses**:

- `plane.map(S::from_f64)` — the `f64` frame lifted whole, so at
  `Interval` the `f64` rounding of the stored normal's cross product is
  carried as if exact;
- `SketchPlane::from_frame(frame)` for a frame minted at `S`, where at
  `Interval` the cross product of point intervals rounds outward and
  the stored normal carries that width.

Those are different planes wherever the cross product rounds, and the
choice has no home. Each caller decides it in prose, and none of them
says which it chose or why: `eval/wire.rs`'s `Pinned` lift,
`eval/anchor.rs`'s `embed_profile`, and two sites in
`crates/sweep/src/loft.rs`. `SketchPlane::try_map`, added by the
affine-try-map lane, inherits the same doc paragraph and the same
silence — it is the fallible direction of exactly this lift.

Either the door decides (one spelling, and the other stops being
reachable by accident), or the choice becomes a typed argument the
caller must name. What it must not stay is a sentence in a doc comment
with four callers who never answer it.

## Why this slate

The door is `crates/profile/src/lib.rs`, which
`scripts/work.py territory` reads as **bool**'s ground, so the row is
filed here rather than on the slate of the program that found it
(`work/README.md`: a finding goes straight onto the slate of the
program whose ground it lands on, and a lane needs no permission to put
it there). The callers are wire's and sweep's, but what the finding
asks for is a decision about the DOOR, and the door is bool's.

Filed from outside the fence by PROPS' affine-try-map lane, which
minted `SketchPlane::try_map` beside `SketchPlane::map` and so added a
second inheritor of the unanswered sentence without answering it.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to PATHS (opened at this exit as S-BOOL's successor for the profile lattice) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.
