---
id: sense-sign-multiplies-fold-onto-outward-normal
kind: unit
title: The ten hand multiplies of normal by sense_sign fold onto OutwardNormal, and Face::sense_sign retires
status: open
opened: 2026-09-15
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
