---
id: two-copies-filling-one-volume-certify-at-rest
kind: issue
title: Two copies whose material fills one volume certify at rest with no interference: the census has no word for a total coincidence of volumes, and since stage 4 I a wrong-sense seat reaches it
status: open
opened: 2026-10-24
priority: P1
cost: M
---


Found by review 1 of PR 5637 (stage 4 I, `mates-declare-no-contact`);
its probes are on `review/atrest-mates-declare-no-contact-R1`
(`crates/editor-core/tests/asm_r2b_assembly.rs`,
`r1_probe_a_wrong_sense_mate_quiets_nothing` and
`r1_probe_two_unmated_copies_filling_one_volume`).

## What

Two unit cubes, one pinned at the identity and one seated on it by a
plane mate written without its flip (`stacked_as` with the `Flip`
stripped from `to`, origin `(0, 0, 1)`). A plain plane mate equates
outward normals, so the copy turns over and lands exactly inside the
base: the two copies fill one volume.

- **Since stage 4 I, `assemble` answers Ok with zero interference.** The
  census records five rows: the caps and walls on one carrier, one
  opposed pair and four same-side pairs. The `unproven-coincidence`
  check reports the four the door leaves, and `Separation` reports the
  pair. But no census finding reads an overlap, so the gate has nothing
  to refuse and `Assembly::interference` is empty. Before stage 4 I,
  the mint attributed the wrong-sense seat's faces to the mate and the
  gate refused the contradicted declaration. That refusal was the only
  thing standing between this geometry and a clean certificate.
- **Two UNMATED copies filling one volume certify at rest on base too**
  (`r1_probe_two_unmated_copies_filling_one_volume`: the same pair
  pinned where it solved). That is the older gap behind the regression.

At `(0.5, 0.5, 1)` the wrong-sense copy only partly overlaps the base,
and the census reads the pierce: the gate refuses, as it should.

## Why

The census reads an overlap off the touches it finds: a vertex inside
the other's material, an edge piercing a face, a crossing whose side
test reads same-side. When the two copies' boundaries coincide
everywhere, every touch is a coincidence (vertex on vertex, edge on
edge, face on face). Nothing pierces and no vertex lies strictly
inside, so the census has no word for "these volumes are one".

## What closing it takes

A census (or `checks::at_rest`) reading for a coincident-boundary pair
whose materials agree on both sides of each shared face: a same-side
face pair whose two solids lie on the same side of it is an overlap of
volumes, not a touch. Both probes above become rows: the wrong-sense
seat at `(0, 0, 1)` and the unmated twin each report one interference
finding, loud.
