---
id: a-moved-copys-pick-span-crosses-its-frame-unconverted
kind: issue
title: Two copies' crossings are ordered by spans each certified on the ray carried into its own frame, which is the caller's only to within that carry's rounding
status: open
opened: 2026-10-22
priority: P1
cost: M
design: true
---


First written (2026-10-22, split out of
`pickindex-merges-parts-on-a-rounded-t-it-never-converts`) as "the
span crosses the frame unconverted"; `vgeom/project-refuses` found
that premise false and narrowed the row (2026-11-25);
`vgeom/moved-copy-witness` (PR 7792) built the witness, which
flipped, fixed the self-occlusion half and narrowed the row to the
cross-copy ordering left.

## Finding

A part drawn under a display map (`DisplayView::display_map`: a
space's free-move frame composed with the placement editor's carry)
or a body frame (`[placement.frame]`) is picked on the caller's ray
carried back through those maps: `PickIndex::pick_for` carries it
through the display map's inverse, then `pick_face` reads it into each
target's frame (`PickTarget::ray_in_frame`). The parameter carries
exactly (an affine map keeps it); the CERTIFICATE is the carried
ray's as rounded, which is the caller's only to the carry's rounding.
Spans of different copies are then ordered against each other as if
certified on one ray:

- `crates/viewer/src/pickindex.rs`, `PickIndex::pick_for`: the
  cross-group merge (`TSpan::survivors`, `answer_of`) over the unmoved
  batches (one per space) and each moved body's group.
- `crates/editor-core/src/resolve/pick.rs`, `pick_face`: the
  survivors taken over every target of one call, each target's spans
  certified on the ray read into its own frame.
- The viewer's merge of unmoved batches of different spaces, which
  carry body frames only;
  `pick3_acceptance::the_door_answers_the_exhaustive_walk_and_keeps_every_aimed_vertex`
  pins those spans bit for bit against `FlatReference`, which carries
  the same frames.
- `PickIndex::near`'s outline arm: a profile outline's span is read on
  the caller's ray against its world-drawn points
  (`ray_segment_point(&ray, candidate.ends, s)`), while the face it
  lies on is certified on the ray carried through that face's display
  map and body frame, so under a large free move an outline can read
  as behind its own face. It is not repaired alongside the edge's
  reading because an outline holds no part frame: it knows its space
  and the copy that carries it (`DisplayView::outline`), not the body
  whose face it lies on, and that body's frame is a second carry the
  outline's space coordinates do not share — so "the face it lies on"
  is a second drawing, ordered across frames like any two copies.

Two P3 rows filed on PR 7792's branch covered the kernel's merge and
the cross-space merge above; they never reached main, and are folded
in here as one class.

**The witness.** A copy posed by an exact frame (a dyadic translation
up to the session box, a half turn or not) and free-moved back onto
its twin by the exact inverse is drawn on the twin's own points; a
ray onto that face should refuse with both. On main 11–39 of 180 such
rays per seed answered one copy alone: the move's translation cancels
the pose's, and the carried ray keeps none of its origin's bits below
that coordinate's rounding, some two hundred times the width a
crossing near the origin certifies. The fuzzer and a pinned ray
(pose half turn + `[-423.0, 395.375, 177.0]`) are on branch
`vgeom/moved-copy-twin-witness` (`crates/viewer/tests/moved_copy_pick.rs`:
`a_moved_copy_drawn_on_its_twin_ties_with_it`,
`a_copy_moved_back_far_onto_its_twin_ties_with_it`), to land with the
fix. **Take only those two rows** (and the helpers they call) from
that branch: at b94591be3b it also carries the margined repair PR 7792
took out — in `pickindex.rs`, editor-core's `pick.rs`, `gpu.rs` and
pncad-py's `pick.rs` — and the two issue files folded in here.

**Not here any more.** The edge lane's own-face reading: an edge of a
moved or posed copy read as hidden by its own face (130 of 554 hovers
on the pinned row before the fix, and one silhouette edge of a copy
merely posed). `own_crossing` now reads the chord on the ray the
part's faces were certified on, against the mesh's own corners, so a
face that is one of the chord's triangles is the same crossing bit for
bit (PR 7792; guarded by `moved_copy_pick.rs`).

**What a margined repair cannot be.** Growing a carried span by a
bound on the carry's rounding (the row's first repair shape) is a
rounding charge, which `[simple.conditioned]` forbids
(`scripts/gates/bare-rounding-allowlist.sh` reds it); PR 7792 tried it
and took it out.

## Next step

The band verdict, `[ev]` pending on PR 7793: two designers agree the pick should
order crossings by a margined band verdict at ε (no certificate
widths), with editor-core owning the one merge and each target handed
in under its drawn map. Land the twin witness with that fix.

## History

Parked 2026-10-22 on the stage-3 placement pivot (D′,
`work/intent/a-placement-pins-by-written-values-without-a-mate.md`);
moved from FIT into VGEOM 2026-10-26; released by INTENT 2026-10-26
when D′1–D′3 (PRs 5592, 5613, 5636) merged. A re-read after D′ said
"nothing picks through world coordinates, so D′ did not overtake it",
which was false: the edge lane's `own_crossing` and `Candidate`'s
chord ends were placed in the world, which is what the self-occlusion
half was. PR 7792 moved the chord's reading into the part's frame.
