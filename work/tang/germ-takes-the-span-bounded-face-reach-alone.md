---
id: germ-takes-the-span-bounded-face-reach-alone
kind: issue
title: the germ frame reads a wall face's reach both whole-turn and span-bounded and escalates where they disagree
status: parked
opened: 2026-10-08
priority: P3
cost: E
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---

## What

`boolean::join::frame_extent` hands the germ frame two measures of a
plane×cylinder wall face's reach across:
- `splitting::rules::face_reach_from`, each edge read over the span it holds;
- `face_reach_round_from`, each edge levered round its whole carrier, main's measure before the span-bounded conic reach.

`agreed_section` serves only where both serve the same class. It escalates on a split (`pc_axis_plane_parallel_disagreement`, `pc_parallel_gap_disagreement`).

The germ is a declared-tangency path, which D10 (ratified on #3990) does not let a shorter lever widen. Until a declaration channel lands, the germ cannot tell a value-inferred pose from a declared one, so it keeps main's served set: escalating where main escalated, and on main's wrong conics.

**Evidence.** `crates/geom-brep/tests/span_reach_differential.rs`, germ column:
- 74 poses where main serves a conic against a Zero truth now escalate on the split;
- 151 escalate where the span-bounded reach alone would serve the rulings, every one against a Zero truth (offsets 0.06–0.96 zero).

## The shape of a fix

Once the declared path is built, the germ reads `face_reach_from` alone where no declaration is carried, and `face_reach_round_from` and `agreed_section` go.
