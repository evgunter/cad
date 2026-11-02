---
id: a-torus-pairs-closed-form-circles-take-no-join-lane
kind: issue
title: A torus pair's closed-form circles take no join lane: carried as they are, the coaxial torus x torus union fails certification
status: open
opened: 2026-11-02
priority: P2
cost: M
refs: [the-carried-lane-chords-only-spline-components]
---


Filed by the cylinder × cylinder stage 3b-i lane (branch
`sectarms/cylinder-carried-closed-form`, 2026-11-02).

## What

The carried lane now chords a closed-form component (a line, a circle or
an ellipse) as its own carrier (`chord_join::carried_chord_spec`). The
join admits one only on a pair with no torus
(`topo::boolean::join::chord_spec_carries`). A torus pair's closed forms
are the coaxial parallels of sphere × torus, cylinder × torus and
torus × torus (`geom_brep::pair_section::torus`, `ClassCarrier::Closed`).
They keep the route they had: no kind arm, so
`CurvedBooleanUnsupported` at the join.

Admitted as they are, they do not build. Measured on
`sweep/tests/germ_circle_torus.rs::a_coaxial_seam_on_the_torus_is_placed_by_its_arc`:
- **before:** A ∪ B and B ∪ A refuse `CurvedBooleanUnsupported { kind: Torus }`;
- **with the torus gate removed:** A ∪ B refuses
  `Join(BooleanJoin(Euler(Certification { error: StatedInterval { refused: Winding(Exceeded) } })))`.

So a closed-form chord on a torus face reaches the Euler layer and fails
its winding certification. The likely place is the stated interval of a
torus circle arc, either `ring_path::Quadric::torus_circle_arc` or the
edge certifier's winding budget on a torus. That is a guess, not
measured.

## What would close it

- Find which certification refuses: the chord's edge or the island's
  winding.
- Either make the torus arc read, or keep the gate and give the pair a
  typed refusal that names the lane it lacks.
- Then drop the torus clause in `chord_spec_carries`.
