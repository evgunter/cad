---
id: the-general-pose-torus-germ-unions-that-build-have-no-volume-truth
kind: issue
title: The general-pose torus germ unions that build since the torus chart are held to no volume of their own
status: open
opened: 2026-11-10
priority: P2
cost: M
---


Filed by the rotational lane's stage 3 (`spiricarm/rotational-lane-torus`), a
disclosed gap of its own.

## What

Once every trimmed torus face reads through the rotational lane, these
general-pose unions build where they stopped at the result gate's
volume, and their rows are re-pinned to a seamed body only:

- `crates/sweep/tests/germ_torus_components.rs`: the drum at placements
  0, 1, 2, 4 and 5, the ball at placement 5, and the two donuts at
  placement 2, each in both member orders;
- `crates/sweep/tests/germ_sphere_torus_placements.rs`
  `a_ball_against_a_donut_carries_onto_the_torus_and_builds`: four
  unions of a ball and a donut.

A body that builds passed tier 3, its volume included, but nothing holds
that volume to a truth computed apart from the lane, and a wrong torus
term would read as an improvement. The donut against a rod in its
symmetric poses is held to a slicing oracle
(`crates/sweep/tests/common/donut_slices.rs`); these poses are general
(a turned donut, a tilted drum), where a line meets the torus at a
quartic's roots.

## Done when

Each of these bodies is held to an independent volume: a slicing oracle
that reads a line against a turned torus (the quartic's real roots,
bracketed and bisected) and against a ball or a drum, or a closed form
where one exists. Inclusion–exclusion over the operands is not
admissible.
