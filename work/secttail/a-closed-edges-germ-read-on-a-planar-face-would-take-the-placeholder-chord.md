---
id: a-closed-edges-germ-read-on-a-planar-face-would-take-the-placeholder-chord
kind: issue
title: A closed edge's germ attributed to a planar face would give the planar partner the plane x plane lane, whose self-loop chord is the placeholder circle
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [5159]
---



Found by PR 5159 (the blind spot its sweep disclosed), filed on its fix
pass (branch `paths/in-face-section-self-loop`).

## What

A closed edge (one vertex, both its ends) lying in the partner's face
joins as one segment (`boolean/join.rs` `one_site_segments`). The
edge's own side copies the edge whole in every lane (`chord_join.rs`
`closed_edge_spec`). The partner's side takes the lane of the germ's
face pair. In every pose measured the germ's face is the closed edge's
curved wall (a boss's cylinder, a tube's, a ball's sphere), so a planar
partner takes the plane × wall lane and its chord is the section conic.

If the germ were attributed to a planar face of the edge's solid (a
boss's floor, which the rim also bounds), the planar partner would take
the plane × plane lane. There a self-loop chord is a lone site and
rides `mef`'s placeholder circle (`chord_spec`'s guard,
`EdgeCurveSpec::self_loop_circle_at`), which bounds nothing: the
partner's face would be divided by a chord that is not the rim.

## Measured

Unreached: the rim's germ face is the wall in every pose tried, in both
operand orders (the witness file
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs` and both
reviews' probes, several hundred op-level runs). The two-arc rim makes
the same assumption, and there it would mint straight chords.

## What a fix has to supply

Either a reason the germ of an edge lying in a face is always read on
its curved face (and an assertion a bug could break), or a lane for the
planar attribution: the partner's chord copies the closed edge, as the
edge's own side does.

## Moved to SECTTAIL (2026-10-25)

From SECT, cut on its priority seam (SECT measured 46 budget points against 31) in a chat-approved refold. Read SECT's step 3 first: SECT's plan says several of these may close by retirement then.
