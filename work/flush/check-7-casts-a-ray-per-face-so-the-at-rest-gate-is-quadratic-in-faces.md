---
id: check-7-casts-a-ray-per-face-so-the-at-rest-gate-is-quadratic-in-faces
kind: issue
title: Check 7 reads every aim, one ray per face across every face, so the at-rest gate is quadratic in a body's faces
status: open
opened: 2026-10-20
refs: [a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass]
priority: P1
cost: M
design: true
---


Found by FLUSH's fin-crown unit (`flush/fin-crown-winding`), which made
check 7 read every aim that decides (D1 tier 3: definitely inside out
by any reading is invalid).

## Measured

`check_7_reads_every_aim_at_a_cost`
(`crates/topo/tests/the_fin_crown_winds_as_a_solid.rs`, ignored, release,
ε 1e-9) times `validate_geometric` on a prism over a regular `n`-gon:

| faces | first ray that decides | every aim |
|---|---|---|
| 18 | 1.6 ms | 2.5 ms |
| 66 | 1.7 ms | 12 ms |
| 258 | 6.6 ms | 231 ms |
| 1026 | 47 ms | 5.7 s |

A valid body's rays all read one side, so every face that offers an aim
casts one, and each ray asks every face of the selection
(`boolean::solid_contain::ray_crossings`: the plane hit, then
`point_in_face`) and measures the selection's reach from its aim twice
(`side_reader::beyond`, `selection_reach`). Both are linear in the
faces, so the gate is quadratic in them: about 5.5 ms a ray at 1026
faces. Extrapolated, a 10 000-face import would spend minutes in
check 7.

**It blocks the symbolic tier.** `editor_core::coincide::prove`
replays each operand's at-rest validation at `Sym<f64>`, where every
arithmetic step is interned in the session. Reading every aim makes that
replay quadratic too: `docm5_subject::the_registry_wrapper_and_its_door_agree_on_every_corpus_document`
(slow set, release, ε 1e-9) holds 2.9 GB and takes 79 s on main, and on
the branch passes 13 GB on the corpus's first document (`die`) and is
killed. A stack sample puts it in `cast_aims` → `ray_crossings` →
`census::ee_lines_closest` → `sym::intern`, under
`wire::finished_operand` at `Sym<f64>`. Running the extra rays detached,
with nothing recorded, does not help (12.9 GB): it is the arithmetic,
not the decision log.

## What to find

A cheaper ray for the same reading. Two candidates, both on shared
ground (`solid_contain.rs` is INSIDE's and CLEAVE's):
- a per-selection box or ball per face, so that a ray skips a face it
  definitely misses by more than the band;
- the selection's reach taken once about a centre, rather than from
  each aim.

The box cull is the one that removes the quadratic arithmetic: each
face's certified, padded box (`boolean::boxes::face_box`, padded by
`sweep_pad` as the census's prune is) taken once per selection, and each
ray asked against them with `bvh::Ray::slab_enter`, a conservative test
that never calls a box the ray meets disjoint. A face whose padded box
the ray misses is more than the band from it, so no crossing and no
in-band reading of it is lost. It needs the reader's scalar to carry
`Bounds`, and `side_reader.rs`, `validate::wound_negative` and
`AtRestBody::gate_unverdicted` write no `Decide + Bounds` today, which
`scripts/gates/bounds-allowlist.sh` holds for ratification. That is the
design question on this row.

Either candidate moves the aimed rays' recorded verdicts
(`side_reader::FAMILY`), and the ball also moves where each ray starts,
so a fix re-reads the batteries. The reading itself must not change:
every aim that decides is still read.
