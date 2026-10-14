---
id: minting-doors-take-a-callers-description-unasked-where-the-new-face-keeps-the-key
kind: issue
title: mef under its parent's key, mekr and mev mint an edge with a caller's description and never ask whether it names the faces it bounds
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [mef-and-mfkrh-onto-a-new-chart-strand-the-edges-they-move, kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers]
---

## What

`Body::set_edge_curve` refuses a description that does not name its
edge's two faces' surfaces (`EulerOpError::DescriptionNotAdjacent`, from
`require_description_adjacent` in `crates/topo/src/attach.rs`). PR 3673's
fix pass gave `Body::mef` the same question for its chord, through the
same function, but only where the new face moves off the parent's key
(`require_chord_adjacent` in `crates/topo/src/euler.rs`). Three minting
doors still take a caller's description and ask nothing:

- `mef` / `mef_chords` / `mef_lone` where the new face keeps the
  parent's key (`Inherit`, or `Shared` on that key). The chord's two
  faces then both wear that key.
- `mekr` (`mekr_cycles`, `mekr_empty_ring`, `mekr_empty_target`,
  `mekr_both_empty` in `crates/topo/src/euler_ring.rs`). Both halves lie
  on one face.
- `mev` (`mev_fan` / `mev_lone` in `crates/topo/src/euler.rs`). The
  halves lie on the faces of `he1`'s and `he2`'s loops.

A description these doors take can name a key neither face wears, and
tier 3 reports it at rest. Pinned for `mef`:
`attach::tests::under_inherit_mef_takes_a_chord_naming_a_key_neither_face_wears_unasked`.

## Measurement

Instrumented at the head of PR 3673's fix pass: each door logged, without
refusing, every call whose description would fail the adjacency
question, with its caller. Topo's whole suite (1906 tests) and sweep's
`ci` profile (1817 tests).

| Door | Would refuse | Production sites |
|---|---|---|
| `mef`, new face on the parent's key | 5,792 | `chord_join.rs`: the chord-join mints at `:2270` (2,501) and `:2360` (3,288); the other 3 are rows |
| `mekr` | 798 | `chord_join.rs`: the cross-loop join at `:2305` (798) |
| `mev` | 1,283 | none |

Every production call comes through sweep: the boolean and split
pipelines' chord joins. The `mef` arm hands a curved face's section chord
its final intrinsic description (`chord_spec`), `Intersection(face
surface, section plane)`, under `Inherit`. The section face is glued
along it afterwards. Measured with the check in place for `Inherit`
too, sweep's `ci` profile had 209 failures. Their panic messages name
that refusal raised through the join (`DescriptionNotAdjacent { edge:
None }`, or its text), except a few that print no error ("no body").

`mev`'s 1,283 are all fixtures. They build sheet bodies whose edges are
described against planes no face wears: `test_support_fixtures.rs`
(`:1259`, `:1280`), `boolean/boxes.rs` (`:2155`, `:2331`, `:2352`,
`:3169`, `:3180`, `:3192`, `:3522`), `boolean/section_cert_rows.rs`
(`:1191`, `:1213`, `:1379`), `census.rs` (`:7042`, `:7114`),
`chord_join.rs`'s tests (`:2660`), and `tests/` (`fixture/mod.rs:323`,
`m5_pr7_split_meter.rs`, `m6_3_chart_completion.rs:179`,
`review_ssiflat_r2_probes.rs:352`, `trim_3_chart_bound.rs:478`).

## Why it is not built

- `mef` under the parent's key and `mekr`: production relies on them.
  The answer is to reorder the chord join (mint with a scaffold or chart
  description, then describe once the section face is glued, through
  `set_edge_curve`), or to give these doors a describing twin. Either
  reworks the boolean pipeline's core, so it is a design question. It
  is the same question `kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`
  asks of the kill side.
- `mev`: the door's fix is one call, but about twenty fixtures would
  have to be rebuilt with every face on the surface its edges name.
  That belongs with the `mef` arm, so the three doors share one answer.
