---
id: a-dip-inside-a-rest-contact-is-refused-by-the-result-gate
kind: issue
title: A dip of one solid into the other inside a declared Rest contact has no section segment, so the REST zip admits it and only the result gate refuses it
status: open
opened: 2026-10-06
priority: P3
cost: M
design: true
refs: [a-flush-declared-reflex-union-ships-the-wrong-volume]
---


Found by ZIP's `zip-reflex` lane while it measured the REST zip's
admission (branch `zip/rest-admission`).

## Fixture

`crates/sweep/tests/rest_zip_admission.rs`
`a_box_dipping_into_a_plate_behind_a_tangent_lever_never_ships_the_overlap_twice`,
pose "inside the contact". The plate is the rounded 6 × 4 × 1 plate,
`r = 1`. A slab x∈(0,3), y∈(1 − 1/√2, 3), z∈(1,2) rests on it, its
west wall flush where the plate's south-west fillet starts. That is the
lever: the join refuses the tangent pair, so the union is the REST
zip's. The slab is united first with a box x∈(1.5,2.5), y∈(1.5,2.5),
z∈(0.8,1.5) that dips 0.2 into the plate, entirely inside the contact.
The interiors overlap by 0.2.

## Measured

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

## Directions

1. **The coincidence records.** The reduction records each edge of one
   solid that lies in the other's face (the contact records,
   `BooleanReduction::contacts`). On a pure REST contact, every such
   edge inside the contact bounds the patch on its own solid with its
   other face outside the other solid. Here the box's walls run into the
   plate. The zip would check, at each recorded edge-in-face contact
   inside a patch, that the edge's non-patch face leaves the other
   solid. That is a sector read the classification has already made.
2. **The gate.** Leave the admission structural and let the result
   gate refuse, as it does today. Name the refusal instead: the zip
   would map the gate's `ResultInvalid` on a mate it admitted to a
   `RestZipUnsupported` frontier, or decline so the join's refusal
   stands. That answers in the right words but still builds the body
   first.

Which is right depends on whether the zip should read the
classification at all; its module doc says it consumes the reduction's
records and pairs no germs itself.
