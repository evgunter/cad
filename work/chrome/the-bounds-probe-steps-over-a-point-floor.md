---
id: the-bounds-probe-steps-over-a-point-floor
kind: issue
title: The bounds probe's doubling ladder steps over a failing region that is a point, so an extrude's depth reads open below zero for most seeds
status: open
opened: 2026-11-18
priority: P2
cost: E
---

Found by DOCTAIL's review of its signed depth (2026-11-18, `work/doctail/a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face.md`).

## What

An extrude's depth is signed now (DESIGN.md `[placement]`, Ev's ruling on `[ev]` #7073): a `−` depth builds a pocket's tool, and only zero and the band about it refuse. So the depth's failing region is no longer a half-line below zero. It is a point, a few ε wide.

`crates/viewer/src/bounds.rs`'s probe (`BoundsProbe`, `reach_offset`: `seed · 2^doubling` outward from the field's value) finds a failure only where a reach sample lands inside it. A doubling ladder lands inside an ε-wide point only by arithmetic coincidence, so for most seeds it steps over the floor and reports `Bound::Open` far below zero. The module header concedes this for "an invalid sliver narrower than the sampling stride"; an extrude's depth is now a field whose only failure is such a sliver. The CI evidence on DOCTAIL's PR: the 0.8 mm seed from 8 mm reads `low: Open { probed: -1.63 }`, in metres.

`crates/viewer/tests/valid_range.rs`, `the_session_probes_a_real_slots_range`, still probes the extrude's depth and passes only because 8 mm stepped by 1 mm lands on 0 exactly (8 = 2³). Its doc says so. The two rows that assumed a half-line, `every_seed_brackets_the_half_line_floor` and `a_millimetre_parameter_is_probed_at_millimetre_scale`, moved to a disc's radius, whose `NonpositiveCircleRadius` region is still a half-line.

## Owed

The probe answers for a point floor, or says it cannot. Options the probe's owner chooses between:

- Probe the field's known refusing points as samples of their own: a magnitude slot that refuses at zero (an extrude's depth, and its siblings named in `work/verdict/coincidence-zone-priced-budget-at-the-floor.md`) has its floor at a value the slot's kind names, so the ladder can place a sample at zero whenever a reach crosses it.
- Detect the crossing: a reach that crosses zero on a slot whose sign flips the body (the decision log's `extrusion_normal_component` changes sign between two valid samples) brackets there, as `editor_core::range`'s `DecisionFlip` does.
- Say so: the Open sentence names the stride it sampled at, which is `the-range-panels-open-sentence-claims-values-it-never-sampled`'s half.

Then `the_session_probes_a_real_slots_range` should bracket from a seed that does not divide its value.
