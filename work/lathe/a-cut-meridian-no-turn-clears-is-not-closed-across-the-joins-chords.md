---
id: a-cut-meridian-no-turn-clears-is-not-closed-across-the-joins-chords
kind: issue
title: A crossings-path cut whose meridian no turn clears of a first-pass section is not cut in: the closing cannot read the join's chords along a cut
status: open
opened: 2026-11-27
priority: P2
cost: H
refs: [close-cuts-is-not-total-on-crossings-path-cut-ins, a-section-circle-through-both-chart-poles-leaves-a-cut-face-with-no-pole-to-read]
---


## What

A crossings-path interior-loop cut-in on a sphere face runs its cut in a
meridian plane chosen clear of the face's first-pass sections
(`crates/topo/src/boolean/ops/interior_cuts.rs`, `closing_meridians`).
It tries a fixed sequence of turns of the meridian about the axis, and
takes the first plane decidedly off every partner plane (a plane, or a
sphere's radical plane) and every other cut-in circle's plane on the
face. That choice holds the closing's first premise: no join chord runs
along a cut (`crates/topo/src/boolean/ops.rs`, `cuts_in_result`).

**The ladder** (`meridian_turns`) is nine turns, `t ∈ {0, ±1/8, ±1/4,
±3/8, ±1/2}`, of the normal `k0 + t·(axis × k0)`, where `k0` is the
circle's own meridian. A section plane blocks a candidate only where it
holds the axis and the centre, so it is one meridian plane, and it blocks
one candidate at most. Nine distinct candidates so clear any eight
blocking planes on the face. The turns are literals, so every pose picks
the same plane at every scalar. The largest turn is `atan(1/2) ≈ 26.6°`
either side of the own meridian. The reach is kept small because the
own meridian is the plane that puts both of an interior circle's
crossings on the cut's half-plane, and a large turn can carry a crossing
of a circle near the pole across the axis to the other half.

A pose can so fail to cut in for three reasons:

- **every candidate within ±26.6° is blocked**, decidedly: by more than
  eight section planes, or where the circle's chord leaves the half or
  is not read meeting the circle twice at every turn. The op goes on
  uncut, to the guard. Where a turn is read in band and none clears, the
  op escalates as that reading instead.
- **a partner on the cut face that is no plane or sphere** (a cylinder,
  a cone, a torus). Its section is no plane, so `closing_meridians` has
  no plane to read a meridian against, and it returns `Ok(None)`.
- **a loop's cut**, which keeps its own meridian (`axis × toward`) and
  is not turned. Where a section crosses that meridian, the op does not
  cut in.

## What would close it

The closing reads the join's chords on the cut and closes across them
(ORBIT's `crates/topo/src/boolean/slit.rs`, `close_cuts`). The meridian
choice then stops being a premise.

## Witnesses

In `crates/sweep/tests/a_crossings_path_cut_in_is_made_only_where_its_closing_is_total.rs`:

- **Every turn blocked:** `a_box_in_every_turn_blocks_escalates_or_builds_as_its_offset_reads`, nine thin boxes in the ladder's nine planes beside the cap circle. On the planes the guard's refusal stands; a hair off them, in band, the op escalates.
- **A cylinder on the cut face:** `a_rod_crossing_the_cut_face_is_no_section_a_meridian_turns_off`, the guard's refusal in every op.
- **A loop's cut:** `a_loops_cut_beside_a_first_pass_section_builds` reaches the loop arm, whose own meridian clears, and builds sound. No row has a section across a loop's own meridian.
- **A loop's cut-in and a circle's on one face:** `a_loop_and_a_circle_cut_in_on_one_face_refuse_at_the_guard`. The replaced gate refused any face holding a loop's cut-in and another; this one does not, and the pose refuses at the guard, typed.

**Another cut-in's circle on the face** is read too, and is expected never to block. At exactness no candidate lies in another cut-in circle's plane: such a circle would be a great circle through both poles, met by the cut's own circle wherever a candidate crosses it twice. The check is kept because blocking is decided against the band, where a near-great circle through near-poles within the band of a candidate is the case that argument does not close. No row is built at that band edge: it needs two disjoint cut-in circles, one within the band of a great circle through both poles and the other's candidate, which no cheap fixture gives. A mutation of the check is equivalent at exactness (R2's M6, PR 7831).

A sphere partner blocks a meridian only through a trimmed sphere whose section arc lies in a meridian plane, so runs toward a pole without reaching the cut circle. No row builds that fixture; `a_trimmed_ball_crossing_the_cut_face_is_read_by_its_radical_plane` reaches the radical-plane arm off the meridian.

This item is what remains of `close-cuts-is-not-total-on-crossings-path-cut-ins` (1b's generality) after `lathe/close-cuts-total`.
