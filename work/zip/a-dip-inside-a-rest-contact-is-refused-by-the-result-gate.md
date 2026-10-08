---
id: a-dip-inside-a-rest-contact-is-refused-by-the-result-gate
kind: issue
title: An edge-in-face contact beside a declared Rest contact has no section segment, so the REST zip admits it (a dip inside the contact reaches the result gate; a line kiss beside it ships failing tier 3′)
status: parked
opened: 2026-10-06
priority: P3
cost: M
design: true
refs: [a-flush-declared-reflex-union-ships-the-wrong-volume]
blocked_on: [intent-stage4-is-built]
---


Found by ZIP's `zip-reflex` lane while it measured the REST zip's
admission (branch `zip/rest-admission`). Two shapes, one mechanism: an
edge of one solid lies in a face of the other beside the contact patch,
nothing crosses there, so the join's matching has no segment there and
the admission (`rest::patch_faces`: every segment bounds a patch on
both solids) cannot see it.

## Shape 1: a dip inside the contact

### Fixture

`crates/sweep/tests/rest_zip_admission.rs`
`a_box_dipping_into_a_plate_behind_a_tangent_lever_never_ships_the_overlap_twice`,
pose "inside the contact". The plate is the rounded 6 × 4 × 1 plate,
`r = 1`. A slab x∈(0,3), y∈(1 − 1/√2, 3), z∈(1,2) rests on it, its
west wall flush where the plate's south-west fillet starts. That is the
lever: the join refuses the tangent pair, so the union is the REST
zip's. The slab is united first with a box x∈(1.5,2.5), y∈(1.5,2.5),
z∈(0.8,1.5) that dips 0.2 into the plate, entirely inside the contact.
The interiors overlap by 0.2.

### Measured

- `upper ∪ plate`: the zip identifies the mate and glues it. The result
  gate refuses `ResultInvalid { errors: [RingOutsideOuter { face:
  FaceKey(2v1), ring: LoopKey(9v1), ring_vertex: VertexKey(14v1) }] }`.
- `plate ∪ upper`: the zip declines (no qualifying patch on B), and the
  join's `Join(Euler(NotSameFace { target: LoopKey(2v1), ring:
  LoopKey(6v1) }))` stands.

The dip's boundary on the plate's top is four edges of the slab's own
bottom face, lying in the plate's face. Nothing crosses there, so the
join's matching has no segment there. The admission check
(`rest::patch_faces`: every segment bounds a patch on both solids) sees
the same five segments as the box-free control and passes. What stops
the body is tier 1 at the gate. So this is a refusal in the wrong words,
not a wrong body: the body the zip built is invalid, where the honest
answer is that the contact is not a pure REST mate.

## Shape 2: a line kiss beside the contact

### Fixture

`crates/sweep/tests/rest_zip_admission.rs` `reflex_beside_a_post(profile,
sx, sy, -2.0)`: the reflex pose's `b` (a diamond profile with a vertex
at `a`'s reflex corner, its cap sheared `z' = z + sx·x + sy·y`) bridged
to the post whose west wall is flush with `a`'s at `x = −2`, so the
join refuses the post's tangent site and the union is the zip's. At
these poses one cap edge from the corner has `sx·x + sy·y = 0` along
it, so it lies in `a`'s top (`z = 1`) with the cap above it elsewhere:
`v∩ = 0`, and `b` kisses `a` along that edge and at its far vertex.
Ten poses, both orders:

- `dUp` (0.25, 0.25), (0.5, 0.5)
- `dLeft` (−0.5, −0.5), (−0.25, 0.25), (−0.1, −0.1), (−1, −1)
- `dDown` (−0.5, −0.5), (−0.1, −0.1), (−1, −1)
- `dRight` (0.25, 0.25)

Not pinned by a test (a scratch probe on `zip/rest-admission`, removed).

### Measured (2026-10-06, `zip/rest-admission` after the admission check)

All 20 runs: the join refuses, the zip builds, the volume is
`vol a + vol b′` to 5e-15 (correct, `v∩ = 0`), tier 2 passes, and tier
3′ (`validate_pseudomanifold` against the result's `contacts`) fails
with two findings. `dUp (0.25, 0.25)`, `a ∪ b′`:

```text
Err([UndeclaredContact { contact: VertexOnFace { vertex: VertexKey(22v3), face: FaceKey(9v1) }, witness: "(-0.5, 0.5, 1.0)" },
     UndeclaredContact { contact: EdgeFaceOverlap { edge: EdgeKey(31v1), face: FaceKey(9v1) }, witness: "(-0.25, 0.25, 1.0)" }])
```

The witnesses follow the kissing edge: `(−0.25, 0.25, 1)` and
`(−0.5, 0.5, 1)` for `dUp` and for `dLeft` at (−0.5, −0.5), (−0.1, −0.1),
(−1, −1); `(−0.25, −0.25, 1)` and `(−0.5, −0.5, 1)` for `dLeft`
(−0.25, 0.25); `(0.25, −0.25, 1)` and `(0.5, −0.5, 1)` for `dDown` and
`dRight`. The boolean's gate does not run tier 3′, so these bodies
ship: the right volume, with a contact the result does not declare.
Inferred, not measured on main, to predate this branch: the check it
added only declines more than main did.

## Directions

1. **The coincidence records.** The reduction records each edge of one
   solid that lies in the other's face (the contact records,
   `BooleanReduction::contacts`). On a pure REST contact, every such
   edge bounds the patch on its own solid with its other face outside
   the other solid. In shape 1 the box's walls run into the plate; in
   shape 2 the kissing edge bounds no patch at all. The zip would
   check, at each recorded edge-in-face contact, that the edge bounds a
   patch and that its non-patch face leaves the other solid. That is a
   sector read the classification has already made.
2. **The gate.** Leave the admission structural and let the result
   gate refuse, as it does today for shape 1 (shape 2 would need the
   gate to run tier 3′). Name the refusal instead: the zip
   would map the gate's `ResultInvalid` on a mate it admitted to a
   `RestZipUnsupported` frontier, or decline so the join's refusal
   stands. That answers in the right words but still builds the body
   first.

Which is right depends on whether the zip should read the
classification at all; its module doc says it consumes the reduction's
records and pairs no germs itself.

## Parked on the D10 hold (2026-10-06)

This row is on declared-contact ground, so it waits on `d10-one-way-to-say-intent-is-unbuilt` (`work/join/log.md`, the 2026-10-03 hold). D10 stage 4 retires the declared-REST zip: `work/intent/the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms.md`. When the hold lifts, close this row if its code is gone, or move it to the join if its scene still refuses there.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the fix site is the declared-REST zip's admission (rest::patch_faces), which stage 4 deletes. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)
