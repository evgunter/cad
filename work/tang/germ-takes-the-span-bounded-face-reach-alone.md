---
id: germ-takes-the-span-bounded-face-reach-alone
kind: issue
title: the germ frame and chord_join read a cylinder wall face's reach both whole-turn and span-bounded and escalate where they disagree
status: parked
opened: 2026-10-08
priority: P3
cost: E
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---

## What

A plane×cylinder section is read on a wall face's reach across, measured two ways:
- `splitting::rules::face_reach_from`, each edge read over the span it holds;
- `face_reach_round_from`, each edge levered round its whole carrier, main's measure before the span-bounded conic reach.

`chord_join::agreed_section` serves only where both serve the same class. It escalates on a split (`pc_axis_plane_parallel_disagreement`, `pc_parallel_gap_disagreement`). Two readers take it:
- the germ frame (`boolean::join::frame_extent`, `pair_section_frame_at`);
- chord_join's cylinder lane (`chord_join::section_reach`, `section_case`) for every caller of `wall_section`. That includes the Boolean's germ join, which mints its wall-side chord through the split lane (`boolean::join` `GermLane::PlaneWall`/`WallPlane` → `split_curve` → `JoinLane::Split`).

Both sit on the declared-tangency path, which D10 (ratified on #3990) does not let a shorter lever widen. Until a declaration channel lands, a pose read there cannot be told value-inferred from declared, so the section keeps main's served set: escalating where main escalated, and on main's wrong conics. `wall_section` stays lane-neutral, so the split's own chords take the agreed reading too.

`face_reach_round_from` and `agreed_section` duplicate the whole-turn measure on purpose; they go together when this item lands.

**Evidence.** `crates/geom-brep/tests/span_reach_differential.rs`:
- germ column: 88 poses where main serves a conic against a Zero truth now escalate on the split, and 155 stay escalated against a Zero truth;
- chord_join column: 71 and 116 the same way.

Before chord_join took the agreed reading, the span alone served the rulings in 140 chord_join poses where main escalated, every one against a Zero truth (offsets 0.06–0.96 zero).

## The shape of a fix

Once the declared path is built, a section read where no declaration is carried takes `face_reach_from` alone, and `face_reach_round_from` and `agreed_section` go.
