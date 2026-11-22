---
id: the-plane-and-sphere-conic-pairs-on-the-section-value
kind: unit
title: Plane × sphere, sphere × sphere and plane × cylinder are built on the section value and ride the carried lane; their frame arms retire
status: closed
opened: 2026-11-21
priority: P1
cost: H
parent: conic-germ-pairs-rank-by-the-frames-turn-not-the-foot-parameter
branch: sectcone/conic-pairs-on-the-value
closed: 2026-11-22
pr: 7354
---


The first of three units on `conic-germ-pairs-rank-by-the-frames-turn-not-the-foot-parameter`, surveyed 2026-11-21:

- **U1, this unit:** plane × sphere, sphere × sphere and plane × cylinder.
- **U2:** the plane × torus poses the value serves.
- **U3:** plane × cone, then deleting the turn lane. U3 waits on the cone's region and its tangency lever, from `the-cone-has-no-implicit-enclosure-or-metres-conversion`, and it owes the germs along a conic edge (below).

**Review tier: single, FULL** (Opus; Ev 2026-11-21). This follows the cylinder pairs' established pattern, but it rewires the join's partner ordering for everyday pairs, so believing it takes more than reading it. Difficulty letter: M.

## Spec

1. **Build the three pairs on `geom_brep::pair_section`.** Add `PairPose` arms, with components from the existing closed-form doors in `geom_brep::intersect` (`plane_sphere_section`, `sphere_sphere_section`, `plane_cylinder_section`):
   - **Circles and the tilted ellipse:** `ClassCarrier::Closed`.
   - **Plane × cylinder's parallel lines:** `ClassCarrier::Span` lines, cut by the operation's region exactly as `cylinders.rs` cuts rulings. A plane has no bounded side, so C3 (`crates/geom-brep/README.md`, the domain paragraph) reads the pair over the region alone. No face's trim or reach enters.
   - **Tangent poses:** as the value's tangent reading.
   - **Empty:** an empty `Counted`.
   
   Widen `serves` (it is a second spelling of the `pair_section` dispatch, so derive one from the other if you can) and the region predicate. A plane's residual is already in metres.
2. **Topo:** `rides_carried` and the germ lanes for these pairs move to `GermLane::Carried`. The partner ordering then goes through the foot-parameter branch of `along_sign`, not the turn branch. Delete what these pairs alone kept alive:
   - in `join.rs`: their arms of `conic_section_frame_at`, `GermLane::Radical`, and the plane-wall lanes where they serve only the cylinder;
   - `chord_join`'s `wall_face_section` and the matching `section_case` arms;
   - in `glue`: any reading made redundant.
   
   Keep the turn lane itself: plane × torus, plane × cone and the germs along a conic edge still use it.
3. **Tests:** retarget `crates/topo/tests/join_walk_order_rows.rs`, or wherever it lives, from turn order to foot order for the circle and the tilted ellipse. Retarget or retire the frame rows that pinned the retired arms. Run the sweep tests `band_apart_partners_on_a_steep_ellipse` and `four_crossings_on_one_section_circle`. A frame or golden that moves is re-baselined with the reason (implementer discipline §3). If the change moves demo frames, push a `[render]` commit.

**Coupling:** ONEROUND's open PRs 7228 (`oneround/lever-sites-topo`) and 7262 (`oneround/turn-lever`) edit `join.rs`'s `frame_lever` and `frame_reading`, and 7203 edits `pair_section.rs`. Run `python3 scripts/work.py territory --base origin/main`, read those PRs' diffs before you edit the same functions, and say in the PR how the two interact.

**Owed by U3, not this unit:** a germ along a conic edge has no face-pair section (`along_edge_conic` → `germ_section_loops`, and `germ_branch`'s `Only` lane). So "the turn lane is gone" needs a foot ordering on the edge's carrier for it.

## Closed (2026-11-22, PR 7354)

Plane × sphere, sphere × sphere and plane × cylinder are built on the section value (`pair_section/conic_pairs.rs`) and ride the carried lane. Their frame arms, `GermLane::Radical`, `FrameExtent::Wall`, `chord_join::wall_face_section` and the section certificate's three hand classifiers retired.

- **Foot-slack fork:** weighed by two designers (log, 2026-11-22). No along-section slack is owed. The join's foot travel meters at a certified-positive speed floor between the two feet (`FootSpeed`, `speed_between`), and a foot tie is a decided tie.
- **Review:** single FULL Opus review, APPROVE-WITH-FIXES with two MAJORs, both fixed with rows that were red before the fix:
  - the glue face filter dropped implicit faces against spline partners;
  - an unguarded speed floor.
- **Still open:** the germs along a conic edge keep the turn lane, which is U3's on the parent row.
