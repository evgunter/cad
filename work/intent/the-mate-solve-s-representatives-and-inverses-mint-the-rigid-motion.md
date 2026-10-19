---
id: the-mate-solve-s-representatives-and-inverses-mint-the-rigid-motion
kind: issue
title: the mate solve's representatives, half-turns and inverses are Affine3 maps: type them as RigidMotion (mints by construction, transpose inverses), so a mated pose moves a body without the decided door
status: open
opened: 2026-10-12
priority: P2
cost: M
refs: [SYM-17]
---

## What

Ev's answer on SYM-17 (`work/sym/a-widened-rotation-angle-refuses-on-the-plain-interval-lane.md`,
"Question, and Ev's answer") types the mate solve's maps as the
rigid-motion witness `geom_core::RigidMotion`, and SYM-17's spec
(`docs/SYM-17-SPEC.md`, Phase 2 item 5) names the sites:
- the representatives, `mate/coset.rs`'s `candidate_rotation` and
  `candidate_translation`, and a face base;
- `mate/solve.rs`'s `opposed` half-turn (`RigidMotion::half_turn_x` is
  that matrix, exact);
- every solve-side inverse as the transpose
  (`RigidMotion::inverse`): `mate_coset`'s `fb.inverse()`, `invert`,
  `coset.rs`'s `clocking_about`, and the relative pose's
  `oc.inverse()` in `solve.rs`.

SYM-17's Phase 2 (PR #4197) built the type and typed every other
placement, but did not touch `mate/coset.rs` or `mate/solve.rs` beyond
the rename `OrthoFrame` → `RigidMotion`: its stop rule applies, because
INTENT's stage 3 B branch (`intent/s3-b-placement-is-the-bundle-of-mates`)
is rewriting both files.

So today a mated pose is an `Affine3` that `placement::Motion::Map`
carries, and it is decided at the door that moves a body
(`Motion::rigid`, through `RigidMotion::check_rigid`). On a box run a
tilted mate frame therefore still refuses there as `Undecided`, as
TOPOTAIL's `a-boxed-rotation-refuses-not-rigid-at-every-placer` says.

## What answers it

When stage 3 has rewritten the two files, type the representatives and
inverses as `RigidMotion` (products of witnesses are witnesses, the
inverse is the transpose), so `Motion` carries the solve's poses as
`Motion::Rigid` and the decided door is asked only of literal frames.
Stage 3 F retired `candidate_rotation`, `candidate_translation`,
`clocking_about` and `member_of` (the fold is now the structural meet,
`mate/coset.rs`'s `intersect`), so the sites today are `mate_coset`'s
representative (`frame_of(to) * frame_of(on).inverse()`,
`mate/solve.rs`), the meeting `ball_meets` builds (`mate/coset.rs`), and
`value.rs`'s settle.

MSOLVE's `a-box-over-a-solved-clocking-widens-thirty-thousandfold` is
the measurement to re-take then: the transpose inverse does not reach
it until this lands.

