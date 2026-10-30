---
id: main-red-trace-rs-reads-as-a-planar-sense-flip-re-fork
kind: issue
title: main red: topo boolean/trace.rs folds a chart gradient through from_chart beside a Surface::Plane arm, which the planar-flip row reads as a re-fork (topo face_normal, every ε)
status: closed
opened: 2026-10-30
priority: P1
cost: E
closed: 2026-10-30
---


**What fails.** `topo` `face_normal::tests::the_planar_sense_flip_lives_in_one_place` (`crates/topo/src/face_normal.rs:793`, slow set) is red at every ε in nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467) (main `39bbdfc777`), and was green on 2026-10-29 (run 40823227622, `5648e577eb`):

> `crates/topo/src/boolean/trace.rs` mints an OutwardNormal from a plane's chart normal — the planar sense flip has been re-forked out of face_normal.rs, which must hold the only one. Call `face_outward_normal` instead.

The row is file-level. It fires on any `topo/src` file other than `face_normal.rs` whose code holds both `Surface::Plane {` and `from_chart`. `trace.rs` holds both:

- `crossing` (`trace.rs:366`) folds a chart gradient through the sanctioned door, as `OutwardNormal::from_chart(implicit_gradient(surface, point), sense)`, for every surface kind. That includes a plane, whose implicit gradient is its normal.
- The root-finding arms match `Surface::Plane { .. }` (`:182`, `:244`, `:432`, `:479`).

**Attribution (read, not bisected).** PR 6043 (`intent/s6-c-trace-continuation`, merge `9a5cefe9`) added every one of these lines. `trace.rs` holds no `from_chart` at `9a5cefe9^1` and holds one at `9a5cefe9`, and the row is unchanged across the merge. The row is in the slow set, so the PR merged green.

**Judgement owed.** The fold goes through `from_chart`, so it is not a hand-written flip. It is a second reading of a planar face's outward normal, though, beside `face_outward_normal_at`, and that is the class the row exists for (P1: two implementations of one logic). The two remedies are:

- read the outward normal through `face_normal`'s door. `crossing` holds a surface and a sense rather than a face, so the door's shape is the question.
- or show that the general gradient fold is the right single home for a trace, and narrow the row so it tells a plane-specific flip from a gradient fold.

**Reproduce.** `cargo nextest run -p topo --lib --run-ignored all -E 'test(=face_normal::tests::the_planar_sense_flip_lives_in_one_place)'`

**What closing it takes.** The row is green on main, with whichever remedy is chosen stated in the PR.

## Closed (2026-10-30): trace reads the outward normal through face_normal

The first remedy. `face_normal.rs` gains `surface_outward_normal`, the
by-value door for a reader holding a carrier, a point and the bit
rather than a face: it folds the implicit gradient through
`OutwardNormal::from_chart`. `boolean/trace.rs` `crossing` calls it and
no longer names `from_chart`. `face_outward_normal_at`'s cone arm calls
it too. The trace keeps the RAW gradient (its `Crossing::outward`
contract), so `geom_brep::implicit_outward_normal`, which normalizes,
was not the door.

