---
id: sense-sign-multiplies-fold-onto-outward-normal
kind: unit
title: The ten hand multiplies of normal by sense_sign fold onto OutwardNormal, and Face::sense_sign retires
status: open
opened: 2026-09-15
branch: scalar/sense-fold
pr: 2668
---


## What

`D6`'s ruling (PR 2457), second unit, after
`sense-sign-doors-take-the-bit`. The ten `normal * face.sense_sign()`
sites (`emit_topo.rs` `face_plane`, `blend/build.rs` `outward_of`,
`blend/battery.rs` `outward`, `boolean/join.rs` `ring_run_ccw`,
`boolean/rest.rs` `face_carrier`, `boolean/solid_contain.rs`
`face_plane` and `face_geo`, `validate.rs` check 6, `merge_faces.rs`
`merged_outline_ring` and `planes_declared_equal`, `mesh/walk.rs`
`loop_polygon`) fold onto `OutwardNormal::from_chart` /
`face_outward_normal`; where a scalar (an area, a winding) is genuinely
signed, the bit is spelled as a conditional negation. Then
`Face::sense_sign<T>()` retires with `face_normal.rs`'s hand-kept
census. Pin: D9 bit identity over the touched crates' suites. Many
programs' ground (TOPO, BLEND, WIRE, S-MESH, BOOL); announce each.

## An eleventh site, found by unit 1's reviewers (2026-09-15)

`crates/sweep/src/blend/arms.rs` — `Sheet::trace` and `Ruling::trace`
both open `let side = if sense { T::one() } else { -T::one() }` from a
face bit (the `sense` parameter, fed from `battery.rs`'s
`convexity.ball_side(senses.0)`) and store it as
`SupportTrace::{Straight, Round}.side: T`, documented "The material
side, `±1`", consumed a function boundary away in `SupportTrace::contact`
and in the centre closed forms. It is D6 §4's class — a sense CROSSING a
boundary as a scalar `T` — and unit 1's three sweep patterns all miss
it: the bit arrives as a parameter named `sense` (not a `sense_sign`
read), and it leaves as a struct FIELD rather than a parameter.

**Two readings, and this unit decides between them.** Either the field
is a sense wearing a `T` and folds like the other ten, or the closed
forms consume a genuinely signed σ (a curvature/offset direction) that
happens to be seeded from the bit, in which case what is owed is the
bit selecting σ once at the mint and the field keeping its own name.
Read `contact` and the centre forms before choosing. BLEND's ground.

Test-side residue in the same class, to follow its door:
`crates/topo/tests/readback_sense_kind.rs` mints a `±1` from
`pose.sense`.

## Digest receipt: the D9 pin for "nothing's bits move"

The zero-parameter recipe of `work/scalar/rate-pair-in-geom-core.md`
§Digest receipt (build `demos/tour` release, run the binary directly
into the literal relative outdir `tour-out`, digest the sorted per-file
listing and the whole narration), taken at this branch's merge base
`origin/main` `d71bb6a78` before the first code change:

- **1766 emitted files**; digest of the sorted per-file digest
  listing:
  `87be4dd9df4cc3af9bd44593a6b981608c8e73721c746413a00322ff61e4a892`
- the tour's **narration**, 729 lines:
  `e930abf542c371677b2c0d87b02c2bf84eb15fbaf62ded6389f48c899ef14d49`

Both are the values RATE-PAIR recorded at `4f71edaea` and at its head.
The head-of-branch pair is recorded in the PR body beside these.
